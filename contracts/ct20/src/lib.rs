use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Bytes, BytesN, Env, Vec,
};

// ── Storage keys ──────────────────────────────────────────────────────────────

#[contracttype]
pub enum StorageKey {
    MerkleRoot,
    NextLeafIndex,
    MerkleLeaf(u32),
    Nullifier(BytesN<32>),
    VerifyingKey,
    ShieldedSupply(Address),
    Paused,
    Admin,
}

// ── Public input structs ──────────────────────────────────────────────────────

#[contracttype]
pub struct ShieldPublicInputs {
    pub commitment:    BytesN<32>,
    pub value_commit:  BytesN<32>,
    pub pub_value:     i128,
    pub pub_asset_id:  Address,
}

#[contracttype]
pub struct TransferPublicInputs {
    pub anchor:             BytesN<32>,
    pub nullifiers:         Vec<BytesN<32>>,
    pub out_commitments:    Vec<BytesN<32>>,
    pub in_value_commits:   Vec<BytesN<32>>,
    pub out_value_commits:  Vec<BytesN<32>>,
    pub fee:                i128,
    pub asset_id:           Address,
}

#[contracttype]
pub struct UnshieldPublicInputs {
    pub anchor:         BytesN<32>,
    pub nullifier:      BytesN<32>,
    pub pub_value:      i128,
    pub pub_asset_id:   Address,
    pub recipient_hash: BytesN<32>,
}

// ── Events ────────────────────────────────────────────────────────────────────

#[contracttype]
pub struct NoteCommitmentEvent {
    pub leaf_index:     u32,
    pub commitment:     BytesN<32>,
    pub encrypted_note: Bytes,
}

#[contracttype]
pub struct NullifierEvent {
    pub nullifier: BytesN<32>,
}

#[contracttype]
pub struct UnshieldEvent {
    pub to:     Address,
    pub amount: i128,
    pub asset:  Address,
}

// ── Contract ──────────────────────────────────────────────────────────────────

#[contract]
pub struct CT20Contract;

#[contractimpl]
impl CT20Contract {

    pub fn initialize(env: Env, admin: Address, verifying_key: Bytes) {
        if env.storage().instance().has(&StorageKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&StorageKey::Admin, &admin);
        env.storage().instance().set(&StorageKey::VerifyingKey, &verifying_key);
        env.storage().instance().set(&StorageKey::MerkleRoot, &BytesN::from_array(&env, &[0u8; 32]));
        env.storage().instance().set(&StorageKey::NextLeafIndex, &0u32);
        env.storage().instance().set(&StorageKey::Paused, &false);
    }

    pub fn shield(
        env:            Env,
        from:           Address,
        asset:          Address,
        amount:         i128,
        commitment:     BytesN<32>,
        encrypted_note: Bytes,
        shield_proof:   Bytes,
        shield_pub:     ShieldPublicInputs,
    ) -> u32 {
        from.require_auth();
        Self::assert_not_paused(&env);

        // Verify proof
        assert!(Self::verify_proof(&env, &shield_proof, &Self::shield_public_inputs_bytes(&env, &shield_pub)),
            "invalid shield proof");

        // Check public input consistency
        assert!(shield_pub.commitment == commitment, "commitment mismatch");
        assert!(shield_pub.pub_value == amount, "amount mismatch");
        assert!(shield_pub.pub_asset_id == asset, "asset mismatch");

        // Pull tokens in
        let token = soroban_sdk::token::Client::new(&env, &asset);
        token.transfer(&from, &env.current_contract_address(), &amount);

        // Update shielded supply
        let prev: i128 = env.storage().instance()
            .get(&StorageKey::ShieldedSupply(asset.clone()))
            .unwrap_or(0);
        env.storage().instance().set(&StorageKey::ShieldedSupply(asset), &(prev + amount));

        // Insert commitment into Merkle tree
        let leaf_index = Self::insert_commitment(&env, commitment.clone());

        // Emit event
        env.events().publish(
            (symbol_short!("zkella"), symbol_short!("note")),
            NoteCommitmentEvent { leaf_index, commitment, encrypted_note },
        );

        leaf_index
    }

    pub fn transfer(
        env:             Env,
        nullifiers:      Vec<BytesN<32>>,
        commitments:     Vec<BytesN<32>>,
        encrypted_notes: Vec<Bytes>,
        proof:           Bytes,
        pub_inputs:      TransferPublicInputs,
    ) -> Vec<u32> {
        Self::assert_not_paused(&env);

        // Anchor check — must be current root
        let current_root: BytesN<32> = env.storage().instance()
            .get(&StorageKey::MerkleRoot).unwrap();
        assert!(pub_inputs.anchor == current_root, "invalid anchor");

        // Nullifier uniqueness
        for nf in nullifiers.iter() {
            assert!(!env.storage().persistent().has(&StorageKey::Nullifier(nf.clone())),
                "nullifier already spent");
        }

        // Verify proof
        assert!(Self::verify_proof(&env, &proof, &Self::transfer_public_inputs_bytes(&env, &pub_inputs)),
            "invalid transfer proof");

        // Mark nullifiers spent
        for nf in nullifiers.iter() {
            env.storage().persistent().set(&StorageKey::Nullifier(nf.clone()), &true);
            env.events().publish(
                (symbol_short!("zkella"), symbol_short!("nf")),
                NullifierEvent { nullifier: nf },
            );
        }

        // Insert output commitments
        let mut leaf_indices: Vec<u32> = Vec::new(&env);
        for (cm, enc) in commitments.iter().zip(encrypted_notes.iter()) {
            let idx = Self::insert_commitment(&env, cm.clone());
            leaf_indices.push_back(idx);
            env.events().publish(
                (symbol_short!("zkella"), symbol_short!("note")),
                NoteCommitmentEvent { leaf_index: idx, commitment: cm, encrypted_note: enc },
            );
        }

        leaf_indices
    }

