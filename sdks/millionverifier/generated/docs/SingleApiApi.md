# \SingleApiApi

All URIs are relative to *https://api.millionverifier.com*

Method | HTTP request | Description
------------- | ------------- | -------------
[**single_verification**](SingleApiApi.md#single_verification) | **GET** /api/v3/ | Verify an email address in real time



## single_verification

> models::SingleApiResponse single_verification(api, email, timeout)
Verify an email address in real time

Verify an email address in real time and get resutls in just a second.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**api** | **String** | Your API key | [required] |
**email** | **String** | Email address that needs to be verified | [required] |
**timeout** | Option<**i32**> | Time in seconds to terminate the connection in case no response recevied from the recipient server. You can set between 2 and 60 seconds. Default timeout is 20 seconds. |  |

### Return type

[**models::SingleApiResponse**](SingleApiResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

