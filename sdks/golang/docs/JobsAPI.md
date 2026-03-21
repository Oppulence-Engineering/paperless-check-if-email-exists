# \JobsAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1CancelJob**](JobsAPI.md#V1CancelJob) | **Post** /v1/jobs/{job_id}/cancel | POST /v1/jobs/{job_id}/cancel
[**V1DownloadJobResults**](JobsAPI.md#V1DownloadJobResults) | **Get** /v1/jobs/{job_id}/download | GET /v1/jobs/{job_id}/download
[**V1GetBulkJobProgress**](JobsAPI.md#V1GetBulkJobProgress) | **Get** /v1/bulk/{job_id} | GET /v1/bulk/{job_id}
[**V1GetBulkJobResults**](JobsAPI.md#V1GetBulkJobResults) | **Get** /v1/bulk/{job_id}/results | GET /v1/bulk/{job_id}/results
[**V1GetJobEvents**](JobsAPI.md#V1GetJobEvents) | **Get** /v1/jobs/{job_id}/events | GET /v1/jobs/{job_id}/events
[**V1GetJobResults**](JobsAPI.md#V1GetJobResults) | **Get** /v1/jobs/{job_id}/results | GET /v1/jobs/{job_id}/results
[**V1GetJobStatus**](JobsAPI.md#V1GetJobStatus) | **Get** /v1/jobs/{job_id} | GET /v1/jobs/{job_id}
[**V1JobApprovalChecklist**](JobsAPI.md#V1JobApprovalChecklist) | **Get** /v1/jobs/{job_id}/approval | GET /v1/jobs/{job_id}/approval
[**V1RetryJob**](JobsAPI.md#V1RetryJob) | **Post** /v1/jobs/{job_id}/retry | POST /v1/jobs/{job_id}/retry



## V1CancelJob

> V1CancelJob(ctx, jobId).Execute()

POST /v1/jobs/{job_id}/cancel



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
	jobId := int32(56) // int32 | Bulk job identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.JobsAPI.V1CancelJob(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JobsAPI.V1CancelJob``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1CancelJobRequest struct via the builder pattern


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


## V1DownloadJobResults

> *os.File V1DownloadJobResults(ctx, jobId).Format(format).Execute()

GET /v1/jobs/{job_id}/download



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
	jobId := int32(56) // int32 | Bulk job identifier
	format := "format_example" // string | Supported values: `csv`, `json` (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.JobsAPI.V1DownloadJobResults(context.Background(), jobId).Format(format).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JobsAPI.V1DownloadJobResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1DownloadJobResults`: *os.File
	fmt.Fprintf(os.Stdout, "Response from `JobsAPI.V1DownloadJobResults`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1DownloadJobResultsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **format** | **string** | Supported values: &#x60;csv&#x60;, &#x60;json&#x60; | 

### Return type

[***os.File**](*os.File.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/x-ndjson, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetBulkJobProgress

> V1GetBulkJobProgress(ctx, jobId).Execute()

GET /v1/bulk/{job_id}



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
	jobId := int32(56) // int32 | V1 bulk job identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.JobsAPI.V1GetBulkJobProgress(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JobsAPI.V1GetBulkJobProgress``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | V1 bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetBulkJobProgressRequest struct via the builder pattern


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


## V1GetBulkJobResults

> BulkJobResultsResponse V1GetBulkJobResults(ctx, jobId).Format(format).Limit(limit).Offset(offset).Execute()

GET /v1/bulk/{job_id}/results



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
	jobId := int32(56) // int32 | V1 bulk job identifier
	format := "format_example" // string | Supported values: `json`, `csv` (optional)
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.JobsAPI.V1GetBulkJobResults(context.Background(), jobId).Format(format).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JobsAPI.V1GetBulkJobResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1GetBulkJobResults`: BulkJobResultsResponse
	fmt.Fprintf(os.Stdout, "Response from `JobsAPI.V1GetBulkJobResults`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | V1 bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetBulkJobResultsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **format** | **string** | Supported values: &#x60;json&#x60;, &#x60;csv&#x60; | 
 **limit** | **int64** |  | 
 **offset** | **int64** |  | 

### Return type

[**BulkJobResultsResponse**](BulkJobResultsResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, text/csv

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetJobEvents

> V1GetJobEvents(ctx, jobId).Limit(limit).Offset(offset).Execute()

GET /v1/jobs/{job_id}/events



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
	jobId := int32(56) // int32 | Bulk job identifier
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.JobsAPI.V1GetJobEvents(context.Background(), jobId).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JobsAPI.V1GetJobEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetJobEventsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **limit** | **int64** |  | 
 **offset** | **int64** |  | 

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


## V1GetJobResults

> JobResultPageResponse V1GetJobResults(ctx, jobId).Cursor(cursor).Limit(limit).State(state).Execute()

GET /v1/jobs/{job_id}/results



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
	jobId := int32(56) // int32 | Bulk job identifier
	cursor := int64(789) // int64 |  (optional)
	limit := int64(789) // int64 |  (optional)
	state := "state_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.JobsAPI.V1GetJobResults(context.Background(), jobId).Cursor(cursor).Limit(limit).State(state).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JobsAPI.V1GetJobResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1GetJobResults`: JobResultPageResponse
	fmt.Fprintf(os.Stdout, "Response from `JobsAPI.V1GetJobResults`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetJobResultsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **cursor** | **int64** |  | 
 **limit** | **int64** |  | 
 **state** | **string** |  | 

### Return type

[**JobResultPageResponse**](JobResultPageResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetJobStatus

> V1GetJobStatus(ctx, jobId).Execute()

GET /v1/jobs/{job_id}



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
	jobId := int32(56) // int32 | Bulk job identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.JobsAPI.V1GetJobStatus(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JobsAPI.V1GetJobStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetJobStatusRequest struct via the builder pattern


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


## V1JobApprovalChecklist

> ApprovalChecklistResponse V1JobApprovalChecklist(ctx, jobId).Execute()

GET /v1/jobs/{job_id}/approval



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
	jobId := int32(56) // int32 | Bulk job identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.JobsAPI.V1JobApprovalChecklist(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JobsAPI.V1JobApprovalChecklist``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1JobApprovalChecklist`: ApprovalChecklistResponse
	fmt.Fprintf(os.Stdout, "Response from `JobsAPI.V1JobApprovalChecklist`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1JobApprovalChecklistRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ApprovalChecklistResponse**](ApprovalChecklistResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1RetryJob

> RetryJobResponse V1RetryJob(ctx, jobId).Execute()

POST /v1/jobs/{job_id}/retry



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
	jobId := int32(56) // int32 | Bulk job identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.JobsAPI.V1RetryJob(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `JobsAPI.V1RetryJob``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1RetryJob`: RetryJobResponse
	fmt.Fprintf(os.Stdout, "Response from `JobsAPI.V1RetryJob`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Bulk job identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1RetryJobRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**RetryJobResponse**](RetryJobResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

