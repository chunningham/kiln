/*
 * Tezos RPC
 *
 * Tezos client RPC API.
 *
 * The version of the OpenAPI document: 7.5
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Origination2Metadata {
    #[serde(rename = "balance_updates")]
    pub balance_updates: Vec<crate::models::OneOfobjectobjectobjectobject>,
    #[serde(rename = "operation_result")]
    pub operation_result: crate::models::Model007PsDelph1OperationAlphaOperationResultOrigination,
    #[serde(rename = "internal_operation_results", skip_serializing_if = "Option::is_none")]
    pub internal_operation_results: Option<Vec<crate::models::Model007PsDelph1OperationAlphaInternalOperationResult>>,
}

impl Origination2Metadata {
    pub fn new(balance_updates: Vec<crate::models::OneOfobjectobjectobjectobject>, operation_result: crate::models::Model007PsDelph1OperationAlphaOperationResultOrigination) -> Origination2Metadata {
        Origination2Metadata {
            balance_updates,
            operation_result,
            internal_operation_results: None,
        }
    }
}


