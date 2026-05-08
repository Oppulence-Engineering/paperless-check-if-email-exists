# EventsApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1ListEvents**](#v1listevents) | **GET** /v1/events | GET /v1/events|

# **v1ListEvents**
> v1ListEvents()

Returns a paginated, filterable audit log of all job events for the tenant.

### Example

```typescript
import {
    EventsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new EventsApi(configuration);

let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)
let eventType: string; // (optional) (default to undefined)
let actor: string; // (optional) (default to undefined)
let jobId: number; // (optional) (default to undefined)
let since: string; // (optional) (default to undefined)
let until: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1ListEvents(
    limit,
    offset,
    eventType,
    actor,
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
| **eventType** | [**string**] |  | (optional) defaults to undefined|
| **actor** | [**string**] |  | (optional) defaults to undefined|
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
|**200** | Audit log events |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

