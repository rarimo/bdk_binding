use std::collections::HashMap;

use bdk::{
    bitcoin::{Network, PrivateKey, PublicKey, Transaction, key::Secp256k1, psbt::Psbt},
    miniscript::psbt::PsbtExt,
};

// #[no_mangle]
// pub extern "C" fn bdk_sign_psbt() ->  {

// }

fn sign_psbt(psbt_json: Vec<u8>, private_key_data: Vec<u8>) -> Vec<u8> {
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
    #[test]
    fn sign_test() {}
}
