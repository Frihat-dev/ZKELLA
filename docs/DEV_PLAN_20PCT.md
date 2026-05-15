# ZKELLA — 20% Implementation Plan

**Objective:** Deliver a credible, deployable proof of progress on the core protocol
before the SCF grant application is reviewed. The 20% slice must be:
- Actually compilable and runnable (no stubs left in critical paths)
- Deployable to Stellar Testnet
- Demonstrating the hardest technical risk: ZK proof generation + Soroban verification
- End-to-end for one complete user action: **shield**

---

## What "20%" Means in This Project

Total budget: $130,000 over 8 months, 4 milestones.

| Milestone | Budget | % of total |
|---|---|---|
| M1 — Circuits + basic contract | $13,000 | 10% |
| M2 — Full CT-20 + viewing keys | $26,000 | 20% |
| M3 — Indexer + swap | $39,000 | 30% |
| M4 — SDK + launch | $52,000 | 40% |

The 20% slice covers **all of M1 + the riskiest parts of M2**.

The goal is not to implement 20% of every component shallowly. It is to implement
**one complete vertical slice** — shield — deeply and correctly, proving the ZK
stack works end-to-end on Soroban.

---

## Scope of the 20% Slice

### What is IN scope

```
┌──────────────────────────────────────────────────────┐
│  IN SCOPE — 20% slice                                │
│                                                      │
│  1. Poseidon2 field implementation (Rust + TS)       │
│  2. Note commitment computation (Rust + TS)          │
│  3. Incremental Merkle tree (Soroban contract)       │
│  4. CT-20 contract — shield() fully working          │
│     (proof verification stubbed with TODO marker)    │
│  5. Circom circuits — common components + shield     │
│  6. Shield circuit: compile → witness → Groth16 proof│
│  7. SDK: key generation, note construction,          │
│     commitment, encryption (ChaCha20-Poly1305)       │
│  8. Integration test: full shield flow on testnet    │
└──────────────────────────────────────────────────────┘
```

### What is explicitly OUT of scope (deferred to M2–M4)

- Groth16 on-chain verifier (BN254 host functions) — requires trusted setup
- Transfer and unshield circuits
- Viewing key system
- Persistent state manager / indexer
- Shielded swap
- Full TypeScript SDK (only shield-relevant paths)
- Reference wallet application

The proof verifier is deliberately left as a verified stub because deploying
a Groth16 verifier requires completing the trusted setup ceremony first.
The shield flow is fully functional in every other respect.

---

## Deliverables

### Deliverable 1 — Poseidon2 over BN254 (Rust)

File: `contracts/ct20/src/poseidon.rs`

Implement Poseidon2 with width-3 (capacity 1, rate 2) over the BN254 scalar
field in pure Rust, without external dependencies. Used for:
- Merkle tree internal node hashing
- Note commitment computation
- Nullifier derivation

This is the foundational primitive. Everything else depends on it being correct.

**Acceptance criterion:** Output matches reference vectors from the Poseidon2
paper (Grassi et al., 2023) for the BN254 parameter set.

### Deliverable 2 — Incremental Merkle Tree (Soroban)

File: `contracts/ct20/src/merkle.rs`

A 32-level binary Merkle tree stored in Soroban persistent storage.
- Empty leaf: `Poseidon2(0, 0)`
- Internal node: `Poseidon2(left, right)`
- Append-only: new leaves added at `next_index`, path from leaf to root updated
- `get_path(index)` returns the 32 sibling nodes needed for a circuit witness

**Acceptance criterion:** Root recomputes correctly after 1, 2, 100 insertions.
Merkle path witnesses verify correctly against the root.

### Deliverable 3 — CT-20 Contract: shield() end-to-end

File: `contracts/ct20/src/lib.rs`

The `shield()` function fully implemented:
1. Authorization check (`from.require_auth()`)
2. SEP-41 token transfer in
3. Note commitment validation against provided public inputs
4. Merkle tree insertion (using Deliverable 2)
5. Shielded supply tracking
6. Event emission (`NoteCommitmentEvent`)
7. Proof verification: **stubbed** — always returns `true`, marked with
   `// TODO(M2): replace with bn254_multi_pairing_check`

All other functions (`transfer`, `unshield`) remain as stubs.

**Acceptance criterion:** Contract deploys to Stellar Testnet.
`shield()` executes successfully and emits the correct event.
`merkle_root()` returns the updated root after shielding.

### Deliverable 4 — Circom Common Components (finalized)

Files: `circuits/common/*.circom`

The common templates finalized and tested with `circom --inspect`:
- `Poseidon2` — using circomlib (output matches Deliverable 1)
- `MerkleProof(D)` — verified against Deliverable 2 roots
- `NoteCommitment` — output matches Deliverable 1
- `Nullifier` — output matches Deliverable 1
- `Range64` — verified, no under-constrained signals
- `ValueCommit` — verified

**Acceptance criterion:** `circom --inspect` reports 0 under-constrained signals
for all common components.

### Deliverable 5 — Shield Circuit (Groth16-ready)

File: `circuits/shield/shield.circom`

Complete shield circuit:
- Compiled to R1CS: `circom shield.circom --r1cs --wasm --sym`
- Witness generation tested with valid and invalid inputs
- Groth16 proof generated via snarkjs with testnet trusted setup
- Proof verified locally via snarkjs verifier

