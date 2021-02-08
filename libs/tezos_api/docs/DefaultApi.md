# \DefaultApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**context_big_maps_big_map_id_script_expr_get**](DefaultApi.md#context_big_maps_big_map_id_script_expr_get) | **get** /context/big_maps/{big_map_id}/{script_expr} | 



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

