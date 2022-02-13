#[cfg(test)]
mod tests {
    use cosmwasm_std::{Binary, Extern, InitResponse, StdError, StdResult};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, MockApi, MockQuerier, MockStorage};
    use flexible_permits::transaction::{PermitSignature, PubKey};
    use shade_protocol::utils::errors::DetailedError;
    use crate::contract::{handle, init};
    use crate::errors::Error;
    use crate::msgs::{HandleMsg, InitMsg, PermitMsg, QueryPermit};

    fn init_helper() -> (
        StdResult<InitResponse>,
        Extern<MockStorage, MockApi, MockQuerier>,
    ) {
        let mut deps = mock_dependencies(20, &[]);
        let env = mock_env("instantiator", &[]);

        let init_msg = InitMsg {
        };

        (init(&mut deps, env, init_msg), deps)
    }

    fn handle_error(response: StdError) -> DetailedError<Error> {
        match response {
            StdError::GenericErr { msg, .. } => {
                let error = serde_json::from_str(&msg)
                    .expect("Should return a Detailed Error");
                error
            }
            _ => panic!("Should return generic error")
        }
    }

    fn expect_handle_error(
        deps: &mut Extern<MockStorage, MockApi, MockQuerier>,
        msg: HandleMsg,
        sender: &str,
        expected_code: u8,
    ) {
        let handle_result = handle(
            deps,
            mock_env(sender, &[]),
            msg
        );

        match handle_result {
            Ok(_) => assert!(false, "Expected error"),
            Err(std_err) => {
                let err = handle_error(std_err);

                assert_eq!(expected_code, err.code);
            }
        }
    }

    fn expect_handle_success(
        deps: &mut Extern<MockStorage, MockApi, MockQuerier>,
        msg: HandleMsg,
        sender: &str,
    ) {
        let handle_result = handle(
            deps,
            mock_env(sender, &[]),
            msg
        );

        if let Some(err) = handle_result.err() {
            assert!(false, "Unexpected error: {}", match err {
                StdError::GenericErr { msg, .. } => msg,
                _ => "unexpected".to_string()
            })
        }
    }

    #[test]
    fn viewing_keys() {
        let (init_result, mut deps) = init_helper();
        assert!(init_result.is_ok());

        // Query without viewing key
        expect_handle_error(
            &mut deps,
            HandleMsg::UseViewingKey { key: "key".to_string(), padding: None },
            "user",
            2
        );

        // Set key
        expect_handle_success(
            &mut deps,
            HandleMsg::SetViewingKey { key: "key".to_string(), padding: None },
            "user",
        );

        // Query with key
        expect_handle_success(
            &mut deps,
            HandleMsg::UseViewingKey { key: "key".to_string(), padding: None },
            "user",
        );

        // Query with wrong key
        expect_handle_error(
            &mut deps,
            HandleMsg::UseViewingKey { key: "wrong".to_string(), padding: None },
            "user",
            1
        );
    }
}