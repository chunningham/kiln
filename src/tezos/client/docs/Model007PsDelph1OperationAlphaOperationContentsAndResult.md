# Model007PsDelph1OperationAlphaOperationContentsAndResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**kind** | **String** |  | 
**level** | **i32** |  | 
**metadata** | [**crate::models::Delegation2Metadata**](Delegation_2_metadata.md) |  | 
**nonce** | **String** |  | 
**op1** | [**crate::models::Model007PsDelph1InlinedEndorsement**](007-PsDELPH1.inlined.endorsement.md) |  | 
**op2** | [**crate::models::Model007PsDelph1InlinedEndorsement**](007-PsDELPH1.inlined.endorsement.md) |  | 
**bh1** | [**crate::models::Model007PsDelph1BlockHeaderAlphaFullHeader**](007-PsDELPH1.block_header.alpha.full_header.md) |  | 
**bh2** | [**crate::models::Model007PsDelph1BlockHeaderAlphaFullHeader**](007-PsDELPH1.block_header.alpha.full_header.md) |  | 
**pkh** | [**crate::models::Ed25519PublicKeyHash**](Ed25519.Public_key_hash.md) |  | 
**secret** | **String** |  | 
**source** | [**crate::models::SignaturePublicKeyHash**](Signature.Public_key_hash.md) |  | 
**period** | **i32** |  | 
**proposals** | [**Vec<crate::models::ProtocolHash>**](Protocol_hash.md) |  | 
**proposal** | [**crate::models::ProtocolHash**](Protocol_hash.md) |  | 
**ballot** | **String** |  | 
**fee** | **String** | Decimal representation of a positive big number | 
**counter** | **String** | Decimal representation of a positive big number | 
**gas_limit** | **String** | Decimal representation of a positive big number | 
**storage_limit** | **String** | Decimal representation of a positive big number | 
**public_key** | [**crate::models::SignaturePublicKey**](Signature.Public_key.md) |  | 
**amount** | **String** | Decimal representation of a positive big number | 
**destination** | [**crate::models::Model007PsDelph1ContractId**](007-PsDELPH1.contract_id.md) |  | 
**parameters** | Option<[**crate::models::TransactionParameters**](Transaction_parameters.md)> |  | [optional]
**balance** | **String** | Decimal representation of a positive big number | 
**delegate** | Option<[**crate::models::SignaturePublicKeyHash**](Signature.Public_key_hash.md)> |  | [optional]
**script** | [**crate::models::Model007PsDelph1ScriptedContracts**](007-PsDELPH1.scripted.contracts.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


