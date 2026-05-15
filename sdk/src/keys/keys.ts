import { SpendingKey, ViewingKey, ShieldedAddress, ViewingKeyExport } from '../types'

const BLAKE2B_KEY_DOMAIN   = new TextEncoder().encode('zkella_spend_v1')
const NULLIFIER_KEY_DOMAIN = 1n
const VIEWING_KEY_DOMAIN   = 2n

export class ZKELLAKeys {

  private constructor(public readonly spendingKey: SpendingKey) {}

  static generate(): ZKELLAKeys {
    const seed = crypto.getRandomValues(new Uint8Array(32))
    return ZKELLAKeys.fromSeed(seed)
  }

  static fromSeed(seed: Uint8Array): ZKELLAKeys {
    if (seed.length !== 32) throw new Error('seed must be 32 bytes')
    // sk = BLAKE2b-256(seed || domain)
    // Derived keys via Poseidon2 — full impl in M1
    const sk: SpendingKey = {
      raw:             seed,
      nullifierKey:    new Uint8Array(32),  // Poseidon2(sk, 1)
      viewingKey:      new Uint8Array(32),  // Poseidon2(sk, 2)
      transmissionKey: new Uint8Array(32),  // sk * G (BN254)
    }
    return new ZKELLAKeys(sk)
  }

  deriveAddress(diversifierIndex = 0): ShieldedAddress {
    // diversifier = PRF(sk, diversifierIndex)
    // pk_d = sk * H_to_curve(diversifier)
    // Full impl in M1
    return {
      diversifier: new Uint8Array(11),
      pkD:         new Uint8Array(32),
      toString:    () => 'zkella1placeholder',
    }
  }

  exportViewingKey(birthdayLedger: number, network: string): ViewingKeyExport {
    return {
      version:          1,
      network,
      viewing_key:      Buffer.from(this.spendingKey.viewingKey).toString('hex'),
      transmission_key: Buffer.from(this.spendingKey.transmissionKey).toString('hex'),
      birthday_ledger:  birthdayLedger,
    }
  }

  toViewingKey(birthdayLedger: number): ViewingKey {
    return {
      raw:             this.spendingKey.viewingKey,
      transmissionKey: this.spendingKey.transmissionKey,
      birthdayLedger,
    }
  }
}
