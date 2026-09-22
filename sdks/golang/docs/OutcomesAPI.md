# \OutcomesAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1CreateProviderEndpoint**](OutcomesAPI.md#V1CreateProviderEndpoint) | **Post** /v1/provider-endpoints |
[**V1DeleteProviderEndpoint**](OutcomesAPI.md#V1DeleteProviderEndpoint) | **Delete** /v1/provider-endpoints/{endpoint_id} |
[**V1IngestOutcomes**](OutcomesAPI.md#V1IngestOutcomes) | **Post** /v1/outcomes | POST /v1/outcomes
[**V1IngestProviderOutcomes**](OutcomesAPI.md#V1IngestProviderOutcomes) | **Post** /v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token} |
[**V1ListOutcomes**](OutcomesAPI.md#V1ListOutcomes) | **Get** /v1/outcomes | GET /v1/outcomes
[**V1ListProviderEndpoints**](OutcomesAPI.md#V1ListProviderEndpoints) | **Get** /v1/provider-endpoints |
[**V1UpdateProviderEndpoint**](OutcomesAPI.md#V1UpdateProviderEndpoint) | **Patch** /v1/provider-endpoints/{endpoint_id} |



## V1CreateProviderEndpoint

> ProviderEndpointView V1CreateProviderEndpoint(ctx).CreateProviderEndpointInput(createProviderEndpointInput).Execute()



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
	createProviderEndpointInput := *openapiclient.NewCreateProviderEndpointInput("Label_example", "Provider_example") // CreateProviderEndpointInput |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.OutcomesAPI.V1CreateProviderEndpoint(context.Background()).CreateProviderEndpointInput(createProviderEndpointInput).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OutcomesAPI.V1CreateProviderEndpoint``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1CreateProviderEndpoint`: ProviderEndpointView
	fmt.Fprintf(os.Stdout, "Response from `OutcomesAPI.V1CreateProviderEndpoint`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CreateProviderEndpointRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createProviderEndpointInput** | [**CreateProviderEndpointInput**](CreateProviderEndpointInput.md) |  |

### Return type

[**ProviderEndpointView**](ProviderEndpointView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1DeleteProviderEndpoint

> ProviderDeleteResponse V1DeleteProviderEndpoint(ctx, endpointId).Execute()



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
	endpointId := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Provider endpoint identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.OutcomesAPI.V1DeleteProviderEndpoint(context.Background(), endpointId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OutcomesAPI.V1DeleteProviderEndpoint``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1DeleteProviderEndpoint`: ProviderDeleteResponse
	fmt.Fprintf(os.Stdout, "Response from `OutcomesAPI.V1DeleteProviderEndpoint`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**endpointId** | **string** | Provider endpoint identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1DeleteProviderEndpointRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ProviderDeleteResponse**](ProviderDeleteResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1IngestOutcomes

> OutcomeIngestResponse V1IngestOutcomes(ctx).OutcomeIngestRequest(outcomeIngestRequest).Execute()

POST /v1/outcomes

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
	outcomeIngestRequest := *openapiclient.NewOutcomeIngestRequest([]openapiclient.OutcomeInput{*openapiclient.NewOutcomeInput("Email_example", "EventType_example")}, "Provider_example") // OutcomeIngestRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.OutcomesAPI.V1IngestOutcomes(context.Background()).OutcomeIngestRequest(outcomeIngestRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OutcomesAPI.V1IngestOutcomes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1IngestOutcomes`: OutcomeIngestResponse
	fmt.Fprintf(os.Stdout, "Response from `OutcomesAPI.V1IngestOutcomes`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1IngestOutcomesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **outcomeIngestRequest** | [**OutcomeIngestRequest**](OutcomeIngestRequest.md) |  |

### Return type

[**OutcomeIngestResponse**](OutcomeIngestResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1IngestProviderOutcomes

> InboundOutcomeResponse V1IngestProviderOutcomes(ctx, provider, endpointId, deliveryToken).Body(body).Execute()



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
	provider := "provider_example" // string | sendgrid, ses, mailgun, or postmark
	endpointId := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Provider endpoint identifier
	deliveryToken := "deliveryToken_example" // string | Secret endpoint delivery token
	body := interface{}(987) // interface{} |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.OutcomesAPI.V1IngestProviderOutcomes(context.Background(), provider, endpointId, deliveryToken).Body(body).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OutcomesAPI.V1IngestProviderOutcomes``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1IngestProviderOutcomes`: InboundOutcomeResponse
	fmt.Fprintf(os.Stdout, "Response from `OutcomesAPI.V1IngestProviderOutcomes`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**provider** | **string** | sendgrid, ses, mailgun, or postmark |
**endpointId** | **string** | Provider endpoint identifier |
**deliveryToken** | **string** | Secret endpoint delivery token |

### Other Parameters

Other parameters are passed through a pointer to a apiV1IngestProviderOutcomesRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------



 **body** | **interface{}** |  |

### Return type

[**InboundOutcomeResponse**](InboundOutcomeResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListOutcomes

> OutcomeListResponse V1ListOutcomes(ctx).Limit(limit).Offset(offset).Email(email).EventType(eventType).SourceKey(sourceKey).Since(since).Execute()

GET /v1/outcomes

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
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)
	email := "email_example" // string |  (optional)
	eventType := "eventType_example" // string |  (optional)
	sourceKey := "sourceKey_example" // string |  (optional)
	since := time.Now() // time.Time |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.OutcomesAPI.V1ListOutcomes(context.Background()).Limit(limit).Offset(offset).Email(email).EventType(eventType).SourceKey(sourceKey).Since(since).Execute()
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
 **limit** | **int64** |  |
 **offset** | **int64** |  |
 **email** | **string** |  |
 **eventType** | **string** |  |
 **sourceKey** | **string** |  |
 **since** | **time.Time** |  |

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


## V1ListProviderEndpoints

> ProviderEndpointListResponse V1ListProviderEndpoints(ctx).Execute()



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
	resp, r, err := apiClient.OutcomesAPI.V1ListProviderEndpoints(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OutcomesAPI.V1ListProviderEndpoints``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListProviderEndpoints`: ProviderEndpointListResponse
	fmt.Fprintf(os.Stdout, "Response from `OutcomesAPI.V1ListProviderEndpoints`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiV1ListProviderEndpointsRequest struct via the builder pattern


### Return type

[**ProviderEndpointListResponse**](ProviderEndpointListResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1UpdateProviderEndpoint

> ProviderEndpointView V1UpdateProviderEndpoint(ctx, endpointId).UpdateProviderEndpointInput(updateProviderEndpointInput).Execute()



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
	endpointId := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Provider endpoint identifier
	updateProviderEndpointInput := *openapiclient.NewUpdateProviderEndpointInput() // UpdateProviderEndpointInput |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.OutcomesAPI.V1UpdateProviderEndpoint(context.Background(), endpointId).UpdateProviderEndpointInput(updateProviderEndpointInput).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `OutcomesAPI.V1UpdateProviderEndpoint``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1UpdateProviderEndpoint`: ProviderEndpointView
	fmt.Fprintf(os.Stdout, "Response from `OutcomesAPI.V1UpdateProviderEndpoint`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**endpointId** | **string** | Provider endpoint identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1UpdateProviderEndpointRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **updateProviderEndpointInput** | [**UpdateProviderEndpointInput**](UpdateProviderEndpointInput.md) |  |

### Return type

[**ProviderEndpointView**](ProviderEndpointView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
