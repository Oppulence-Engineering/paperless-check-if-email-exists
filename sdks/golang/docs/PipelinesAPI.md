# \PipelinesAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1CreatePipeline**](PipelinesAPI.md#V1CreatePipeline) | **Post** /v1/pipelines | POST /v1/pipelines
[**V1DeletePipeline**](PipelinesAPI.md#V1DeletePipeline) | **Delete** /v1/pipelines/{pipeline_id} | DELETE /v1/pipelines/{pipeline_id}
[**V1GetPipeline**](PipelinesAPI.md#V1GetPipeline) | **Get** /v1/pipelines/{pipeline_id} | GET /v1/pipelines/{pipeline_id}
[**V1GetPipelineRun**](PipelinesAPI.md#V1GetPipelineRun) | **Get** /v1/pipelines/{pipeline_id}/runs/{run_id} | GET /v1/pipelines/{pipeline_id}/runs/{run_id}
[**V1ListPipelineRuns**](PipelinesAPI.md#V1ListPipelineRuns) | **Get** /v1/pipelines/{pipeline_id}/runs | GET /v1/pipelines/{pipeline_id}/runs
[**V1ListPipelines**](PipelinesAPI.md#V1ListPipelines) | **Get** /v1/pipelines | GET /v1/pipelines
[**V1PausePipeline**](PipelinesAPI.md#V1PausePipeline) | **Post** /v1/pipelines/{pipeline_id}/pause | POST /v1/pipelines/{pipeline_id}/pause
[**V1ResumePipeline**](PipelinesAPI.md#V1ResumePipeline) | **Post** /v1/pipelines/{pipeline_id}/resume | POST /v1/pipelines/{pipeline_id}/resume
[**V1TriggerPipeline**](PipelinesAPI.md#V1TriggerPipeline) | **Post** /v1/pipelines/{pipeline_id}/trigger | POST /v1/pipelines/{pipeline_id}/trigger
[**V1UpdatePipeline**](PipelinesAPI.md#V1UpdatePipeline) | **Patch** /v1/pipelines/{pipeline_id} | PATCH /v1/pipelines/{pipeline_id}



## V1CreatePipeline

> PipelineView V1CreatePipeline(ctx).CreatePipelineInput(createPipelineInput).Execute()

POST /v1/pipelines

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
	createPipelineInput := *openapiclient.NewCreatePipelineInput("Name_example", *openapiclient.NewPipelineSchedule("Cron_example", "Timezone_example"), openapiclient.PipelineSource{PipelineSourceOneOf: openapiclient.NewPipelineSourceOneOf(int32(123), "Type_example")}) // CreatePipelineInput | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1CreatePipeline(context.Background()).CreatePipelineInput(createPipelineInput).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1CreatePipeline``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1CreatePipeline`: PipelineView
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1CreatePipeline`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CreatePipelineRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createPipelineInput** | [**CreatePipelineInput**](CreatePipelineInput.md) |  | 

### Return type

[**PipelineView**](PipelineView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1DeletePipeline

> DeletePipelineResponse V1DeletePipeline(ctx, pipelineId).Execute()

DELETE /v1/pipelines/{pipeline_id}

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
	pipelineId := int64(789) // int64 | Pipeline identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1DeletePipeline(context.Background(), pipelineId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1DeletePipeline``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1DeletePipeline`: DeletePipelineResponse
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1DeletePipeline`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**pipelineId** | **int64** | Pipeline identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1DeletePipelineRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**DeletePipelineResponse**](DeletePipelineResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetPipeline

> PipelineView V1GetPipeline(ctx, pipelineId).Execute()

GET /v1/pipelines/{pipeline_id}

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
	pipelineId := int64(789) // int64 | Pipeline identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1GetPipeline(context.Background(), pipelineId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1GetPipeline``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1GetPipeline`: PipelineView
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1GetPipeline`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**pipelineId** | **int64** | Pipeline identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetPipelineRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**PipelineView**](PipelineView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetPipelineRun

> PipelineRunView V1GetPipelineRun(ctx, pipelineId, runId).Execute()

GET /v1/pipelines/{pipeline_id}/runs/{run_id}

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
	pipelineId := int64(789) // int64 | Pipeline identifier
	runId := int64(789) // int64 | Pipeline run identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1GetPipelineRun(context.Background(), pipelineId, runId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1GetPipelineRun``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1GetPipelineRun`: PipelineRunView
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1GetPipelineRun`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**pipelineId** | **int64** | Pipeline identifier | 
**runId** | **int64** | Pipeline run identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetPipelineRunRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



### Return type

[**PipelineRunView**](PipelineRunView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListPipelineRuns

> ListPipelineRunsResponse V1ListPipelineRuns(ctx, pipelineId).Limit(limit).Offset(offset).Execute()

GET /v1/pipelines/{pipeline_id}/runs

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
	pipelineId := int64(789) // int64 | Pipeline identifier
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1ListPipelineRuns(context.Background(), pipelineId).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1ListPipelineRuns``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListPipelineRuns`: ListPipelineRunsResponse
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1ListPipelineRuns`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**pipelineId** | **int64** | Pipeline identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1ListPipelineRunsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **limit** | **int64** |  | 
 **offset** | **int64** |  | 

### Return type

[**ListPipelineRunsResponse**](ListPipelineRunsResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListPipelines

> ListPipelinesResponse V1ListPipelines(ctx).Status(status).Limit(limit).Offset(offset).Execute()

GET /v1/pipelines

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
	status := "status_example" // string |  (optional)
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1ListPipelines(context.Background()).Status(status).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1ListPipelines``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListPipelines`: ListPipelinesResponse
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1ListPipelines`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1ListPipelinesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **status** | **string** |  | 
 **limit** | **int64** |  | 
 **offset** | **int64** |  | 

### Return type

[**ListPipelinesResponse**](ListPipelinesResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1PausePipeline

> PipelineView V1PausePipeline(ctx, pipelineId).Execute()

POST /v1/pipelines/{pipeline_id}/pause

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
	pipelineId := int64(789) // int64 | Pipeline identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1PausePipeline(context.Background(), pipelineId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1PausePipeline``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1PausePipeline`: PipelineView
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1PausePipeline`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**pipelineId** | **int64** | Pipeline identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1PausePipelineRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**PipelineView**](PipelineView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ResumePipeline

> PipelineView V1ResumePipeline(ctx, pipelineId).Execute()

POST /v1/pipelines/{pipeline_id}/resume

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
	pipelineId := int64(789) // int64 | Pipeline identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1ResumePipeline(context.Background(), pipelineId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1ResumePipeline``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ResumePipeline`: PipelineView
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1ResumePipeline`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**pipelineId** | **int64** | Pipeline identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1ResumePipelineRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**PipelineView**](PipelineView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1TriggerPipeline

> TriggerPipelineResponse V1TriggerPipeline(ctx, pipelineId).TriggerPipelineInput(triggerPipelineInput).Execute()

POST /v1/pipelines/{pipeline_id}/trigger

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
	pipelineId := int64(789) // int64 | Pipeline identifier
	triggerPipelineInput := *openapiclient.NewTriggerPipelineInput() // TriggerPipelineInput | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1TriggerPipeline(context.Background(), pipelineId).TriggerPipelineInput(triggerPipelineInput).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1TriggerPipeline``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1TriggerPipeline`: TriggerPipelineResponse
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1TriggerPipeline`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**pipelineId** | **int64** | Pipeline identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1TriggerPipelineRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **triggerPipelineInput** | [**TriggerPipelineInput**](TriggerPipelineInput.md) |  | 

### Return type

[**TriggerPipelineResponse**](TriggerPipelineResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1UpdatePipeline

> PipelineView V1UpdatePipeline(ctx, pipelineId).UpdatePipelineInput(updatePipelineInput).Execute()

PATCH /v1/pipelines/{pipeline_id}

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
	pipelineId := int64(789) // int64 | Pipeline identifier
	updatePipelineInput := *openapiclient.NewUpdatePipelineInput() // UpdatePipelineInput | 

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.PipelinesAPI.V1UpdatePipeline(context.Background(), pipelineId).UpdatePipelineInput(updatePipelineInput).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `PipelinesAPI.V1UpdatePipeline``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1UpdatePipeline`: PipelineView
	fmt.Fprintf(os.Stdout, "Response from `PipelinesAPI.V1UpdatePipeline`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**pipelineId** | **int64** | Pipeline identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiV1UpdatePipelineRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **updatePipelineInput** | [**UpdatePipelineInput**](UpdatePipelineInput.md) |  | 

### Return type

[**PipelineView**](PipelineView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)

