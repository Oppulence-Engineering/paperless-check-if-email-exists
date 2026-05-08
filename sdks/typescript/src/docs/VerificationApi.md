# VerificationApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1EmailHistory**](#v1emailhistory) | **GET** /v1/emails/{email}/history | GET /v1/emails/{email}/history|

# **v1EmailHistory**
> v1EmailHistory()

Returns the verification history timeline for a specific email address.

### Example

```typescript
import {
    VerificationApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new VerificationApi(configuration);

let email: string; //Email address to look up (default to undefined)
let limit: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1EmailHistory(
    email,
    limit
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **email** | [**string**] | Email address to look up | defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|


### Return type

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Verification history for the email |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
