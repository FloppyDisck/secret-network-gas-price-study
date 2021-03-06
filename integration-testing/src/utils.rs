use cosmwasm_std::Binary;
use query_authentication::permit::Permit;
use query_authentication::transaction::{PermitSignature, PubKey};
use rand::distributions::Alphanumeric;
use rand::Rng;
use secretcli::secretcli::create_permit;
use serde::Serialize;
use std::fs;

pub const PERMITS_FILE: &str = "../compiled/permits.wasm.gz";
pub const STORAGE_FILE: &str = "../compiled/storage.wasm.gz";

pub const STORE_GAS: &str = "10000000";
pub const GAS: &str = "1000000";

pub fn get_average(arr: Vec<u64>) -> u64 {
    arr.iter().sum::<u64>() / arr.len() as u64
}

pub fn generate_label(size: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect()
}

pub fn create_signed_permit<T: Clone + Serialize>(
    params: T,
    msg_type: Option<String>,
    signer: &str,
) -> Permit<T> {
    let mut permit = Permit {
        params,
        signature: PermitSignature {
            pub_key: PubKey {
                r#type: "".to_string(),
                value: Default::default(),
            },
            signature: Default::default(),
        },
        account_number: None,
        chain_id: Some("testnet".to_string()),
        sequence: None,
        memo: None,
    };

    let unsigned_msg = permit.create_signed_tx(msg_type);

    let signed_info = create_permit(unsigned_msg, signer).unwrap();

    permit.signature = PermitSignature {
        pub_key: query_authentication::transaction::PubKey {
            r#type: signed_info.pub_key.msg_type,
            value: Binary::from_base64(&signed_info.pub_key.value).unwrap(),
        },
        signature: Binary::from_base64(&signed_info.signature).unwrap(),
    };

    permit
}

pub fn store_struct<T: serde::Serialize>(path: &str, data: &T) {
    fs::write(
        path,
        serde_json::to_string_pretty(data).expect("Could not serialize data"),
    )
    .expect(&format!("Could not store {}", path));
}

