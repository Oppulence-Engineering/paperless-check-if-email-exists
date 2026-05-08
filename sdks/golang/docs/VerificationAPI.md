# \VerificationAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1EmailHistory**](VerificationAPI.md#V1EmailHistory) | **Get** /v1/emails/{email}/history | GET /v1/emails/{email}/history
[**V1ListAlerts**](VerificationAPI.md#V1ListAlerts) | **Get** /v1/alerts | GET /v1/alerts
[**V1UpdateAlert**](VerificationAPI.md#V1UpdateAlert) | **Patch** /v1/alerts/{alert_id} | PATCH /v1/alerts/{alert_id}



## V1EmailHistory

> V1EmailHistory(ctx, email).Limit(limit).Execute()

GET /v1/emails/{email}/history



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
	email := "email_example" // string | Email address to look up
	limit := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.VerificationAPI.V1EmailHistory(context.Background(), email).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `VerificationAPI.V1EmailHistory``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**email** | **string** | Email address to look up |

### Other Parameters

Other parameters are passed through a pointer to a apiV1EmailHistoryRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **limit** | **int64** |  |

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


## V1ListAlerts

> AlertListResponse V1ListAlerts(ctx).Status(status).Type_(type_).Limit(limit).Offset(offset).Execute()

GET /v1/alerts

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
	type_ := "type__example" // string |  (optional)
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.VerificationAPI.V1ListAlerts(context.Background()).Status(status).Type_(type_).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `VerificationAPI.V1ListAlerts``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListAlerts`: AlertListResponse
	fmt.Fprintf(os.Stdout, "Response from `VerificationAPI.V1ListAlerts`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1ListAlertsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **status** | **string** |  |
 **type_** | **string** |  |
 **limit** | **int64** |  |
 **offset** | **int64** |  |

### Return type

[**AlertListResponse**](AlertListResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1UpdateAlert

> AlertView V1UpdateAlert(ctx, alertId).UpdateAlertRequest(updateAlertRequest).Execute()

PATCH /v1/alerts/{alert_id}

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
	alertId := int64(789) // int64 | Alert identifier
	updateAlertRequest := *openapiclient.NewUpdateAlertRequest("Status_example") // UpdateAlertRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.VerificationAPI.V1UpdateAlert(context.Background(), alertId).UpdateAlertRequest(updateAlertRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `VerificationAPI.V1UpdateAlert``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1UpdateAlert`: AlertView
	fmt.Fprintf(os.Stdout, "Response from `VerificationAPI.V1UpdateAlert`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**alertId** | **int64** | Alert identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1UpdateAlertRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **updateAlertRequest** | [**UpdateAlertRequest**](UpdateAlertRequest.md) |  |

### Return type

[**AlertView**](AlertView.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
