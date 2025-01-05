pub mod client;
pub mod errors;
pub mod types;

#[cfg(test)]
mod tests {
    use crate::client::*;
    use crate::errors::*;
    use crate::types::*;
    use dotenv::dotenv;
    use std::env;

    fn get_client_from_env() -> SignalWireClient {
        dotenv().ok();

        let space_name = env::var("SIGNALWIRE_SPACE_NAME").expect("Missing space name");
        let project_id = env::var("SIGNALWIRE_PROJECT_ID").expect("Missing project ID");
        let api_key = env::var("SIGNALWIRE_API_KEY").expect("Missing API key");

        SignalWireClient::new(&space_name, &project_id, &api_key)
    }

    #[tokio::test]
    async fn test_get_jwt() {
        let client = get_client_from_env();
        let result = client.get_jwt().await;

        match result {
            Ok(jwt_response) => {
                assert!(!jwt_response.jwt_token.is_empty(), "JWT token should not be empty");
                assert!(!jwt_response.refresh_token.is_empty(), "Refresh token should not be empty");
            }
            Err(e) => {
                eprintln!("Observed error with test credentials: {:?}", e);
                assert!(false, "Test should fail with invalid credentials");
            }
        }
    }

    #[tokio::test]
    async fn test_get_phone_numbers_available() {
        let client = get_client_from_env();
        let query_params = PhoneNumberAvailableQueryParams::new().build();

        match client.get_phone_numbers_available("US", &query_params).await {
            Ok(response) => {
                assert!(!response.phone_numbers_available.is_empty(), "Expected non-empty phone numbers list");
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                assert_eq!(e.to_string(), "Expected error message");
            }
        }
    }

    #[tokio::test]
    async fn test_get_phone_numbers_owned() {
        let client = get_client_from_env();
        let query_params = PhoneNumberOwnedFilterParams::new().build();

        let result = client.get_phone_numbers_owned(&query_params).await;

        match result {
            Ok(phone_numbers) => {
                assert!(!phone_numbers.data.is_empty(), "Expected non-empty phone numbers list");
            }
            Err(SignalWireError::Unauthorized) => {
                println!("Error: Unauthorized - Test passed as expected.");
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }
    }
}
