# \V1API

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1AddSuppressions**](V1API.md#V1AddSuppressions) | **Post** /v1/suppressions | POST /v1/suppressions
[**V1CheckEmail**](V1API.md#V1CheckEmail) | **Post** /v1/check_email | POST /v1/check_email
[**V1CheckEmailWithOnboard**](V1API.md#V1CheckEmailWithOnboard) | **Post** /v1/check-email-with-onboard | POST /v1/check-email-with-onboard — Operator onboarding + email verification in one call. Requires the backend admin secret: it creates a tenant with the requested plan, generates a non-admin API key, verifies the email, and returns all three. Workspaces sign up through the web app instead.
[**V1CheckReputation**](V1API.md#V1CheckReputation) | **Post** /v1/reputation/check | POST /v1/reputation/check
[**V1CheckSuppression**](V1API.md#V1CheckSuppression) | **Get** /v1/suppressions/check | GET /v1/suppressions/check
[**V1CreateBulkJob**](V1API.md#V1CreateBulkJob) | **Post** /v1/bulk | Create the v1 bulk endpoint.
[**V1CreateList**](V1API.md#V1CreateList) | **Post** /v1/lists | POST /v1/lists
[**V1CreateResearch**](V1API.md#V1CreateResearch) | **Post** /v1/research |
[**V1DeleteList**](V1API.md#V1DeleteList) | **Delete** /v1/lists/{list_id} | DELETE /v1/lists/{list_id}
[**V1DeleteSuppression**](V1API.md#V1DeleteSuppression) | **Delete** /v1/suppressions/{id} | DELETE /v1/suppressions/{id}
[**V1DownloadList**](V1API.md#V1DownloadList) | **Get** /v1/lists/{list_id}/download | GET /v1/lists/{list_id}/download
[**V1ExportResearch**](V1API.md#V1ExportResearch) | **Post** /v1/research/{session_id}/export |
[**V1ExportSuppressions**](V1API.md#V1ExportSuppressions) | **Get** /v1/suppressions/export | Export suppressions
[**V1FindEmail**](V1API.md#V1FindEmail) | **Post** /v1/find_email | POST /v1/find_email
[**V1GetFindEmail**](V1API.md#V1GetFindEmail) | **Get** /v1/find_email/{job_id} | GET /v1/find_email/{job_id}
[**V1GetList**](V1API.md#V1GetList) | **Get** /v1/lists/{list_id} | GET /v1/lists/{list_id}
[**V1GetResearch**](V1API.md#V1GetResearch) | **Get** /v1/research/{session_id} |
[**V1GetVerificationOperation**](V1API.md#V1GetVerificationOperation) | **Get** /v1/operations/{operation_id} |
[**V1ImportSuppressions**](V1API.md#V1ImportSuppressions) | **Post** /v1/suppressions/import | Import suppressions
[**V1ListLists**](V1API.md#V1ListLists) | **Get** /v1/lists | GET /v1/lists
[**V1ListResearch**](V1API.md#V1ListResearch) | **Get** /v1/research |
[**V1ListSuppressionEvents**](V1API.md#V1ListSuppressionEvents) | **Get** /v1/suppressions/{id}/events | List suppression events
[**V1ListSuppressions**](V1API.md#V1ListSuppressions) | **Get** /v1/suppressions | GET /v1/suppressions
[**V1RestoreSuppression**](V1API.md#V1RestoreSuppression) | **Post** /v1/suppressions/{id}/restore |
[**V1ReverificationStatus**](V1API.md#V1ReverificationStatus) | **Get** /v1/reverification/status | GET /v1/reverification/status
[**V1SourceQuality**](V1API.md#V1SourceQuality) | **Get** /v1/sources/quality | List source quality
[**V1UpdateReverification**](V1API.md#V1UpdateReverification) | **Put** /v1/reverification | PUT /v1/reverification



## V1AddSuppressions

> AddSuppressionsResponse V1AddSuppressions(ctx).AddSuppressionsRequest(addSuppressionsRequest).Execute()

POST /v1/suppressions

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
	addSuppressionsRequest := *openapiclient.NewAddSuppressionsRequest([]string{"Emails_example"}) // AddSuppressionsRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1AddSuppressions(context.Background()).AddSuppressionsRequest(addSuppressionsRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1AddSuppressions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1AddSuppressions`: AddSuppressionsResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1AddSuppressions`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1AddSuppressionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addSuppressionsRequest** | [**AddSuppressionsRequest**](AddSuppressionsRequest.md) |  |

### Return type

[**AddSuppressionsResponse**](AddSuppressionsResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1CheckEmail

> VerificationResponse V1CheckEmail(ctx).CheckEmailRequest(checkEmailRequest).IdempotencyKey(idempotencyKey).Execute()

POST /v1/check_email



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
	resp, r, err := apiClient.V1API.V1CheckEmail(context.Background()).CheckEmailRequest(checkEmailRequest).IdempotencyKey(idempotencyKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1CheckEmail``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1CheckEmail`: VerificationResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1CheckEmail`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CheckEmailRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **checkEmailRequest** | [**CheckEmailRequest**](CheckEmailRequest.md) |  |
 **idempotencyKey** | **string** | Optional idempotency key |

### Return type

[**VerificationResponse**](VerificationResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1CheckEmailWithOnboard

> V1CheckEmailWithOnboard(ctx).OnboardRequest(onboardRequest).Execute()

POST /v1/check-email-with-onboard — Operator onboarding + email verification in one call. Requires the backend admin secret: it creates a tenant with the requested plan, generates a non-admin API key, verifies the email, and returns all three. Workspaces sign up through the web app instead.

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
	onboardRequest := *openapiclient.NewOnboardRequest("ContactEmail_example", "EmailToVerify_example", "TenantName_example") // OnboardRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.V1API.V1CheckEmailWithOnboard(context.Background()).OnboardRequest(onboardRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1CheckEmailWithOnboard``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CheckEmailWithOnboardRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **onboardRequest** | [**OnboardRequest**](OnboardRequest.md) |  |

### Return type

 (empty response body)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1CheckReputation

> ReputationCheckResponse V1CheckReputation(ctx).ReputationCheckRequest(reputationCheckRequest).Execute()

POST /v1/reputation/check

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
	reputationCheckRequest := *openapiclient.NewReputationCheckRequest("Domain_example") // ReputationCheckRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1CheckReputation(context.Background()).ReputationCheckRequest(reputationCheckRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1CheckReputation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1CheckReputation`: ReputationCheckResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1CheckReputation`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CheckReputationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **reputationCheckRequest** | [**ReputationCheckRequest**](ReputationCheckRequest.md) |  |

### Return type

[**ReputationCheckResponse**](ReputationCheckResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1CheckSuppression

> SuppressionCheckResponse V1CheckSuppression(ctx).Email(email).Execute()

GET /v1/suppressions/check

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
	email := "email_example" // string |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1CheckSuppression(context.Background()).Email(email).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1CheckSuppression``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1CheckSuppression`: SuppressionCheckResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1CheckSuppression`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CheckSuppressionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **email** | **string** |  |

### Return type

[**SuppressionCheckResponse**](SuppressionCheckResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1CreateBulkJob

> BulkCreateResponse V1CreateBulkJob(ctx).BulkCreateRequest(bulkCreateRequest).IdempotencyKey(idempotencyKey).Execute()

Create the v1 bulk endpoint.



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
	bulkCreateRequest := *openapiclient.NewBulkCreateRequest([]string{"Input_example"}) // BulkCreateRequest |
	idempotencyKey := "idempotencyKey_example" // string | Optional idempotency key (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1CreateBulkJob(context.Background()).BulkCreateRequest(bulkCreateRequest).IdempotencyKey(idempotencyKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1CreateBulkJob``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1CreateBulkJob`: BulkCreateResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1CreateBulkJob`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CreateBulkJobRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **bulkCreateRequest** | [**BulkCreateRequest**](BulkCreateRequest.md) |  |
 **idempotencyKey** | **string** | Optional idempotency key |

### Return type

[**BulkCreateResponse**](BulkCreateResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1CreateList

> ListUploadResponse V1CreateList(ctx).File(file).IdempotencyKey(idempotencyKey).EmailColumn(emailColumn).Name(name).PolicyId(policyId).SourceKey(sourceKey).Execute()

POST /v1/lists

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
	file := os.NewFile(1234, "some_file") // *os.File |
	idempotencyKey := "idempotencyKey_example" // string | Reuse for retries of the same upload to prevent duplicate jobs and charges (optional)
	emailColumn := "emailColumn_example" // string |  (optional)
	name := "name_example" // string |  (optional)
	policyId := int64(789) // int64 |  (optional)
	sourceKey := "sourceKey_example" // string | Optional source key used for source quality analytics, for example apollo, hubspot, salesforce, signup_form, csv_vendor. (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1CreateList(context.Background()).File(file).IdempotencyKey(idempotencyKey).EmailColumn(emailColumn).Name(name).PolicyId(policyId).SourceKey(sourceKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1CreateList``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1CreateList`: ListUploadResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1CreateList`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CreateListRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **file** | ***os.File** |  |
 **idempotencyKey** | **string** | Reuse for retries of the same upload to prevent duplicate jobs and charges |
 **emailColumn** | **string** |  |
 **name** | **string** |  |
 **policyId** | **int64** |  |
 **sourceKey** | **string** | Optional source key used for source quality analytics, for example apollo, hubspot, salesforce, signup_form, csv_vendor. |

### Return type

[**ListUploadResponse**](ListUploadResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: multipart/form-data
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1CreateResearch

> ResearchCreated V1CreateResearch(ctx).CreateResearchRequest(createResearchRequest).Execute()



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
	createResearchRequest := *openapiclient.NewCreateResearchRequest("IdempotencyKey_example", "ImportContent_example", openapiclient.ResearchImportFormat("csv"), int32(123), int32(123), "Name_example", "TargetRole_example") // CreateResearchRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1CreateResearch(context.Background()).CreateResearchRequest(createResearchRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1CreateResearch``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1CreateResearch`: ResearchCreated
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1CreateResearch`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CreateResearchRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **createResearchRequest** | [**CreateResearchRequest**](CreateResearchRequest.md) |  |

### Return type

[**ResearchCreated**](ResearchCreated.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1DeleteList

> ListDeleteResponse V1DeleteList(ctx, listId).Execute()

DELETE /v1/lists/{list_id}

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
	listId := int32(56) // int32 | List identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1DeleteList(context.Background(), listId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1DeleteList``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1DeleteList`: ListDeleteResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1DeleteList`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**listId** | **int32** | List identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1DeleteListRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ListDeleteResponse**](ListDeleteResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1DeleteSuppression

> SuppressionDeleteResponse V1DeleteSuppression(ctx, id).Execute()

DELETE /v1/suppressions/{id}

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
	id := int32(56) // int32 | Suppression entry identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1DeleteSuppression(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1DeleteSuppression``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1DeleteSuppression`: SuppressionDeleteResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1DeleteSuppression`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **int32** | Suppression entry identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1DeleteSuppressionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**SuppressionDeleteResponse**](SuppressionDeleteResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1DownloadList

> *os.File V1DownloadList(ctx, listId).Filter(filter).Format(format).SegmentId(segmentId).ChangedSinceListId(changedSinceListId).Execute()

GET /v1/lists/{list_id}/download

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
	listId := int32(56) // int32 | List identifier
	filter := "filter_example" // string |  (optional)
	format := "format_example" // string |  (optional)
	segmentId := int64(789) // int64 |  (optional)
	changedSinceListId := int32(56) // int32 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1DownloadList(context.Background(), listId).Filter(filter).Format(format).SegmentId(segmentId).ChangedSinceListId(changedSinceListId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1DownloadList``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1DownloadList`: *os.File
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1DownloadList`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**listId** | **int32** | List identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1DownloadListRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **filter** | **string** |  |
 **format** | **string** |  |
 **segmentId** | **int64** |  |
 **changedSinceListId** | **int32** |  |

### Return type

[***os.File**](*os.File.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ExportResearch

> ResearchExportResponse V1ExportResearch(ctx, sessionId).ResearchExportRequest(researchExportRequest).Execute()



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
	sessionId := "sessionId_example" // string | Research session ID as a positive decimal string
	researchExportRequest := *openapiclient.NewResearchExportRequest(openapiclient.ResearchExportMode("approved"), []string{"RowKeys_example"}) // ResearchExportRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1ExportResearch(context.Background(), sessionId).ResearchExportRequest(researchExportRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1ExportResearch``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ExportResearch`: ResearchExportResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1ExportResearch`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**sessionId** | **string** | Research session ID as a positive decimal string |

### Other Parameters

Other parameters are passed through a pointer to a apiV1ExportResearchRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **researchExportRequest** | [**ResearchExportRequest**](ResearchExportRequest.md) |  |

### Return type

[**ResearchExportResponse**](ResearchExportResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ExportSuppressions

> *os.File V1ExportSuppressions(ctx).Format(format).Reason(reason).Status(status).SourceType(sourceType).SourceRef(sourceRef).IncludeExpired(includeExpired).Execute()

Export suppressions

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
	format := "format_example" // string |  (optional)
	reason := openapiclient.SuppressionReason("manual") // SuppressionReason |  (optional)
	status := "status_example" // string |  (optional)
	sourceType := "sourceType_example" // string |  (optional)
	sourceRef := "sourceRef_example" // string |  (optional)
	includeExpired := true // bool |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1ExportSuppressions(context.Background()).Format(format).Reason(reason).Status(status).SourceType(sourceType).SourceRef(sourceRef).IncludeExpired(includeExpired).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1ExportSuppressions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ExportSuppressions`: *os.File
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1ExportSuppressions`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1ExportSuppressionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **format** | **string** |  |
 **reason** | [**SuppressionReason**](SuppressionReason.md) |  |
 **status** | **string** |  |
 **sourceType** | **string** |  |
 **sourceRef** | **string** |  |
 **includeExpired** | **bool** |  |

### Return type

[***os.File**](*os.File.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv, application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1FindEmail

> FindEmailAcceptedResponse V1FindEmail(ctx).FindEmailRequest(findEmailRequest).Execute()

POST /v1/find_email

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
	findEmailRequest := *openapiclient.NewFindEmailRequest("Domain_example", "FirstName_example", "LastName_example") // FindEmailRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1FindEmail(context.Background()).FindEmailRequest(findEmailRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1FindEmail``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1FindEmail`: FindEmailAcceptedResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1FindEmail`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1FindEmailRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **findEmailRequest** | [**FindEmailRequest**](FindEmailRequest.md) |  |

### Return type

[**FindEmailAcceptedResponse**](FindEmailAcceptedResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetFindEmail

> FindEmailStatusResponse V1GetFindEmail(ctx, jobId).Execute()

GET /v1/find_email/{job_id}

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
	jobId := int32(56) // int32 | Finder job identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1GetFindEmail(context.Background(), jobId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1GetFindEmail``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1GetFindEmail`: FindEmailStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1GetFindEmail`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**jobId** | **int32** | Finder job identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetFindEmailRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**FindEmailStatusResponse**](FindEmailStatusResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetList

> ListDetailResponse V1GetList(ctx, listId).Execute()

GET /v1/lists/{list_id}

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
	listId := int32(56) // int32 | List identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1GetList(context.Background(), listId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1GetList``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1GetList`: ListDetailResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1GetList`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**listId** | **int32** | List identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetListRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ListDetailResponse**](ListDetailResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetResearch

> ResearchDetail V1GetResearch(ctx, sessionId).Execute()



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
	sessionId := "sessionId_example" // string | Research session ID as a positive decimal string

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1GetResearch(context.Background(), sessionId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1GetResearch``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1GetResearch`: ResearchDetail
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1GetResearch`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**sessionId** | **string** | Research session ID as a positive decimal string |

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetResearchRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**ResearchDetail**](ResearchDetail.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1GetVerificationOperation

> VerificationResponse V1GetVerificationOperation(ctx, operationId).Execute()



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
	operationId := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Verification operation identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1GetVerificationOperation(context.Background(), operationId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1GetVerificationOperation``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1GetVerificationOperation`: VerificationResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1GetVerificationOperation`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**operationId** | **string** | Verification operation identifier |

### Other Parameters

Other parameters are passed through a pointer to a apiV1GetVerificationOperationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

[**VerificationResponse**](VerificationResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ImportSuppressions

> AddSuppressionsResponse V1ImportSuppressions(ctx).AddSuppressionsRequest(addSuppressionsRequest).Execute()

Import suppressions

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
	addSuppressionsRequest := *openapiclient.NewAddSuppressionsRequest([]string{"Emails_example"}) // AddSuppressionsRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1ImportSuppressions(context.Background()).AddSuppressionsRequest(addSuppressionsRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1ImportSuppressions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ImportSuppressions`: AddSuppressionsResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1ImportSuppressions`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1ImportSuppressionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **addSuppressionsRequest** | [**AddSuppressionsRequest**](AddSuppressionsRequest.md) |  |

### Return type

[**AddSuppressionsResponse**](AddSuppressionsResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListLists

> ListListResponse V1ListLists(ctx).Limit(limit).Offset(offset).Execute()

GET /v1/lists

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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1ListLists(context.Background()).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1ListLists``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListLists`: ListListResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1ListLists`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1ListListsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int64** |  |
 **offset** | **int64** |  |

### Return type

[**ListListResponse**](ListListResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListResearch

> ResearchList V1ListResearch(ctx).Execute()



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
	resp, r, err := apiClient.V1API.V1ListResearch(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1ListResearch``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListResearch`: ResearchList
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1ListResearch`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiV1ListResearchRequest struct via the builder pattern


### Return type

[**ResearchList**](ResearchList.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListSuppressionEvents

> SuppressionEventListResponse V1ListSuppressionEvents(ctx, id).Limit(limit).Offset(offset).Execute()

List suppression events

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
	id := int32(56) // int32 |
	limit := int64(789) // int64 |  (optional)
	offset := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1ListSuppressionEvents(context.Background(), id).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1ListSuppressionEvents``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListSuppressionEvents`: SuppressionEventListResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1ListSuppressionEvents`: %v\n", resp)
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **int32** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiV1ListSuppressionEventsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------

 **limit** | **int64** |  |
 **offset** | **int64** |  |

### Return type

[**SuppressionEventListResponse**](SuppressionEventListResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ListSuppressions

> SuppressionListResponse V1ListSuppressions(ctx).Limit(limit).Offset(offset).Reason(reason).Status(status).SourceType(sourceType).SourceRef(sourceRef).IncludeExpired(includeExpired).Execute()

GET /v1/suppressions

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
	reason := "reason_example" // string |  (optional)
	status := "status_example" // string |  (optional)
	sourceType := "sourceType_example" // string |  (optional)
	sourceRef := "sourceRef_example" // string |  (optional)
	includeExpired := true // bool |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1ListSuppressions(context.Background()).Limit(limit).Offset(offset).Reason(reason).Status(status).SourceType(sourceType).SourceRef(sourceRef).IncludeExpired(includeExpired).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1ListSuppressions``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ListSuppressions`: SuppressionListResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1ListSuppressions`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1ListSuppressionsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **limit** | **int64** |  |
 **offset** | **int64** |  |
 **reason** | **string** |  |
 **status** | **string** |  |
 **sourceType** | **string** |  |
 **sourceRef** | **string** |  |
 **includeExpired** | **bool** |  |

### Return type

[**SuppressionListResponse**](SuppressionListResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1RestoreSuppression

> V1RestoreSuppression(ctx, id).Execute()



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
	id := int32(56) // int32 |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.V1API.V1RestoreSuppression(context.Background(), id).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1RestoreSuppression``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**id** | **int32** |  |

### Other Parameters

Other parameters are passed through a pointer to a apiV1RestoreSuppressionRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------


### Return type

 (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1ReverificationStatus

> ReverificationStatusResponse V1ReverificationStatus(ctx).Execute()

GET /v1/reverification/status

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
	resp, r, err := apiClient.V1API.V1ReverificationStatus(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1ReverificationStatus``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1ReverificationStatus`: ReverificationStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1ReverificationStatus`: %v\n", resp)
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiV1ReverificationStatusRequest struct via the builder pattern


### Return type

[**ReverificationStatusResponse**](ReverificationStatusResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1SourceQuality

> SourceQualityResponse V1SourceQuality(ctx).SourceKey(sourceKey).MinRecords(minRecords).Limit(limit).Execute()

List source quality

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
	sourceKey := "sourceKey_example" // string |  (optional)
	minRecords := int64(789) // int64 |  (optional)
	limit := int64(789) // int64 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1SourceQuality(context.Background()).SourceKey(sourceKey).MinRecords(minRecords).Limit(limit).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1SourceQuality``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1SourceQuality`: SourceQualityResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1SourceQuality`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1SourceQualityRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **sourceKey** | **string** |  |
 **minRecords** | **int64** |  |
 **limit** | **int64** |  |

### Return type

[**SourceQualityResponse**](SourceQualityResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## V1UpdateReverification

> ReverificationStatusResponse V1UpdateReverification(ctx).UpdateReverificationRequest(updateReverificationRequest).Execute()

PUT /v1/reverification

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
	updateReverificationRequest := *openapiclient.NewUpdateReverificationRequest(false, int32(123)) // UpdateReverificationRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1UpdateReverification(context.Background()).UpdateReverificationRequest(updateReverificationRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1UpdateReverification``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `V1UpdateReverification`: ReverificationStatusResponse
	fmt.Fprintf(os.Stdout, "Response from `V1API.V1UpdateReverification`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1UpdateReverificationRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **updateReverificationRequest** | [**UpdateReverificationRequest**](UpdateReverificationRequest.md) |  |

### Return type

[**ReverificationStatusResponse**](ReverificationStatusResponse.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
