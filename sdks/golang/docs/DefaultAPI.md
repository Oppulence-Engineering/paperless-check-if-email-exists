# \DefaultAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetV1Bulk**](DefaultAPI.md#GetV1Bulk) | **Get** /v1/bulk/{job_id} | /v1/bulk/{job_id}
[**GetV1BulkResults**](DefaultAPI.md#GetV1BulkResults) | **Get** /v1/bulk/{job_id}/results | Retrieve bulk verification results
[**PostV0CheckEmail**](DefaultAPI.md#PostV0CheckEmail) | **Post** /v0/check_email | /v0/check_email
[**PostV1Bulk**](DefaultAPI.md#PostV1Bulk) | **Post** /v1/bulk | /v1/bulk
[**PostV1CheckEmail**](DefaultAPI.md#PostV1CheckEmail) | **Post** /v1/check_email | /v1/check_email



## GetV1Bulk

> GetV1Bulk200Response GetV1Bulk(ctx, jobId).Execute()

/v1/bulk/{job_id}



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
	jobId := int32(56) // int32 | The unique bulk verification job ID

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetV1Bulk(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetV1Bulk``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetV1Bulk`: GetV1Bulk200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetV1Bulk`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | The unique bulk verification job ID |

### Other Parameters

Other parameters are passed through a pointer to a apiGetV1BulkRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**GetV1Bulk200Response**](GetV1Bulk200Response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetV1BulkResults

> GetV1BulkResults200Response GetV1BulkResults(ctx, jobId).Limit(limit).Offset(offset).Execute()

Retrieve bulk verification results



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
	jobId := "jobId_example" // string | The unique bulk verification job ID
	limit := int32(56) // int32 | The number of results to return. (optional) (default to 50)
	offset := int32(56) // int32 | The offset from which to return the results, equivalent to the number of elements in the array to skip. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.GetV1BulkResults(context.Background(), jobId).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.GetV1BulkResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetV1BulkResults`: GetV1BulkResults200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.GetV1BulkResults`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **string** | The unique bulk verification job ID |

### Other Parameters

Other parameters are passed through a pointer to a apiGetV1BulkResultsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **limit** | **int32** | The number of results to return. | [default to 50]
 **offset** | **int32** | The offset from which to return the results, equivalent to the number of elements in the array to skip. |

### Return type

[**GetV1BulkResults200Response**](GetV1BulkResults200Response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostV0CheckEmail

> CheckEmailOutput PostV0CheckEmail(ctx).Authorization(authorization).CheckEmailRequest(checkEmailRequest).Execute()

/v0/check_email



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
	authorization := "authorization_example" // string | Your personal Reacher API key
	checkEmailRequest := *openapiclient.NewCheckEmailRequest("ToEmail_example") // CheckEmailRequest | Request object containing all parameters necessary for an email verification. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PostV0CheckEmail(context.Background()).Authorization(authorization).CheckEmailRequest(checkEmailRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PostV0CheckEmail``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostV0CheckEmail`: CheckEmailOutput
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PostV0CheckEmail`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostV0CheckEmailRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **authorization** | **string** | Your personal Reacher API key |
 **checkEmailRequest** | [**CheckEmailRequest**](CheckEmailRequest.md) | Request object containing all parameters necessary for an email verification. |

### Return type

[**CheckEmailOutput**](CheckEmailOutput.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostV1Bulk

> PostV1Bulk200Response PostV1Bulk(ctx).PostV1BulkRequest(postV1BulkRequest).Execute()

/v1/bulk



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
	postV1BulkRequest := *openapiclient.NewPostV1BulkRequest([]string{"Input_example"}) // PostV1BulkRequest |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PostV1Bulk(context.Background()).PostV1BulkRequest(postV1BulkRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PostV1Bulk``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostV1Bulk`: PostV1Bulk200Response
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PostV1Bulk`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostV1BulkRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **postV1BulkRequest** | [**PostV1BulkRequest**](PostV1BulkRequest.md) |  |

### Return type

[**PostV1Bulk200Response**](PostV1Bulk200Response.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## PostV1CheckEmail

> CheckEmailOutput PostV1CheckEmail(ctx).CheckEmailRequest(checkEmailRequest).Execute()

/v1/check_email



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
	checkEmailRequest := map[string][]openapiclient.CheckEmailRequest{ ... } // CheckEmailRequest | Request object containing all parameters necessary for an email verification. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.DefaultAPI.PostV1CheckEmail(context.Background()).CheckEmailRequest(checkEmailRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `DefaultAPI.PostV1CheckEmail``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostV1CheckEmail`: CheckEmailOutput
	fmt.Fprintf(os.Stdout, "Response from `DefaultAPI.PostV1CheckEmail`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostV1CheckEmailRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **checkEmailRequest** | [**CheckEmailRequest**](CheckEmailRequest.md) | Request object containing all parameters necessary for an email verification. |

### Return type

[**CheckEmailOutput**](CheckEmailOutput.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
