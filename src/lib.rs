use std::ffi::{CString, c_char};

use bdk::bitcoin::{Address, Network, PrivateKey, key::Secp256k1};

#[unsafe(no_mangle)]
pub extern "C" fn bdk_get_public_key(data: *const u8, len: usize, out_len: *mut usize) -> *mut u8 {
    let private_key_data = unsafe {
        assert!(!data.is_null(), "Input data pointer is null");
        std::slice::from_raw_parts(data, len)
    };

    let public_key = get_public_key(private_key_data.to_vec());

    let output_ptr = public_key.as_ptr() as *mut u8;

    unsafe {
        *out_len = public_key.len();
    }

    output_ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn bdk_get_address(data: *const u8, len: usize) -> *const c_char {
    let private_key_data = unsafe {
        assert!(!data.is_null(), "Input data pointer is null");
        std::slice::from_raw_parts(data, len)
    };

    let address = get_address(private_key_data.to_vec());

    let c_string = CString::new(address).expect("CString::new failed");
    c_string.into_raw()
}

fn get_public_key(private_key_data: Vec<u8>) -> Vec<u8> {
    let private_key = PrivateKey::from_slice(&private_key_data, Network::Regtest)
        .expect("Failed to create PrivateKey from slice");

    let secp = Secp256k1::new();
    let public_key = private_key.public_key(&secp);

    return public_key.to_bytes();
}

fn get_address(private_key_data: Vec<u8>) -> String {
    let private_key = PrivateKey::from_slice(&private_key_data, Network::Regtest)
        .expect("Failed to create PrivateKey from slice");

    let secp = Secp256k1::new();
    let public_key = private_key.public_key(&secp);

    let address = Address::p2pkh(&public_key, Network::Bitcoin);

    return address.to_string();
}

mod tests {
    #[test]
    fn test_get_address() {
        let private_key =
            hex::decode("23d4a09295be678b21a5f1dceae1f634a69c1b41775f680ebf8165266471401b")
                .unwrap();

        let address = super::get_address(private_key).as_bytes().to_vec();

        println!("Address: {}", hex::encode(address));
    }
}
