# \AdminAPI

All URIs are relative to *https://api.reacher.email*

Method | HTTP request | Description
------------- | ------------- | -------------
[**CreateApiKey**](AdminAPI.md#CreateApiKey) | **Post** /v1/admin/tenants/{tenant_id}/api-keys | POST /v1/admin/tenants/{tenant_id}/api-keys
[**CreateTenant**](AdminAPI.md#CreateTenant) | **Post** /v1/admin/tenants | POST /v1/admin/tenants
[**DeleteTenant**](AdminAPI.md#DeleteTenant) | **Delete** /v1/admin/tenants/{tenant_id} | DELETE /v1/admin/tenants/{tenant_id}
[**GetApiKey**](AdminAPI.md#GetApiKey) | **Get** /v1/admin/tenants/{tenant_id}/api-keys/{key_id} | GET /v1/admin/tenants/{tenant_id}/api-keys/{key_id}
[**GetTenant**](AdminAPI.md#GetTenant) | **Get** /v1/admin/tenants/{tenant_id} | GET /v1/admin/tenants/{tenant_id}
[**GetTenantQuota**](AdminAPI.md#GetTenantQuota) | **Get** /v1/admin/tenants/{tenant_id}/quota | GET /v1/admin/tenants/{tenant_id}/quota
[**ListAllApiKeys**](AdminAPI.md#ListAllApiKeys) | **Get** /v1/admin/api-keys | GET /v1/admin/api-keys
[**ListApiKeys**](AdminAPI.md#ListApiKeys) | **Get** /v1/admin/tenants/{tenant_id}/api-keys | GET /v1/admin/tenants/{tenant_id}/api-keys
[**ListTenants**](AdminAPI.md#ListTenants) | **Get** /v1/admin/tenants | GET /v1/admin/tenants
[**ReactivateApiKey**](AdminAPI.md#ReactivateApiKey) | **Post** /v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate | POST /v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate
[**ResetTenantQuota**](AdminAPI.md#ResetTenantQuota) | **Post** /v1/admin/tenants/{tenant_id}/quota/reset | POST /v1/admin/tenants/{tenant_id}/quota/reset
[**RevokeApiKey**](AdminAPI.md#RevokeApiKey) | **Delete** /v1/admin/tenants/{tenant_id}/api-keys/{key_id} | DELETE /v1/admin/tenants/{tenant_id}/api-keys/{key_id}
[**UpdateApiKey**](AdminAPI.md#UpdateApiKey) | **Patch** /v1/admin/tenants/{tenant_id}/api-keys/{key_id} | PATCH /v1/admin/tenants/{tenant_id}/api-keys/{key_id}
[**UpdateTenant**](AdminAPI.md#UpdateTenant) | **Put** /v1/admin/tenants/{tenant_id} | PUT /v1/admin/tenants/{tenant_id}
[**UpdateTenantQuota**](AdminAPI.md#UpdateTenantQuota) | **Patch** /v1/admin/tenants/{tenant_id}/quota | PATCH /v1/admin/tenants/{tenant_id}/quota



## CreateApiKey

> CreateApiKey(ctx, tenantId).Execute()

POST /v1/admin/tenants/{tenant_id}/api-keys



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
	tenantId := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | Tenant identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.CreateApiKey(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.CreateApiKey``: %v\n", err)
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

Other parameters are passed through a pointer to a apiCreateApiKeyRequest struct via the builder pattern


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


## CreateTenant

> CreateTenant(ctx).Execute()

POST /v1/admin/tenants



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
	r, err := apiClient.AdminAPI.CreateTenant(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.CreateTenant``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiCreateTenantRequest struct via the builder pattern


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


## DeleteTenant

> DeleteTenant(ctx, tenantId).Execute()

DELETE /v1/admin/tenants/{tenant_id}



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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.DeleteTenant(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.DeleteTenant``: %v\n", err)
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

Other parameters are passed through a pointer to a apiDeleteTenantRequest struct via the builder pattern


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


## GetApiKey

> GetApiKey(ctx, tenantId, keyId).Execute()

GET /v1/admin/tenants/{tenant_id}/api-keys/{key_id}



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
	keyId := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | API key identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.GetApiKey(context.Background(), tenantId, keyId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.GetApiKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | Tenant identifier | 
**keyId** | **string** | API key identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiGetApiKeyRequest struct via the builder pattern


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


## GetTenant

> GetTenant(ctx, tenantId).Execute()

GET /v1/admin/tenants/{tenant_id}



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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.GetTenant(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.GetTenant``: %v\n", err)
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

Other parameters are passed through a pointer to a apiGetTenantRequest struct via the builder pattern


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


## GetTenantQuota

> GetTenantQuota(ctx, tenantId).Execute()

GET /v1/admin/tenants/{tenant_id}/quota



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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.GetTenantQuota(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.GetTenantQuota``: %v\n", err)
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

Other parameters are passed through a pointer to a apiGetTenantQuotaRequest struct via the builder pattern


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


## ListAllApiKeys

> ListAllApiKeys(ctx).Execute()

GET /v1/admin/api-keys



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
	r, err := apiClient.AdminAPI.ListAllApiKeys(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.ListAllApiKeys``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiListAllApiKeysRequest struct via the builder pattern


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


## ListApiKeys

> ListApiKeys(ctx, tenantId).Execute()

GET /v1/admin/tenants/{tenant_id}/api-keys



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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.ListApiKeys(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.ListApiKeys``: %v\n", err)
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

Other parameters are passed through a pointer to a apiListApiKeysRequest struct via the builder pattern


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


## ListTenants

> ListTenants(ctx).Execute()

GET /v1/admin/tenants



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
	r, err := apiClient.AdminAPI.ListTenants(context.Background()).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.ListTenants``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters

This endpoint does not need any parameter.

### Other Parameters

Other parameters are passed through a pointer to a apiListTenantsRequest struct via the builder pattern


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


## ReactivateApiKey

> ReactivateApiKey(ctx, tenantId, keyId).Execute()

POST /v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate



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
	keyId := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | API key identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.ReactivateApiKey(context.Background(), tenantId, keyId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.ReactivateApiKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | Tenant identifier | 
**keyId** | **string** | API key identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiReactivateApiKeyRequest struct via the builder pattern


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


## ResetTenantQuota

> ResetTenantQuota(ctx, tenantId).Execute()

POST /v1/admin/tenants/{tenant_id}/quota/reset



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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.ResetTenantQuota(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.ResetTenantQuota``: %v\n", err)
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

Other parameters are passed through a pointer to a apiResetTenantQuotaRequest struct via the builder pattern


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


## RevokeApiKey

> RevokeApiKey(ctx, tenantId, keyId).Execute()

DELETE /v1/admin/tenants/{tenant_id}/api-keys/{key_id}



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
	keyId := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | API key identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.RevokeApiKey(context.Background(), tenantId, keyId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.RevokeApiKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | Tenant identifier | 
**keyId** | **string** | API key identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiRevokeApiKeyRequest struct via the builder pattern


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


## UpdateApiKey

> UpdateApiKey(ctx, tenantId, keyId).Execute()

PATCH /v1/admin/tenants/{tenant_id}/api-keys/{key_id}



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
	keyId := "38400000-8cf0-11bd-b23e-10b96e4ef00d" // string | API key identifier

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.UpdateApiKey(context.Background(), tenantId, keyId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.UpdateApiKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
}
```

### Path Parameters


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
**ctx** | **context.Context** | context for authentication, logging, cancellation, deadlines, tracing, etc.
**tenantId** | **string** | Tenant identifier | 
**keyId** | **string** | API key identifier | 

### Other Parameters

Other parameters are passed through a pointer to a apiUpdateApiKeyRequest struct via the builder pattern


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


## UpdateTenant

> UpdateTenant(ctx, tenantId).Execute()

PUT /v1/admin/tenants/{tenant_id}



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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.UpdateTenant(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.UpdateTenant``: %v\n", err)
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

Other parameters are passed through a pointer to a apiUpdateTenantRequest struct via the builder pattern


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


## UpdateTenantQuota

> UpdateTenantQuota(ctx, tenantId).Execute()

PATCH /v1/admin/tenants/{tenant_id}/quota



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

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	r, err := apiClient.AdminAPI.UpdateTenantQuota(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.UpdateTenantQuota``: %v\n", err)
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

Other parameters are passed through a pointer to a apiUpdateTenantQuotaRequest struct via the builder pattern


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

