# DefaultApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getV1Bulk**](#getv1bulk) | **GET** /v1/bulk/{job_id} | /v1/bulk/{job_id}|
|[**getV1BulkResults**](#getv1bulkresults) | **GET** /v1/bulk/{job_id}/results | Retrieve bulk verification results|
|[**postV0CheckEmail**](#postv0checkemail) | **POST** /v0/check_email | /v0/check_email|
|[**postV1Bulk**](#postv1bulk) | **POST** /v1/bulk | /v1/bulk|
|[**postV1CheckEmail**](#postv1checkemail) | **POST** /v1/check_email | /v1/check_email|

# **getV1Bulk**
> GetV1Bulk200Response getV1Bulk()

Retrieve the progress of a bulk verification job.

### Example

```typescript
import {
    DefaultApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let jobId: number; //The unique bulk verification job ID (default to undefined)

const { status, data } = await apiInstance.getV1Bulk(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | The unique bulk verification job ID | defaults to undefined|


### Return type

**GetV1Bulk200Response**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | OK |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getV1BulkResults**
> GetV1BulkResults200Response getV1BulkResults()

Retrieve the results of a bulk verification job. This endpoint will return an error if the job is still running. Please query `GET /v1/bulk/{job_id}` first to check the job\'s progress.

### Example

```typescript
import {
    DefaultApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let jobId: string; //The unique bulk verification job ID (default to undefined)
let limit: number; //The number of results to return. (optional) (default to 50)
let offset: number; //The offset from which to return the results, equivalent to the number of elements in the array to skip. (optional) (default to undefined)

const { status, data } = await apiInstance.getV1BulkResults(
    jobId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**string**] | The unique bulk verification job ID | defaults to undefined|
| **limit** | [**number**] | The number of results to return. | (optional) defaults to 50|
| **offset** | [**number**] | The offset from which to return the results, equivalent to the number of elements in the array to skip. | (optional) defaults to undefined|


### Return type

**GetV1BulkResults200Response**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | OK |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postV0CheckEmail**
> CheckEmailOutput postV0CheckEmail()

**Sunset notice: please use `/v1/check_email` instead.** Both endpoints accept the same input arguments and return the same output; only their internal implementation differs. Perform a comprehensive verification of an email address. Unlike the `/v1/check_email` endpoint, this endpoint performs an email verification immediately, without considering the Reacher server\'s throttling, concurrency, and other configurations. As such, this endpoint is slightly riskier than `/v1/check_email`, as the Reacher server\'s IP reputation can be impacted if this endpoint is called too frequently.

### Example

```typescript
import {
    DefaultApi,
    Configuration,
    CheckEmailRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let authorization: string; //Your personal Reacher API key (default to undefined)
let checkEmailRequest: CheckEmailRequest; //Request object containing all parameters necessary for an email verification. (optional)

const { status, data } = await apiInstance.postV0CheckEmail(
    authorization,
    checkEmailRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **checkEmailRequest** | **CheckEmailRequest**| Request object containing all parameters necessary for an email verification. | |
| **authorization** | [**string**] | Your personal Reacher API key | defaults to undefined|


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
|**200** | OK |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postV1Bulk**
> PostV1Bulk200Response postV1Bulk()

Initiate a bulk email verification.

### Example

```typescript
import {
    DefaultApi,
    Configuration,
    PostV1BulkRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let postV1BulkRequest: PostV1BulkRequest; // (optional)

const { status, data } = await apiInstance.postV1Bulk(
    postV1BulkRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **postV1BulkRequest** | **PostV1BulkRequest**|  | |


### Return type

**PostV1Bulk200Response**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | OK |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **postV1CheckEmail**
> CheckEmailOutput postV1CheckEmail()

Perform a comprehensive verification of an email address. This endpoint supersedes the previous `/v0/check_email` endpoint, maintaining the same input and output format. Unlike the `/v0/check_email` endpoint, the new `/v1/check_email` endpoint queues the email for verification, and the Reacher server processes the queue based on its configuration settings such as throttle and concurrency.

### Example

```typescript
import {
    DefaultApi,
    Configuration,
    CheckEmailRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new DefaultApi(configuration);

let checkEmailRequest: CheckEmailRequest; //Request object containing all parameters necessary for an email verification. (optional)

const { status, data } = await apiInstance.postV1CheckEmail(
    checkEmailRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **checkEmailRequest** | **CheckEmailRequest**| Request object containing all parameters necessary for an email verification. | |


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
|**200** | OK |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