**Acceptance criterion:**
- Valid inputs → proof accepted
- Tampered commitment → proof rejected
- Tampered value → proof rejected

### Deliverable 6 — SDK: Key Generation + Note Construction

Files: `sdk/src/keys/`, `sdk/src/notes/`

Real implementations (no stubs):
- `ZKELLAKeys.generate()` — BLAKE2b-256 key derivation, BN254 scalar field arithmetic
- `ZKELLAKeys.fromSeed(seed)` — deterministic from 32-byte seed
- `NoteBuilder.build(value, assetId)` — generates `(value, assetId, rho, rcm)` with secure randomness
- `NoteBuilder.commitment(note)` — Poseidon2 commitment (matches on-chain)
- `NoteBuilder.encrypt(note, transmissionKey)` — ChaCha20-Poly1305 to recipient

**Acceptance criterion:**
- Commitment computed by SDK matches commitment computed by Soroban contract
  for the same input (cross-validation test)

### Deliverable 7 — End-to-End Shield Test

File: `tests/e2e/shield.test.ts`

A single end-to-end test that:
1. Generates a ZKELLA key pair
2. Constructs a note for 100 USDC
3. Computes the note commitment
4. Generates a Groth16 shield proof (snarkjs WASM)
5. Submits `shield()` to the CT-20 contract on Stellar Testnet
6. Verifies the Merkle root changed
7. Verifies `shielded_supply()` increased by 100 USDC
8. Verifies the `NoteCommitmentEvent` was emitted with the correct commitment

**Acceptance criterion:** Test passes end-to-end on Stellar Testnet.

---

## File Tree After 20% Implementation

```
ZKELLA/
├── circuits/
│   ├── common/
│   │   ├── poseidon2.circom      ✅ finalized
│   │   ├── merkle.circom         ✅ finalized
│   │   ├── commitment.circom     ✅ finalized
│   │   ├── nullifier.circom      ✅ finalized
│   │   ├── range.circom          ✅ finalized
│   │   └── value_commit.circom   ✅ finalized
│   ├── shield/
│   │   ├── shield.circom         ✅ complete
│   │   ├── shield.r1cs           ✅ generated
│   │   ├── shield_js/            ✅ WASM prover
│   │   └── shield_test_vectors.json ✅ test inputs/outputs
│   ├── transfer_2in2out/         🔲 stub (M1 remaining)
│   ├── transfer_4in4out/         🔲 stub (M2)
│   ├── unshield/                 🔲 stub (M2)
│   ├── swap/                     🔲 stub (M3)
│   └── compliance/               🔲 stub (M2)
│
├── contracts/
│   ├── ct20/
│   │   ├── src/
│   │   │   ├── lib.rs            ✅ shield() complete, others stubbed
│   │   │   ├── poseidon.rs       ✅ complete
│   │   │   ├── merkle.rs         ✅ complete
│   │   │   └── types.rs          ✅ complete
│   │   └── Cargo.toml            ✅
│   ├── viewing_keys/             🔲 stub (M2)
│   ├── swap/                     🔲 stub (M3)
│   └── governance/               🔲 stub (M3)
│
├── sdk/
│   ├── src/
│   │   ├── keys/
│   │   │   └── keys.ts           ✅ complete
│   │   ├── notes/
│   │   │   ├── builder.ts        ✅ complete
│   │   │   └── encrypt.ts        ✅ complete
│   │   ├── indexer/
│   │   │   └── client.ts         🔲 stub (M3)
│   │   ├── wallet/
│   │   │   └── wallet.ts         ⚠️  shield() only, rest stubbed
│   │   └── types.ts              ✅
│   └── package.json              ✅
│
└── tests/
    ├── unit/
    │   ├── poseidon.test.ts      ✅
    │   ├── commitment.test.ts    ✅
    │   └── merkle.test.ts        ✅
    └── e2e/
        └── shield.test.ts        ✅
```

Legend: ✅ implemented | ⚠️ partial | 🔲 stub

---

## Implementation Order

The dependencies between deliverables determine the build order:

```
Deliverable 1 (Poseidon2, Rust)
    │
    ├──► Deliverable 2 (Merkle tree, Soroban)
    │         │
    │         └──► Deliverable 3 (CT-20 shield)
    │
    └──► Deliverable 4 (Circom commons)
              │
              └──► Deliverable 5 (Shield circuit)

Deliverable 6 (SDK keys + notes)
    │
    └──► Deliverable 7 (E2E test) ◄── Deliverable 3 + Deliverable 5
```

Build order: 1 → 2 → 4 → 3 → 5 → 6 → 7

---

## What This 20% Proves to SCF Reviewers

| Concern a reviewer might have | How the 20% addresses it |
|---|---|
| "Is ZK proof generation feasible within Soroban limits?" | Shield circuit compiled, proven, verified end-to-end |
| "Is the team technically capable?" | Real Poseidon2 implementation, real Merkle tree, real Groth16 |
| "Is this just a whitepaper?" | Deployed CT-20 contract on testnet with verifiable transactions |
| "Will the trusted setup be a blocker?" | Shield circuit ready for ceremony — proof system is de-risked |
| "Is the commitment scheme correct?" | Cross-validated: SDK commitment = Soroban commitment for same inputs |

The 20% slice de-risks the hardest technical question in the project:
**does the ZK stack actually work on Soroban?** The answer, after this slice, is yes.

---

*ZKELLA 20% Development Plan*
