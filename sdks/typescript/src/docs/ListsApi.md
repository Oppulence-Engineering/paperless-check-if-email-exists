# ListsApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1CreateSavedSegment**](#v1createsavedsegment) | **POST** /v1/segments | POST /v1/segments|
|[**v1DeleteSavedSegment**](#v1deletesavedsegment) | **DELETE** /v1/segments/{segment_id} | DELETE /v1/segments/{segment_id}|
|[**v1DiffLists**](#v1difflists) | **GET** /v1/lists/{base_list_id}/diff/{compare_list_id} | GET /v1/lists/{base_list_id}/diff/{compare_list_id}|
|[**v1GetSavedSegment**](#v1getsavedsegment) | **GET** /v1/segments/{segment_id} | GET /v1/segments/{segment_id}|
|[**v1ListQuality**](#v1listquality) | **GET** /v1/lists/{list_id}/quality | GET /v1/lists/{list_id}/quality|
|[**v1ListSavedSegments**](#v1listsavedsegments) | **GET** /v1/segments | GET /v1/segments|
|[**v1UpdateSavedSegment**](#v1updatesavedsegment) | **PATCH** /v1/segments/{segment_id} | PATCH /v1/segments/{segment_id}|

# **v1CreateSavedSegment**
> SavedSegmentView v1CreateSavedSegment(createSavedSegmentRequest)


### Example

```typescript
import {
    ListsApi,
    Configuration,
    CreateSavedSegmentRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let createSavedSegmentRequest: CreateSavedSegmentRequest; //

const { status, data } = await apiInstance.v1CreateSavedSegment(
    createSavedSegmentRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **createSavedSegmentRequest** | **CreateSavedSegmentRequest**|  | |


### Return type

**SavedSegmentView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**201** | Saved segment created |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DeleteSavedSegment**
> v1DeleteSavedSegment()


### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let segmentId: number; //Saved segment identifier (default to undefined)

const { status, data } = await apiInstance.v1DeleteSavedSegment(
    segmentId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **segmentId** | [**number**] | Saved segment identifier | defaults to undefined|


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
|**200** | Saved segment deleted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DiffLists**
> ListDiffResponse v1DiffLists()


### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let baseListId: number; //Base list identifier (default to undefined)
let compareListId: number; //Compare list identifier (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1DiffLists(
    baseListId,
    compareListId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **baseListId** | [**number**] | Base list identifier | defaults to undefined|
| **compareListId** | [**number**] | Compare list identifier | defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**ListDiffResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | List diff |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1GetSavedSegment**
> SavedSegmentView v1GetSavedSegment()


### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let segmentId: number; //Saved segment identifier (default to undefined)

const { status, data } = await apiInstance.v1GetSavedSegment(
    segmentId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **segmentId** | [**number**] | Saved segment identifier | defaults to undefined|


### Return type

**SavedSegmentView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Saved segment |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListQuality**
> v1ListQuality()

Returns a quality benchmark report for a list.

### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let listId: number; //List identifier (default to undefined)

const { status, data } = await apiInstance.v1ListQuality(
    listId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **listId** | [**number**] | List identifier | defaults to undefined|


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
|**200** | List quality benchmark report |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListSavedSegments**
> SavedSegmentListResponse v1ListSavedSegments()


### Example

```typescript
import {
    ListsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let scope: string; // (optional) (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1ListSavedSegments(
    scope,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **scope** | [**string**] |  | (optional) defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**SavedSegmentListResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Saved segments |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1UpdateSavedSegment**
> SavedSegmentView v1UpdateSavedSegment(updateSavedSegmentRequest)


### Example

```typescript
import {
    ListsApi,
    Configuration,
    UpdateSavedSegmentRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new ListsApi(configuration);

let segmentId: number; //Saved segment identifier (default to undefined)
let updateSavedSegmentRequest: UpdateSavedSegmentRequest; //

const { status, data } = await apiInstance.v1UpdateSavedSegment(
    segmentId,
    updateSavedSegmentRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateSavedSegmentRequest** | **UpdateSavedSegmentRequest**|  | |
| **segmentId** | [**number**] | Saved segment identifier | defaults to undefined|


### Return type

**SavedSegmentView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Saved segment updated |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
