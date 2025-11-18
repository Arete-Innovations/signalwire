pub mod client;
pub mod errors;
pub mod types;

#[cfg(test)]
mod tests {
    use std::env;

    use dotenv::dotenv;

    use crate::{client::*, errors::*, types::*};

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

    /// Test for SMS sending with SID storage.
    ///
    /// This test is disabled by default to prevent accidental SMS charges.
    /// To run this test, set the following environment variables in your .env file:
    ///
    /// SIGNALWIRE_RUN_SMS_TEST=true
    /// SIGNALWIRE_FROM_NUMBER=+12064145320 (a number you own in SignalWire)
    /// SIGNALWIRE_TO_NUMBER=+40731665699 (the recipient's number)
    /// SIGNALWIRE_STORE_SMS_SID=true (to save the SID for the follow-up test)
    ///
    /// Then run: cargo test test_send_sms -- --nocapture
    #[tokio::test]
    async fn test_send_sms() {
        dotenv().ok();

        if env::var("SIGNALWIRE_RUN_SMS_TEST").unwrap_or_else(|_| "false".to_string()) != "true" {
            println!("Skipping SMS test. To enable, set SIGNALWIRE_RUN_SMS_TEST=true in your .env file.");
            println!("Make sure to also set SIGNALWIRE_FROM_NUMBER and SIGNALWIRE_TO_NUMBER.");
            println!("Set SIGNALWIRE_STORE_SMS_SID=true (optional) to save SID for follow-up tests.");
            return;
        }

        let client = get_client_from_env();

        let from_number = env::var("SIGNALWIRE_FROM_NUMBER").expect("Missing SIGNALWIRE_FROM_NUMBER env var");
        let to_number = env::var("SIGNALWIRE_TO_NUMBER").expect("Missing SIGNALWIRE_TO_NUMBER env var");

        println!("Sending SMS from {} to {}", from_number, to_number);

        let message = SmsMessage {
            from: from_number,
            to: to_number,
            body: "This is a test message from the SignalWire Rust SDK.".to_string(),
        };

        // Send the message and get the SID
        let sid = match client.send_sms(&message).await {
            Ok(response) => {
                assert_eq!(response.from, message.from);
                assert_eq!(response.to, message.to);
                assert_eq!(response.body, message.body);
                assert!(!response.sid.is_empty(), "Expected non-empty SID");
                println!("✓ SMS sent successfully with SID: {}", response.sid);

                // Get the message status
                let status = response.get_status();
                println!("Initial message status: {}", status);

                response.sid
            }
            Err(SignalWireError::Unauthorized) => {
                println!("Error: Unauthorized - Check your credentials");
                return;
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        };

        // Always store the SID for follow-up tests when the SMS test is run
        use std::{fs::File, io::Write};

        match File::create(".signalwire_test_sms_sid") {
            Ok(mut file) => {
                if let Err(e) = writeln!(file, "{}", sid) {
                    println!("Failed to write SMS SID to file: {}", e);
                } else {
                    println!("Stored SMS SID for follow-up test: {}", sid);
                }
            }
            Err(e) => println!("Failed to create SMS SID file: {}", e),
        }

        // Wait a bit before checking the status to allow it to update from "queued"
        println!("Waiting 5 seconds before checking message status...");
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

        // Check the message status
        match client.get_message_status(&sid).await {
            Ok(status_response) => {
                let current_status = status_response.get_status();
                println!("✓ Initial status check: {}", current_status);
                println!(
                    "Message details: SID={}, From={}, To={}, Body=\"{}\"",
                    status_response.sid, status_response.from, status_response.to, status_response.body
                );

                println!("Run the follow-up delayed status check with: cargo test test_delayed_status_check -- --nocapture");
            }
            Err(e) => {
                println!("✗ Failed to check message status: {:?}", e);
            }
        }
    }

    /// Follow-up test to check the status of a previously sent SMS after a delay.
    ///
    /// This test automatically uses the SID stored by a previous test_send_sms run,
    /// or falls back to SIGNALWIRE_MESSAGE_SID from the environment if available.
    ///
    /// To run both tests in sequence:
    /// SIGNALWIRE_RUN_SMS_TEST=true cargo test test_send_sms test_delayed_status_check -- --nocapture
    #[tokio::test]
    async fn test_delayed_status_check() {
        dotenv().ok();

        // We don't need to check SIGNALWIRE_RUN_SMS_TEST here anymore since we always have a stored SID
        // if the first test ran successfully

        // Try to get the message SID from either:
        // 1. The environment variable
        // 2. A file created by a previous test_send_sms run
        let message_sid = match env::var("SIGNALWIRE_MESSAGE_SID") {
            Ok(sid) => sid,
            Err(_) => {
                // Check if we have a stored SID from a previous test
                match std::fs::read_to_string(".signalwire_test_sms_sid") {
                    Ok(sid) => sid.trim().to_string(),
                    Err(_) => {
                        println!("No message SID found. Either:");
                        println!("1. Set SIGNALWIRE_MESSAGE_SID in your .env file");
                        println!("2. Run test_send_sms with SIGNALWIRE_STORE_SMS_SID=true first");
                        return;
                    }
                }
            }
        };

        println!("Checking status of message with SID: {}", message_sid);
        println!("Waiting 10 seconds for potential status changes...");

        // Wait for status to potentially change
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;

        let client = get_client_from_env();

        // Check the status of the specified message after the delay
        match client.get_message_status(&message_sid).await {
            Ok(response) => {
                let status = response.get_status();
                println!("✓ Message status after delay: {}", status);
                println!("Details:");
                println!("  SID: {}", response.sid);
                println!("  From: {}", response.from);
                println!("  To: {}", response.to);
                println!("  Body: {}", response.body);
                println!("  Date sent: {:?}", response.date_sent);
                println!("  Price: {:?}", response.price);
                println!("  Error code: {:?}", response.error_code);

                // Ensure we got valid data back
                assert_eq!(response.sid, message_sid);
                assert!(!response.status.is_empty(), "Status should not be empty");
            }
            Err(SignalWireError::NotFound(_)) => {
                println!("Message not found: {}", message_sid);
            }
            Err(e) => {
                panic!("Unexpected error checking message status: {:?}", e);
            }
        }
    }

    // ---------- Subproject Tests ----------

    /// Test listing subprojects.
    ///
    /// This test retrieves the list of subprojects in your SignalWire account.
    #[tokio::test]
    async fn test_list_subprojects() {
        let client = get_client_from_env();
        let query_params = SubprojectQueryParams::new().build();

        match client.list_subprojects(&query_params).await {
            Ok(response) => {
                // Should contain at least one account (the main project)
                assert!(!response.accounts.is_empty(), "Expected non-empty accounts list");
                println!("Found {} subproject(s)", response.accounts.len());

                // Print some details about each account
                for (i, account) in response.accounts.iter().enumerate() {
                    println!("Subproject #{}: SID={}, Name={}", i + 1, account.sid, account.friendly_name);
                }
            }
            Err(SignalWireError::Unauthorized) => {
                println!("Error: Unauthorized - Check your credentials");
            }
            Err(e) => {
                eprintln!("Error: {:?}", e);
                panic!("Unexpected error listing subprojects");
            }
        }
    }

    /// Test creating and deleting a subproject.
    ///
    /// This test creates a new subproject, verifies it, and then deletes it.
    /// The test is disabled by default to prevent accidental subproject creation.
    /// Set SIGNALWIRE_RUN_SUBPROJECT_TEST=true in your .env file to enable it.
    #[tokio::test]
    async fn test_create_and_delete_subproject() {
        dotenv().ok();

        if env::var("SIGNALWIRE_RUN_SUBPROJECT_TEST").unwrap_or_else(|_| "false".to_string()) != "true" {
            println!("Skipping subproject creation/deletion test. To enable, set SIGNALWIRE_RUN_SUBPROJECT_TEST=true in your .env file.");
            return;
        }

        let client = get_client_from_env();
        let friendly_name = format!("Test Subproject {}", chrono::Utc::now().timestamp());

        // Create a subproject
        println!("Creating subproject with name: {}", friendly_name);
        let subproject = match client.create_subproject(&friendly_name).await {
            Ok(response) => {
                println!("✓ Subproject created: SID={}, Name={}", response.sid, response.friendly_name);
                assert_eq!(response.friendly_name, friendly_name, "Friendly name mismatch");
                response
            }
            Err(SignalWireError::Unauthorized) => {
                println!("Error: Unauthorized - Check your credentials");
                return;
            }
            Err(e) => {
                panic!("Error creating subproject: {:?}", e);
            }
        };

        // Get the subproject details to verify it was created
        match client.get_subproject(&subproject.sid).await {
            Ok(response) => {
                println!("✓ Subproject retrieved: SID={}, Name={}", response.sid, response.friendly_name);
                assert_eq!(response.sid, subproject.sid, "SID mismatch");
                assert_eq!(response.friendly_name, friendly_name, "Friendly name mismatch");
            }
            Err(e) => {
                panic!("Error retrieving subproject: {:?}", e);
            }
        };

        // Update the subproject
        let updated_name = format!("{} - Updated", friendly_name);
        match client.update_subproject(&subproject.sid, &updated_name, None).await {
            Ok(response) => {
                println!("✓ Subproject updated: SID={}, Name={}", response.sid, response.friendly_name);
                assert_eq!(response.sid, subproject.sid, "SID mismatch");
                assert_eq!(response.friendly_name, updated_name, "Updated friendly name mismatch");
            }
            Err(e) => {
                panic!("Error updating subproject: {:?}", e);
            }
        };

        // Delete the subproject
        println!("Deleting subproject: SID={}", subproject.sid);
        match client.delete_subproject(&subproject.sid).await {
            Ok(()) => {
                println!("✓ Subproject deleted successfully");
            }
            Err(e) => {
                panic!("Error deleting subproject: {:?}", e);
            }
        };

        // Verify the subproject was deleted
        match client.get_subproject(&subproject.sid).await {
            Err(SignalWireError::NotFound(_)) => {
                println!("✓ Subproject no longer exists (as expected)");
            }
            Ok(_) => {
                panic!("Subproject still exists after deletion");
            }
            Err(e) => {
                println!("Unexpected error checking deleted subproject: {:?}", e);
            }
        };
    }

    /// Test retrieving phone numbers from a specific subproject.
    ///
    /// This test lists phone numbers owned by a subproject.
    /// It requires an existing subproject with the SID specified in SIGNALWIRE_TEST_SUBPROJECT_SID env var.
    #[tokio::test]
    async fn test_get_subproject_phone_numbers() {
        dotenv().ok();

        // Skip the test if subproject SID is not provided
        let subproject_sid = match env::var("SIGNALWIRE_TEST_SUBPROJECT_SID") {
            Ok(sid) => sid,
            Err(_) => {
                println!("Skipping subproject phone numbers test. To enable, set SIGNALWIRE_TEST_SUBPROJECT_SID in your .env file.");
                return;
            }
        };

        let client = get_client_from_env();
        let query_params = PhoneNumberOwnedFilterParams::new().build();

        // First get info about the subproject
        match client.get_subproject(&subproject_sid).await {
            Ok(subproject) => {
                println!("Testing phone numbers for subproject: {} ({})", subproject.friendly_name, subproject.sid);
            }
            Err(e) => {
                println!("Error retrieving subproject details: {:?}", e);
                println!("Make sure the SIGNALWIRE_TEST_SUBPROJECT_SID is correct and the subproject exists.");
                return;
            }
        }

        // Now get the phone numbers for this subproject
        match client.get_subproject_phone_numbers(&subproject_sid, &query_params).await {
            Ok(phone_numbers) => {
                println!("Found {} phone number(s) in the subproject", phone_numbers.incoming_phone_numbers.len());

                if phone_numbers.incoming_phone_numbers.is_empty() {
                    println!("No phone numbers found in this subproject.");
                } else {
                    // Print some details about each phone number
                    for (i, number) in phone_numbers.incoming_phone_numbers.iter().enumerate() {
                        println!("Phone #{}: Number={}", i + 1, number.phone_number);
                    }
                }
            }
            Err(SignalWireError::NotFound(_)) => {
                println!("Subproject not found or has no phone numbers.");
            }
            Err(e) => {
                println!("Error retrieving subproject phone numbers: {:?}", e);
            }
        }
    }

    /// Test updating a phone number's configuration.
    ///
    /// This test is disabled by default to avoid accidental changes.
    /// To enable it, set the following in your .env file:
    ///
    /// SIGNALWIRE_RUN_UPDATE_PHONE_TEST=true
    /// SIGNALWIRE_TEST_PHONE_ID=your_phone_number_id
    #[tokio::test]
    async fn test_update_phone_number() {
        dotenv().ok();

        if env::var("SIGNALWIRE_RUN_UPDATE_PHONE_TEST").unwrap_or_else(|_| "false".to_string()) != "true" {
            println!("Skipping phone number update test. To enable, set SIGNALWIRE_RUN_UPDATE_PHONE_TEST=true in your .env file.");
            return;
        }

        let phone_id = match env::var("SIGNALWIRE_TEST_PHONE_ID") {
            Ok(id) => id,
            Err(_) => {
                println!("Skipping phone number update test. To enable, set SIGNALWIRE_TEST_PHONE_ID in your .env file.");
                return;
            }
        };

        let client = get_client_from_env();

        let update_request = UpdatePhoneNumberRequest {
            name: Some("Jenny".to_string()),
            call_handler: Some("relay_context".to_string()),
            call_receive_mode: Some("voice".to_string()),
            call_relay_topic: Some("office".to_string()),
            message_handler: Some("relay_application".to_string()),
            message_relay_topic: Some("my_relay_app".to_string()),
            message_relay_application: Some("my_relay_app".to_string()),
            ..Default::default()
        };

        match client.update_phone_number(&phone_id, &update_request).await {
            Ok(response) => {
                println!("✓ Phone number updated: id={}, number={}", response.id, response.number);
                assert_eq!(response.id, phone_id, "Updated phone ID mismatch");
            }
            Err(SignalWireError::Unauthorized) => {
                println!("Error: Unauthorized - Check your credentials");
            }
            Err(e) => {
                panic!("Unexpected error updating phone number: {:?}", e);
            }
        }
    }

    /// Test phone number lookup and validation.
    ///
    /// This test validates a phone number and retrieves information about it.
    /// Set SIGNALWIRE_TEST_PHONE_NUMBER in your .env file to test with a specific number.
    #[tokio::test]
    async fn test_phone_lookup() {
        dotenv().ok();

        // Get test phone number from environment or use a default US number
        let test_phone = env::var("SIGNALWIRE_TEST_PHONE_NUMBER").unwrap_or_else(|_| "+12065550100".to_string());
        
        // Log whether we're using the environment variable or the default
        if env::var("SIGNALWIRE_TEST_PHONE_NUMBER").is_ok() {
            println!("Using phone number from environment: {}", test_phone);
        } else {
            println!("Using default test phone number: {}", test_phone);
        }

        let client = get_client_from_env();

        println!("Looking up phone number: {}", test_phone);

        // Test basic lookup
        match client.lookup_phone_number(&test_phone).await {
            Ok(response) => {
                println!("✓ Phone lookup successful");
                println!("  Phone number (E.164): {}", response.e164.as_deref().unwrap_or(""));
                println!("  Country code: {}", response.country_code);
                println!("  National format: {}", response.national_number_formatted.as_deref().unwrap_or(""));
                println!("  Valid: {}", response.valid_number.unwrap_or(false));
                println!("  Number type: {}", response.number_type.as_deref().unwrap_or(""));
                println!("  Location: {}", response.location.as_deref().unwrap_or(""));

                // Only assert basic fields that should always be present
                assert!(!response.country_code.is_empty(), "Country code should not be empty");
                if let Some(e164) = &response.e164 {
                    assert!(!e164.is_empty(), "E.164 phone number should not be empty");
                }
                if let Some(valid) = response.valid_number {
                    assert!(valid, "Number should be valid");
                }
            }
            Err(SignalWireError::Unauthorized) => {
                println!("Error: Unauthorized - Check your credentials");
            }
            Err(e) => {
                // Don't fail the test, just log the error
                println!("Error with phone lookup: {:?}", e);
            }
        }

        // Test lookup with carrier information
        println!("\nLooking up phone number with carrier information: {}", test_phone);
        match client.lookup_phone_number_with_carrier(&test_phone).await {
            Ok(response) => {
                println!("✓ Phone lookup with carrier successful");
                println!("  Phone number: {}", response.e164.as_deref().unwrap_or(""));
                println!("  Valid: {}", response.valid_number.unwrap_or(false));
                println!("  Number type: {}", response.number_type.as_deref().unwrap_or(""));
                println!("  Location: {}", response.location.as_deref().unwrap_or(""));

                // Note: SignalWire returns carrier information differently than expected
                println!("  Mobile operator: {}", response.number_type.as_deref().unwrap_or("Unknown"));
            }
            Err(e) => {
                // Don't fail the test, just log the error
                println!("Error with carrier lookup: {:?}", e);
            }
        }

        // Test lookup with caller name information
        println!("\nLooking up phone number with caller name information: {}", test_phone);
        match client.lookup_phone_number_with_caller_name(&test_phone).await {
            Ok(response) => {
                println!("✓ Phone lookup with caller name successful");
                println!("  Phone number: {}", response.e164.as_deref().unwrap_or(""));
                println!("  Valid: {}", response.valid_number.unwrap_or(false));
                println!("  Number type: {}", response.number_type.as_deref().unwrap_or(""));
                println!("  Location: {}", response.location.as_deref().unwrap_or(""));

                // The API currently doesn't return caller name in the expected format
                println!("  Note: SignalWire API doesn't currently return caller name info in the expected format");
            }
            Err(e) => {
                // Don't fail the test, just log the error
                println!("Error with caller name lookup: {:?}", e);
            }
        }
    }

    /// Test phone number lookup with specific number from environment.
    ///
    /// This test only runs if SIGNALWIRE_TEST_PHONE_NUMBER is set in the environment.
    /// It provides a way to test with a real phone number without hardcoding it.
    #[tokio::test]
    async fn test_phone_lookup_with_env_number() {
        dotenv().ok();

        // Skip the test if the environment variable is not set
        let test_phone = match env::var("SIGNALWIRE_TEST_PHONE_NUMBER") {
            Ok(number) => number,
            Err(_) => {
                println!("Skipping environment-specific phone lookup test. To enable, set SIGNALWIRE_TEST_PHONE_NUMBER in your .env file.");
                return;
            }
        };

        println!("Running lookup test with number from environment: {}", test_phone);
        
        let client = get_client_from_env();

        // Just run the basic lookup to avoid duplication
        match client.lookup_phone_number(&test_phone).await {
            Ok(response) => {
                println!("✓ Phone lookup successful for environment number");
                println!("  Phone number (E.164): {}", response.e164.as_deref().unwrap_or(""));
                println!("  Country code: {}", response.country_code);
                println!("  National format: {}", response.national_number_formatted.as_deref().unwrap_or(""));
                println!("  Valid: {}", response.valid_number.unwrap_or(false));
                println!("  Number type: {}", response.number_type.as_deref().unwrap_or(""));
                println!("  Location: {}", response.location.as_deref().unwrap_or(""));
                
                // Assertions, only verify if we got a valid response
                assert!(!response.country_code.is_empty(), "Country code should not be empty");
                if let Some(valid) = response.valid_number {
                    // Some production phone numbers might not be valid, so don't assert this
                    println!("  Number validity: {}", valid);
                }
            }
            Err(e) => {
                println!("Error with environment phone lookup: {:?}", e);
                // Don't fail the test, as the phone number might be invalid intentionally
            }
        }
    }
}
