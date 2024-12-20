use crate::errors::SignalWireError;
use crate::types::*;
use reqwest::Client as HttpClient;
use reqwest::Url;

impl Client {
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
    /// A new instance of `Client`.
    pub fn new(space_name: &str, project_id: &str, api_key: &str) -> Self {
        Client {
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
    /// A `Result` containing either an `AvailablePhoneNumbersResponse` or a `SignalWireError`.
    pub async fn get_available_phone_numbers(&self, iso_country: &str, query_params: &[(String, String)]) -> Result<AvailablePhoneNumbersResponse, SignalWireError> {
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

        let phone_numbers_response: AvailablePhoneNumbersResponse = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

        Ok(phone_numbers_response)
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
    /// - `OwnedPhoneNumbers` with detailed phone number info if successful.
    /// - `SignalWireError` if the request fails or is unauthorized.
    ///
    /// # Errors
    ///
    /// Returns `SignalWireError::Unauthorized` if authentication fails.
    /// Other `SignalWireError` variants may be returned for unexpected issues.
    pub async fn get_owned_phone_numbers(&self, query_params: &[(String, String)]) -> Result<OwnedPhoneNumbers, SignalWireError> {
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
            let phone_numbers_response: OwnedPhoneNumbers = serde_json::from_str(&response_text).map_err(|e| SignalWireError::Unexpected(e.to_string()))?;

            Ok(phone_numbers_response)
        }
    }
}
