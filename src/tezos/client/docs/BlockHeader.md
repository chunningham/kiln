# BlockHeader

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**protocol** | **String** |  | 
**chain_id** | [**crate::models::ChainId**](Chain_id.md) |  | 
**hash** | [**crate::models::BlockHash**](block_hash.md) |  | 
**level** | **i32** |  | 
**proto** | **i32** |  | 
**predecessor** | [**crate::models::BlockHash**](block_hash.md) |  | 
**timestamp** | [**crate::models::TimestampProtocol**](timestamp.protocol.md) |  | 
**validation_pass** | **i32** |  | 
**operations_hash** | [**crate::models::OperationListListHash**](Operation_list_list_hash.md) |  | 
**fitness** | **Vec<String>** | The fitness, or score, of a block, that allow the Tezos to decide which chain is the best. A fitness value is a list of byte sequences. They are compared as follows: shortest lists are smaller; lists of the same length are compared according to the lexicographical order. | 
**context** | [**crate::models::ContextHash**](Context_hash.md) |  | 
**content** | [**crate::models::OneOfobjectobject**](oneOf<object,object>.md) |  | 
**signature** | [**crate::models::Signature**](Signature.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


