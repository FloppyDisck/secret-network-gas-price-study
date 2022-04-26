use integration_testing::utils::{generate_label, store_struct, GAS, STORAGE_FILE, STORE_GAS};
use secretcli::cli_types::NetContract;
use secretcli::secretcli::{handle, init, query};
use serde_json::Result;
use shade_protocol::utils::generic_response::ResponseStatus::{Failure, Success};
use storage::handle;
use storage::msgs::{HandleAnswer, HandleMsg, InitMsg, SimpleConfig};

fn init_contract() -> Result<NetContract> {
    init(
        &InitMsg {},
        STORAGE_FILE,
        &*generate_label(8),
        "a",
        Some(STORE_GAS),
        Some(GAS),
        Some("test"),
        &mut vec![],
    )
}

#[test]
fn test_storage() -> Result<()> {
    let contract = init_contract()?;

    // Test that singleton works
    {
        let config = SimpleConfig {
            address: "test".into(),
            some_number: cosmwasm_std::Uint128(5),
            other_data: "singleton".to_owned(),
        };

        let msg = HandleMsg::SingletonWrite { data: config };

        handle(
            &msg,
            &contract,
            "a",
            None,
            Some("test"),
            None,
            &mut vec![],
            None,
        )?;
    }

    // Test that permit key banning works
    {
        let msg = HandleMsg::BlockPermitKey {
            key: "key".to_string(),
            padding: None,
        };

        handle(
            &msg,
            &contract,
            "a",
            Some(GAS),
            Some("test"),
            None,
            &mut vec![],
            None,
        )?;
    }

    // Test that permits work
    {
        let msg = QueryMsg::Permit { permit };

        let query: QueryAnswer = query(&contract, msg, None)?;

        if let QueryAnswer::Permit { status } = query {
            assert_eq!(status, Failure);
        }
    }

    Ok(())
}

#[test]
fn gas_study() -> Result<()> {
    let permit = create_signed_permit(
        PermitMsg {
            key: "key".to_string(),
        },
        None,
        "a",
    );

    println!("{}", serde_json::to_string(&permit)?);

    let mut report = vec![];

    let mut set_key_gas = vec![];
    let mut use_key_gas = vec![];
    let mut use_permit_gas = vec![];
    let mut block_permit_gas = vec![];

    for _ in 0..10 {
        let contract = init_contract()?;

        // Create viewing key
        {
            let msg = HandleMsg::SetViewingKey {
                key: "key".to_string(),
                padding: None,
            };

            let (_, res) = handle(
                &msg,
                &contract,
                "a",
                Some(GAS),
                Some("test"),
                None,
                &mut report,
                None,
            )?;

            set_key_gas.push(res.gas_used.parse::<u64>().unwrap());
        }

        // "query" viewing key
        {
            let msg = HandleMsg::UseViewingKey {
                key: "key".to_string(),
                padding: None,
            };

            let (_, res) = handle(
                &msg,
                &contract,
                "a",
                Some(GAS),
                Some("test"),
                None,
                &mut report,
                None,
            )?;

            use_key_gas.push(res.gas_used.parse::<u64>().unwrap());
        }

        // "query" permit
        {
            let msg = HandleMsg::UsePermit {
                permit: permit.clone(),
                padding: None,
            };

            let (_, res) = handle(
                &msg,
                &contract,
                "a",
                Some(GAS),
                Some("test"),
                None,
                &mut report,
                None,
            )?;

            use_permit_gas.push(res.gas_used.parse::<u64>().unwrap());
        }

        // Ban permit
        {
            let msg = HandleMsg::BlockPermitKey {
                key: "key".to_string(),
                padding: None,
            };

            let (_, res) = handle(
                &msg,
                &contract,
                "a",
                Some(GAS),
                Some("test"),
                None,
                &mut report,
                None,
            )?;

            block_permit_gas.push(res.gas_used.parse::<u64>().unwrap());
        }
    }

    println!(
        "Set viewing key average gas: {}",
        set_key_gas.iter().sum::<u64>() / set_key_gas.len() as u64
    );
    println!(
        "Get viewing key average gas: {}",
        use_key_gas.iter().sum::<u64>() / use_key_gas.len() as u64
    );
    println!(
        "Validate permit average gas: {}",
        use_permit_gas.iter().sum::<u64>() / use_permit_gas.len() as u64
    );
    println!(
        "Blocking permit average gas: {}",
        block_permit_gas.iter().sum::<u64>() / block_permit_gas.len() as u64
    );

    store_struct("./permit_gas_study.json", &report);

    Ok(())
}
