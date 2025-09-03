# DefaultApi

All URIs are relative to **

Method | HTTP request | Description
------------- | ------------- | -------------
[**hello**](DefaultApi.md#hello) | **GET** /hello | Returns a greeting
[**testauth**](DefaultApi.md#testauth) | **GET** /testauth | Get test if I logged



## hello

Returns a greeting

### Example

```bash
my-api.sh hello
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Hello200Response**](Hello200Response.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## testauth

Get test if I logged

Returns string test if user logged.

### Example

```bash
my-api.sh testauth
```

### Parameters

This endpoint does not need any parameter.

### Return type

[**Testauth200Response**](Testauth200Response.md)

### Authorization

[bearerAuth](../README.md#bearerAuth)

### HTTP request headers

- **Content-Type**: Not Applicable
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

