#![no_std]

//! # dental_history
//!
//! On-chain dental visit log. Dentists add visit records
//! (procedure, date, tooth) for a patient; the patient grants or
//! revokes access for any dentist that may read or extend their
//! history. Distinct from a general medical record: scope is
//! limited to dental procedures per tooth, with patient-controlled
//! access delegation.

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, Vec};

/// Visit record stored on-chain for each dental appointment.
#[contracttype]
#[derive(Clone)]
pub struct Visit {
    /// Address of the dentist who performed the procedure.
    pub dentist: Address,
    /// Short code for the procedure (e.g. "FILLING", "EXTRACT", "CLEAN").
    pub procedure: Symbol,
    /// Tooth identifier (e.g. "UR-3" for upper-right third tooth).
    pub tooth: Symbol,
    /// Human-supplied date for the visit (e.g. 20260504).
    pub date: u64,
    /// Ledger timestamp at which the record was written.
    pub timestamp: u64,
}

/// Storage keys used by the contract.
#[contracttype]
pub enum DataKey {
    /// Vector of visits belonging to a patient.
    Visits(Address),
    /// Access flag: true when a patient has authorized a dentist.
    Access(Address, Address),
}

#[contract]
pub struct DentalHistory;

#[contractimpl]
impl DentalHistory {
    /// Record a new dental visit for `patient` performed by `dentist`.
    ///
    /// The dentist must authorize the transaction, and the patient
    /// must have previously granted access to that dentist (see
    /// `grant_access`). Returns the updated total visit count for
    /// the patient.
    pub fn add_visit(
        env: Env,
        dentist: Address,
        patient: Address,
        procedure: Symbol,
        tooth: Symbol,
        date: u64,
    ) -> u32 {
        // The dentist signs the write: they attest to having
        // performed the procedure.
        dentist.require_auth();

        // The patient must have authorized this dentist to act on
        // their dental record.
        let access_key = DataKey::Access(patient.clone(), dentist.clone());
        let granted: bool = env
            .storage()
            .instance()
            .get(&access_key)
            .unwrap_or(false);
        if !granted {
            panic!("dentist is not authorized by patient");
        }

        let visits_key = DataKey::Visits(patient.clone());
        let mut visits: Vec<Visit> = env
            .storage()
            .instance()
            .get(&visits_key)
            .unwrap_or_else(|| Vec::new(&env));

        let visit = Visit {
            dentist: dentist.clone(),
            procedure,
            tooth,
            date,
            timestamp: env.ledger().timestamp(),
        };
        visits.push_back(visit);
        env.storage().instance().set(&visits_key, &visits);

        visits.len()
    }

    /// Patient grants `new_dentist` permission to read and append
    /// to their dental history.
    pub fn grant_access(env: Env, patient: Address, new_dentist: Address) {
        patient.require_auth();
        let key = DataKey::Access(patient, new_dentist);
        env.storage().instance().set(&key, &true);
    }

    /// Patient revokes a dentist's access to their dental history.
    /// Existing visit records are preserved; only future writes
    /// from that dentist are blocked.
    pub fn revoke_access(env: Env, patient: Address, dentist: Address) {
        patient.require_auth();
        let key = DataKey::Access(patient, dentist);
        env.storage().instance().remove(&key);
    }

    /// Return the total number of visits recorded for a patient.
    pub fn visit_count(env: Env, patient: Address) -> u32 {
        let key = DataKey::Visits(patient);
        let visits: Vec<Visit> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        visits.len()
    }

    /// Return whether `dentist` currently has access to `patient`'s
    /// record.
    pub fn has_access(env: Env, patient: Address, dentist: Address) -> bool {
        let key = DataKey::Access(patient, dentist);
        env.storage().instance().get(&key).unwrap_or(false)
    }

    /// Return the visit at `index` for `patient`. Panics if the
    /// index is out of range.
    pub fn get_visit(env: Env, patient: Address, index: u32) -> Visit {
        let key = DataKey::Visits(patient);
        let visits: Vec<Visit> = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or_else(|| Vec::new(&env));
        visits
            .get(index)
            .unwrap_or_else(|| panic!("visit index out of bounds"))
    }
}
