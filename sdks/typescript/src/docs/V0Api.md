# V0Api

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**createBulkJob**](#createbulkjob) | **POST** /v0/bulk | POST /v0/bulk|
|[**getBulkJobResult**](#getbulkjobresult) | **GET** /v0/bulk/{job_id}/results | GET /v0/bulk/{job_id}/results|
|[**getBulkJobStatus**](#getbulkjobstatus) | **GET** /v0/bulk/{job_id} | GET /v0/bulk/{job_id}|
|[**postCheckEmail**](#postcheckemail) | **POST** /v0/check_email | POST /v0/check_email|

# **createBulkJob**
> createBulkJob()

Creates a legacy bulk verification job and queues tasks synchronously via sqlxmq.

### Example

```typescript
import {
    V0Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V0Api(configuration);

const { status, data } = await apiInstance.createBulkJob();
```

### Parameters
This endpoint does not have any parameters.


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
|**200** | Legacy bulk job created |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getBulkJobResult**
> getBulkJobResult()

Returns terminal result rows for a legacy bulk job.

### Example

```typescript
import {
    V0Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V0Api(configuration);

let jobId: number; //Legacy bulk job identifier (default to undefined)

const { status, data } = await apiInstance.getBulkJobResult(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Legacy bulk job identifier | defaults to undefined|


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
|**200** | Bulk job results |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getBulkJobStatus**
> getBulkJobStatus()

Returns current status and summary for a legacy bulk verification job.

### Example

```typescript
import {
    V0Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V0Api(configuration);

let jobId: number; //Legacy bulk job identifier (default to undefined)

const { status, data } = await apiInstance.getBulkJobStatus(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Legacy bulk job identifier | defaults to undefined|


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
|**200** | Bulk status response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postCheckEmail**
> CheckEmailOutput postCheckEmail(checkEmailRequest)

Legacy email verification endpoint (deprecated, retained for compatibility).

### Example

```typescript
import {
    V0Api,
    Configuration,
    CheckEmailRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V0Api(configuration);

let checkEmailRequest: CheckEmailRequest; //
let idempotencyKey: string; //Optional idempotency key (optional) (default to undefined)

const { status, data } = await apiInstance.postCheckEmail(
    checkEmailRequest,
    idempotencyKey
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **checkEmailRequest** | **CheckEmailRequest**|  | |
| **idempotencyKey** | [**string**] | Optional idempotency key | (optional) defaults to undefined|


### Return type

**CheckEmailOutput**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Email verification result |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

