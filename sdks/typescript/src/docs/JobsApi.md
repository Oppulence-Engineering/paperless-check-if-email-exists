# JobsApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1CancelJob**](#v1canceljob) | **POST** /v1/jobs/{job_id}/cancel | POST /v1/jobs/{job_id}/cancel|
|[**v1DownloadJobResults**](#v1downloadjobresults) | **GET** /v1/jobs/{job_id}/download | GET /v1/jobs/{job_id}/download|
|[**v1GetBulkJobProgress**](#v1getbulkjobprogress) | **GET** /v1/bulk/{job_id} | GET /v1/bulk/{job_id}|
|[**v1GetBulkJobResults**](#v1getbulkjobresults) | **GET** /v1/bulk/{job_id}/results | GET /v1/bulk/{job_id}/results|
|[**v1GetJobEvents**](#v1getjobevents) | **GET** /v1/jobs/{job_id}/events | GET /v1/jobs/{job_id}/events|
|[**v1GetJobResults**](#v1getjobresults) | **GET** /v1/jobs/{job_id}/results | GET /v1/jobs/{job_id}/results|
|[**v1GetJobStatus**](#v1getjobstatus) | **GET** /v1/jobs/{job_id} | GET /v1/jobs/{job_id}|
|[**v1RetryJob**](#v1retryjob) | **POST** /v1/jobs/{job_id}/retry | POST /v1/jobs/{job_id}/retry|

# **v1CancelJob**
> v1CancelJob()

Requests cancellation for a tenant-scoped bulk job.

### Example

```typescript
import {
    JobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new JobsApi(configuration);

let jobId: number; //Bulk job identifier (default to undefined)

const { status, data } = await apiInstance.v1CancelJob(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Bulk job identifier | defaults to undefined|


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
|**200** | Job cancellation accepted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DownloadJobResults**
> File v1DownloadJobResults()

Streams all terminal task results for a tenant-scoped bulk job.

### Example

```typescript
import {
    JobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new JobsApi(configuration);

let jobId: number; //Bulk job identifier (default to undefined)
let format: string; //Supported values: `csv`, `json` (optional) (default to undefined)

const { status, data } = await apiInstance.v1DownloadJobResults(
    jobId,
    format
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Bulk job identifier | defaults to undefined|
| **format** | [**string**] | Supported values: &#x60;csv&#x60;, &#x60;json&#x60; | (optional) defaults to undefined|


### Return type

**File**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/x-ndjson, text/csv


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Job result download stream |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetBulkJobProgress**
> v1GetBulkJobProgress()

Returns status and current progress for a tenant-scoped v1 bulk job.

### Example

```typescript
import {
    JobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new JobsApi(configuration);

let jobId: number; //V1 bulk job identifier (default to undefined)

const { status, data } = await apiInstance.v1GetBulkJobProgress(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | V1 bulk job identifier | defaults to undefined|


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
|**200** | Bulk job progress |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetBulkJobResults**
> BulkJobResultsResponse v1GetBulkJobResults()

Returns terminal results for a tenant-scoped v1 bulk job.

### Example

```typescript
import {
    JobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new JobsApi(configuration);

let jobId: number; //V1 bulk job identifier (default to undefined)
let format: string; //Supported values: `json`, `csv` (optional) (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1GetBulkJobResults(
    jobId,
    format,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | V1 bulk job identifier | defaults to undefined|
| **format** | [**string**] | Supported values: &#x60;json&#x60;, &#x60;csv&#x60; | (optional) defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**BulkJobResultsResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json, text/csv


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Bulk job results |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetJobEvents**
> v1GetJobEvents()

Returns paginated event history for a tenant-scoped bulk job.

### Example

```typescript
import {
    JobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new JobsApi(configuration);

let jobId: number; //Bulk job identifier (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1GetJobEvents(
    jobId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Bulk job identifier | defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


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
|**200** | Job events |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetJobResults**
> JobResultPageResponse v1GetJobResults()

Returns paginated completed task results for a tenant-scoped bulk job.

### Example

```typescript
import {
    JobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new JobsApi(configuration);

let jobId: number; //Bulk job identifier (default to undefined)
let cursor: number; // (optional) (default to undefined)
let limit: number; // (optional) (default to undefined)
let state: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1GetJobResults(
    jobId,
    cursor,
    limit,
    state
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Bulk job identifier | defaults to undefined|
| **cursor** | [**number**] |  | (optional) defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **state** | [**string**] |  | (optional) defaults to undefined|


### Return type

**JobResultPageResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Job result page |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetJobStatus**
> v1GetJobStatus()

Returns progress summary and state for a tenant-scoped bulk job.

### Example

```typescript
import {
    JobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new JobsApi(configuration);

let jobId: number; //Bulk job identifier (default to undefined)

const { status, data } = await apiInstance.v1GetJobStatus(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Bulk job identifier | defaults to undefined|


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
|**200** | Bulk job progress summary |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1RetryJob**
> RetryJobResponse v1RetryJob()

Retries all failed or dead-lettered tasks in a tenant-scoped bulk job.

### Example

```typescript
import {
    JobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new JobsApi(configuration);

let jobId: number; //Bulk job identifier (default to undefined)

const { status, data } = await apiInstance.v1RetryJob(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Bulk job identifier | defaults to undefined|


### Return type

**RetryJobResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Retry initiated |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

