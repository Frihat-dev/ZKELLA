# ZKELLA Protocol

**ZK-native confidential finance infrastructure for the Stellar Soroban ecosystem.**

ZKELLA delivers the first production-ready confidential token standard for Soroban — with auditor viewing keys, FATF Travel Rule compatibility, and a shielded swap primitive — giving Stellar the privacy infrastructure that regulated institutions require to operate on the network at scale.

---

## Overview

Stellar is transparent by default. Protocol 25 (X-Ray, January 2026) changed the underlying capability: BN254 elliptic curve host functions, Poseidon hash, and Groth16 zk-SNARK verification are now live on Soroban. The cryptographic primitives exist. What does not exist is production-grade infrastructure built on top of them.

ZKELLA fills that gap. It is a suite of open-source contracts, circuits, and developer tooling that any Soroban developer can use to add confidentiality to tokens, payments, and swaps — without rebuilding the ZK layer from scratch.

---

## Problem

### Transparency is a blocker for institutional adoption

Stellar processes $30B+ in quarterly payment volume. Its institutional users — payment processors, stablecoin issuers, regulated treasury operations — cannot expose transaction amounts and counterparty identities on a public ledger for all use cases. Today they either accept full transparency or route activity to other chains.

### Existing privacy work on Stellar is incomplete

The Nethermind `stellar-private-payments` prototype (February 2026) established proof of concept but has documented limitations:

- Single circuit only (2-input / 2-output), no multi-asset support
- No security audit — explicitly not for production use
- 7-day RPC event retention window breaks new user onboarding
- No viewing keys or selective disclosure for compliance
- No shielded DeFi — peer-to-peer transfers only
- No developer SDK

