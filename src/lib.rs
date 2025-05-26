use std::collections::HashMap;

use bdk::{
    bitcoin::{Network, PrivateKey, PublicKey, key::Secp256k1, psbt::Psbt},
    miniscript::psbt::PsbtExt,
};

// #[no_mangle]
// pub extern "C" fn bdk_sign_psbt() ->  {

// }

fn sign_tx(psbt_json: Vec<u8>, private_key_data: Vec<u8>) -> Vec<u8> {
    let mut psbt: Psbt = serde_json::from_slice(&psbt_json).expect("Failed to deserialize PSBT");

    let private_key = PrivateKey::from_slice(&private_key_data, Network::Bitcoin)
        .expect("Failed to create PrivateKey from slice");

    let secp = Secp256k1::new();
    let public_key = private_key.public_key(&secp);

    let keys: HashMap<PublicKey, PrivateKey> = [(public_key, private_key)].into();

    let _ = psbt.sign(&keys, &secp).expect("Failed to sign PSBT");

    psbt.finalize_mut(&secp).expect("Failed to finalize PSBT");

    let tx = psbt.extract_tx();

    serde_json::to_vec(&tx).expect("Failed to serialize transaction to JSON")
}

#[cfg(test)]
mod tests {
    use bdk::bitcoin::{
        Amount, ScriptBuf, Transaction, TxIn, TxOut,
        absolute::LockTime,
        block::Version,
        psbt::Psbt,
        secp256k1::{SecretKey, rand},
    };

    use super::sign_tx;

    #[test]
    fn sign_test() {
        let unsigned_tx = Transaction {
            version: Version::TWO.to_consensus(),
            lock_time: LockTime::ZERO,
            input: vec![TxIn::default()],
            output: vec![TxOut {
                value: Amount::ZERO.to_sat(),
                script_pubkey: ScriptBuf::new(),
            }],
        };

        let psbt = Psbt::from_unsigned_tx(unsigned_tx)
            .expect("Failed to create PSBT from unsigned transaction");

        let psbt_json = serde_json::to_vec(&psbt).expect("Failed to serialize PSBT to JSON");

        let secret_key = SecretKey::new(&mut rand::thread_rng());

        let private_key_data = secret_key.secret_bytes().to_vec();

        let tx = sign_tx(psbt_json, private_key_data);

        println!("Signed transaction: {}", String::from_utf8(tx).unwrap());
    }
}
