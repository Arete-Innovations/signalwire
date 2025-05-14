use reqwest::{Client as HttpClient, Url};

use crate::{errors::SignalWireError, types::*};

#[derive(Debug)]
pub struct SignalWireClient {
    pub project_id: String,
    pub api_key: String,
    pub space_name: String,
    pub http_client: HttpClient,
}

impl SignalWireClient {
    /// Creates a new SignalWire client.
    ///
    /// # Arguments
    ///
    /// * `space_name` - The space name of your SignalWire project.
    /// * `project_id` - The project ID for authentication.
    /// * `api_key` - The API key for authentication.
    ///
    /// # Returns
    ///
    /// A new instance of `SignalWireClient`.
    pub fn new(space_name: &str, project_id: &str, api_key: &str) -> Self {
        SignalWireClient {
            space_name: space_name.to_string(),
            project_id: project_id.to_string(),
            api_key: api_key.to_string(),
            http_client: HttpClient::new(),
        }
    }

    /// Retrieves a JSON Web Token (JWT) and a refresh token for authentication.
    ///
    /// This method fetches a JWT used for authenticating further requests to the SignalWire API.
    /// Both a JWT token and a refresh token are returned upon a successful call.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `JwtResponse` with `jwt_token` and `refresh_token` if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn get_jwt(&self) -> Result<JwtResponse, SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/relay/rest/jwt", self.space_name);
        let response = self
            .http_client
            .post(&url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .header("Content-Length", "0")
            .body("")
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        }

