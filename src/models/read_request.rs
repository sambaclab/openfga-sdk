/*
 * OpenFGA
 *
 * A high performance and flexible authorization/permission engine built for developers and inspired by Google Zanzibar.
 *
 * The version of the OpenAPI document: 0.1
 * Contact: community@openfga.dev
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ReadRequest {
    #[serde(rename = "tuple_key", skip_serializing_if = "Option::is_none")]
    pub tuple_key: Option<Box<crate::models::ReadRequestTupleKey>>,
    #[serde(rename = "page_size", skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(rename = "continuation_token", skip_serializing_if = "Option::is_none")]
    pub continuation_token: Option<String>,
}

impl ReadRequest {
    pub fn new() -> ReadRequest {
        ReadRequest {
            tuple_key: None,
            page_size: None,
            continuation_token: None,
        }
    }
}
