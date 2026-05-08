# CommentsApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1CreateComment**](#v1createcomment) | **POST** /v1/comments | POST /v1/comments|
|[**v1DeleteComment**](#v1deletecomment) | **DELETE** /v1/comments/{comment_id} | DELETE /v1/comments/{comment_id}|
|[**v1ListComments**](#v1listcomments) | **GET** /v1/comments | GET /v1/comments|

# **v1CreateComment**
> v1CreateComment()


### Example

```typescript
import {
    CommentsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new CommentsApi(configuration);

const { status, data } = await apiInstance.v1CreateComment();
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
|**201** | Comment created |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1DeleteComment**
> v1DeleteComment()


### Example

```typescript
import {
    CommentsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new CommentsApi(configuration);

let commentId: number; //Comment identifier (default to undefined)

const { status, data } = await apiInstance.v1DeleteComment(
    commentId
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **commentId** | [**number**] | Comment identifier | defaults to undefined|


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
|**200** | Comment deleted |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListComments**
> v1ListComments()


### Example

```typescript
import {
    CommentsApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new CommentsApi(configuration);

let jobId: number; // (optional) (default to undefined)
let listId: number; // (optional) (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1ListComments(
    jobId,
    listId,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **jobId** | [**number**] |  | (optional) defaults to undefined|
| **listId** | [**number**] |  | (optional) defaults to undefined|
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
|**200** | Comments list |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
