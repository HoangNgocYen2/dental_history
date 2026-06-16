# dental_history

## Project Title
dental_history

## Project Description
dental_history is a Soroban smart contract that keeps an on-chain, append-only log of dental visits. Each visit is signed by the dentist who performed it and contains the procedure, the affected tooth, and the visit date. Patients are the ultimate owners of their dental record and explicitly grant or revoke access to individual dentists - no dental office can read or extend a record without the patient's authorization. The contract focuses exclusively on dental care, so it is narrower than a general medical record and avoids carrying unrelated health data on-chain.

## Project Vision
Give patients a portable, verifiable, censorship-resistant dental history that follows them across clinics, cities, and countries. The long-term goal is to make "I lost my chart" a thing of the past, reduce redundant X-rays and repeat procedures, and let patients prove prior treatments to a new dentist in seconds instead of weeks.

## Key Features
- **Patient-owned access control** - the patient, not the clinic, decides which dentist address can read or append to their record. `grant_access` and `revoke_access` are gated by `require_auth` on the patient.
- **Dentist-signed visits** - every entry on the ledger is attested by the performing dentist's signature, creating a tamper-evident trail of who did what and when.
- **Append-only history** - existing visits are never edited or deleted, so the chain of care remains auditable. Revoking a dentist only blocks future writes from that address.
- **Per-tooth granularity** - each visit is tagged with a `tooth` symbol (e.g. upper-right third tooth) and a `procedure` symbol, so dentists can quickly review work on a specific tooth across providers.
- **Cheap, fast on-chain storage** - Soroban's instance storage and ~5 second finality mean a new visit costs a fraction of a cent and is queryable immediately.

## Contract

- **Network:** Stellar Testnet (Public)
- **Scope:** healthcare dApp — see `contracts/dental_history/src/lib.rs` for the full dental_history business logic.
- **Functions exposed:** see `Key Features` above and the `pub fn` list in `lib.rs`.
- **Contract ID:** CCUM5IH73ZSOLG42VQ36APKL2NGJ3B4FYQIRQMGVB27I3M5AYJBOZPC4
- **Explorer template:** https://stellar.expert/explorer/testnet/tx/cff706964cca95e8c1286f1f24227af1a87d07a99224ee16317609d4a6d4f628
- **Screenshot of deployed contract on Stellar Expert:**
![screenshot](https://ibb.co/1GnLm9r9)


## Future Scope
- Attach encrypted clinical notes or X-ray image hashes (IPFS CID) to each visit while keeping the on-chain payload small.
- Add a time-bounded access grant (e.g. "valid for 30 days") so patients can share history with a new dentist during a consultation window.
- Support dentist-issued treatment plans and follow-up reminders, with the patient able to accept or decline.
- Build a minimal frontend that connects with Freighter, lets a patient manage access, and lets a clinic sign and view visits by `visit_count` and `get_visit`.
- Add unit tests covering unauthorized writes, revoked access, and out-of-bounds `get_visit` indices, then expand to an integration test suite.

## Profile

- **Name:** <!-- Fill github name -->
- **Project:** `dental_history` (healthcare)
- **Built with:** Soroban SDK 25, Rust, Stellar Testnet
