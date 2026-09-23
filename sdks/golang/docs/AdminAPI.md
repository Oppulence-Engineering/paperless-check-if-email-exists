# \AdminAPI

All URIs are relative to *http://localhost*

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

> AdminCreatedApiKey CreateApiKey(ctx, tenantId).AdminApiKeyWriteRequest(adminApiKeyWriteRequest).Execute()

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
	adminApiKeyWriteRequest := *openapiclient.NewAdminApiKeyWriteRequest() // AdminApiKeyWriteRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AdminAPI.CreateApiKey(context.Background(), tenantId).AdminApiKeyWriteRequest(adminApiKeyWriteRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.CreateApiKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateApiKey`: AdminCreatedApiKey
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.CreateApiKey`: %v\n", resp)
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

 **adminApiKeyWriteRequest** | [**AdminApiKeyWriteRequest**](AdminApiKeyWriteRequest.md) |  |

### Return type

[**AdminCreatedApiKey**](AdminCreatedApiKey.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## CreateTenant

> AdminTenant CreateTenant(ctx).AdminCreateTenantRequest(adminCreateTenantRequest).Execute()

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
	adminCreateTenantRequest := *openapiclient.NewAdminCreateTenantRequest("Name_example", "Slug_example", "ContactEmail_example") // AdminCreateTenantRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AdminAPI.CreateTenant(context.Background()).AdminCreateTenantRequest(adminCreateTenantRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.CreateTenant``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `CreateTenant`: AdminTenant
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.CreateTenant`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiCreateTenantRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **adminCreateTenantRequest** | [**AdminCreateTenantRequest**](AdminCreateTenantRequest.md) |  |

### Return type

