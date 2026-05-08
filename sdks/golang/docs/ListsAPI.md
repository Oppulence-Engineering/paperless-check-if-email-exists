# \ListsAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1CreateSavedSegment**](ListsAPI.md#V1CreateSavedSegment) | **Post** /v1/segments | POST /v1/segments
[**V1DeleteSavedSegment**](ListsAPI.md#V1DeleteSavedSegment) | **Delete** /v1/segments/{segment_id} | DELETE /v1/segments/{segment_id}
[**V1DiffLists**](ListsAPI.md#V1DiffLists) | **Get** /v1/lists/{base_list_id}/diff/{compare_list_id} | GET /v1/lists/{base_list_id}/diff/{compare_list_id}
[**V1GetSavedSegment**](ListsAPI.md#V1GetSavedSegment) | **Get** /v1/segments/{segment_id} | GET /v1/segments/{segment_id}
[**V1ListQuality**](ListsAPI.md#V1ListQuality) | **Get** /v1/lists/{list_id}/quality | GET /v1/lists/{list_id}/quality
[**V1ListSavedSegments**](ListsAPI.md#V1ListSavedSegments) | **Get** /v1/segments | GET /v1/segments
[**V1UpdateSavedSegment**](ListsAPI.md#V1UpdateSavedSegment) | **Patch** /v1/segments/{segment_id} | PATCH /v1/segments/{segment_id}



## V1CreateSavedSegment

> SavedSegmentView V1CreateSavedSegment(ctx).CreateSavedSegmentRequest(createSavedSegmentRequest).Execute()

POST /v1/segments

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/reacher"
)

func main() {
	createSavedSegmentRequest := *openapiclient.NewCreateSavedSegmentRequest("Name_example") // CreateSavedSegmentRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ListsAPI.V1CreateSavedSegment(context.Background()).CreateSavedSegmentRequest(createSavedSegmentRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1CreateSavedSegment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1CreateSavedSegment`: SavedSegmentView
	fmt.Fprintf(os.Stdout, "Response from `ListsAPI.V1CreateSavedSegment`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CreateSavedSegmentRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createSavedSegmentRequest** | [**CreateSavedSegmentRequest**](CreateSavedSegmentRequest.md) |  |

### Return type

[**SavedSegmentView**](SavedSegmentView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1DeleteSavedSegment

> V1DeleteSavedSegment(ctx, segmentId).Execute()

DELETE /v1/segments/{segment_id}

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/reacher"
)

func main() {
	segmentId := int64(789) // int64 | Saved segment identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.ListsAPI.V1DeleteSavedSegment(context.Background(), segmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1DeleteSavedSegment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**segmentId** | **int64** | Saved segment identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1DeleteSavedSegmentRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1DiffLists

> ListDiffResponse V1DiffLists(ctx, baseListId, compareListId).Limit(limit).Offset(offset).Execute()

GET /v1/lists/{base_list_id}/diff/{compare_list_id}

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/reacher"
)

func main() {
	baseListId := int32(56) // int32 | Base list identifier
	compareListId := int32(56) // int32 | Compare list identifier
	limit := int32(56) // int32 |  (optional)
	offset := int32(56) // int32 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ListsAPI.V1DiffLists(context.Background(), baseListId, compareListId).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1DiffLists``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1DiffLists`: ListDiffResponse
	fmt.Fprintf(os.Stdout, "Response from `ListsAPI.V1DiffLists`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**baseListId** | **int32** | Base list identifier |
**compareListId** | **int32** | Compare list identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1DiffListsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


 **limit** | **int32** |  |
 **offset** | **int32** |  |

### Return type

[**ListDiffResponse**](ListDiffResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetSavedSegment

> SavedSegmentView V1GetSavedSegment(ctx, segmentId).Execute()

GET /v1/segments/{segment_id}

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/reacher"
)

func main() {
	segmentId := int64(789) // int64 | Saved segment identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ListsAPI.V1GetSavedSegment(context.Background(), segmentId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1GetSavedSegment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1GetSavedSegment`: SavedSegmentView
	fmt.Fprintf(os.Stdout, "Response from `ListsAPI.V1GetSavedSegment`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**segmentId** | **int64** | Saved segment identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetSavedSegmentRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**SavedSegmentView**](SavedSegmentView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListQuality

> V1ListQuality(ctx, listId).Execute()

GET /v1/lists/{list_id}/quality



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/reacher"
)

func main() {
	listId := int32(56) // int32 | List identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.ListsAPI.V1ListQuality(context.Background(), listId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1ListQuality``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**listId** | **int32** | List identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1ListQualityRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListSavedSegments

> SavedSegmentListResponse V1ListSavedSegments(ctx).Scope(scope).Limit(limit).Offset(offset).Execute()

GET /v1/segments

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/reacher"
)

func main() {
	scope := "scope_example" // string |  (optional)
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ListsAPI.V1ListSavedSegments(context.Background()).Scope(scope).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1ListSavedSegments``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListSavedSegments`: SavedSegmentListResponse
	fmt.Fprintf(os.Stdout, "Response from `ListsAPI.V1ListSavedSegments`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1ListSavedSegmentsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **scope** | **string** |  |
 **limit** | **int64** |  |
 **offset** | **int64** |  |

### Return type

[**SavedSegmentListResponse**](SavedSegmentListResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1UpdateSavedSegment

> SavedSegmentView V1UpdateSavedSegment(ctx, segmentId).UpdateSavedSegmentRequest(updateSavedSegmentRequest).Execute()

PATCH /v1/segments/{segment_id}

### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/reacher"
)

func main() {
	segmentId := int64(789) // int64 | Saved segment identifier
	updateSavedSegmentRequest := *openapiclient.NewUpdateSavedSegmentRequest() // UpdateSavedSegmentRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ListsAPI.V1UpdateSavedSegment(context.Background(), segmentId).UpdateSavedSegmentRequest(updateSavedSegmentRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1UpdateSavedSegment``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1UpdateSavedSegment`: SavedSegmentView
	fmt.Fprintf(os.Stdout, "Response from `ListsAPI.V1UpdateSavedSegment`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**segmentId** | **int64** | Saved segment identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1UpdateSavedSegmentRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **updateSavedSegmentRequest** | [**UpdateSavedSegmentRequest**](UpdateSavedSegmentRequest.md) |  |

### Return type

[**SavedSegmentView**](SavedSegmentView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
