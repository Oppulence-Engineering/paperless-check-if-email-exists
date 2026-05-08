# QueryApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1QueryResults**](#v1queryresults) | **GET** /v1/query | GET /v1/query|

# **v1QueryResults**
> v1QueryResults()

Flexible historical query API for verification results across jobs.

### Example

```typescript
import {
    QueryApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new QueryApi(configuration);

let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)
let category: string; // (optional) (default to undefined)
let minScore: number; // (optional) (default to undefined)
let maxScore: number; // (optional) (default to undefined)
let safeToSend: boolean; // (optional) (default to undefined)
let jobId: number; // (optional) (default to undefined)
let since: string; // (optional) (default to undefined)
let until: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1QueryResults(
    limit,
    offset,
    category,
    minScore,
    maxScore,
    safeToSend,
    jobId,
    since,
    until
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|
| **category** | [**string**] |  | (optional) defaults to undefined|
| **minScore** | [**number**] |  | (optional) defaults to undefined|
| **maxScore** | [**number**] |  | (optional) defaults to undefined|
| **safeToSend** | [**boolean**] |  | (optional) defaults to undefined|
| **jobId** | [**number**] |  | (optional) defaults to undefined|
| **since** | [**string**] |  | (optional) defaults to undefined|
| **until** | [**string**] |  | (optional) defaults to undefined|


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
|**200** | Filtered verification results |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
