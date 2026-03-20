# \V1API

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**V1AddSuppressions**](V1API.md#V1AddSuppressions) | **Post** /v1/suppressions | POST /v1/suppressions
[**V1CheckEmail**](V1API.md#V1CheckEmail) | **Post** /v1/check_email | POST /v1/check_email
[**V1CheckReputation**](V1API.md#V1CheckReputation) | **Post** /v1/reputation/check | POST /v1/reputation/check
[**V1CheckSuppression**](V1API.md#V1CheckSuppression) | **Get** /v1/suppressions/check | GET /v1/suppressions/check
[**V1CreateBulkJob**](V1API.md#V1CreateBulkJob) | **Post** /v1/bulk | Create the v1 bulk endpoint.
[**V1CreateList**](V1API.md#V1CreateList) | **Post** /v1/lists | POST /v1/lists
[**V1DeleteList**](V1API.md#V1DeleteList) | **Delete** /v1/lists/{list_id} | DELETE /v1/lists/{list_id}
[**V1DeleteSuppression**](V1API.md#V1DeleteSuppression) | **Delete** /v1/suppressions/{id} | DELETE /v1/suppressions/{id}
[**V1DownloadList**](V1API.md#V1DownloadList) | **Get** /v1/lists/{list_id}/download | GET /v1/lists/{list_id}/download
[**V1FindEmail**](V1API.md#V1FindEmail) | **Post** /v1/find_email | POST /v1/find_email
[**V1GetFindEmail**](V1API.md#V1GetFindEmail) | **Get** /v1/find_email/{job_id} | GET /v1/find_email/{job_id}
[**V1GetList**](V1API.md#V1GetList) | **Get** /v1/lists/{list_id} | GET /v1/lists/{list_id}
[**V1ListLists**](V1API.md#V1ListLists) | **Get** /v1/lists | GET /v1/lists
[**V1ListSuppressions**](V1API.md#V1ListSuppressions) | **Get** /v1/suppressions | GET /v1/suppressions



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

> CheckEmailOutput V1CheckEmail(ctx).CheckEmailRequest(checkEmailRequest).IdempotencyKey(idempotencyKey).Execute()

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
	// response from `V1CheckEmail`: CheckEmailOutput
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

[**CheckEmailOutput**](CheckEmailOutput.md)

### Authorization

[Authorization](../README.md#Authorization)

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

> V1CreateBulkJob(ctx).IdempotencyKey(idempotencyKey).Execute()

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
	idempotencyKey := "idempotencyKey_example" // string | Optional idempotency key (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.V1API.V1CreateBulkJob(context.Background()).IdempotencyKey(idempotencyKey).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `V1API.V1CreateBulkJob``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiV1CreateBulkJobRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **idempotencyKey** | **string** | Optional idempotency key | 

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


## V1CreateList

> ListUploadResponse V1CreateList(ctx).File(file).EmailColumn(emailColumn).Name(name).Execute()

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
	emailColumn := "emailColumn_example" // string |  (optional)
	name := "name_example" // string |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1CreateList(context.Background()).File(file).EmailColumn(emailColumn).Name(name).Execute()
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
 **emailColumn** | **string** |  | 
 **name** | **string** |  | 

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

> *os.File V1DownloadList(ctx, listId).Filter(filter).Format(format).Execute()

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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1DownloadList(context.Background(), listId).Filter(filter).Format(format).Execute()
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

### Return type

[***os.File**](*os.File.md)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: text/csv

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


## V1ListSuppressions

> SuppressionListResponse V1ListSuppressions(ctx).Limit(limit).Offset(offset).Reason(reason).Execute()

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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.V1API.V1ListSuppressions(context.Background()).Limit(limit).Offset(offset).Reason(reason).Execute()
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

