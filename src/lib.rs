use bdk::bitcoin::{Network, PrivateKey, key::Secp256k1};

#[no_mangle]
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

fn get_public_key(private_key_data: Vec<u8>) -> Vec<u8> {
    let private_key = PrivateKey::from_slice(&private_key_data, Network::Bitcoin)
        .expect("Failed to create PrivateKey from slice");

    let secp = Secp256k1::new();
    let public_key = private_key.public_key(&secp);

    return public_key.to_bytes();
}
