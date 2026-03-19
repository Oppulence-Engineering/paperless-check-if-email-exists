# \V0API

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateBulkJob**](V0API.md#CreateBulkJob) | **Post** /v0/bulk | POST /v0/bulk
[**GetBulkJobResult**](V0API.md#GetBulkJobResult) | **Get** /v0/bulk/{job_id}/results | GET /v0/bulk/{job_id}/results
[**GetBulkJobStatus**](V0API.md#GetBulkJobStatus) | **Get** /v0/bulk/{job_id} | GET /v0/bulk/{job_id}
[**PostCheckEmail**](V0API.md#PostCheckEmail) | **Post** /v0/check_email | POST /v0/check_email



## CreateBulkJob

> CreateBulkJob(ctx).Execute()

POST /v0/bulk



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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.V0API.CreateBulkJob(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V0API.CreateBulkJob``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiCreateBulkJobRequest struct via the builder pattern


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


## GetBulkJobResult

> GetBulkJobResult(ctx, jobId).Execute()

GET /v0/bulk/{job_id}/results



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
	jobId := int32(56) // int32 | Legacy bulk job identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.V0API.GetBulkJobResult(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V0API.GetBulkJobResult``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Legacy bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetBulkJobResultRequest struct via the builder pattern


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


## GetBulkJobStatus

> GetBulkJobStatus(ctx, jobId).Execute()

GET /v0/bulk/{job_id}



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
	jobId := int32(56) // int32 | Legacy bulk job identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.V0API.GetBulkJobStatus(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V0API.GetBulkJobStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Legacy bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetBulkJobStatusRequest struct via the builder pattern


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


## PostCheckEmail

> CheckEmailOutput PostCheckEmail(ctx).CheckEmailRequest(checkEmailRequest).IdempotencyKey(idempotencyKey).Execute()

POST /v0/check_email



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
	checkEmailRequest := *openapiclient.NewCheckEmailRequest("ToEmail_example") // CheckEmailRequest | 
	idempotencyKey := "idempotencyKey_example" // string | Optional idempotency key (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V0API.PostCheckEmail(context.Background()).CheckEmailRequest(checkEmailRequest).IdempotencyKey(idempotencyKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V0API.PostCheckEmail``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `PostCheckEmail`: CheckEmailOutput
	fmt.Fprintf(os.Stdout, "Response from `V0API.PostCheckEmail`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiPostCheckEmailRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **checkEmailRequest** | [**CheckEmailRequest**](CheckEmailRequest.md) |  | 
 **idempotencyKey** | **string** | Optional idempotency key | 

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

