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
pub struct AssertionTupleKey {
    #[serde(rename = "object")]
    pub object: String,
    #[serde(rename = "relation")]
    pub relation: String,
    #[serde(rename = "user")]
    pub user: String,
}

impl AssertionTupleKey {
    pub fn new(object: String, relation: String, user: String) -> AssertionTupleKey {
        AssertionTupleKey {
            object,
            relation,
            user,
        }
    }
}
