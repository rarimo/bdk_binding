use std::ffi::{CString, c_char};

use bdk::bitcoin::{Address, Network, PrivateKey, key::Secp256k1};

use std::alloc::{self, Layout};
use std::mem;

#[unsafe(no_mangle)]
pub extern "C" fn bdk_get_public_key(
    data: *const u8,
    len: usize,
    network_id: u8,
    out_len: *mut usize,
) -> *mut u8 {
    let private_key_data = unsafe {
        assert!(!data.is_null(), "Input data pointer is null");
        std::slice::from_raw_parts(data, len)
    };

    let network = get_network(network_id);

    let public_key = get_public_key(private_key_data.to_vec(), network);

    unsafe { *out_len = public_key.len() };
    let ptr = bdk_alloc(public_key.len());
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    unsafe {
        std::ptr::copy_nonoverlapping(public_key.as_ptr(), ptr, public_key.len());
    }

    ptr
}

#[unsafe(no_mangle)]
pub extern "C" fn bdk_get_address(data: *const u8, len: usize, network_id: u8) -> *const c_char {
    let private_key_data = unsafe {
        assert!(!data.is_null(), "Input data pointer is null");
        std::slice::from_raw_parts(data, len)
    };

    let network = get_network(network_id);

    let address = get_address(private_key_data.to_vec(), network);

    let c_string = CString::new(address).expect("CString::new failed");
    c_string.into_raw()
}

#[unsafe(no_mangle)]
pub extern "C" fn bdk_alloc(len: usize) -> *mut u8 {
    unsafe {
        let layout = Layout::from_size_align_unchecked(len, mem::align_of::<u8>());
        alloc::alloc(layout)
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn bdk_dealloc(ptr: *mut u8, len: usize) {
    unsafe {
        let layout = Layout::from_size_align_unchecked(len, mem::align_of::<u8>());
        alloc::dealloc(ptr, layout);
    }
}

fn get_public_key(private_key_data: Vec<u8>, network: Network) -> Vec<u8> {
    let private_key = PrivateKey::from_slice(&private_key_data, network)
        .expect("Failed to create PrivateKey from slice");

    let secp = Secp256k1::new();
    let public_key = private_key.public_key(&secp);

    return public_key.to_bytes();
}

fn get_address(private_key_data: Vec<u8>, network: Network) -> String {
    let private_key = PrivateKey::from_slice(&private_key_data, network)
        .expect("Failed to create PrivateKey from slice");

    let secp = Secp256k1::new();
    let public_key = private_key.public_key(&secp);

    let address =
        Address::p2wpkh(&public_key, network).expect("Failed to create address from public key");

    return address.to_string();
}

fn get_network(network_id: u8) -> Network {
    match network_id {
        0 => Network::Bitcoin,
        1 => Network::Testnet,
        2 => Network::Regtest,
        _ => panic!("Unsupported network ID"),
    }
}

mod tests {
    #[test]
    fn test_get_address() {
        let private_key =
            hex::decode("23d4a09295be678b21a5f1dceae1f634a69c1b41775f680ebf8165266471401b")
                .unwrap();

        let address = super::get_address(private_key, bdk::bitcoin::Network::Regtest)
            .as_bytes()
            .to_vec();

        println!("Address: {}", hex::encode(address));
    }

    #[test]
    fn test_serialization() {
        let network_json = serde_json::to_string(&bdk::bitcoin::Network::Regtest)
            .expect("Failed to serialize Network::Regtest");

        println!("Serialized: {}", network_json);
    }
}
