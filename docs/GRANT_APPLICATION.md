# ZKELLA — SCF Grant Application

**Award type:** Build Award  
**Requested amount:** $130,000  
**Track:** Open  
**Duration:** 8 months

---

## Project Name

**ZKELLA Protocol** — ZK Confidential Finance Infrastructure for Stellar Soroban

---

## One-Line Description

The first production-ready confidential token standard for Soroban, with auditor viewing keys and FATF Travel Rule compliance, built on Stellar's Protocol 25 ZK primitives.

---

## Problem Statement

Stellar is transparent by default. Every transaction — amounts, asset types, counterparties — is visible to anyone on the network. For most use cases this is fine. For a growing segment of institutional users, it is a blocker.

Payment processors, stablecoin issuers, and regulated treasury operations cannot expose transaction amounts and counterparty identities publicly. Competitors learn their order flow. Clients expect financial confidentiality. Regulators require selective disclosure — not full public transparency.

Today these users have three options:
1. Accept full transparency and operate with reduced confidentiality
2. Route activity to other networks (Ethereum/Aztec, Cosmos/Penumbra)
3. Don't build on Stellar at all

None of these outcomes are good for the Stellar ecosystem.

Protocol 25 (X-Ray, January 2026) changed what is technically possible on Stellar by adding BN254 elliptic curve host functions, Poseidon2 hashing, and Groth16 zk-SNARK verification to Soroban. The cryptographic primitives are now available. What does not exist is production-grade infrastructure built on top of them.

The Nethermind `stellar-private-payments` prototype demonstrated feasibility but has documented limitations: a single transfer circuit, no multi-asset support, no security audit, a 7-day RPC event retention problem that breaks new user onboarding, no viewing keys for compliance, and no developer SDK. It is explicitly not for production use.

ZKELLA closes this gap.

---

## Solution

ZKELLA is a suite of five open-source, production-ready components that together form a complete confidential finance infrastructure for Soroban:

**1. CT-20 Confidential Token Standard**  
Any Stellar asset (USDC, XLM, any SEP-41 token) can be wrapped into a CT-20 shielded token. Balances are stored as Pedersen commitments. Transfers require a Groth16 range proof. Amounts are never revealed on-chain. The standard supports multiple input/output configurations (2x2 and 4x4) and multi-asset management.

**2. Auditor Viewing Key System**  
Each account has a spending key (private) and a viewing key (shareable). Viewing key holders can decrypt transaction history without spending capability. The system includes a compliance API for regulated institutions to verify transaction history and a ZK proof-of-non-sanctions endpoint compatible with FATF Travel Rule requirements.

**3. Persistent State Manager**  
Stellar RPC nodes retain events for only 7 days. ZKELLA ships a lightweight indexer that stores encrypted note commitments indefinitely, a WASM client library for state reconstruction, and an encrypted backup format. This component becomes shared infrastructure for every privacy project on Stellar.

**4. Shielded Swap Primitive**  
The first private swap interface on Stellar. Users commit to an encrypted swap intent; a relayer executes via the Stellar DEX; a ZK fairness proof confirms the execution honoured the user's slippage tolerance. Output tokens arrive as shielded notes. No new AMM required — it wraps the existing Stellar DEX.

**5. Developer SDK and Reference Application**  
A TypeScript package (`@zkella/sdk`) that abstracts all ZK complexity: WASM proof generation, key management, note selection, indexer sync, Soroban calls. A reference wallet application demonstrating the full flow from shield to private transfer to unshield.

---

## Why ZKELLA is Original

