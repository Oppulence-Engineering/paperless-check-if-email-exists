# AdminJobsApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getJob**](#getjob) | **GET** /v1/admin/jobs/{job_id} | GET /v1/admin/jobs/{job_id}|
|[**getJobEvents**](#getjobevents) | **GET** /v1/admin/jobs/{job_id}/events | GET /v1/admin/jobs/{job_id}/events|
|[**getJobResults**](#getjobresults) | **GET** /v1/admin/jobs/{job_id}/results | GET /v1/admin/jobs/{job_id}/results|
|[**listJobs**](#listjobs) | **GET** /v1/admin/jobs | GET /v1/admin/jobs|
|[**listTenantJobs**](#listtenantjobs) | **GET** /v1/admin/tenants/{tenant_id}/jobs | GET /v1/admin/tenants/{tenant_id}/jobs|

# **getJob**
> getJob()

Fetch a single job.

### Example

```typescript
import {
    AdminJobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminJobsApi(configuration);

let jobId: number; //Job identifier (default to undefined)

const { status, data } = await apiInstance.getJob(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Job identifier | defaults to undefined|


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
|**200** | Admin job details |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getJobEvents**
> getJobEvents()

Fetch events for a job.

### Example

```typescript
import {
    AdminJobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminJobsApi(configuration);

let jobId: number; //Job identifier (default to undefined)

const { status, data } = await apiInstance.getJobEvents(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Job identifier | defaults to undefined|


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
|**200** | Job event list |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getJobResults**
> getJobResults()

Fetch task results for a job.

### Example

```typescript
import {
    AdminJobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminJobsApi(configuration);

let jobId: number; //Job identifier (default to undefined)

const { status, data } = await apiInstance.getJobResults(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Job identifier | defaults to undefined|


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
|**200** | Job result list |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listJobs**
> listJobs()

List jobs across all tenants with optional filters.

### Example

```typescript
import {
    AdminJobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminJobsApi(configuration);

const { status, data } = await apiInstance.listJobs();
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
|**200** | Admin job list |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **listTenantJobs**
> listTenantJobs()

List jobs scoped to one tenant.

### Example

```typescript
import {
    AdminJobsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new AdminJobsApi(configuration);

let tenantId: string; //Tenant identifier (default to undefined)

const { status, data } = await apiInstance.listTenantJobs(
    tenantId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **tenantId** | [**string**] | Tenant identifier | defaults to undefined|


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
|**200** | Tenant job list |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