        let jwt_response: JwtResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        Ok(jwt_response)
    }

    /// Blocking version of `get_jwt`.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `JwtResponse` with `jwt_token` and `refresh_token` if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.

    #[cfg_attr(feature = "blocking", doc = "Blocking version of `get_jwt`.")]
    #[cfg(feature = "blocking")]
    pub fn get_jwt_blocking(&self) -> Result<JwtResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_jwt())
    }

    /// Fetches available phone numbers for a given country.
    /// Currently the only country supported by SignalWire is "US".
    ///
    /// # Arguments
    ///
    /// * `iso_country` - The ISO country code to query against.
    /// * `query_params` - Additional query parameters as key-value pairs.
    ///
    /// # Returns
    ///
    /// A `Result` containing either an `PhoneNumbersAvailableResponse` or a `SignalWireError`.
    pub async fn get_phone_numbers_available(&self, iso_country: &str, query_params: &[(String, String)]) -> Result<PhoneNumbersAvailableResponse, SignalWireError> {
        let url = format!(
            "https://{}.signalwire.com/api/laml/2010-04-01/Accounts/{}/AvailablePhoneNumbers/{}/Local",
            self.space_name, self.project_id, iso_country
        );
        println!("URL: {}", url);

        let url = Url::parse_with_params(&url, query_params).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        let response = self
            .http_client
            .get(url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        }

        let phone_numbers_response: PhoneNumbersAvailableResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        Ok(phone_numbers_response)
    }

    /// Blocking version of `get_phone_numbers_available`.
    ///
    /// # Arguments
    ///
    /// * `iso_country` - The ISO country code to query against.
    /// * `query_params` - Additional query parameters as key-value pairs.
    ///
    /// # Returns
    ///
    /// A `Result` containing either an `PhoneNumbersAvailableResponse` or a `SignalWireError`.

    #[cfg_attr(feature = "blocking", doc = "Blocking version of `get_phone_numbers_available`.")]
    #[cfg(feature = "blocking")]
    pub fn get_phone_numbers_available_blocking(&self, iso_country: &str, query_params: &[(String, String)]) -> Result<PhoneNumbersAvailableResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_phone_numbers_available(iso_country, query_params))
    }

    /// Retrieves a list of phone numbers owned by the client.
    ///
    /// # Arguments
    ///
    /// * `query_params` - Additional query parameters as key-value pairs.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `PhoneNumbersOwnedResponse` with detailed phone number info if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn get_phone_numbers_owned(&self, query_params: &[(String, String)]) -> Result<PhoneNumbersOwnedResponse, SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/relay/rest/phone_numbers", self.space_name);

        let url = Url::parse_with_params(&url, query_params).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        let response = self
            .http_client
            .get(url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        } else if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        } else {
            let phone_numbers_response: PhoneNumbersOwnedResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

            Ok(phone_numbers_response)
        }
    }

    /// Blocking version of `get_phone_numbers_owned`.
    ///
    /// # Arguments
    ///
    /// * `query_params` - Additional query parameters as key-value pairs.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `OwnedPhoneNumbersResponse` with detailed phone number info if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    #[cfg_attr(feature = "blocking", doc = "Blocking version of `get_phone_numbers_owned`.")]
    #[cfg(feature = "blocking")]
    pub fn get_phone_numbers_owned_blocking(&self, query_params: &[(String, String)]) -> Result<PhoneNumbersOwnedResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_phone_numbers_owned(query_params))
    }

    /// Buy a phone number.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - The phone number to buy.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `BuyPhoneNumberResponse` with detailed phone number info if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.

    pub async fn buy_phone_number(&self, phone_number: &str) -> Result<BuyPhoneNumberResponse, SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/relay/rest/phone_numbers", self.space_name);

        let response = self
            .http_client
            .post(&url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .json(&BuyPhoneNumberRequest { number: phone_number.to_string() })
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        }

        let buy_phone_number_response: BuyPhoneNumberResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        Ok(buy_phone_number_response)
    }

    /// Blocking version of `buy_phone_number`.
    ///
    /// # Arguments
    ///
    /// * `phone_number` - The phone number to buy.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `BuyPhoneNumberResponse` with detailed phone number info if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.

    #[cfg_attr(feature = "blocking", doc = "Blocking version of `buy_phone_number`.")]
    #[cfg(feature = "blocking")]
    pub fn buy_phone_number_blocking(&self, phone_number: &str) -> Result<BuyPhoneNumberResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.buy_phone_number(phone_number))
    }

    /// Sends an SMS message using the SignalWire API.
    ///
    /// # Arguments
    ///
    /// * `message` - The SMS message details including `body`, `from`, and `to`.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SmsResponse` with details about the sent message if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn send_sms(&self, message: &SmsMessage) -> Result<SmsResponse, SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/laml/2010-04-01/Accounts/{}/Messages", self.space_name, self.project_id);

        let form = [("From", &message.from), ("To", &message.to), ("Body", &message.body)];

        let response = self
            .http_client
            .post(&url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .form(&form)
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        } else if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        }

        let sms_response: SmsResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(format!("Failed to parse response: {}. Response was: {}", e, response_text)))?;

        Ok(sms_response)
    }

    /// Blocking version of `send_sms`.
    ///
    /// # Arguments
    ///
    /// * `message` - The SMS message details including `body`, `from`, and `to`.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SmsResponse` with details about the sent message if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    #[cfg_attr(feature = "blocking", doc = "Blocking version of `send_sms`.")]
    #[cfg(feature = "blocking")]
    pub fn send_sms_blocking(&self, message: &SmsMessage) -> Result<SmsResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.send_sms(message))
    }

    /// Get the status of a message by its SID (message identifier).
    ///
    /// This method allows you to check the current delivery status of a message
    /// that was previously sent via the SignalWire API.
    ///
    /// # Arguments
    ///
    /// * `message_sid` - The SID (unique identifier) of the message to check
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SmsResponse` with the complete message details, including its current status
    /// - `SignalWireError` if the request fails or the message can't be found
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the message SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn get_message_status(&self, message_sid: &str) -> Result<SmsResponse, SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/laml/2010-04-01/Accounts/{}/Messages/{}", self.space_name, self.project_id, message_sid);

        let response = self
            .http_client
            .get(&url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        } else if status == reqwest::StatusCode::NOT_FOUND {
            return Err(SignalWireError::NotFound(format!("Message with SID {} not found", message_sid)));
        } else if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        }

        let sms_response: SmsResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(format!("Failed to parse response: {}. Response was: {}", e, response_text)))?;

        Ok(sms_response)
    }

    /// Blocking version of `get_message_status`.
    ///
    /// # Arguments
    ///
    /// * `message_sid` - The SID (unique identifier) of the message to check
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SmsResponse` with the complete message details, including its current status
    /// - `SignalWireError` if the request fails or the message can't be found
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the message SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    #[cfg_attr(feature = "blocking", doc = "Blocking version of `get_message_status`.")]
    #[cfg(feature = "blocking")]
    pub fn get_message_status_blocking(&self, message_sid: &str) -> Result<SmsResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_message_status(message_sid))
    }

    // ---------- Subproject (Account) Methods ----------

    /// Lists all subprojects (accounts) for the current project.
    ///
    /// This method returns a list that contains the current Project and any subprojects.
    ///
    /// # Arguments
    ///
    /// * `query_params` - Optional query parameters for filtering subprojects
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectsListResponse` with the list of subprojects if successful
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn list_subprojects(&self, query_params: &[(String, String)]) -> Result<SubprojectsListResponse, SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/laml/2010-04-01/Accounts", self.space_name);

        let url = Url::parse_with_params(&url, query_params).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        let response = self
            .http_client
            .get(url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        } else if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        }

        let subprojects_response: SubprojectsListResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(format!("Failed to parse response: {}. Response was: {}", e, response_text)))?;

        Ok(subprojects_response)
    }

    /// Blocking version of `list_subprojects`.
    ///
    /// # Arguments
    ///
    /// * `query_params` - Optional query parameters for filtering subprojects
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectsListResponse` with the list of subprojects if successful
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    #[cfg_attr(feature = "blocking", doc = "Blocking version of `list_subprojects`.")]
    #[cfg(feature = "blocking")]
    pub fn list_subprojects_blocking(&self, query_params: &[(String, String)]) -> Result<SubprojectsListResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.list_subprojects(query_params))
    }

    /// Get details for a specific subproject (account).
    ///
    /// # Arguments
    ///
    /// * `subproject_sid` - The SID (unique identifier) of the subproject to retrieve
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectResponse` with the subproject details if successful
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the subproject SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn get_subproject(&self, subproject_sid: &str) -> Result<SubprojectResponse, SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/laml/2010-04-01/Accounts/{}", self.space_name, subproject_sid);

        let response = self
            .http_client
            .get(&url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        } else if status == reqwest::StatusCode::NOT_FOUND {
            return Err(SignalWireError::NotFound(format!("Subproject with SID {} not found", subproject_sid)));
        } else if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        }

        let subproject_response: SubprojectResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(format!("Failed to parse response: {}. Response was: {}", e, response_text)))?;

        Ok(subproject_response)
    }

    /// Blocking version of `get_subproject`.
    ///
    /// # Arguments
    ///
    /// * `subproject_sid` - The SID (unique identifier) of the subproject to retrieve
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectResponse` with the subproject details if successful
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the subproject SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    #[cfg_attr(feature = "blocking", doc = "Blocking version of `get_subproject`.")]
    #[cfg(feature = "blocking")]
    pub fn get_subproject_blocking(&self, subproject_sid: &str) -> Result<SubprojectResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_subproject(subproject_sid))
    }

    /// Creates a new subproject (account) within the current project.
    ///
    /// # Arguments
    ///
    /// * `friendly_name` - A human-readable name for the subproject
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectResponse` with the details of the created subproject if successful
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn create_subproject(&self, friendly_name: &str) -> Result<SubprojectResponse, SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/laml/2010-04-01/Accounts", self.space_name);

        let form = [("FriendlyName", friendly_name)];

        let response = self
            .http_client
            .post(&url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .form(&form)
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        } else if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        }

        let subproject_response: SubprojectResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(format!("Failed to parse response: {}. Response was: {}", e, response_text)))?;

        Ok(subproject_response)
    }

    /// Blocking version of `create_subproject`.
    ///
    /// # Arguments
    ///
    /// * `friendly_name` - A human-readable name for the subproject
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectResponse` with the details of the created subproject if successful
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    #[cfg_attr(feature = "blocking", doc = "Blocking version of `create_subproject`.")]
    #[cfg(feature = "blocking")]
    pub fn create_subproject_blocking(&self, friendly_name: &str) -> Result<SubprojectResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.create_subproject(friendly_name))
    }

    /// Updates an existing subproject (account).
    ///
    /// # Arguments
    ///
    /// * `subproject_sid` - The SID (unique identifier) of the subproject to update
    /// * `friendly_name` - A new human-readable name for the subproject
    /// * `status` - Optional status to set for the subproject ("active" or "suspended")
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectResponse` with the updated subproject details if successful
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the subproject SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn update_subproject(&self, subproject_sid: &str, friendly_name: &str, status: Option<&str>) -> Result<SubprojectResponse, SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/laml/2010-04-01/Accounts/{}", self.space_name, subproject_sid);

        let mut form = vec![("FriendlyName", friendly_name)];
        if let Some(status_value) = status {
            form.push(("Status", status_value));
        }

        let response = self
            .http_client
            .post(&url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .form(&form)
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        } else if status == reqwest::StatusCode::NOT_FOUND {
            return Err(SignalWireError::NotFound(format!("Subproject with SID {} not found", subproject_sid)));
        } else if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        }

        let subproject_response: SubprojectResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(format!("Failed to parse response: {}. Response was: {}", e, response_text)))?;

        Ok(subproject_response)
    }

    /// Blocking version of `update_subproject`.
    ///
    /// # Arguments
    ///
    /// * `subproject_sid` - The SID (unique identifier) of the subproject to update
    /// * `friendly_name` - A new human-readable name for the subproject
    /// * `status` - Optional status to set for the subproject ("active" or "suspended")
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectResponse` with the updated subproject details if successful
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the subproject SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    #[cfg_attr(feature = "blocking", doc = "Blocking version of `update_subproject`.")]
    #[cfg(feature = "blocking")]
    pub fn update_subproject_blocking(&self, subproject_sid: &str, friendly_name: &str, status: Option<&str>) -> Result<SubprojectResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.update_subproject(subproject_sid, friendly_name, status))
    }

    /// Deletes a subproject (account).
    ///
    /// # Arguments
    ///
    /// * `subproject_sid` - The SID (unique identifier) of the subproject to delete
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `()` if the subproject was successfully deleted
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the subproject SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn delete_subproject(&self, subproject_sid: &str) -> Result<(), SignalWireError> {
        let url = format!("https://{}.signalwire.com/api/laml/2010-04-01/Accounts/{}", self.space_name, subproject_sid);

        let response = self
            .http_client
            .delete(&url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        } else if status == reqwest::StatusCode::NOT_FOUND {
            return Err(SignalWireError::NotFound(format!("Subproject with SID {} not found", subproject_sid)));
        } else if status.is_client_error() || status.is_server_error() {
            let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;
            return Err(SignalWireError::Unexpected(response_text));
        }

        // Success, return empty result
        Ok(())
    }

    /// Blocking version of `delete_subproject`.
    ///
    /// # Arguments
    ///
    /// * `subproject_sid` - The SID (unique identifier) of the subproject to delete
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `()` if the subproject was successfully deleted
    /// - `SignalWireError` if the request fails
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the subproject SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    #[cfg_attr(feature = "blocking", doc = "Blocking version of `delete_subproject`.")]
    #[cfg(feature = "blocking")]
    pub fn delete_subproject_blocking(&self, subproject_sid: &str) -> Result<(), SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.delete_subproject(subproject_sid))
    }

    // ---------- Subproject Resource Methods ----------

    /// Lists phone numbers owned by a specific subproject.
    ///
    /// This method allows you to retrieve all phone numbers that belong to a specific subproject
    /// using your main account's credentials.
    ///
    /// # Arguments
    ///
    /// * `subproject_sid` - The SID (unique identifier) of the subproject
    /// * `query_params` - Additional query parameters as key-value pairs.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectPhoneNumbersResponse` with detailed phone number info if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the subproject SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn get_subproject_phone_numbers(&self, subproject_sid: &str, query_params: &[(String, String)]) -> Result<SubprojectPhoneNumbersResponse, SignalWireError> {
        // First check if the subproject exists
        self.get_subproject(subproject_sid).await?;

        // URL to get phone numbers from a specific subproject
        let url = format!("https://{}.signalwire.com/api/laml/2010-04-01/Accounts/{}/IncomingPhoneNumbers", self.space_name, subproject_sid);

        let url = Url::parse_with_params(&url, query_params).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        let response = self
            .http_client
            .get(url)
            .basic_auth(&self.project_id, Some(&self.api_key))
            .send()
            .await
            .map_err(|e| SignalWireError::HttpError(e.to_string()))?;

        let status = response.status();
        let response_text = response.text().await.map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        if status == reqwest::StatusCode::UNAUTHORIZED {
            return Err(SignalWireError::Unauthorized);
        } else if status == reqwest::StatusCode::NOT_FOUND {
            return Err(SignalWireError::NotFound(format!("Subproject with SID {} not found", subproject_sid)));
        } else if status.is_client_error() || status.is_server_error() {
            return Err(SignalWireError::Unexpected(response_text));
        }

        let phone_numbers_response: SubprojectPhoneNumbersResponse =
            serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(format!("Failed to parse response: {}. Response was: {}", e, response_text)))?;

        Ok(phone_numbers_response)
    }

    /// Blocking version of `get_subproject_phone_numbers`.
    ///
    /// # Arguments
    ///
    /// * `subproject_sid` - The SID (unique identifier) of the subproject
    /// * `query_params` - Additional query parameters as key-value pairs.
    ///
    /// # Returns
    ///
    /// A `Result` containing either:
    /// - `SubprojectPhoneNumbersResponse` with detailed phone number info if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Returns `SignalWireError::NotFound` if the subproject SID doesn't exist.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    #[cfg_attr(feature = "blocking", doc = "Blocking version of `get_subproject_phone_numbers`.")]
    #[cfg(feature = "blocking")]
    pub fn get_subproject_phone_numbers_blocking(&self, subproject_sid: &str, query_params: &[(String, String)]) -> Result<SubprojectPhoneNumbersResponse, SignalWireError> {
        tokio::runtime::Runtime::new().unwrap().block_on(self.get_subproject_phone_numbers(subproject_sid, query_params))
    }
}
