# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**context_big_maps_big_map_id_script_expr_get**](DefaultApi.md#context_big_maps_big_map_id_script_expr_get) | **get** /context/big_maps/{big_map_id}/{script_expr} | 
[**context_constants_errors_get**](DefaultApi.md#context_constants_errors_get) | **get** /context/constants/errors | 
[**context_constants_get**](DefaultApi.md#context_constants_get) | **get** /context/constants | 
[**context_contracts_contract_id_balance_get**](DefaultApi.md#context_contracts_contract_id_balance_get) | **get** /context/contracts/{contract_id}/balance | 
[**context_contracts_contract_id_big_map_get_post**](DefaultApi.md#context_contracts_contract_id_big_map_get_post) | **post** /context/contracts/{contract_id}/big_map_get | 
[**context_contracts_contract_id_counter_get**](DefaultApi.md#context_contracts_contract_id_counter_get) | **get** /context/contracts/{contract_id}/counter | 
[**context_contracts_contract_id_delegate_get**](DefaultApi.md#context_contracts_contract_id_delegate_get) | **get** /context/contracts/{contract_id}/delegate | 
[**context_contracts_contract_id_entrypoints_get**](DefaultApi.md#context_contracts_contract_id_entrypoints_get) | **get** /context/contracts/{contract_id}/entrypoints | 
[**context_contracts_contract_id_entrypoints_string_get**](DefaultApi.md#context_contracts_contract_id_entrypoints_string_get) | **get** /context/contracts/{contract_id}/entrypoints/{string} | 
[**context_contracts_contract_id_get**](DefaultApi.md#context_contracts_contract_id_get) | **get** /context/contracts/{contract_id} | 
[**context_contracts_contract_id_manager_key_get**](DefaultApi.md#context_contracts_contract_id_manager_key_get) | **get** /context/contracts/{contract_id}/manager_key | 
[**context_contracts_contract_id_script_get**](DefaultApi.md#context_contracts_contract_id_script_get) | **get** /context/contracts/{contract_id}/script | 
[**context_contracts_contract_id_storage_get**](DefaultApi.md#context_contracts_contract_id_storage_get) | **get** /context/contracts/{contract_id}/storage | 
[**context_contracts_get**](DefaultApi.md#context_contracts_get) | **get** /context/contracts | 
[**context_delegates_get**](DefaultApi.md#context_delegates_get) | **get** /context/delegates | 
[**context_delegates_pkh_balance_get**](DefaultApi.md#context_delegates_pkh_balance_get) | **get** /context/delegates/{pkh}/balance | 
[**context_delegates_pkh_deactivated_get**](DefaultApi.md#context_delegates_pkh_deactivated_get) | **get** /context/delegates/{pkh}/deactivated | 
[**context_delegates_pkh_delegated_balance_get**](DefaultApi.md#context_delegates_pkh_delegated_balance_get) | **get** /context/delegates/{pkh}/delegated_balance | 
[**context_delegates_pkh_delegated_contracts_get**](DefaultApi.md#context_delegates_pkh_delegated_contracts_get) | **get** /context/delegates/{pkh}/delegated_contracts | 
[**context_delegates_pkh_frozen_balance_by_cycle_get**](DefaultApi.md#context_delegates_pkh_frozen_balance_by_cycle_get) | **get** /context/delegates/{pkh}/frozen_balance_by_cycle | 
[**context_delegates_pkh_frozen_balance_get**](DefaultApi.md#context_delegates_pkh_frozen_balance_get) | **get** /context/delegates/{pkh}/frozen_balance | 
[**context_delegates_pkh_get**](DefaultApi.md#context_delegates_pkh_get) | **get** /context/delegates/{pkh} | 
[**context_delegates_pkh_grace_period_get**](DefaultApi.md#context_delegates_pkh_grace_period_get) | **get** /context/delegates/{pkh}/grace_period | 
[**context_delegates_pkh_staking_balance_get**](DefaultApi.md#context_delegates_pkh_staking_balance_get) | **get** /context/delegates/{pkh}/staking_balance | 
[**context_nonces_block_level_get**](DefaultApi.md#context_nonces_block_level_get) | **get** /context/nonces/{block_level} | 
[**context_raw_bytes_get**](DefaultApi.md#context_raw_bytes_get) | **get** /context/raw/bytes | 
[**context_seed_post**](DefaultApi.md#context_seed_post) | **post** /context/seed | 
[**endorsing_power_post**](DefaultApi.md#endorsing_power_post) | **post** /endorsing_power | 
[**hash_get**](DefaultApi.md#hash_get) | **get** /hash | 
[**header_get**](DefaultApi.md#header_get) | **get** /header | 
[**header_protocol_data_get**](DefaultApi.md#header_protocol_data_get) | **get** /header/protocol_data | 
[**header_protocol_data_raw_get**](DefaultApi.md#header_protocol_data_raw_get) | **get** /header/protocol_data/raw | 
[**header_raw_get**](DefaultApi.md#header_raw_get) | **get** /header/raw | 
[**header_shell_get**](DefaultApi.md#header_shell_get) | **get** /header/shell | 
[**helpers_baking_rights_get**](DefaultApi.md#helpers_baking_rights_get) | **get** /helpers/baking_rights | 
[**helpers_complete_prefix_get**](DefaultApi.md#helpers_complete_prefix_get) | **get** /helpers/complete/{prefix} | 
[**helpers_current_level_get**](DefaultApi.md#helpers_current_level_get) | **get** /helpers/current_level | 
[**helpers_endorsing_rights_get**](DefaultApi.md#helpers_endorsing_rights_get) | **get** /helpers/endorsing_rights | 
[**helpers_forge_block_header_post**](DefaultApi.md#helpers_forge_block_header_post) | **post** /helpers/forge_block_header | 
[**helpers_forge_operations_post**](DefaultApi.md#helpers_forge_operations_post) | **post** /helpers/forge/operations | 
[**helpers_forge_protocol_data_post**](DefaultApi.md#helpers_forge_protocol_data_post) | **post** /helpers/forge/protocol_data | 
[**helpers_levels_in_current_cycle_get**](DefaultApi.md#helpers_levels_in_current_cycle_get) | **get** /helpers/levels_in_current_cycle | 
[**helpers_parse_block_post**](DefaultApi.md#helpers_parse_block_post) | **post** /helpers/parse/block | 
[**helpers_parse_operations_post**](DefaultApi.md#helpers_parse_operations_post) | **post** /helpers/parse/operations | 
[**helpers_preapply_block_post**](DefaultApi.md#helpers_preapply_block_post) | **post** /helpers/preapply/block | 
[**helpers_preapply_operations_post**](DefaultApi.md#helpers_preapply_operations_post) | **post** /helpers/preapply/operations | 
[**helpers_scripts_entrypoint_post**](DefaultApi.md#helpers_scripts_entrypoint_post) | **post** /helpers/scripts/entrypoint | 
[**helpers_scripts_entrypoints_post**](DefaultApi.md#helpers_scripts_entrypoints_post) | **post** /helpers/scripts/entrypoints | 
[**helpers_scripts_pack_data_post**](DefaultApi.md#helpers_scripts_pack_data_post) | **post** /helpers/scripts/pack_data | 
[**helpers_scripts_run_code_post**](DefaultApi.md#helpers_scripts_run_code_post) | **post** /helpers/scripts/run_code | 
[**helpers_scripts_run_operation_post**](DefaultApi.md#helpers_scripts_run_operation_post) | **post** /helpers/scripts/run_operation | 
[**helpers_scripts_trace_code_post**](DefaultApi.md#helpers_scripts_trace_code_post) | **post** /helpers/scripts/trace_code | 
[**helpers_scripts_typecheck_code_post**](DefaultApi.md#helpers_scripts_typecheck_code_post) | **post** /helpers/scripts/typecheck_code | 
[**helpers_scripts_typecheck_data_post**](DefaultApi.md#helpers_scripts_typecheck_data_post) | **post** /helpers/scripts/typecheck_data | 
[**live_blocks_get**](DefaultApi.md#live_blocks_get) | **get** /live_blocks | 
[**metadata_get**](DefaultApi.md#metadata_get) | **get** /metadata | 
[**minimal_valid_time_get**](DefaultApi.md#minimal_valid_time_get) | **get** /minimal_valid_time | 
[**operation_hashes_get**](DefaultApi.md#operation_hashes_get) | **get** /operation_hashes | 
[**operation_hashes_list_offset_get**](DefaultApi.md#operation_hashes_list_offset_get) | **get** /operation_hashes/{list_offset} | 
[**operation_hashes_list_offset_operation_offset_get**](DefaultApi.md#operation_hashes_list_offset_operation_offset_get) | **get** /operation_hashes/{list_offset}/{operation_offset} | 
[**operations_get**](DefaultApi.md#operations_get) | **get** /operations | 
[**operations_list_offset_get**](DefaultApi.md#operations_list_offset_get) | **get** /operations/{list_offset} | 
[**operations_list_offset_operation_offset_get**](DefaultApi.md#operations_list_offset_operation_offset_get) | **get** /operations/{list_offset}/{operation_offset} | 
[**protocols_get**](DefaultApi.md#protocols_get) | **get** /protocols | 
[**required_endorsements_get**](DefaultApi.md#required_endorsements_get) | **get** /required_endorsements | 
[**root_get**](DefaultApi.md#root_get) | **get** / | 
[**votes_ballot_list_get**](DefaultApi.md#votes_ballot_list_get) | **get** /votes/ballot_list | 
[**votes_ballots_get**](DefaultApi.md#votes_ballots_get) | **get** /votes/ballots | 
[**votes_current_period_kind_get**](DefaultApi.md#votes_current_period_kind_get) | **get** /votes/current_period_kind | 
[**votes_current_proposal_get**](DefaultApi.md#votes_current_proposal_get) | **get** /votes/current_proposal | 
[**votes_current_quorum_get**](DefaultApi.md#votes_current_quorum_get) | **get** /votes/current_quorum | 
[**votes_listings_get**](DefaultApi.md#votes_listings_get) | **get** /votes/listings | 
[**votes_proposals_get**](DefaultApi.md#votes_proposals_get) | **get** /votes/proposals | 



## context_big_maps_big_map_id_script_expr_get

> crate::models::Micheline007PsDelph1MichelsonV1Expression context_big_maps_big_map_id_script_expr_get(big_map_id, script_expr)


Access the value associated with a key in a big map.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**big_map_id** | **String** | A big map identifier | [required] |
**script_expr** | **String** | script_expr (Base58Check-encoded) | [required] |

### Return type

[**crate::models::Micheline007PsDelph1MichelsonV1Expression**](micheline.007-PsDELPH1.michelson_v1.expression.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_constants_errors_get

> serde_json::Value context_constants_errors_get()


Schema for all the RPC errors from this protocol version

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_constants_get

> crate::models::InlineResponse2001 context_constants_get()


All constants

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2001**](inline_response_200_1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_balance_get

> String context_contracts_contract_id_balance_get(contract_id)


Access the balance of a contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_big_map_get_post

> crate::models::OneOfobject context_contracts_contract_id_big_map_get_post(contract_id, inline_object)


Access the value associated with a key in a big map of the contract (deprecated).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |
**inline_object** | Option<[**InlineObject**](InlineObject.md)> |  |  |

### Return type

[**crate::models::OneOfobject**](oneOf<object>.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_counter_get

> String context_contracts_contract_id_counter_get(contract_id)


Access the counter of a contract, if any.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_delegate_get

> crate::models::SignaturePublicKeyHash context_contracts_contract_id_delegate_get(contract_id)


Access the delegate of a contract, if any.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |

### Return type

[**crate::models::SignaturePublicKeyHash**](Signature.Public_key_hash.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_entrypoints_get

> crate::models::InlineResponse2003 context_contracts_contract_id_entrypoints_get(contract_id)


Return the list of entrypoints of the contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_entrypoints_string_get

> crate::models::Micheline007PsDelph1MichelsonV1Expression context_contracts_contract_id_entrypoints_string_get(contract_id, string)


Return the type of the given entrypoint of the contract

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |
**string** | **String** |  | [required] |

### Return type

[**crate::models::Micheline007PsDelph1MichelsonV1Expression**](micheline.007-PsDELPH1.michelson_v1.expression.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_get

> crate::models::InlineResponse2002 context_contracts_contract_id_get(contract_id)


Access the complete status of a contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |

### Return type

[**crate::models::InlineResponse2002**](inline_response_200_2.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_manager_key_get

> crate::models::OneOfobject context_contracts_contract_id_manager_key_get(contract_id)


Access the manager of a contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |

### Return type

[**crate::models::OneOfobject**](oneOf<object>.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_script_get

> crate::models::Model007PsDelph1ScriptedContracts context_contracts_contract_id_script_get(contract_id)


Access the code and data of the contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |

### Return type

[**crate::models::Model007PsDelph1ScriptedContracts**](007-PsDELPH1.scripted.contracts.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_contract_id_storage_get

> crate::models::Micheline007PsDelph1MichelsonV1Expression context_contracts_contract_id_storage_get(contract_id)


Access the data of the contract.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**contract_id** | **String** | A contract identifier encoded in b58check. | [required] |

### Return type

[**crate::models::Micheline007PsDelph1MichelsonV1Expression**](micheline.007-PsDELPH1.michelson_v1.expression.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_contracts_get

> Vec<crate::models::Model007PsDelph1ContractId> context_contracts_get()


All existing contracts (including non-empty default contracts).

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::Model007PsDelph1ContractId>**](007-PsDELPH1.contract_id.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_get

> Vec<crate::models::SignaturePublicKeyHash> context_delegates_get()


Lists all registered delegates.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::SignaturePublicKeyHash>**](Signature.Public_key_hash.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_pkh_balance_get

> String context_delegates_pkh_balance_get(pkh)


Returns the full balance of a given delegate, including the frozen balances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pkh** | **String** | A Secp256k1 of a Ed25519 public key hash (Base58Check-encoded) | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_pkh_deactivated_get

> bool context_delegates_pkh_deactivated_get(pkh)


Tells whether the delegate is currently tagged as deactivated or not.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pkh** | **String** | A Secp256k1 of a Ed25519 public key hash (Base58Check-encoded) | [required] |

### Return type

**bool**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_pkh_delegated_balance_get

> String context_delegates_pkh_delegated_balance_get(pkh)


Returns the balances of all the contracts that delegate to a given delegate. This excludes the delegate's own balance and its frozen balances.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pkh** | **String** | A Secp256k1 of a Ed25519 public key hash (Base58Check-encoded) | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_pkh_delegated_contracts_get

> Vec<crate::models::Model007PsDelph1ContractId> context_delegates_pkh_delegated_contracts_get(pkh)


Returns the list of contracts that delegate to a given delegate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pkh** | **String** | A Secp256k1 of a Ed25519 public key hash (Base58Check-encoded) | [required] |

### Return type

[**Vec<crate::models::Model007PsDelph1ContractId>**](007-PsDELPH1.contract_id.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_pkh_frozen_balance_by_cycle_get

> Vec<crate::models::InlineResponse2004FrozenBalanceByCycle> context_delegates_pkh_frozen_balance_by_cycle_get(pkh)


Returns the frozen balances of a given delegate, indexed by the cycle by which it will be unfrozen

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pkh** | **String** | A Secp256k1 of a Ed25519 public key hash (Base58Check-encoded) | [required] |

### Return type

[**Vec<crate::models::InlineResponse2004FrozenBalanceByCycle>**](inline_response_200_4_frozen_balance_by_cycle.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_pkh_frozen_balance_get

> String context_delegates_pkh_frozen_balance_get(pkh)


Returns the total frozen balances of a given delegate, this includes the frozen deposits, rewards and fees.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pkh** | **String** | A Secp256k1 of a Ed25519 public key hash (Base58Check-encoded) | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_pkh_get

> crate::models::InlineResponse2004 context_delegates_pkh_get(pkh)


Everything about a delegate.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pkh** | **String** | A Secp256k1 of a Ed25519 public key hash (Base58Check-encoded) | [required] |

### Return type

[**crate::models::InlineResponse2004**](inline_response_200_4.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_pkh_grace_period_get

> i32 context_delegates_pkh_grace_period_get(pkh)


Returns the cycle by the end of which the delegate might be deactivated if she fails to execute any delegate action. A deactivated delegate might be reactivated (without loosing any rolls) by simply re-registering as a delegate. For deactivated delegates, this value contains the cycle by which they were deactivated.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pkh** | **String** | A Secp256k1 of a Ed25519 public key hash (Base58Check-encoded) | [required] |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_delegates_pkh_staking_balance_get

> String context_delegates_pkh_staking_balance_get(pkh)


Returns the total amount of tokens delegated to a given delegate. This includes the balances of all the contracts that delegate to it, but also the balance of the delegate itself and its frozen fees and deposits. The rewards do not count in the delegated balance until they are unfrozen.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**pkh** | **String** | A Secp256k1 of a Ed25519 public key hash (Base58Check-encoded) | [required] |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_nonces_block_level_get

> crate::models::OneOfobjectobjectobject context_nonces_block_level_get(block_level)


Info about the nonce of a previous block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_level** | **String** | A level integer | [required] |

### Return type

[**crate::models::OneOfobjectobjectobject**](oneOf<object,object,object>.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_raw_bytes_get

> crate::models::RawContext context_raw_bytes_get()


Returns the raw context.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::RawContext**](raw_context.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## context_seed_post

> String context_seed_post(body)


Seed of the cycle to which the block belongs.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**body** | Option<**serde_json::Value**> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## endorsing_power_post

> i32 endorsing_power_post(inline_object1)


Get the endorsing power of an endorsement, that is, the number of slots that the endorser has

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object1** | Option<[**InlineObject1**](InlineObject1.md)> |  |  |

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## hash_get

> crate::models::BlockHash hash_get()


The block's hash, its unique identifier.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlockHash**](block_hash.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## header_get

> crate::models::BlockHeader header_get()


The whole block header.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlockHeader**](block_header.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## header_protocol_data_get

> crate::models::InlineResponse2005 header_protocol_data_get()


The version-specific fragment of the block header.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2005**](inline_response_200_5.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## header_protocol_data_raw_get

> String header_protocol_data_raw_get()


The version-specific fragment of the block header (unparsed).

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## header_raw_get

> String header_raw_get()


The whole block header (unparsed).

### Parameters

This endpoint does not need any parameter.

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## header_shell_get

> crate::models::BlockHeaderShell header_shell_get()


The shell-specific fragment of the block header.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlockHeaderShell**](block_header.shell.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_baking_rights_get

> Vec<crate::models::InlineResponse2006> helpers_baking_rights_get()


Retrieves the list of delegates allowed to bake a block. By default, it gives the best baking priorities for bakers that have at least one opportunity below the 64th priority for the next block. Parameters `level` and `cycle` can be used to specify the (valid) level(s) in the past or future at which the baking rights have to be returned. Parameter `delegate` can be used to restrict the results to the given delegates. If parameter `all` is set, all the baking opportunities for each baker at each level are returned, instead of just the first one. Returns the list of baking slots. Also returns the minimal timestamps that correspond to these slots. The timestamps are omitted for levels in the past, and are only estimates for levels later that the next block, based on the hypothesis that all predecessor blocks were baked at the first priority.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse2006>**](inline_response_200_6.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_complete_prefix_get

> Vec<crate::models::Unistring> helpers_complete_prefix_get(prefix)


Try to complete a prefix of a Base58Check-encoded data. This RPC is actually able to complete hashes of block, operations, public_keys and contracts.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**prefix** | **String** |  | [required] |

### Return type

[**Vec<crate::models::Unistring>**](unistring.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_current_level_get

> crate::models::InlineResponse2007 helpers_current_level_get()


Returns the level of the interrogated block, or the one of a block located `offset` blocks after in the chain (or before when negative). For instance, the next block if `offset` is 1.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse2007**](inline_response_200_7.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_endorsing_rights_get

> Vec<crate::models::InlineResponse2008> helpers_endorsing_rights_get()


Retrieves the delegates allowed to endorse a block. By default, it gives the endorsement slots for delegates that have at least one in the next block. Parameters `level` and `cycle` can be used to specify the (valid) level(s) in the past or future at which the endorsement rights have to be returned. Parameter `delegate` can be used to restrict the results to the given delegates. Returns the list of endorsement slots. Also returns the minimal timestamps that correspond to these slots. The timestamps are omitted for levels in the past, and are only estimates for levels later that the next block, based on the hypothesis that all predecessor blocks were baked at the first priority.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse2008>**](inline_response_200_8.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_forge_block_header_post

> crate::models::InlineResponse20010 helpers_forge_block_header_post(block_header)


Forge a block header

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_header** | Option<[**BlockHeader**](BlockHeader.md)> |  |  |

### Return type

[**crate::models::InlineResponse20010**](inline_response_200_10.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_forge_operations_post

> String helpers_forge_operations_post(model007_ps_delph1_operation_alpha_unsigned_operation)


Forge an operation

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**model007_ps_delph1_operation_alpha_unsigned_operation** | Option<[**Model007PsDelph1OperationAlphaUnsignedOperation**](Model007PsDelph1OperationAlphaUnsignedOperation.md)> |  |  |

### Return type

**String**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_forge_protocol_data_post

> crate::models::InlineResponse2009 helpers_forge_protocol_data_post(inline_object2)


Forge the protocol-specific part of a block header

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object2** | Option<[**InlineObject2**](InlineObject2.md)> |  |  |

### Return type

[**crate::models::InlineResponse2009**](inline_response_200_9.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_levels_in_current_cycle_get

> crate::models::InlineResponse20011 helpers_levels_in_current_cycle_get()


Levels of a cycle

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20011**](inline_response_200_11.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_parse_block_post

> crate::models::Model007PsDelph1BlockHeaderAlphaSignedContents helpers_parse_block_post(block_header)


Parse a block

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**block_header** | Option<[**BlockHeader**](BlockHeader.md)> |  |  |

### Return type

[**crate::models::Model007PsDelph1BlockHeaderAlphaSignedContents**](007-PsDELPH1.block_header.alpha.signed_contents.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_parse_operations_post

> Vec<crate::models::EndorsingPowerEndorsementOperation> helpers_parse_operations_post(inline_object3)


Parse operations

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object3** | Option<[**InlineObject3**](InlineObject3.md)> |  |  |

### Return type

[**Vec<crate::models::EndorsingPowerEndorsementOperation>**](_endorsing_power_endorsement_operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_preapply_block_post

> crate::models::InlineResponse20012 helpers_preapply_block_post(inline_object4)


Simulate the validation of a block that would contain the given operations and return the resulting fitness and context hash.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object4** | Option<[**InlineObject4**](InlineObject4.md)> |  |  |

### Return type

[**crate::models::InlineResponse20012**](inline_response_200_12.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_preapply_operations_post

> Vec<crate::models::Model007PsDelph1OperationAlphaOperationWithMetadata> helpers_preapply_operations_post(next_operation)


Simulate the validation of an operation.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**next_operation** | Option<[**Vec<crate::models::NextOperation>**](next_operation.md)> |  |  |

### Return type

[**Vec<crate::models::Model007PsDelph1OperationAlphaOperationWithMetadata>**](007-PsDELPH1.operation.alpha.operation_with_metadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_scripts_entrypoint_post

> crate::models::InlineResponse20013 helpers_scripts_entrypoint_post(inline_object5)


Return the type of the given entrypoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object5** | Option<[**InlineObject5**](InlineObject5.md)> |  |  |

### Return type

[**crate::models::InlineResponse20013**](inline_response_200_13.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_scripts_entrypoints_post

> crate::models::InlineResponse2003 helpers_scripts_entrypoints_post(inline_object6)


Return the list of entrypoints of the given script

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object6** | Option<[**InlineObject6**](InlineObject6.md)> |  |  |

### Return type

[**crate::models::InlineResponse2003**](inline_response_200_3.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_scripts_pack_data_post

> crate::models::InlineResponse20014 helpers_scripts_pack_data_post(inline_object7)


Computes the serialized version of some data expression using the same algorithm as script instruction PACK

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object7** | Option<[**InlineObject7**](InlineObject7.md)> |  |  |

### Return type

[**crate::models::InlineResponse20014**](inline_response_200_14.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_scripts_run_code_post

> crate::models::InlineResponse20015 helpers_scripts_run_code_post(inline_object8)


Run a piece of code in the current context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object8** | Option<[**InlineObject8**](InlineObject8.md)> |  |  |

### Return type

[**crate::models::InlineResponse20015**](inline_response_200_15.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_scripts_run_operation_post

> crate::models::Model007PsDelph1OperationAlphaOperationWithMetadata helpers_scripts_run_operation_post(inline_object9)


Run an operation without signature checks

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object9** | Option<[**InlineObject9**](InlineObject9.md)> |  |  |

### Return type

[**crate::models::Model007PsDelph1OperationAlphaOperationWithMetadata**](007-PsDELPH1.operation.alpha.operation_with_metadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_scripts_trace_code_post

> crate::models::InlineResponse20016 helpers_scripts_trace_code_post(inline_object10)


Run a piece of code in the current context, keeping a trace

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object10** | Option<[**InlineObject10**](InlineObject10.md)> |  |  |

### Return type

[**crate::models::InlineResponse20016**](inline_response_200_16.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_scripts_typecheck_code_post

> crate::models::InlineResponse20017 helpers_scripts_typecheck_code_post(inline_object11)


Typecheck a piece of code in the current context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object11** | Option<[**InlineObject11**](InlineObject11.md)> |  |  |

### Return type

[**crate::models::InlineResponse20017**](inline_response_200_17.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## helpers_scripts_typecheck_data_post

> crate::models::InlineResponse20018 helpers_scripts_typecheck_data_post(inline_object12)


Check that some data expression is well formed and of a given type in the current context

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**inline_object12** | Option<[**InlineObject12**](InlineObject12.md)> |  |  |

### Return type

[**crate::models::InlineResponse20018**](inline_response_200_18.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## live_blocks_get

> Vec<crate::models::BlockHash> live_blocks_get()


List the ancestors of the given block which, if referred to as the branch in an operation header, are recent enough for that operation to be included in the current block.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::BlockHash>**](block_hash.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## metadata_get

> crate::models::BlockHeaderMetadata metadata_get()


All the metadata associated to the block.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::BlockHeaderMetadata**](block_header_metadata.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## minimal_valid_time_get

> crate::models::TimestampProtocol minimal_valid_time_get()


Minimal valid time for a block given a priority and an endorsing power.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TimestampProtocol**](timestamp.protocol.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operation_hashes_get

> Vec<Vec<crate::models::OperationHash>> operation_hashes_get()


The hashes of all the operations included in the block.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<Vec<crate::models::OperationHash>>**](array.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operation_hashes_list_offset_get

> Vec<crate::models::OperationHash> operation_hashes_list_offset_get(list_offset)


All the operations included in `n-th` validation pass of the block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_offset** | **String** | Index `n` of the requested validation pass. | [required] |

### Return type

[**Vec<crate::models::OperationHash>**](Operation_hash.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operation_hashes_list_offset_operation_offset_get

> crate::models::OperationHash operation_hashes_list_offset_operation_offset_get(list_offset, operation_offset)


The hash of then `m-th` operation in the `n-th` validation pass of the block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_offset** | **String** | Index `n` of the requested validation pass. | [required] |
**operation_offset** | **String** | Index `m` of the requested operation in its validation pass. | [required] |

### Return type

[**crate::models::OperationHash**](Operation_hash.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operations_get

> Vec<Vec<crate::models::Operation>> operations_get()


All the operations included in the block.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<Vec<crate::models::Operation>>**](array.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operations_list_offset_get

> Vec<crate::models::Operation> operations_list_offset_get(list_offset)


All the operations included in `n-th` validation pass of the block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_offset** | **String** | Index `n` of the requested validation pass. | [required] |

### Return type

[**Vec<crate::models::Operation>**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## operations_list_offset_operation_offset_get

> crate::models::Operation operations_list_offset_operation_offset_get(list_offset, operation_offset)


The `m-th` operation in the `n-th` validation pass of the block.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**list_offset** | **String** | Index `n` of the requested validation pass. | [required] |
**operation_offset** | **String** | Index `m` of the requested operation in its validation pass. | [required] |

### Return type

[**crate::models::Operation**](operation.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## protocols_get

> ::std::collections::HashMap<String, serde_json::Value> protocols_get()


Current and next protocol.

### Parameters

This endpoint does not need any parameter.

### Return type

[**::std::collections::HashMap<String, serde_json::Value>**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## required_endorsements_get

> i32 required_endorsements_get()


Minimum number of endorsements for a block to be valid, given a delay of the block's timestamp with respect to the minimum time to bake at the block's priority

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## root_get

> crate::models::InlineResponse200 root_get()


All the information about a block. The associated metadata may not be present depending on the history mode and block's distance from the head.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse200**](inline_response_200.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_ballot_list_get

> Vec<crate::models::InlineResponse20019> votes_ballot_list_get()


Ballots casted so far during a voting period.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse20019>**](inline_response_200_19.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_ballots_get

> crate::models::InlineResponse20020 votes_ballots_get()


Sum of ballots casted so far during a voting period.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::InlineResponse20020**](inline_response_200_20.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_current_period_kind_get

> crate::models::OneOfstringstringstringstring votes_current_period_kind_get()


Current period kind.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OneOfstringstringstringstring**](oneOf<string,string,string,string>.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_current_proposal_get

> crate::models::OneOfobject votes_current_proposal_get()


Current proposal under evaluation.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::OneOfobject**](oneOf<object>.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_current_quorum_get

> i32 votes_current_quorum_get()


Current expected quorum.

### Parameters

This endpoint does not need any parameter.

### Return type

**i32**

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_listings_get

> Vec<crate::models::InlineResponse20021> votes_listings_get()


List of delegates with their voting weight, in number of rolls.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<crate::models::InlineResponse20021>**](inline_response_200_21.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## votes_proposals_get

> Vec<Vec<crate::models::OneOfProtocolHashinteger>> votes_proposals_get()


List of proposals with number of supporters.

### Parameters

This endpoint does not need any parameter.

### Return type

[**Vec<Vec<crate::models::OneOfProtocolHashinteger>>**](array.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

