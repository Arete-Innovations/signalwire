use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JwtResponse {
    pub jwt_token: String,
    pub refresh_token: String,
}

#[derive(Default)]
pub struct PhoneNumberAvailableQueryParams {
    params: Vec<(String, String)>,
}

impl PhoneNumberAvailableQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn area_code(mut self, code: &str) -> Self {
        self.params.push(("AreaCode".to_string(), code.to_string()));
        self
    }

    pub fn beta(mut self, beta: bool) -> Self {
        self.params.push(("Beta".to_string(), beta.to_string()));
        self
    }

    pub fn contains(mut self, value: &str) -> Self {
        self.params.push(("Contains".to_string(), value.to_string()));
        self
    }

    pub fn exclude_all_address_required(mut self, value: bool) -> Self {
        self.params.push(("ExcludeAllAddressRequired".to_string(), value.to_string()));
        self
    }

    pub fn exclude_foreign_address_required(mut self, value: bool) -> Self {
        self.params.push(("ExcludeForeignAddressRequired".to_string(), value.to_string()));
        self
    }

    pub fn exclude_local_address_required(mut self, value: bool) -> Self {
        self.params.push(("ExcludeLocalAddressRequired".to_string(), value.to_string()));
        self
    }

    pub fn fax_enabled(mut self, enabled: bool) -> Self {
        self.params.push(("FaxEnabled".to_string(), enabled.to_string()));
        self
    }

    pub fn in_region(mut self, region: &str) -> Self {
        self.params.push(("InRegion".to_string(), region.to_string()));
        self
    }

    pub fn mms_enabled(mut self, enabled: bool) -> Self {
        self.params.push(("MmsEnabled".to_string(), enabled.to_string()));
        self
    }

    pub fn sms_enabled(mut self, enabled: bool) -> Self {
        self.params.push(("SmsEnabled".to_string(), enabled.to_string()));
        self
    }

    pub fn voice_enabled(mut self, enabled: bool) -> Self {
        self.params.push(("VoiceEnabled".to_string(), enabled.to_string()));
        self
    }

    pub fn build(self) -> Vec<(String, String)> {
        self.params
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneNumbersAvailableResponse {
    pub uri: String,
    #[serde(rename = "available_phone_numbers")]
    pub phone_numbers_available: Vec<PhoneNumberAvailable>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneNumberAvailable {
    pub beta: bool,
    pub capabilities: Capabilities,
    pub friendly_name: String,
    pub iso_country: String,
    pub lata: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub phone_number: String,
    pub postal_code: Option<String>,
    pub rate_center: String,
    pub region: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Capabilities {
    pub voice: Option<bool>,
    #[serde(rename = "SMS")]
    pub sms: Option<bool>,
    #[serde(rename = "MMS")]
    pub mms: Option<bool>,
    pub fax: Option<bool>,
}

#[derive(Default)]
pub struct PhoneNumberOwnedFilterParams {
    params: Vec<(String, String)>,
}

impl PhoneNumberOwnedFilterParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn filter_name(mut self, name: &str) -> Self {
        self.params.push(("filter_name".to_string(), name.to_string()));
        self
    }

    pub fn filter_number(mut self, number: &str) -> Self {
        self.params.push(("filter_number".to_string(), number.to_string()));
        self
    }

    pub fn build(self) -> Vec<(String, String)> {
        self.params
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneNumbersOwnedResponse {
    pub links: Links,
    pub data: Vec<Daum>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Links {
    #[serde(rename = "self")]
    pub self_field: String,
    pub first: String,
    pub next: Option<String>,
    pub prev: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Daum {
    pub id: String,
    pub number: String,
    pub name: Option<String>,
    pub call_handler: Option<String>,
    pub call_receive_mode: Option<String>,
    pub call_request_url: Option<String>,
    pub call_request_method: Option<String>,
    pub call_fallback_url: Option<String>,
    pub call_fallback_method: Option<String>,
    pub call_status_callback_url: Option<String>,
    pub call_status_callback_method: Option<String>,
    pub call_laml_application_id: Option<String>,
    pub call_dialogflow_agent_id: Option<String>,
    pub call_relay_topic: Option<String>,
    pub call_relay_topic_status_callback_url: Option<String>,
    pub call_relay_context: Option<String>,
    pub call_relay_context_status_callback_url: Option<String>,
    pub call_relay_application: Option<String>,
    pub call_relay_connector_id: Option<String>,
    pub call_sip_endpoint_id: Option<String>,
    pub call_verto_resource: Option<String>,
    pub call_video_room_id: Option<String>,
    pub message_handler: Option<String>,
    pub message_request_url: Option<String>,
    pub message_request_method: Option<String>,
    pub message_fallback_url: Option<String>,
    pub message_fallback_method: Option<String>,
    pub message_laml_application_id: Option<String>,
    pub message_relay_topic: Option<String>,
    pub message_relay_context: Option<String>,
    pub message_relay_application: Option<String>,
    pub capabilities: Vec<String>,
    pub number_type: Option<String>,
    pub e911_address_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub next_billed_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuyPhoneNumberRequest {
    pub number: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BuyPhoneNumberResponse {
    pub id: String,
    pub number: String,
    pub name: Option<String>,
    pub call_handler: Option<String>,
    pub call_receive_mode: Option<String>,
    pub call_request_url: Option<String>,
    pub call_request_method: Option<String>,
    pub call_fallback_url: Option<String>,
    pub call_fallback_method: Option<String>,
    pub call_status_callback_url: Option<String>,
    pub call_status_callback_method: Option<String>,
    pub call_laml_application_id: Option<String>,
    pub call_dialogflow_agent_id: Option<String>,
    pub call_relay_topic: Option<String>,
    pub call_relay_topic_status_callback_url: Option<String>,
    pub call_relay_context: Option<String>,
    pub call_relay_context_status_callback_url: Option<String>,
    pub call_relay_application: Option<String>,
    pub call_relay_connector_id: Option<String>,
    pub call_sip_endpoint_id: Option<String>,
    pub call_verto_resource: Option<String>,
    pub call_video_room_id: Option<String>,
    pub message_handler: Option<String>,
    pub message_request_url: Option<String>,
    pub message_request_method: Option<String>,
    pub message_fallback_url: Option<String>,
    pub message_fallback_method: Option<String>,
    pub message_laml_application_id: Option<String>,
    pub message_relay_topic: Option<String>,
    pub message_relay_context: Option<String>,
    pub message_relay_application: Option<String>,
    pub capabilities: Vec<String>,
    pub number_type: Option<String>,
    pub e911_address_id: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub next_billed_at: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmsMessage {
    pub body: String,
    pub from: String,
    pub to: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SmsResponse {
    pub sid: String,
    pub date_created: String,
    pub date_updated: String,
    pub date_sent: Option<String>,
    pub account_sid: String,
    pub to: String,
    pub from: String,
    pub messaging_service_sid: Option<String>,
    pub body: String,
    pub status: String,
    pub num_segments: i32,
    pub num_media: i32,
    pub direction: String,
    pub api_version: String,
    pub price: Option<f64>,
    pub price_unit: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub uri: String,
    #[serde(default)]
    pub subresource_uris: SubresourceUris,
}

impl SmsResponse {
    /// Get the message status as an enum value.
    ///
    /// This method converts the string status field to a more
    /// programmer-friendly enum variant.
    ///
    /// # Returns
    ///
    /// A `MessageStatus` enum representing the current status of the message.
    pub fn get_status(&self) -> MessageStatus {
        MessageStatus::from(self.status.as_str())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubresourceUris {
    #[serde(default)]
    pub media: String,
}

// Message status values according to SignalWire API
#[derive(Debug, Clone, PartialEq)]
pub enum MessageStatus {
    Queued,      // The message is queued and waiting to be sent
    Sending,     // The message is in the process of being sent
    Sent,        // The message has been sent to the carrier
    Delivered,   // The message has been delivered to the recipient
    Failed,      // The message failed to be sent
    Undelivered, // The message was sent but not delivered
    Unknown,     // The status is unknown
}

impl From<&str> for MessageStatus {
    fn from(status: &str) -> Self {
        match status.to_lowercase().as_str() {
            "queued" => MessageStatus::Queued,
            "sending" => MessageStatus::Sending,
            "sent" => MessageStatus::Sent,
            "delivered" => MessageStatus::Delivered,
            "failed" => MessageStatus::Failed,
            "undelivered" => MessageStatus::Undelivered,
            _ => MessageStatus::Unknown,
        }
    }
}

impl std::fmt::Display for MessageStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageStatus::Queued => write!(f, "queued"),
            MessageStatus::Sending => write!(f, "sending"),
            MessageStatus::Sent => write!(f, "sent"),
            MessageStatus::Delivered => write!(f, "delivered"),
            MessageStatus::Failed => write!(f, "failed"),
            MessageStatus::Undelivered => write!(f, "undelivered"),
            MessageStatus::Unknown => write!(f, "unknown"),
        }
    }
}

// Subproject (Account) related types
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubprojectResponse {
    pub sid: String,
    pub friendly_name: String,
    pub status: String,
    pub auth_token: String,
    pub date_created: String,
    pub date_updated: String,
    #[serde(rename = "type")]
    pub account_type: Option<String>,
    pub owner_account_sid: Option<String>,
    pub uri: Option<String>,
    pub subproject: Option<bool>,
    pub signing_key: Option<String>,
    pub subresource_uris: SubprojectResourceUris,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubprojectResourceUris {
    pub addresses: Option<String>,
    pub available_phone_numbers: Option<String>,
    pub applications: Option<String>,
    pub authorized_connect_apps: Option<String>,
    pub calls: Option<String>,
    pub conferences: Option<String>,
    pub connect_apps: Option<String>,
    pub incoming_phone_numbers: Option<String>,
    pub keys: Option<String>,
    pub notifications: Option<String>,
    pub outgoing_caller_ids: Option<String>,
    pub queues: Option<String>,
    pub recordings: Option<String>,
    pub sandbox: Option<String>,
    pub sip: Option<String>,
    pub short_codes: Option<String>,
    pub messages: Option<String>,
    pub transcriptions: Option<String>,
    pub usage: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubprojectsListResponse {
    pub uri: Option<String>,
    pub first_page_uri: String,
    pub next_page_uri: Option<String>,
    pub previous_page_uri: Option<String>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
    pub accounts: Vec<SubprojectResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreateSubprojectRequest {
    pub friendly_name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UpdateSubprojectRequest {
    pub friendly_name: String,
    pub status: Option<String>, // "active" or "suspended"
}

#[derive(Default)]
pub struct SubprojectQueryParams {
    params: Vec<(String, String)>,
}

impl SubprojectQueryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn friendly_name(mut self, friendly_name: &str) -> Self {
        self.params.push(("FriendlyName".to_string(), friendly_name.to_string()));
        self
    }

    pub fn status(mut self, status: &str) -> Self {
        self.params.push(("Status".to_string(), status.to_string()));
        self
    }

    pub fn build(self) -> Vec<(String, String)> {
        self.params
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubprojectPhoneNumbersResponse {
    pub uri: String,
    pub first_page_uri: String,
    pub next_page_uri: Option<String>,
    pub previous_page_uri: Option<String>,
    pub page: i32,
    pub page_size: i32,
    pub incoming_phone_numbers: Vec<SubprojectPhoneNumber>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubprojectPhoneNumber {
    pub sid: String,
    pub account_sid: String,
    pub friendly_name: String,
    pub phone_number: String,
    pub voice_url: Option<String>,
    pub voice_method: Option<String>,
    pub voice_fallback_url: Option<String>,
    pub voice_fallback_method: Option<String>,
    pub status_callback: Option<String>,
    pub status_callback_method: Option<String>,
    pub voice_caller_id_lookup: Option<bool>,
    pub voice_application_sid: Option<String>,
    pub date_created: String,
    pub date_updated: String,
    pub sms_url: Option<String>,
    pub sms_method: Option<String>,
    pub sms_fallback_url: Option<String>,
    pub sms_fallback_method: Option<String>,
    pub sms_application_sid: Option<String>,
    pub capabilities: PhoneNumberCapabilities,
    pub beta: bool,
    pub uri: String,
    pub trunk_sid: Option<String>,
    pub emergency_status: Option<String>,
    pub emergency_address_sid: Option<String>,
    pub emergency_address_status: Option<String>,
    pub status: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneNumberCapabilities {
    pub voice: bool,
    pub sms: bool,
    pub mms: bool,
    pub fax: bool,
}

// ---------- Lookup & Validation Types ----------

/// Response for phone number lookup requests
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PhoneLookupResponse {
    #[serde(rename = "country_code_number")]
    pub country_code_number: Option<i32>,
    #[serde(rename = "national_number")]
    pub national_number: Option<String>,
    #[serde(rename = "possible_number")]
    pub possible_number: Option<bool>,
    #[serde(rename = "valid_number")]
    pub valid_number: Option<bool>,
    #[serde(rename = "national_number_formatted")]
    pub national_number_formatted: Option<String>,
    #[serde(rename = "international_number_formatted")]
    pub international_number_formatted: Option<String>,
    #[serde(rename = "e164")]
    pub e164: Option<String>,
    #[serde(rename = "location")]
    pub location: Option<String>,
    #[serde(rename = "country_code")]
    pub country_code: String,
    #[serde(rename = "timezones")]
    pub timezones: Option<Vec<String>>,
    #[serde(rename = "number_type")]
    pub number_type: Option<String>,

    // Fields for backward compatibility with the old structure
    #[serde(skip_deserializing)]
    pub phone_number: String,
    #[serde(skip_deserializing)]
    pub national_format: String,
    #[serde(skip_deserializing)]
    pub valid: bool,
    #[serde(skip_deserializing)]
    pub validation_errors: Option<Vec<String>>,
    #[serde(skip_deserializing)]
    pub formatted: bool,
    #[serde(skip_deserializing)]
    pub url: Option<String>,

    // Optional carrier and caller name info
    #[serde(skip_deserializing)]
    pub carrier: Option<CarrierInfo>,
    #[serde(skip_deserializing)]
    pub caller_name: Option<CallerNameInfo>,
}

impl PhoneLookupResponse {
    /// Gets the actual phone number in E.164 format
    pub fn get_phone_number(&self) -> &str {
        self.e164.as_deref().unwrap_or("")
    }

    /// Gets the formatted national version of the phone number
    pub fn get_national_format(&self) -> &str {
        self.national_number_formatted.as_deref().unwrap_or("")
    }

    /// Gets whether the number is valid
    pub fn is_valid(&self) -> bool {
        self.valid_number.unwrap_or(false)
    }
}

/// Carrier information returned in a phone lookup response
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CarrierInfo {
    pub mobile_country_code: Option<String>,
    pub mobile_network_code: Option<String>,
    pub name: Option<String>,
    pub type_field: Option<String>,
    #[serde(rename = "error_code")]
    pub error_code: Option<String>,
}

/// Caller name information returned in a phone lookup response
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CallerNameInfo {
    pub caller_name: Option<String>,
    pub caller_type: Option<String>,
    pub error_code: Option<String>,
}

/// Parameters for phone number lookup
#[derive(Default)]
pub struct PhoneLookupParams {
    params: Vec<(String, String)>,
}

impl PhoneLookupParams {
    pub fn new() -> Self {
        Self::default()
    }

    /// Include carrier information in the lookup
    pub fn with_carrier(mut self) -> Self {
        self.params.push(("Type".to_string(), "carrier".to_string()));
        self
    }

    /// Include caller name (CNAM) information in the lookup
    pub fn with_caller_name(mut self) -> Self {
        self.params.push(("Type".to_string(), "caller-name".to_string()));
        self
    }

    /// Build the parameter list
    pub fn build(self) -> Vec<(String, String)> {
        self.params
    }
}
