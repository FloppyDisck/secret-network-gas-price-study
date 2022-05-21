// use secretcli::cli_types::NetContract;
// use secretcli::secretcli::{handle, init, query};
// use serde_json::Result;
// use shade_protocol::utils::generic_response::ResponseStatus::{Failure, Success};
// use integration_testing::utils::{create_signed_permit, GAS, generate_label, PERMITS_FILE, STORE_GAS, store_struct};
// use permits::msgs::{HandleMsg, InitMsg, PermitMsg, QueryAnswer, QueryMsg};
//
// fn init_contract() -> Result<NetContract> {
//     init(
//         &InitMsg{},
//         PERMITS_FILE,
//         &*generate_label(8),
//         "a",
//         Some(STORE_GAS),
//         Some(GAS),
//         Some("test"),
//         &mut vec![]
//     )
// }
//
// #[test]
// fn test_permit() -> Result<()> {
//     let permit = create_signed_permit(
//         PermitMsg{key: "key".to_string()},
//         None, "a"
//     );
//
//     let contract = init_contract()?;
//
//     // Test that permits work
//     {
//         let msg = QueryMsg::Permit {
//             permit: permit.clone()
//         };
//
//         let query: QueryAnswer = query(&contract, msg, None)?;
//
//         if let QueryAnswer::Permit {
//             status
//         } = query
//         {
//             assert_eq!(status, Success);
//         }
//     }
//
//     // Test that permit key banning works
//     {
//         let msg = HandleMsg::BlockPermitKey { key: "key".to_string(), padding: None};
//
//         handle(&msg, &contract, "a", Some(GAS), Some("test"), None, &mut vec![], None)?;
//     }
//
//     // Test that permits work
//     {
//         let msg = QueryMsg::Permit {
//             permit
//         };
//
//         let query: QueryAnswer = query(&contract, msg, None)?;
//
//         if let QueryAnswer::Permit {
//             status
//         } = query
//         {
//             assert_eq!(status, Failure);
//         }
//     }
//
//     Ok(())
// }
//
// #[test]
// fn gas_study() -> Result<()> {
//     let permit = create_signed_permit(
//         PermitMsg{key: "key".to_string()},
//         None, "a"
//     );
//
//     println!("{}", serde_json::to_string(&permit)?);
//
//     let mut report = vec![];
//
//     let mut set_key_gas = vec![];
//     let mut use_key_gas = vec![];
//     let mut use_permit_gas = vec![];
//     let mut block_permit_gas = vec![];
//
//     for _ in 0..10 {
//         let contract = init_contract()?;
//
//         // Create viewing key
//         {
//             let msg = HandleMsg::SetViewingKey {
//                 key: "key".to_string(),
//                 padding: None
//             };
//
//             let (_, res) = handle(
//                 &msg,
//                 &contract,
//                 "a",
//                 Some(GAS),
//                 Some("test"),
//                 None,
//                 &mut report,
//                 None
//             )?;
//
//             set_key_gas.push(res.gas_used.parse::<u64>().unwrap());
//
//         }
//
//         // "query" viewing key
//         {
//             let msg = HandleMsg::UseViewingKey {
//                 key: "key".to_string(),
//                 padding: None
//             };
//
//             let (_, res) = handle(
//                 &msg,
//                 &contract,
//                 "a",
//                 Some(GAS),
//                 Some("test"),
//                 None,
//                 &mut report,
//                 None
//             )?;
//
//             use_key_gas.push(res.gas_used.parse::<u64>().unwrap());
//
//         }
//
//         // "query" permit
//         {
//             let msg = HandleMsg::UsePermit {
//                 permit: permit.clone(),
//                 padding: None
//             };
//
//             let (_, res) = handle(
//                 &msg,
//                 &contract,
//                 "a",
//                 Some(GAS),
//                 Some("test"),
//                 None,
//                 &mut report,
//                 None
//             )?;
//
//             use_permit_gas.push(res.gas_used.parse::<u64>().unwrap());
//
//         }
//
//         // Ban permit
//         {
//             let msg = HandleMsg::BlockPermitKey {
//                 key: "key".to_string(),
//                 padding: None
//             };
//
//             let (_, res) = handle(
//                 &msg,
//                 &contract,
//                 "a",
//                 Some(GAS),
//                 Some("test"),
//                 None,
//                 &mut report,
//                 None
//             )?;
//
//             block_permit_gas.push(res.gas_used.parse::<u64>().unwrap());
//
//         }
//     }
//
//     println!("Set viewing key average gas: {}", set_key_gas.iter().sum::<u64>() / set_key_gas.len() as u64);
//     println!("Get viewing key average gas: {}", use_key_gas.iter().sum::<u64>() / use_key_gas.len() as u64);
//     println!("Validate permit average gas: {}", use_permit_gas.iter().sum::<u64>() / use_permit_gas.len() as u64);
//     println!("Blocking permit average gas: {}", block_permit_gas.iter().sum::<u64>() / block_permit_gas.len() as u64);
//
//     store_struct("./permit_gas_study.json", &report);
//
//     Ok(())
// }