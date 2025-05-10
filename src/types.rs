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
    pub price: Option<String>,
    pub price_unit: Option<String>,
    pub error_code: Option<String>,
    pub error_message: Option<String>,
    pub uri: String,
    #[serde(default)]
    pub subresource_uris: SubresourceUris,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubresourceUris {
    #[serde(default)]
    pub media: String,
}
