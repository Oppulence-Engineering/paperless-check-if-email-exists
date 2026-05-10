# \OutcomesAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1ListOutcomes**](OutcomesAPI.md#V1ListOutcomes) | **Get** /v1/outcomes |
[**V1PostOutcomes**](OutcomesAPI.md#V1PostOutcomes) | **Post** /v1/outcomes |
[**V1UploadOutcomes**](OutcomesAPI.md#V1UploadOutcomes) | **Post** /v1/outcomes/upload |



## V1ListOutcomes

> OutcomeListResponse V1ListOutcomes(ctx).Email(email).Source(source).Type_(type_).Since(since).Limit(limit).Offset(offset).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/reacher"
)

func main() {
	email := "email_example" // string |  (optional)
	source := "source_example" // string |  (optional)
	type_ := "type__example" // string |  (optional)
	since := time.Now() // time.Time |  (optional)
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.OutcomesAPI.V1ListOutcomes(context.Background()).Email(email).Source(source).Type_(type_).Since(since).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OutcomesAPI.V1ListOutcomes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListOutcomes`: OutcomeListResponse
	fmt.Fprintf(os.Stdout, "Response from `OutcomesAPI.V1ListOutcomes`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1ListOutcomesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **email** | **string** |  |
 **source** | **string** |  |
 **type_** | **string** |  |
 **since** | **time.Time** |  |
 **limit** | **int64** |  |
 **offset** | **int64** |  |

### Return type

[**OutcomeListResponse**](OutcomeListResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1PostOutcomes

> IngestOutcomesResponse V1PostOutcomes(ctx).IngestOutcomesRequest(ingestOutcomesRequest).Execute()



### Example

```go
package main

import (
	"context"
	"fmt"
	"os"
    "time"
	openapiclient "github.com/GIT_USER_ID/GIT_REPO_ID/reacher"
)

func main() {
	ingestOutcomesRequest := *openapiclient.NewIngestOutcomesRequest([]openapiclient.IngestOutcome{*openapiclient.NewIngestOutcome("Email_example", time.Now(), openapiclient.OutcomeType("delivered"))}) // IngestOutcomesRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.OutcomesAPI.V1PostOutcomes(context.Background()).IngestOutcomesRequest(ingestOutcomesRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OutcomesAPI.V1PostOutcomes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1PostOutcomes`: IngestOutcomesResponse
	fmt.Fprintf(os.Stdout, "Response from `OutcomesAPI.V1PostOutcomes`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1PostOutcomesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **ingestOutcomesRequest** | [**IngestOutcomesRequest**](IngestOutcomesRequest.md) |  |

### Return type

[**IngestOutcomesResponse**](IngestOutcomesResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1UploadOutcomes

> IngestOutcomesResponse V1UploadOutcomes(ctx).Execute()



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
	resp, r, err := apiClient.OutcomesAPI.V1UploadOutcomes(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OutcomesAPI.V1UploadOutcomes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1UploadOutcomes`: IngestOutcomesResponse
	fmt.Fprintf(os.Stdout, "Response from `OutcomesAPI.V1UploadOutcomes`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiV1UploadOutcomesRequest struct via the builder pattern


### Return type

[**IngestOutcomesResponse**](IngestOutcomesResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
