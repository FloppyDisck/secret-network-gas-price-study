use std::fs;
use cosmwasm_std::Binary;
use flexible_permits::permit::Permit;
use flexible_permits::transaction::{PermitSignature, PubKey};
use rand::distributions::Alphanumeric;
use rand::Rng;
use secretcli::secretcli::create_permit;
use serde::Serialize;

pub const PERMITS_FILE: &str = "../compiled/permits.wasm.gz";

pub const STORE_GAS: &str = "10000000";
pub const GAS: &str = "800000";

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
        pub_key: flexible_permits::transaction::PubKey {
            r#type: signed_info.pub_key.msg_type,
            value: Binary::from_base64(&signed_info.pub_key.value).unwrap(),
        },
        signature: Binary::from_base64(&signed_info.signature).unwrap(),
    };

    permit
}

pub fn store_struct<T: serde::Serialize>(path: &str, data: &T){
    fs::write(path, serde_json::to_string_pretty(data)
        .expect("Could not serialize data"))
        .expect(&format!("Could not store {}", path));
}