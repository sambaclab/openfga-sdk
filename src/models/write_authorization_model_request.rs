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
pub struct WriteAuthorizationModelRequest {
    #[serde(rename = "type_definitions")]
    pub type_definitions: Vec<crate::models::TypeDefinition>,
    #[serde(rename = "schema_version")]
    pub schema_version: String,
    #[serde(rename = "conditions", skip_serializing_if = "Option::is_none")]
    pub conditions: Option<::std::collections::HashMap<String, crate::models::Condition>>,
}

impl WriteAuthorizationModelRequest {
    pub fn new(
        type_definitions: Vec<crate::models::TypeDefinition>,
        schema_version: String,
    ) -> WriteAuthorizationModelRequest {
        WriteAuthorizationModelRequest {
            type_definitions,
            schema_version,
            conditions: None,
        }
    }
}
