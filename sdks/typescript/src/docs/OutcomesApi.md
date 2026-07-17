# OutcomesApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1CreateProviderEndpoint**](#v1createproviderendpoint) | **POST** /v1/provider-endpoints | |
|[**v1DeleteProviderEndpoint**](#v1deleteproviderendpoint) | **DELETE** /v1/provider-endpoints/{endpoint_id} | |
|[**v1IngestOutcomes**](#v1ingestoutcomes) | **POST** /v1/outcomes | POST /v1/outcomes|
|[**v1IngestProviderOutcomes**](#v1ingestprovideroutcomes) | **POST** /v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token} | |
|[**v1ListOutcomes**](#v1listoutcomes) | **GET** /v1/outcomes | GET /v1/outcomes|
|[**v1ListProviderEndpoints**](#v1listproviderendpoints) | **GET** /v1/provider-endpoints | |
|[**v1UpdateProviderEndpoint**](#v1updateproviderendpoint) | **PATCH** /v1/provider-endpoints/{endpoint_id} | |

# **v1CreateProviderEndpoint**
> ProviderEndpointView v1CreateProviderEndpoint(createProviderEndpointInput)


### Example

```typescript
import {
    OutcomesApi,
    Configuration,
    CreateProviderEndpointInput
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new OutcomesApi(configuration);

let createProviderEndpointInput: CreateProviderEndpointInput; //

const { status, data } = await apiInstance.v1CreateProviderEndpoint(
    createProviderEndpointInput
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createProviderEndpointInput** | **CreateProviderEndpointInput**|  | |


### Return type

**ProviderEndpointView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Provider endpoint created; delivery token is returned once |  -  |
|**400** | Invalid provider endpoint |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DeleteProviderEndpoint**
> ProviderDeleteResponse v1DeleteProviderEndpoint()


### Example

```typescript
import {
    OutcomesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new OutcomesApi(configuration);

let endpointId: string; //Provider endpoint identifier (default to undefined)

const { status, data } = await apiInstance.v1DeleteProviderEndpoint(
    endpointId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **endpointId** | [**string**] | Provider endpoint identifier | defaults to undefined|


### Return type

**ProviderDeleteResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Provider endpoint disabled and deleted |  -  |
|**404** | Provider endpoint not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1IngestOutcomes**
> OutcomeIngestResponse v1IngestOutcomes(outcomeIngestRequest)


### Example

```typescript
import {
    OutcomesApi,
    Configuration,
    OutcomeIngestRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new OutcomesApi(configuration);

let outcomeIngestRequest: OutcomeIngestRequest; //

const { status, data } = await apiInstance.v1IngestOutcomes(
    outcomeIngestRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **outcomeIngestRequest** | **OutcomeIngestRequest**|  | |


### Return type

**OutcomeIngestResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Provider outcomes ingested |  -  |
|**400** | Invalid outcome payload |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1IngestProviderOutcomes**
> InboundOutcomeResponse v1IngestProviderOutcomes(body)


### Example

```typescript
import {
    OutcomesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new OutcomesApi(configuration);

let provider: string; //sendgrid, ses, mailgun, or postmark (default to undefined)
let endpointId: string; //Provider endpoint identifier (default to undefined)
let deliveryToken: string; //Secret endpoint delivery token (default to undefined)
let body: any; //

const { status, data } = await apiInstance.v1IngestProviderOutcomes(
    provider,
    endpointId,
    deliveryToken,
    body
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **body** | **any**|  | |
| **provider** | [**string**] | sendgrid, ses, mailgun, or postmark | defaults to undefined|
| **endpointId** | [**string**] | Provider endpoint identifier | defaults to undefined|
| **deliveryToken** | [**string**] | Secret endpoint delivery token | defaults to undefined|


### Return type

**InboundOutcomeResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Provider events authenticated and ingested |  -  |
|**202** | Receipt retained while endpoint is paused |  -  |
|**401** | Provider signature rejected |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

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

let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)
let email: string; // (optional) (default to undefined)
let eventType: string; // (optional) (default to undefined)
let sourceKey: string; // (optional) (default to undefined)
let since: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1ListOutcomes(
    limit,
    offset,
    email,
    eventType,
    sourceKey,
    since
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|
| **email** | [**string**] |  | (optional) defaults to undefined|
| **eventType** | [**string**] |  | (optional) defaults to undefined|
| **sourceKey** | [**string**] |  | (optional) defaults to undefined|
| **since** | [**string**] |  | (optional) defaults to undefined|


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
|**200** | Paginated provider outcomes |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListProviderEndpoints**
> ProviderEndpointListResponse v1ListProviderEndpoints()


### Example

```typescript
import {
    OutcomesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new OutcomesApi(configuration);

const { status, data } = await apiInstance.v1ListProviderEndpoints();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**ProviderEndpointListResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Configured provider endpoints |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1UpdateProviderEndpoint**
> ProviderEndpointView v1UpdateProviderEndpoint(updateProviderEndpointInput)


### Example

```typescript
import {
    OutcomesApi,
    Configuration,
    UpdateProviderEndpointInput
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new OutcomesApi(configuration);

let endpointId: string; //Provider endpoint identifier (default to undefined)
let updateProviderEndpointInput: UpdateProviderEndpointInput; //

const { status, data } = await apiInstance.v1UpdateProviderEndpoint(
    endpointId,
    updateProviderEndpointInput
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateProviderEndpointInput** | **UpdateProviderEndpointInput**|  | |
| **endpointId** | [**string**] | Provider endpoint identifier | defaults to undefined|


### Return type

**ProviderEndpointView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Provider endpoint updated |  -  |
|**404** | Provider endpoint not found |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
