use cosmwasm_std::{HumanAddr, Uint128};
use integration_testing::utils::{generate_label, store_struct, GAS, STORAGE_FILE, STORE_GAS, get_average};
use secretcli::cli_types::NetContract;
use secretcli::secretcli::{handle, init, query, Report};
use serde_json::Result;
use shade_protocol::utils::generic_response::ResponseStatus::{Failure, Success};
use storage::handle;
use storage::msgs::{HandleAnswer, HandleMsg, InitMsg, Config, UpdateConfig};

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

fn run_msg<Message: serde::Serialize>(
    msg: &Message,
    contract: &NetContract,
    report: &mut Vec<Report>,
    gas: &mut Vec<u64>
) -> Result<()>
{
    let (_, res) = handle(
        &msg,
        &contract,
        "a",
        Some(GAS),
        Some("test"),
        None,
        report,
        None,
    )?;

    gas.push(res.gas_used.parse::<u64>().unwrap());

    Ok(())
}

#[test]
fn storage_gas_study() -> Result<()> {
    let mut report = vec![];


    let mut singleton_write_gas = vec![];
    let mut singleton_read_gas = vec![];
    let mut item_write_gas = vec![];
    let mut item_read_gas = vec![];

    let mut fractioned_item_write_all_gas = vec![];
    let mut fractioned_item_write_small_gas = vec![];
    let mut fractioned_item_write_large_gas = vec![];
    let mut fractioned_item_read_gas = vec![];

    let mut tiny_vec_gas = vec![];
    let mut small_vec_gas = vec![];
    let mut large_vec_gas = vec![];
    let mut huge_vec_gas = vec![];

    let mut bucket_write_gas = vec![];
    let mut bucket_read_gas = vec![];
    let mut bucket_read_range_gas = vec![];

    let mut map_write_gas = vec![];
    let mut map_read_gas = vec![];
    let mut map_read_range_gas = vec![];

    let tiny_vec: Vec<u64> = (0..10).collect();
    let small_vec: Vec<u64> = (0..100).collect();
    let large_vec: Vec<u64> = (0..1000).collect();
    let huge_vec: Vec<u64> = (0..10000).collect();

    let config = Config {
        address: HumanAddr::from("some_address"),
        number: Uint128(u128::MAX),
        other_data: "some random string".to_string(),
        array: large_vec.clone()
    };

    for _ in 0..10 {
        let contract = init_contract()?;

        // Singleton Write
        {
            let msg = HandleMsg::SingletonWrite {
                data: config.clone()
            };

            run_msg(&msg, &contract, &mut report, &mut singleton_write_gas)?;
        }

        // Singleton Read
        {
            let msg = HandleMsg::SingletonRead {};

            run_msg(&msg, &contract, &mut report, &mut singleton_read_gas)?;
        }

        // Item Write
        {
            let msg = HandleMsg::ItemWrite {
                data: config.clone()
            };

            run_msg(&msg, &contract, &mut report, &mut item_write_gas)?;
        }

        // Item Read
        {
            let msg = HandleMsg::ItemRead {
            };

            run_msg(&msg, &contract, &mut report, &mut item_read_gas)?;
        }

        // Fractioned
        {
            let msg = HandleMsg::FractionedItemWrite {
                data: UpdateConfig {
                    address: Some(HumanAddr::from("Someone")),
                    number: Some(Uint128(u128::MAX)),
                    other_data: Some("some string".to_string()),
                    array: Some(large_vec.clone())
                }
            };

            run_msg(&msg, &contract, &mut report, &mut fractioned_item_write_all_gas)?;
        }

        {
            let msg = HandleMsg::FractionedItemWrite {
                data: UpdateConfig {
                    address: Some(HumanAddr::from("Someone")),
                    number: Some(Uint128(u128::MAX)),
                    other_data: Some("some string".to_string()),
                    array: None
                }
            };

            run_msg(&msg, &contract, &mut report, &mut fractioned_item_write_small_gas)?;
        }

        {
            let msg = HandleMsg::FractionedItemWrite {
                data: UpdateConfig {
                    address: None,
                    number: None,
                    other_data: None,
                    array: Some(large_vec.clone())
                }
            };

            run_msg(&msg, &contract, &mut report, &mut fractioned_item_write_large_gas)?;
        }

        {
            let msg = HandleMsg::FractionedItemRead {};

            run_msg(&msg, &contract, &mut report, &mut fractioned_item_read_gas)?;
        }

        // Vec

        {
            let msg = HandleMsg::FractionedItemWrite {
                data: UpdateConfig {
                    address: None,
                    number: None,
                    other_data: None,
                    array: Some(tiny_vec.clone())
                }
            };

            run_msg(&msg, &contract, &mut report, &mut tiny_vec_gas)?;
        }

        {
            let msg = HandleMsg::FractionedItemWrite {
                data: UpdateConfig {
                    address: None,
                    number: None,
                    other_data: None,
                    array: Some(small_vec.clone())
                }
            };

            run_msg(&msg, &contract, &mut report, &mut small_vec_gas)?;
        }

        {
            let msg = HandleMsg::FractionedItemWrite {
                data: UpdateConfig {
                    address: None,
                    number: None,
                    other_data: None,
                    array: Some(large_vec.clone())
                }
            };

            run_msg(&msg, &contract, &mut report, &mut large_vec_gas)?;
        }

        {
            let msg = HandleMsg::FractionedItemWrite {
                data: UpdateConfig {
                    address: None,
                    number: None,
                    other_data: None,
                    array: Some(huge_vec.clone())
                }
            };

            run_msg(&msg, &contract, &mut report, &mut huge_vec_gas)?;
        }

        // Setup for bucket and map read range
        {
            let data = vec![config.clone(), config.clone(), config.clone(), config.clone()];
            {
                let msg = HandleMsg::BucketWriteRange {
                    id: (1, 0),
                    data: data.clone()
                };

                run_msg(&msg, &contract, &mut report, &mut vec![])?;
            }

            {
                let msg = HandleMsg::MapWriteRange {
                    id: (1, 0),
                    data
                };

                run_msg(&msg, &contract, &mut report, &mut vec![])?;
            }
        }

        {
            let msg = HandleMsg::BucketWrite {
                id: (0, 0),
                data: config.clone()
            };

            run_msg(&msg, &contract, &mut report, &mut bucket_write_gas)?;
        }

        {
            let msg = HandleMsg::BucketRead {
                id: (0, 0),
            };

            run_msg(&msg, &contract, &mut report, &mut bucket_read_gas)?;
        }

        {
            let msg = HandleMsg::BucketRange {
                id: 1,
                min: 0,
                max: 3
            };

            run_msg(&msg, &contract, &mut report, &mut bucket_read_range_gas)?;
        }

        {
            let msg = HandleMsg::MapWrite {
                id: (0, 0),
                data: config.clone()
            };

            run_msg(&msg, &contract, &mut report, &mut map_write_gas)?;
        }

        {
            let msg = HandleMsg::MapRead {
                id: (0, 0),
            };

            run_msg(&msg, &contract, &mut report, &mut map_read_gas)?;
        }

        {
            let msg = HandleMsg::MapRange {
                id: 1,
                min: 0,
                max: 3
            };

            run_msg(&msg, &contract, &mut report, &mut map_read_range_gas)?;
        }
    }

    println!(
        "Singleton write average gas: {}",
        get_average(singleton_write_gas)
    );

    println!(
        "Singleton read average gas: {}",
        get_average(singleton_read_gas)
    );

    println!(
        "Item write average gas: {}",
        get_average(item_write_gas)
    );

    println!(
        "Item read average gas: {}",
        get_average(item_read_gas)
    );

    // Fractioned
    println!(
        "Fractioned Item read average gas: {}",
        get_average(fractioned_item_read_gas)
    );
    println!(
        "Fractioned Item write average gas: {}",
        get_average(fractioned_item_write_all_gas)
    );
    println!(
        "Fractioned Item small partial write average gas: {}",
        get_average(fractioned_item_write_small_gas)
    );
    println!(
        "Fractioned Item large partial write average gas: {}",
        get_average(fractioned_item_write_large_gas)
    );

    // Vec
    println!(
        "Write vec of 10 items average gas: {}",
        get_average(tiny_vec_gas)
    );
    println!(
        "Write vec of 100 items average gas: {}",
        get_average(small_vec_gas)
    );
    println!(
        "Write vec of 1000 items average gas: {}",
        get_average(large_vec_gas)
    );
    println!(
        "Write vec of 10000 items average gas: {}",
        get_average(huge_vec_gas)
    );

    // Bucket
    println!(
        "Write item on bucket average gas: {}",
        get_average(bucket_write_gas)
    );
    println!(
        "Read item on bucket average gas: {}",
        get_average(bucket_read_gas)
    );
    println!(
        "Read 4 item range on bucket average gas: {}",
        get_average(bucket_read_range_gas)
    );

    // Map
    println!(
        "Write item on map average gas: {}",
        get_average(map_write_gas)
    );
    println!(
        "Read item on map average gas: {}",
        get_average(map_read_gas)
    );
    println!(
        "Read 4 item range on map average gas: {}",
        get_average(map_read_range_gas)
    );


    store_struct("./storage_gas_study.json", &report);

    Ok(())
}
