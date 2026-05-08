# \EventsAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1ListEvents**](EventsAPI.md#V1ListEvents) | **Get** /v1/events | GET /v1/events



## V1ListEvents

> V1ListEvents(ctx).Limit(limit).Offset(offset).EventType(eventType).Actor(actor).JobId(jobId).Since(since).Until(until).Execute()

GET /v1/events



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
	eventType := "eventType_example" // string |  (optional)
	actor := "actor_example" // string |  (optional)
	jobId := int32(56) // int32 |  (optional)
	since := "since_example" // string |  (optional)
	until := "until_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.EventsAPI.V1ListEvents(context.Background()).Limit(limit).Offset(offset).EventType(eventType).Actor(actor).JobId(jobId).Since(since).Until(until).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `EventsAPI.V1ListEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1ListEventsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int64** |  | 
 **offset** | **int64** |  | 
 **eventType** | **string** |  | 
 **actor** | **string** |  | 
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

