# \ListsAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1ListQuality**](ListsAPI.md#V1ListQuality) | **Get** /v1/lists/{list_id}/quality | GET /v1/lists/{list_id}/quality
[**V1ListsListIdRemediationExportsExportIdDownloadGet**](ListsAPI.md#V1ListsListIdRemediationExportsExportIdDownloadGet) | **Get** /v1/lists/{list_id}/remediation-exports/{export_id}/download | Download remediation export
[**V1ListsListIdRemediationExportsPost**](ListsAPI.md#V1ListsListIdRemediationExportsPost) | **Post** /v1/lists/{list_id}/remediation-exports | Create remediation export
[**V1ListsListIdRemediationPlanGet**](ListsAPI.md#V1ListsListIdRemediationPlanGet) | **Get** /v1/lists/{list_id}/remediation-plan | Get remediation plan
[**V1ListsListIdRemediationPlanPost**](ListsAPI.md#V1ListsListIdRemediationPlanPost) | **Post** /v1/lists/{list_id}/remediation-plan | Create remediation plan



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


## V1ListsListIdRemediationExportsExportIdDownloadGet

> *os.File V1ListsListIdRemediationExportsExportIdDownloadGet(ctx, listId, exportId).Execute()

Download remediation export

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
	listId := int32(56) // int32 |
	exportId := int64(789) // int64 |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ListsAPI.V1ListsListIdRemediationExportsExportIdDownloadGet(context.Background(), listId, exportId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1ListsListIdRemediationExportsExportIdDownloadGet``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListsListIdRemediationExportsExportIdDownloadGet`: *os.File
	fmt.Fprintf(os.Stdout, "Response from `ListsAPI.V1ListsListIdRemediationExportsExportIdDownloadGet`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**listId** | **int32** |  |
**exportId** | **int64** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiV1ListsListIdRemediationExportsExportIdDownloadGetRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



### Return type

[***os.File**](*os.File.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListsListIdRemediationExportsPost

> map[string]interface{} V1ListsListIdRemediationExportsPost(ctx, listId).RequestBody(requestBody).Execute()

Create remediation export

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
	listId := int32(56) // int32 |
	requestBody := map[string]interface{}{"key": interface{}(123)} // map[string]interface{} |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ListsAPI.V1ListsListIdRemediationExportsPost(context.Background(), listId).RequestBody(requestBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1ListsListIdRemediationExportsPost``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListsListIdRemediationExportsPost`: map[string]interface{}
	fmt.Fprintf(os.Stdout, "Response from `ListsAPI.V1ListsListIdRemediationExportsPost`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**listId** | **int32** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiV1ListsListIdRemediationExportsPostRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **requestBody** | **map[string]interface{}** |  |

### Return type

**map[string]interface{}**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListsListIdRemediationPlanGet

> map[string]interface{} V1ListsListIdRemediationPlanGet(ctx, listId).Execute()

Get remediation plan

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
	listId := int32(56) // int32 |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ListsAPI.V1ListsListIdRemediationPlanGet(context.Background(), listId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1ListsListIdRemediationPlanGet``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListsListIdRemediationPlanGet`: map[string]interface{}
	fmt.Fprintf(os.Stdout, "Response from `ListsAPI.V1ListsListIdRemediationPlanGet`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**listId** | **int32** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiV1ListsListIdRemediationPlanGetRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

**map[string]interface{}**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListsListIdRemediationPlanPost

> map[string]interface{} V1ListsListIdRemediationPlanPost(ctx, listId).RequestBody(requestBody).Execute()

Create remediation plan

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
	listId := int32(56) // int32 |
	requestBody := map[string]interface{}{"key": interface{}(123)} // map[string]interface{} |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.ListsAPI.V1ListsListIdRemediationPlanPost(context.Background(), listId).RequestBody(requestBody).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `ListsAPI.V1ListsListIdRemediationPlanPost``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListsListIdRemediationPlanPost`: map[string]interface{}
	fmt.Fprintf(os.Stdout, "Response from `ListsAPI.V1ListsListIdRemediationPlanPost`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**listId** | **int32** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiV1ListsListIdRemediationPlanPostRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **requestBody** | **map[string]interface{}** |  |

### Return type

**map[string]interface{}**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
