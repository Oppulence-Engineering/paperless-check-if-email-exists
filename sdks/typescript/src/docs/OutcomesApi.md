# OutcomesApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1ListOutcomes**](#v1listoutcomes) | **GET** /v1/outcomes | |
|[**v1PostOutcomes**](#v1postoutcomes) | **POST** /v1/outcomes | |
|[**v1UploadOutcomes**](#v1uploadoutcomes) | **POST** /v1/outcomes/upload | |

# **v1ListOutcomes**
> OutcomeListResponse v1ListOutcomes()


### Example

```typescript
import {
    OutcomesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new OutcomesApi(configuration);

let email: string; // (optional) (default to undefined)
let source: string; // (optional) (default to undefined)
let type: string; // (optional) (default to undefined)
let since: string; // (optional) (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1ListOutcomes(
    email,
    source,
    type,
    since,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **email** | [**string**] |  | (optional) defaults to undefined|
| **source** | [**string**] |  | (optional) defaults to undefined|
| **type** | [**string**] |  | (optional) defaults to undefined|
| **since** | [**string**] |  | (optional) defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**OutcomeListResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List of outcomes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1PostOutcomes**
> IngestOutcomesResponse v1PostOutcomes(ingestOutcomesRequest)


### Example

```typescript
import {
    OutcomesApi,
    Configuration,
    IngestOutcomesRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new OutcomesApi(configuration);

let ingestOutcomesRequest: IngestOutcomesRequest; //

const { status, data } = await apiInstance.v1PostOutcomes(
    ingestOutcomesRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **ingestOutcomesRequest** | **IngestOutcomesRequest**|  | |


### Return type

**IngestOutcomesResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Outcomes accepted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1UploadOutcomes**
> IngestOutcomesResponse v1UploadOutcomes()


### Example

```typescript
import {
    OutcomesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new OutcomesApi(configuration);

const { status, data } = await apiInstance.v1UploadOutcomes();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**IngestOutcomesResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Outcomes ingested via CSV |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
