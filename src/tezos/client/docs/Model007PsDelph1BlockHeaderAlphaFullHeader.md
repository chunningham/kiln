# Model007PsDelph1BlockHeaderAlphaFullHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**level** | **i32** |  | 
**proto** | **i32** |  | 
**predecessor** | [**crate::models::BlockHash**](block_hash.md) |  | 
**timestamp** | [**crate::models::TimestampProtocol**](timestamp.protocol.md) |  | 
**validation_pass** | **i32** |  | 
**operations_hash** | [**crate::models::OperationListListHash**](Operation_list_list_hash.md) |  | 
**fitness** | **Vec<String>** | The fitness, or score, of a block, that allow the Tezos to decide which chain is the best. A fitness value is a list of byte sequences. They are compared as follows: shortest lists are smaller; lists of the same length are compared according to the lexicographical order. | 
**context** | [**crate::models::ContextHash**](Context_hash.md) |  | 
**priority** | **i32** |  | 
**proof_of_work_nonce** | **String** |  | 
**seed_nonce_hash** | Option<[**crate::models::CycleNonce**](cycle_nonce.md)> |  | [optional]
**signature** | [**crate::models::Signature**](Signature.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


