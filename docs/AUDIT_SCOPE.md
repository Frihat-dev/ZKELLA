# ZKELLA — Audit Scope

**Version:** 0.1.0  
**Intended audience:** Security auditors, SCF Soroban Audit Bank

---

## 1. Overview

This document defines the scope, priorities, and context for a security audit of the ZKELLA Protocol. ZKELLA implements confidential token transfers and shielded swaps on Stellar Soroban using Groth16 zk-SNARKs over BN254.

An audit is required before mainnet deployment. ZKELLA intends to apply to the **SCF Soroban Audit Bank** for co-funding.

---

## 2. In-Scope Components

### 2.1 Circom Circuits (Highest Priority)

| File | Description | Risk Level |
|---|---|---|
| `circuits/shield/shield.circom` | Shield circuit | High |
| `circuits/unshield/unshield.circom` | Unshield circuit | High |
| `circuits/transfer_2in2out/transfer.circom` | 2x2 transfer circuit | Critical |
| `circuits/transfer_4in4out/transfer.circom` | 4x4 transfer circuit | Critical |
| `circuits/swap/swap_fairness.circom` | Swap fairness circuit | High |
| `circuits/compliance/non_membership.circom` | Sanctions proof circuit | Medium |
| `circuits/common/*.circom` | Shared components | High |

**Circuit-specific concerns:**
- Under-constrained signals (signals that are not fully constrained can allow proof forgery)
- Missing constraints on intermediate signals
- Incorrect nullifier derivation (could enable double-spend)
- Balance check bypass (out_values > in_values without detection)
- Range proof weaknesses (negative values represented as large field elements)
- Merkle proof verification correctness
- Signal assignment vs constraint distinction (`<--` vs `<==`)

### 2.2 Soroban Smart Contracts (Critical)

| File | Description | Risk Level |
|---|---|---|
| `contracts/ct20/src/lib.rs` | CT-20 token contract | Critical |
| `contracts/viewing_keys/src/lib.rs` | Viewing key registry | Medium |
| `contracts/swap/src/lib.rs` | Shielded swap contract | High |
| `contracts/governance/src/lib.rs` | Governance and timelock | High |

**Contract-specific concerns:**
- Groth16 verifier correctness on Soroban (BN254 host function usage)
- Verifying key loading and deserialization integrity
- Merkle tree insertion logic (off-by-one in leaf index, incorrect root recomputation)
- Nullifier set atomicity (concurrent transaction double-spend under Stellar's parallel execution)
- Anchor validation (accepting proofs against stale or invalid roots)
- Public input binding (ensuring proof public inputs match transaction parameters)
- Integer overflow/underflow in balance accounting
- Authorization checks (who can call pause, update VK, etc.)
- Governance timelock enforcement
- SEP-41 token transfer edge cases (fee-on-transfer tokens, rebasing tokens)
- Soroban storage TTL and rent handling

### 2.3 Indexer (Medium Priority)

| File | Description | Risk Level |
|---|---|---|
| `indexer/src/` | Note indexer service | Medium |

**Concerns:**
- Denial of service via malformed event data
- SQL injection in note queries
- Data integrity: ensuring stored commitments match on-chain values
- Authentication on write endpoints (indexer should be append-only from public input)

### 2.4 TypeScript SDK (Lower Priority — Client-Side)

| File | Description | Risk Level |
|---|---|---|
| `sdk/src/keys/` | Key generation and derivation | High |
| `sdk/src/notes/` | Note construction and encryption | High |
| `sdk/src/circuits/` | WASM proof generation | Medium |

**Concerns:**
- Entropy source for key generation (must use `crypto.getRandomValues`, not `Math.random`)
- Note encryption correctness (ECDH key agreement, ChaCha20-Poly1305)
- Note plaintext leakage via error messages or logs
- WASM circuit integrity (WASM files should be checksummed against published artifacts)
- Spending key never transmitted over network

---

## 3. Out of Scope

- Stellar core protocol and Soroban host environment internals
- Third-party dependencies (circomlib, snarkjs) — assumed reviewed upstream
- Reference wallet application UI/UX
- Relayer infrastructure and off-chain relay server
- Key management in user devices (hardware wallet integration)

---

## 4. Known Issues and Accepted Risks

The following are documented limitations that auditors should note but are accepted design decisions for v1:

1. **Relayer trust:** The shielded swap relayer learns swap parameters off-chain. This is accepted as a weak-privacy trade-off in v1.
2. **Trusted setup:** Groth16 requires a per-circuit trusted setup. The toxic waste risk is mitigated by the multi-party ceremony but not eliminated.
3. **Sorted Merkle tree for sanctions:** Non-membership proofs require a sorted Merkle tree maintained by the sanctions list publisher. The correctness of that list is out of scope.
4. **7-day RPC retention fallback:** The indexer is a liveness dependency — if all indexers go offline, users cannot construct Merkle paths. The encrypted backup mitigates this.
5. **Single asset per proof:** Transfer proofs are homogeneous (all inputs and outputs share one asset_id). Multi-asset atomic swaps are not supported in v1.

---

## 5. Critical Attack Scenarios

Auditors should specifically attempt to construct or demonstrate:

| Scenario | Affected Component |
|---|---|
| Forge a valid transfer proof without knowing input note secrets | Transfer circuits |
| Double-spend a nullifier across two transactions in the same ledger | CT-20 contract |
| Transfer more value than input notes contain | Transfer circuits + contract |
| Create a note with negative value | Range proof in all circuits |
| Bypass the Merkle root anchor validation | CT-20 contract |
| Substitute a malicious verifying key via governance | Governance contract |
| Drain the shielded pool via a malformed unshield transaction | Unshield circuit + contract |
| Manipulate the swap fairness proof to claim more output than received | Swap circuit |

---

## 6. Prior Art and References

Auditors are encouraged to review the following for context:

- Zcash Sapling circuit audit (EBSL, 2018)
- Tornado Cash contracts audit (ABDK, 2020)
- Aztec Connect audit (Consensys Diligence, 2022)
- Nethermind `stellar-private-payments` (unaudited reference implementation)
- Circom language specification and known vulnerability classes

---

## 7. Audit Deliverables Expected

- Full audit report with severity classifications (Critical / High / Medium / Low / Informational)
- Proof-of-concept for any Critical or High findings
- Recommendations for each finding
- Confirmation of fixes in final report revision

---

## 8. Contact and Repository

- Repository: `https://github.com/Frihat-dev/ZKELLA`
- Documentation: `docs/TECHNICAL_SPEC.md`, `docs/CIRCUIT_SPEC.md`
- Circuit artifacts: `circuits/*/` (`.circom`, `.r1cs`, `.wasm`, `.zkey`)
- Contract source: `contracts/*/src/`

---

*ZKELLA Audit Scope v0.1.0*
