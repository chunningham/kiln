# InlineResponse2007

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**level** | **i32** | The level of the block relative to genesis. This is also the Shell's notion of level | 
**level_position** | **i32** | The level of the block relative to the block that starts protocol alpha. This is specific to the protocol alpha. Other protocols might or might not include a similar notion. | 
**cycle** | **i32** | The current cycle's number. Note that cycles are a protocol-specific notion. As a result, the cycle number starts at 0 with the first block of protocol alpha. | 
**cycle_position** | **i32** | The current level of the block relative to the first block of the current cycle. | 
**voting_period** | **i32** | The current voting period's index. Note that cycles are a protocol-specific notion. As a result, the voting period index starts at 0 with the first block of protocol alpha. | 
**voting_period_position** | **i32** | The current level of the block relative to the first block of the current voting period. | 
**expected_commitment** | **bool** | Tells wether the baker of this block has to commit a seed nonce hash. | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