[**AdminTenant**](AdminTenant.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

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

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetApiKey

> AdminApiKey GetApiKey(ctx, tenantId, keyId).Execute()

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
	resp, r, err := apiClient.AdminAPI.GetApiKey(context.Background(), tenantId, keyId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.GetApiKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetApiKey`: AdminApiKey
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.GetApiKey`: %v\n", resp)
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

[**AdminApiKey**](AdminApiKey.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetTenant

> AdminTenant GetTenant(ctx, tenantId).Execute()

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
	resp, r, err := apiClient.AdminAPI.GetTenant(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.GetTenant``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetTenant`: AdminTenant
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.GetTenant`: %v\n", resp)
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

[**AdminTenant**](AdminTenant.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## GetTenantQuota

> AdminTenantQuota GetTenantQuota(ctx, tenantId).Execute()

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
	resp, r, err := apiClient.AdminAPI.GetTenantQuota(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.GetTenantQuota``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `GetTenantQuota`: AdminTenantQuota
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.GetTenantQuota`: %v\n", resp)
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

[**AdminTenantQuota**](AdminTenantQuota.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListAllApiKeys

> AdminAllApiKeys ListAllApiKeys(ctx).TenantId(tenantId).Status(status).Limit(limit).Offset(offset).Execute()

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
	tenantId := "tenantId_example" // string |  (optional)
	status := "status_example" // string |  (optional)
	limit := int32(56) // int32 |  (optional)
	offset := int32(56) // int32 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AdminAPI.ListAllApiKeys(context.Background()).TenantId(tenantId).Status(status).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.ListAllApiKeys``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListAllApiKeys`: AdminAllApiKeys
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.ListAllApiKeys`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListAllApiKeysRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **tenantId** | **string** |  |
 **status** | **string** |  |
 **limit** | **int32** |  |
 **offset** | **int32** |  |

### Return type

[**AdminAllApiKeys**](AdminAllApiKeys.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListApiKeys

> AdminApiKeyList ListApiKeys(ctx, tenantId).Execute()

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
	resp, r, err := apiClient.AdminAPI.ListApiKeys(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.ListApiKeys``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListApiKeys`: AdminApiKeyList
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.ListApiKeys`: %v\n", resp)
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

[**AdminApiKeyList**](AdminApiKeyList.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ListTenants

> AdminTenantList ListTenants(ctx).Status(status).Limit(limit).Offset(offset).Execute()

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
	status := "status_example" // string |  (optional)
	limit := int32(56) // int32 |  (optional)
	offset := int32(56) // int32 |  (optional)

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AdminAPI.ListTenants(context.Background()).Status(status).Limit(limit).Offset(offset).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.ListTenants``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ListTenants`: AdminTenantList
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.ListTenants`: %v\n", resp)
}
```

### Path Parameters



### Other Parameters

Other parameters are passed through a pointer to a apiListTenantsRequest struct via the builder pattern


Name | Type | Description  | Notes
------------- | ------------- | ------------- | -------------
 **status** | **string** |  |
 **limit** | **int32** |  |
 **offset** | **int32** |  |

### Return type

[**AdminTenantList**](AdminTenantList.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## ResetTenantQuota

> AdminTenantQuota ResetTenantQuota(ctx, tenantId).Execute()

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
	resp, r, err := apiClient.AdminAPI.ResetTenantQuota(context.Background(), tenantId).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.ResetTenantQuota``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `ResetTenantQuota`: AdminTenantQuota
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.ResetTenantQuota`: %v\n", resp)
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

[**AdminTenantQuota**](AdminTenantQuota.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

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

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateApiKey

> AdminApiKey UpdateApiKey(ctx, tenantId, keyId).AdminApiKeyWriteRequest(adminApiKeyWriteRequest).Execute()

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
	adminApiKeyWriteRequest := *openapiclient.NewAdminApiKeyWriteRequest() // AdminApiKeyWriteRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AdminAPI.UpdateApiKey(context.Background(), tenantId, keyId).AdminApiKeyWriteRequest(adminApiKeyWriteRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.UpdateApiKey``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateApiKey`: AdminApiKey
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.UpdateApiKey`: %v\n", resp)
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


 **adminApiKeyWriteRequest** | [**AdminApiKeyWriteRequest**](AdminApiKeyWriteRequest.md) |  |

### Return type

[**AdminApiKey**](AdminApiKey.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateTenant

> AdminTenant UpdateTenant(ctx, tenantId).AdminUpdateTenantRequest(adminUpdateTenantRequest).Execute()

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
	adminUpdateTenantRequest := *openapiclient.NewAdminUpdateTenantRequest() // AdminUpdateTenantRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AdminAPI.UpdateTenant(context.Background(), tenantId).AdminUpdateTenantRequest(adminUpdateTenantRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.UpdateTenant``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateTenant`: AdminTenant
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.UpdateTenant`: %v\n", resp)
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

 **adminUpdateTenantRequest** | [**AdminUpdateTenantRequest**](AdminUpdateTenantRequest.md) |  |

### Return type

[**AdminTenant**](AdminTenant.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)


## UpdateTenantQuota

> AdminTenantQuota UpdateTenantQuota(ctx, tenantId).AdminUpdateQuotaRequest(adminUpdateQuotaRequest).Execute()

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
	adminUpdateQuotaRequest := *openapiclient.NewAdminUpdateQuotaRequest(NullableInt32(123)) // AdminUpdateQuotaRequest |

	configuration := openapiclient.NewConfiguration()
	apiClient := openapiclient.NewAPIClient(configuration)
	resp, r, err := apiClient.AdminAPI.UpdateTenantQuota(context.Background(), tenantId).AdminUpdateQuotaRequest(adminUpdateQuotaRequest).Execute()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error when calling `AdminAPI.UpdateTenantQuota``: %v\n", err)
		fmt.Fprintf(os.Stderr, "Full HTTP response: %v\n", r)
	}
	// response from `UpdateTenantQuota`: AdminTenantQuota
	fmt.Fprintf(os.Stdout, "Response from `AdminAPI.UpdateTenantQuota`: %v\n", resp)
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

 **adminUpdateQuotaRequest** | [**AdminUpdateQuotaRequest**](AdminUpdateQuotaRequest.md) |  |

### Return type

[**AdminTenantQuota**](AdminTenantQuota.md)

### Authorization

[AdminSecret](../README.md#AdminSecret)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints)
[[Back to Model list]](../README.md#documentation-for-models)
[[Back to README]](../README.md)