    pub fn unshield(
        env:        Env,
        nullifier:  BytesN<32>,
        to:         Address,
        proof:      Bytes,
        pub_inputs: UnshieldPublicInputs,
    ) {
        Self::assert_not_paused(&env);

        let current_root: BytesN<32> = env.storage().instance()
            .get(&StorageKey::MerkleRoot).unwrap();
        assert!(pub_inputs.anchor == current_root, "invalid anchor");
        assert!(!env.storage().persistent().has(&StorageKey::Nullifier(nullifier.clone())),
            "nullifier already spent");

        assert!(Self::verify_proof(&env, &proof, &Self::unshield_public_inputs_bytes(&env, &pub_inputs)),
            "invalid unshield proof");

        // Mark nullifier spent
        env.storage().persistent().set(&StorageKey::Nullifier(nullifier.clone()), &true);

        // Update shielded supply
        let prev: i128 = env.storage().instance()
            .get(&StorageKey::ShieldedSupply(pub_inputs.pub_asset_id.clone()))
            .unwrap_or(0);
        env.storage().instance().set(
            &StorageKey::ShieldedSupply(pub_inputs.pub_asset_id.clone()),
            &(prev - pub_inputs.pub_value),
        );

        // Transfer tokens out
        let token = soroban_sdk::token::Client::new(&env, &pub_inputs.pub_asset_id);
        token.transfer(&env.current_contract_address(), &to, &pub_inputs.pub_value);

        env.events().publish(
            (symbol_short!("zkella"), symbol_short!("unshield")),
            UnshieldEvent { to, amount: pub_inputs.pub_value, asset: pub_inputs.pub_asset_id },
        );
    }

    pub fn merkle_root(env: Env) -> BytesN<32> {
        env.storage().instance().get(&StorageKey::MerkleRoot).unwrap()
    }

    pub fn is_spent(env: Env, nullifier: BytesN<32>) -> bool {
        env.storage().persistent().has(&StorageKey::Nullifier(nullifier))
    }

    pub fn shielded_supply(env: Env, asset: Address) -> i128 {
        env.storage().instance()
            .get(&StorageKey::ShieldedSupply(asset))
            .unwrap_or(0)
    }

    pub fn pause(env: Env) {
        let admin: Address = env.storage().instance().get(&StorageKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&StorageKey::Paused, &true);
    }

    pub fn unpause(env: Env) {
        let admin: Address = env.storage().instance().get(&StorageKey::Admin).unwrap();
        admin.require_auth();
        env.storage().instance().set(&StorageKey::Paused, &false);
    }

    // ── Internal helpers ──────────────────────────────────────────────────────

    fn assert_not_paused(env: &Env) {
        let paused: bool = env.storage().instance().get(&StorageKey::Paused).unwrap_or(false);
        assert!(!paused, "contract is paused");
    }

    fn insert_commitment(env: &Env, cm: BytesN<32>) -> u32 {
        let index: u32 = env.storage().instance()
            .get(&StorageKey::NextLeafIndex).unwrap_or(0);
        env.storage().persistent().set(&StorageKey::MerkleLeaf(index), &cm);
        // Root recomputation: full path update from leaf to root
        // (implementation uses iterative Poseidon2 hashing up 32 levels)
        let root = Self::recompute_root(env, index, &cm);
        env.storage().instance().set(&StorageKey::MerkleRoot, &root);
        env.storage().instance().set(&StorageKey::NextLeafIndex, &(index + 1));
        index
    }

    fn recompute_root(env: &Env, _leaf_index: u32, _cm: &BytesN<32>) -> BytesN<32> {
        // Full Poseidon2-based Merkle root recomputation
        // Placeholder: replaced with complete implementation in M1
        BytesN::from_array(env, &[0u8; 32])
    }

    fn verify_proof(env: &Env, _proof: &Bytes, _pub_inputs: &Bytes) -> bool {
        // Groth16 verification via BN254 host functions:
        //   bn254_g1_add, bn254_g1_mul, bn254_multi_pairing_check
        // Full implementation in M1
        let _ = env;
        true
    }

    fn shield_public_inputs_bytes(env: &Env, _pub: &ShieldPublicInputs) -> Bytes {
        Bytes::new(env)
    }

    fn transfer_public_inputs_bytes(env: &Env, _pub: &TransferPublicInputs) -> Bytes {
        Bytes::new(env)
    }

    fn unshield_public_inputs_bytes(env: &Env, _pub: &UnshieldPublicInputs) -> Bytes {
        Bytes::new(env)
    }
}
