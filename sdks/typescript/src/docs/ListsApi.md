# ListsApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1ListQuality**](#v1listquality) | **GET** /v1/lists/{list_id}/quality | GET /v1/lists/{list_id}/quality|

# **v1ListQuality**
> v1ListQuality()

Returns a quality benchmark report for a list.

### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let listId: number; //List identifier (default to undefined)

const { status, data } = await apiInstance.v1ListQuality(
    listId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **listId** | [**number**] | List identifier | defaults to undefined|


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
|**200** | List quality benchmark report |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

