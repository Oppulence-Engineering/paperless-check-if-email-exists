# \QueryAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1QueryResults**](QueryAPI.md#V1QueryResults) | **Get** /v1/query | GET /v1/query



## V1QueryResults

> V1QueryResults(ctx).Limit(limit).Offset(offset).Category(category).MinScore(minScore).MaxScore(maxScore).SafeToSend(safeToSend).JobId(jobId).Since(since).Until(until).Execute()

GET /v1/query



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
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)
	category := "category_example" // string |  (optional)
	minScore := int32(56) // int32 |  (optional)
	maxScore := int32(56) // int32 |  (optional)
	safeToSend := true // bool |  (optional)
	jobId := int32(56) // int32 |  (optional)
	since := "since_example" // string |  (optional)
	until := "until_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.QueryAPI.V1QueryResults(context.Background()).Limit(limit).Offset(offset).Category(category).MinScore(minScore).MaxScore(maxScore).SafeToSend(safeToSend).JobId(jobId).Since(since).Until(until).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `QueryAPI.V1QueryResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1QueryResultsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int64** |  | 
 **offset** | **int64** |  | 
 **category** | **string** |  | 
 **minScore** | **int32** |  | 
 **maxScore** | **int32** |  | 
 **safeToSend** | **bool** |  | 
 **jobId** | **int32** |  | 
 **since** | **string** |  | 
 **until** | **string** |  | 

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

