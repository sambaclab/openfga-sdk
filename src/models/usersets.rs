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
pub struct Usersets {
    #[serde(rename = "child")]
    pub child: Vec<crate::models::Userset>,
}

impl Usersets {
    pub fn new(child: Vec<crate::models::Userset>) -> Usersets {
        Usersets { child }
    }
}
