# V1Api

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1CheckEmail**](#v1checkemail) | **POST** /v1/check_email | POST /v1/check_email|
|[**v1CheckReputation**](#v1checkreputation) | **POST** /v1/reputation/check | POST /v1/reputation/check|
|[**v1CreateBulkJob**](#v1createbulkjob) | **POST** /v1/bulk | Create the v1 bulk endpoint.|
|[**v1CreateList**](#v1createlist) | **POST** /v1/lists | POST /v1/lists|
|[**v1DeleteList**](#v1deletelist) | **DELETE** /v1/lists/{list_id} | DELETE /v1/lists/{list_id}|
|[**v1DownloadList**](#v1downloadlist) | **GET** /v1/lists/{list_id}/download | GET /v1/lists/{list_id}/download|
|[**v1FindEmail**](#v1findemail) | **POST** /v1/find_email | POST /v1/find_email|
|[**v1GetFindEmail**](#v1getfindemail) | **GET** /v1/find_email/{job_id} | GET /v1/find_email/{job_id}|
|[**v1GetList**](#v1getlist) | **GET** /v1/lists/{list_id} | GET /v1/lists/{list_id}|
|[**v1ListLists**](#v1listlists) | **GET** /v1/lists | GET /v1/lists|

# **v1CheckEmail**
> CheckEmailOutput v1CheckEmail(checkEmailRequest)

Verifies an email address and returns a result.

### Example

```typescript
import {
    V1Api,
    Configuration,
    CheckEmailRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let checkEmailRequest: CheckEmailRequest; //
let idempotencyKey: string; //Optional idempotency key (optional) (default to undefined)

const { status, data } = await apiInstance.v1CheckEmail(
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

# **v1CheckReputation**
> ReputationCheckResponse v1CheckReputation(reputationCheckRequest)


### Example

```typescript
import {
    V1Api,
    Configuration,
    ReputationCheckRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let reputationCheckRequest: ReputationCheckRequest; //

const { status, data } = await apiInstance.v1CheckReputation(
    reputationCheckRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **reputationCheckRequest** | **ReputationCheckRequest**|  | |


### Return type

**ReputationCheckResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Reputation check response |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1CreateBulkJob**
> v1CreateBulkJob()

Creates a tenant-scoped bulk job for async processing.

### Example

```typescript
import {
    V1Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let idempotencyKey: string; //Optional idempotency key (optional) (default to undefined)

const { status, data } = await apiInstance.v1CreateBulkJob(
    idempotencyKey
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **idempotencyKey** | [**string**] | Optional idempotency key | (optional) defaults to undefined|


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
|**200** | Bulk job created |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1CreateList**
> ListUploadResponse v1CreateList()


### Example

```typescript
import {
    V1Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let file: File; // (default to undefined)
let emailColumn: string; // (optional) (default to undefined)
let name: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1CreateList(
    file,
    emailColumn,
    name
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **file** | [**File**] |  | defaults to undefined|
| **emailColumn** | [**string**] |  | (optional) defaults to undefined|
| **name** | [**string**] |  | (optional) defaults to undefined|


### Return type

**ListUploadResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: multipart/form-data
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | List upload accepted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DeleteList**
> ListDeleteResponse v1DeleteList()


### Example

```typescript
import {
    V1Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let listId: number; //List identifier (default to undefined)

const { status, data } = await apiInstance.v1DeleteList(
    listId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **listId** | [**number**] | List identifier | defaults to undefined|


### Return type

**ListDeleteResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List deleted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DownloadList**
> File v1DownloadList()


### Example

```typescript
import {
    V1Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let listId: number; //List identifier (default to undefined)
let filter: string; // (optional) (default to undefined)
let format: string; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1DownloadList(
    listId,
    filter,
    format
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **listId** | [**number**] | List identifier | defaults to undefined|
| **filter** | [**string**] |  | (optional) defaults to undefined|
| **format** | [**string**] |  | (optional) defaults to undefined|


### Return type

**File**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: text/csv


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Cleaned list CSV download |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1FindEmail**
> FindEmailAcceptedResponse v1FindEmail(findEmailRequest)


### Example

```typescript
import {
    V1Api,
    Configuration,
    FindEmailRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let findEmailRequest: FindEmailRequest; //

const { status, data } = await apiInstance.v1FindEmail(
    findEmailRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **findEmailRequest** | **FindEmailRequest**|  | |


### Return type

**FindEmailAcceptedResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**202** | Finder job accepted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetFindEmail**
> FindEmailStatusResponse v1GetFindEmail()


### Example

```typescript
import {
    V1Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let jobId: number; //Finder job identifier (default to undefined)

const { status, data } = await apiInstance.v1GetFindEmail(
    jobId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] | Finder job identifier | defaults to undefined|


### Return type

**FindEmailStatusResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Finder job result |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetList**
> ListDetailResponse v1GetList()


### Example

```typescript
import {
    V1Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let listId: number; //List identifier (default to undefined)

const { status, data } = await apiInstance.v1GetList(
    listId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **listId** | [**number**] | List identifier | defaults to undefined|


### Return type

**ListDetailResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List detail |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListLists**
> ListListResponse v1ListLists()


### Example

```typescript
import {
    V1Api,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new V1Api(configuration);

let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1ListLists(
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**ListListResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List resources |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