No existing SCF-funded project (LumenShade, BlindPay, Confidential Transfers SCF #40) has shipped a production-ready, compliance-forward confidential token standard. ZKELLA builds exactly that.

---

## Solution

ZKELLA is structured as five layered deliverables, each independently useful and collectively forming a complete privacy infrastructure stack.

### Deliverable 1 — CT-20 Confidential Token Standard

A Soroban token standard where balances are stored as Pedersen commitments. Transfers require a Groth16 range proof — proving the amount is valid and the sender has sufficient balance without revealing either value.

- Circom circuits: 2-input/2-output and 4-input/4-output configurations
- Multi-asset support: one contract handles multiple token types simultaneously
- Wrap any existing Stellar asset (USDC, XLM, any SEP-41 token) into a CT-20 shielded version
- Functions: `shield()`, `transfer()`, `unshield()`

### Deliverable 2 — Auditor Viewing Key System

The compliance layer no other Stellar privacy project has built.

- Each account generates a **spending key** (private) and a **viewing key** (shareable with auditors)
- Viewing key holders decrypt transaction history without spending capability
- Auditor API: regulated institutions verify counterparty transaction history on request
- Proof-of-compliance endpoint: ZK proof that an address is not on a sanctions list, using a Merkle inclusion/exclusion proof over a published sanctions list — without revealing the address
- FATF Travel Rule compatible: issuers satisfy compliance obligations without exposing counterparty data

### Deliverable 3 — Persistent State Manager

Solves the 7-day Stellar RPC event retention problem that breaks the Nethermind prototype for new users.

- Lightweight indexer node operators can run to store encrypted note commitments beyond the RPC window
- Client-side WASM library that reconstructs wallet state from the indexer
- Encrypted note bundle export as user-controlled backup fallback
- This component becomes shared infrastructure for every privacy project on Stellar

### Deliverable 4 — Shielded Swap Primitive

The first private swap interface on Stellar.

- Users swap Token A for Token B through the Stellar DEX without revealing amounts on-chain
- Commit-reveal scheme: user commits to an encrypted swap intent; a relayer executes it; a ZK proof confirms the execution was fair (correct price, no front-running)
- Wraps the existing Stellar DEX — no new AMM required, usable immediately
- Scope is the primitive; a full private AMM is the next phase

### Deliverable 5 — Developer SDK and Reference Application

- TypeScript SDK (`zkella-sdk`): circuit proving (WASM), key management, Soroban calls, and note indexer in a unified API
- Reference wallet application (web, open-source): shield, transfer, unshield, and viewing key export flows
- Full documentation: circuit specifications, API reference, security assumptions, integration guides

---

## Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    zkella-sdk (TypeScript)               │
│         Key mgmt · Proof generation · Note sync         │
└────────────────────────┬────────────────────────────────┘
                         │
         ┌───────────────┼───────────────┐
         ▼               ▼               ▼
  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
  │  CT-20      │ │  Viewing    │ │  Shielded   │
  │  Token      │ │  Key System │ │  Swap       │
  │  Contract   │ │  + Audit    │ │  Primitive  │
  └─────────────┘ └─────────────┘ └─────────────┘
         │               │               │
         └───────────────┼───────────────┘
                         ▼
              ┌─────────────────────┐
              │  Persistent State   │
              │  Manager / Indexer  │
              └─────────────────────┘
                         │
                         ▼
              ┌─────────────────────┐
              │  Soroban / BN254    │
              │  Groth16 Verifier   │
              │  (Protocol 25)      │
              └─────────────────────┘
```

---

## Technology Stack

| Layer | Technology |
|---|---|
| ZK proof system | Groth16 (via BN254 Soroban host functions) |
| Circuit language | Circom 2.0 |
| Hash function | Poseidon2 (native Soroban host function) |
| Commitment scheme | Pedersen commitments over BN254 |
| Smart contracts | Rust / Soroban SDK |
| Client proving | WASM (snarkjs) |
| SDK | TypeScript |
| Reference app | React + Stellar Wallets Kit |

---

## Ecosystem Landscape

| Project | Status | Scope | Gap vs ZKELLA |
|---|---|---|---|
| Nethermind stellar-private-payments | PoC, unaudited | P2P transfers, single circuit | No compliance, no multi-asset, no SDK |
| LumenShade (SCF #37) | Funded | Privacy pool mixer | No token standard, no viewing keys |
| BlindPay (SCF #33) | Funded | Payment privacy | No developer infrastructure |
| Confidential Transfers (SCF #40) | Funded | Tooling prototype | No compliance layer, no swap |
| ZkVM / Slingshot (2019) | Archived | Bulletproofs VM | Never shipped to production |

ZKELLA is the first project to combine a token standard, compliance tooling, persistent state management, shielded swaps, and a developer SDK into a single auditable, production-ready stack.

---

## Grant Budget — $130,000

Audit costs are submitted separately to the SCF Soroban Audit Bank and are not included here.

| Milestone | Amount | Deliverable | Timeline |
|---|---|---|---|
| M1 — Foundation | $13,000 | Circom circuits (2+4 input/output) + basic CT-20 Soroban contract on testnet | Month 1–2 |
| M2 — Core Standard | $26,000 | Full multi-asset CT-20 standard + viewing key system + auditor API | Month 2–4 |
| M3 — Infrastructure | $39,000 | Persistent state manager + shielded swap primitive + FATF Travel Rule layer | Month 4–6 |
| M4 — SDK + Launch | $52,000 | TypeScript SDK + reference app + full documentation + mainnet deploy | Month 6–8 |
| **Total** | **$130,000** | | **8 months** |

### Engineering Cost Justification

| Role | Market Rate | Duration | Cost |
|---|---|---|---|
| ZK Engineer (Circom, cryptography, Groth16) | $130K–$180K/year | 8 months | $87K–$120K |
| Soroban / TypeScript Engineer | $100K–$140K/year | 8 months | $67K–$93K |
| Total at market | | | $154K–$213K |

$130K for this scope is below market — the team accepts a discount in exchange for the grant visibility and open-source ecosystem positioning.

**Comparable:** Nethermind's `stellar-private-payments` PoC — less complete, no compliance layer, no SDK — was produced at Nethermind's published day rates of €1,200–€1,800/engineer/day. A comparable engagement would cost $162K–$243K minimum. ZKELLA delivers more for less.

**Ecosystem leverage:** If 10 projects build on the CT-20 standard over 24 months, each saving ~400 hours of ZK + contract work (~$50K at market), the ecosystem engineering value unlocked exceeds $500K — a 3.8x return on the $130K investment.

---

## Why Fund This Now

Protocol 25 (X-Ray) shipped in January 2026. There is a 12–18 month window in which Stellar can establish itself as the leading compliant privacy chain before other L1s and L2s react. If no production-grade privacy infrastructure ships in that window, institutional users with privacy requirements route to Ethereum (Aztec) or Cosmos (Penumbra), and developers building private payment applications follow.

$130K now captures that window. The cost of not funding it is measured in lost developer and institutional activity — far exceeding $130K.

---

## Roadmap

### Phase 1 — Foundation (Months 1–2)
- [ ] Circom circuit design: 2-input/2-output transfer
- [ ] Circom circuit design: 4-input/4-output transfer
- [ ] Trusted setup ceremony (Groth16)
- [ ] Basic CT-20 Soroban contract: `shield()`, `transfer()`, `unshield()`
- [ ] Testnet deployment and smoke tests

### Phase 2 — Core Standard (Months 2–4)
- [ ] Multi-asset support in CT-20 contract
- [ ] Spending key + viewing key derivation scheme
- [ ] Auditor API: decrypt history with viewing key
- [ ] Proof-of-compliance endpoint (Merkle non-membership proof)
- [ ] FATF Travel Rule compatibility layer
- [ ] Unit and integration test suite

### Phase 3 — Infrastructure (Months 4–6)
- [ ] Persistent note indexer (Go/Rust node)
- [ ] Client-side WASM state reconstruction library
- [ ] Encrypted note bundle export/import
- [ ] Shielded swap commit-reveal circuit
- [ ] Relayer contract + fairness proof
- [ ] Stellar DEX integration

### Phase 4 — SDK and Launch (Months 6–8)
- [ ] `zkella-sdk` TypeScript package (npm)
- [ ] Reference wallet application
- [ ] Security review preparation and documentation
- [ ] Mainnet deployment
- [ ] Integration guides, circuit specifications, API reference
- [ ] Developer onboarding workshop

---

## Repository Structure

```
ZKELLA/
├── circuits/
│   ├── transfer_2in2out/     # Circom circuit: 2-input/2-output
│   ├── transfer_4in4out/     # Circom circuit: 4-input/4-output
│   └── swap/                 # Shielded swap commit-reveal circuit
├── contracts/
│   ├── ct20/                 # Confidential token standard
│   ├── viewing_keys/         # Auditor viewing key system
│   └── swap/                 # Shielded swap primitive
├── indexer/                  # Persistent note state manager
├── sdk/                      # TypeScript zkella-sdk
├── app/                      # Reference wallet application
└── docs/                     # Specifications and guides
```

---

## License

Apache 2.0 — open for the entire Stellar ecosystem to build on.
