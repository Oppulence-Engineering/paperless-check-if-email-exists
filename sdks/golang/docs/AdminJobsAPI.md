# \AdminJobsAPI

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**GetJob**](AdminJobsAPI.md#GetJob) | **Get** /v1/admin/jobs/{job_id} | GET /v1/admin/jobs/{job_id}
[**GetJobEvents**](AdminJobsAPI.md#GetJobEvents) | **Get** /v1/admin/jobs/{job_id}/events | GET /v1/admin/jobs/{job_id}/events
[**GetJobResults**](AdminJobsAPI.md#GetJobResults) | **Get** /v1/admin/jobs/{job_id}/results | GET /v1/admin/jobs/{job_id}/results
[**ListJobs**](AdminJobsAPI.md#ListJobs) | **Get** /v1/admin/jobs | GET /v1/admin/jobs
[**ListTenantJobs**](AdminJobsAPI.md#ListTenantJobs) | **Get** /v1/admin/tenants/{tenant_id}/jobs | GET /v1/admin/tenants/{tenant_id}/jobs



## GetJob

> GetJob(ctx, jobId).Execute()

GET /v1/admin/jobs/{job_id}



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
	jobId := int32(56) // int32 | Job identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminJobsAPI.GetJob(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminJobsAPI.GetJob``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Job identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiGetJobRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetJobEvents

> GetJobEvents(ctx, jobId).Limit(limit).Offset(offset).Execute()

GET /v1/admin/jobs/{job_id}/events



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
	jobId := int32(56) // int32 | Job identifier
	limit := int32(56) // int32 |  (optional)
	offset := int32(56) // int32 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminJobsAPI.GetJobEvents(context.Background(), jobId).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminJobsAPI.GetJobEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Job identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiGetJobEventsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **limit** | **int32** |  |
 **offset** | **int32** |  |

### Return type

 (empty response body)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetJobResults

> GetJobResults(ctx, jobId).Limit(limit).Offset(offset).State(state).Execute()

GET /v1/admin/jobs/{job_id}/results



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
	jobId := int32(56) // int32 | Job identifier
	limit := int32(56) // int32 |  (optional)
	offset := int32(56) // int32 |  (optional)
	state := "state_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminJobsAPI.GetJobResults(context.Background(), jobId).Limit(limit).Offset(offset).State(state).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminJobsAPI.GetJobResults``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Job identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiGetJobResultsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **limit** | **int32** |  |
 **offset** | **int32** |  |
 **state** | **string** |  |

### Return type

 (empty response body)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListJobs

> ListJobs(ctx).Status(status).TenantId(tenantId).Limit(limit).Offset(offset).Execute()

GET /v1/admin/jobs



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
	tenantId := "tenantId_example" // string |  (optional)
	limit := int32(56) // int32 |  (optional)
	offset := int32(56) // int32 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminJobsAPI.ListJobs(context.Background()).Status(status).TenantId(tenantId).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminJobsAPI.ListJobs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListJobsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **status** | **string** |  |
 **tenantId** | **string** |  |
 **limit** | **int32** |  |
 **offset** | **int32** |  |

### Return type

 (empty response body)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListTenantJobs

> ListTenantJobs(ctx, tenantId).Status(status).Limit(limit).Offset(offset).Execute()

GET /v1/admin/tenants/{tenant_id}/jobs



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
	tenantId := "tenantId_example" // string | Tenant identifier
	status := "status_example" // string |  (optional)
	limit := int32(56) // int32 |  (optional)
	offset := int32(56) // int32 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminJobsAPI.ListTenantJobs(context.Background(), tenantId).Status(status).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminJobsAPI.ListTenantJobs``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | Tenant identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiListTenantJobsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **status** | **string** |  |
 **limit** | **int32** |  |
 **offset** | **int32** |  |

### Return type

 (empty response body)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