| Existing Project | What It Does | What It Lacks vs ZKELLA |
|---|---|---|
| Nethermind stellar-private-payments | P2P private transfers, single circuit | No production use, no compliance layer, no SDK, no multi-asset |
| LumenShade (SCF #37) | Privacy pool mixer | No token standard, no viewing keys |
| BlindPay (SCF #33) | Private payment UX | No developer infrastructure |
| Confidential Transfers (SCF #40) | Tooling prototype | No compliance layer, no swap primitive |

ZKELLA is the first project to combine: a token standard, compliance tooling with viewing keys, persistent state management beyond the 7-day window, a shielded swap primitive, and a developer SDK — in a single auditable, production-ready stack.

---

## Alignment with SCF Priorities

SDF's stated 2026 privacy strategy calls for: "open and transparent by default, opt-in and configurable at the application layer, compliance-ready from the start." ZKELLA is the direct technical implementation of that strategy.

SDF is a member of the Confidential Token Association alongside OpenZeppelin, Zama, and Inco. ZKELLA builds exactly the kind of confidential token standard that membership signals SDF wants in the ecosystem.

The SCF Soroban Audit Bank will be applied to separately for the security audit. The $130,000 requested here is development cost only.

---

## Milestones and Budget

| Milestone | Deliverable | Amount | Timeline |
|---|---|---|---|
| M1 | Circom circuits (2x2 + 4x4 transfer, shield, unshield) on testnet | $13,000 | Month 1–2 |
| M2 | CT-20 multi-asset standard + viewing key system + auditor API | $26,000 | Month 2–4 |
| M3 | Persistent state manager + shielded swap primitive + FATF Travel Rule layer | $39,000 | Month 4–6 |
| M4 | TypeScript SDK + reference wallet + full documentation + mainnet deployment | $52,000 | Month 6–8 |
| **Total** | | **$130,000** | **8 months** |

### Budget Justification

The engineering work requires two specialists:
- A ZK engineer (Circom circuit design, Groth16 trusted setup, cryptographic protocol design): market rate $130K–$180K/year
- A Soroban/TypeScript engineer (Rust contracts, SDK, indexer): market rate $100K–$140K/year

Eight months of this team at market rate: $154K–$213K. The $130K budget represents a 16–39% discount, accepted in exchange for the grant's ecosystem visibility and open-source positioning.

**Comparable:** Nethermind's `stellar-private-payments` PoC was produced at their published €1,200–€1,800/engineer/day rates. A comparable engagement (3 engineers, 3 months) costs $162K–$243K. ZKELLA delivers a more complete system for less.

**Ecosystem leverage:** If 10 projects build on the CT-20 standard over 24 months, each saving ~400 hours of ZK engineering (~$50K at market), the ecosystem value unlocked exceeds $500K — a 3.8x return on the $130K investment.

---

## Team

**Mohamed Frihat** — Protocol architect and lead developer. Stellar ecosystem contributor. Author of the stellar-programmable-finance smart account system (multi-plane signer model, intent registry, policy engine, condition verifier). Deep experience in Soroban contract development, DeFi protocol design, and ZK application development.

Repository: `https://github.com/Frihat-dev`

---

## Technical Specification

Full technical specification available at: `docs/TECHNICAL_SPEC.md`  
Circuit specification: `docs/CIRCUIT_SPEC.md`  
Integration guide: `docs/INTEGRATION_GUIDE.md`  
Audit scope: `docs/AUDIT_SCOPE.md`

---

## Open Source Commitment

All code is released under the Apache 2.0 license. Circuit parameters, trusted setup artifacts, and verifying keys are published publicly. The CT-20 standard is designed as a public good — any developer can build on it without permission or licensing fees.

---

## Risk Mitigation

| Risk | Mitigation |
|---|---|
| Soroban resource limits block proof verification | BN254 host functions are native precompiles; verified feasible in Nethermind prototype |
| Trusted setup compromise | Multi-party ceremony with ≥10 contributors; final beacon from Stellar ledger hash |
| Indexer centralization | Open-source indexer anyone can run; encrypted backup for user self-custody |
| Relayer censorship in swap | Multiple competing relayers; time-expiry + cancel path for users |
| Audit findings delay mainnet | Audit scoped and funded separately; M3 completion leaves 2 months buffer |

---

## Success Metrics (12 months post-mainnet)

- ≥5 independent projects integrating CT-20 standard
- ≥3 independent indexer operators
- ≥1 regulated institution pilot using the viewing key compliance layer
- ≥$1M in shielded token value on mainnet
- SDK published with ≥500 weekly npm downloads

---

*ZKELLA Grant Application — Stellar Community Fund Build Award*
