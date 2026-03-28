# PipelinesApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1CreatePipeline**](#v1createpipeline) | **POST** /v1/pipelines | POST /v1/pipelines|
|[**v1DeletePipeline**](#v1deletepipeline) | **DELETE** /v1/pipelines/{pipeline_id} | DELETE /v1/pipelines/{pipeline_id}|
|[**v1GetPipeline**](#v1getpipeline) | **GET** /v1/pipelines/{pipeline_id} | GET /v1/pipelines/{pipeline_id}|
|[**v1GetPipelineRun**](#v1getpipelinerun) | **GET** /v1/pipelines/{pipeline_id}/runs/{run_id} | GET /v1/pipelines/{pipeline_id}/runs/{run_id}|
|[**v1ListPipelineRuns**](#v1listpipelineruns) | **GET** /v1/pipelines/{pipeline_id}/runs | GET /v1/pipelines/{pipeline_id}/runs|
|[**v1ListPipelines**](#v1listpipelines) | **GET** /v1/pipelines | GET /v1/pipelines|
|[**v1PausePipeline**](#v1pausepipeline) | **POST** /v1/pipelines/{pipeline_id}/pause | POST /v1/pipelines/{pipeline_id}/pause|
|[**v1ResumePipeline**](#v1resumepipeline) | **POST** /v1/pipelines/{pipeline_id}/resume | POST /v1/pipelines/{pipeline_id}/resume|
|[**v1TriggerPipeline**](#v1triggerpipeline) | **POST** /v1/pipelines/{pipeline_id}/trigger | POST /v1/pipelines/{pipeline_id}/trigger|
|[**v1UpdatePipeline**](#v1updatepipeline) | **PATCH** /v1/pipelines/{pipeline_id} | PATCH /v1/pipelines/{pipeline_id}|

# **v1CreatePipeline**
> PipelineView v1CreatePipeline(createPipelineInput)


### Example

```typescript
import {
    PipelinesApi,
    Configuration,
    CreatePipelineInput
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let createPipelineInput: CreatePipelineInput; //

const { status, data } = await apiInstance.v1CreatePipeline(
    createPipelineInput
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createPipelineInput** | **CreatePipelineInput**|  | |


### Return type

**PipelineView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Pipeline created |  -  |
|**400** | Bad request |  -  |
|**403** | Forbidden |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DeletePipeline**
> DeletePipelineResponse v1DeletePipeline()


### Example

```typescript
import {
    PipelinesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let pipelineId: number; //Pipeline identifier (default to undefined)

const { status, data } = await apiInstance.v1DeletePipeline(
    pipelineId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **pipelineId** | [**number**] | Pipeline identifier | defaults to undefined|


### Return type

**DeletePipelineResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Pipeline deleted |  -  |
|**403** | Forbidden |  -  |
|**404** | Not found |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetPipeline**
> PipelineView v1GetPipeline()


### Example

```typescript
import {
    PipelinesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let pipelineId: number; //Pipeline identifier (default to undefined)

const { status, data } = await apiInstance.v1GetPipeline(
    pipelineId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **pipelineId** | [**number**] | Pipeline identifier | defaults to undefined|


### Return type

**PipelineView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Pipeline detail |  -  |
|**403** | Forbidden |  -  |
|**404** | Not found |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetPipelineRun**
> PipelineRunView v1GetPipelineRun()


### Example

```typescript
import {
    PipelinesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let pipelineId: number; //Pipeline identifier (default to undefined)
let runId: number; //Pipeline run identifier (default to undefined)

const { status, data } = await apiInstance.v1GetPipelineRun(
    pipelineId,
    runId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **pipelineId** | [**number**] | Pipeline identifier | defaults to undefined|
| **runId** | [**number**] | Pipeline run identifier | defaults to undefined|


### Return type

**PipelineRunView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Pipeline run detail |  -  |
|**403** | Forbidden |  -  |
|**404** | Not found |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListPipelineRuns**
> ListPipelineRunsResponse v1ListPipelineRuns()


### Example

```typescript
import {
    PipelinesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let pipelineId: number; //Pipeline identifier (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1ListPipelineRuns(
    pipelineId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **pipelineId** | [**number**] | Pipeline identifier | defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**ListPipelineRunsResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Pipeline run history |  -  |
|**403** | Forbidden |  -  |
|**404** | Not found |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListPipelines**
> ListPipelinesResponse v1ListPipelines()


### Example

```typescript
import {
    PipelinesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let status: PipelineStatus; // (optional) (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1ListPipelines(
    status,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **status** | **PipelineStatus** |  | (optional) defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**ListPipelinesResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Pipeline list |  -  |
|**400** | Bad request |  -  |
|**403** | Forbidden |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1PausePipeline**
> PipelineView v1PausePipeline()


### Example

```typescript
import {
    PipelinesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let pipelineId: number; //Pipeline identifier (default to undefined)

const { status, data } = await apiInstance.v1PausePipeline(
    pipelineId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **pipelineId** | [**number**] | Pipeline identifier | defaults to undefined|


### Return type

**PipelineView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Pipeline paused |  -  |
|**403** | Forbidden |  -  |
|**404** | Not found |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ResumePipeline**
> PipelineView v1ResumePipeline()


### Example

```typescript
import {
    PipelinesApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let pipelineId: number; //Pipeline identifier (default to undefined)

const { status, data } = await apiInstance.v1ResumePipeline(
    pipelineId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **pipelineId** | [**number**] | Pipeline identifier | defaults to undefined|


### Return type

**PipelineView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Pipeline resumed |  -  |
|**403** | Forbidden |  -  |
|**404** | Not found |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1TriggerPipeline**
> TriggerPipelineResponse v1TriggerPipeline(triggerPipelineInput)


### Example

```typescript
import {
    PipelinesApi,
    Configuration,
    TriggerPipelineInput
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let pipelineId: number; //Pipeline identifier (default to undefined)
let triggerPipelineInput: TriggerPipelineInput; //

const { status, data } = await apiInstance.v1TriggerPipeline(
    pipelineId,
    triggerPipelineInput
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **triggerPipelineInput** | **TriggerPipelineInput**|  | |
| **pipelineId** | [**number**] | Pipeline identifier | defaults to undefined|


### Return type

**TriggerPipelineResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Pipeline run triggered |  -  |
|**400** | Bad request |  -  |
|**403** | Forbidden |  -  |
|**404** | Not found |  -  |
|**409** | Conflict |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1UpdatePipeline**
> PipelineView v1UpdatePipeline(updatePipelineInput)


### Example

```typescript
import {
    PipelinesApi,
    Configuration,
    UpdatePipelineInput
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new PipelinesApi(configuration);

let pipelineId: number; //Pipeline identifier (default to undefined)
let updatePipelineInput: UpdatePipelineInput; //

const { status, data } = await apiInstance.v1UpdatePipeline(
    pipelineId,
    updatePipelineInput
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updatePipelineInput** | **UpdatePipelineInput**|  | |
| **pipelineId** | [**number**] | Pipeline identifier | defaults to undefined|


### Return type

**PipelineView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Pipeline updated |  -  |
|**400** | Bad request |  -  |
|**403** | Forbidden |  -  |
|**404** | Not found |  -  |
|**500** | Internal server error |  -  |
|**503** | Service unavailable |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

