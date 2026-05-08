# VerificationApi

All URIs are relative to *https://api.reacher.email*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**v1EmailHistory**](#v1emailhistory) | **GET** /v1/emails/{email}/history | GET /v1/emails/{email}/history|
|[**v1ListAlerts**](#v1listalerts) | **GET** /v1/alerts | GET /v1/alerts|
|[**v1UpdateAlert**](#v1updatealert) | **PATCH** /v1/alerts/{alert_id} | PATCH /v1/alerts/{alert_id}|

# **v1EmailHistory**
> v1EmailHistory()

Returns the verification history timeline for a specific email address.

### Example

```typescript
import {
    VerificationApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new VerificationApi(configuration);

let email: string; //Email address to look up (default to undefined)
let limit: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1EmailHistory(
    email,
    limit
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **email** | [**string**] | Email address to look up | defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|


### Return type

void (empty response body)

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: Not defined


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Verification history for the email |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1ListAlerts**
> AlertListResponse v1ListAlerts()


### Example

```typescript
import {
    VerificationApi,
    Configuration
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new VerificationApi(configuration);

let status: string; // (optional) (default to undefined)
let type: string; // (optional) (default to undefined)
let limit: number; // (optional) (default to undefined)
let offset: number; // (optional) (default to undefined)

const { status, data } = await apiInstance.v1ListAlerts(
    status,
    type,
    limit,
    offset
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **status** | [**string**] |  | (optional) defaults to undefined|
| **type** | [**string**] |  | (optional) defaults to undefined|
| **limit** | [**number**] |  | (optional) defaults to undefined|
| **offset** | [**number**] |  | (optional) defaults to undefined|


### Return type

**AlertListResponse**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Tenant alert inbox |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **v1UpdateAlert**
> AlertView v1UpdateAlert(updateAlertRequest)


### Example

```typescript
import {
    VerificationApi,
    Configuration,
    UpdateAlertRequest
} from '@oppulence/reacher-sdk';

const configuration = new Configuration();
const apiInstance = new VerificationApi(configuration);

let alertId: number; //Alert identifier (default to undefined)
let updateAlertRequest: UpdateAlertRequest; //

const { status, data } = await apiInstance.v1UpdateAlert(
    alertId,
    updateAlertRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **updateAlertRequest** | **UpdateAlertRequest**|  | |
| **alertId** | [**number**] | Alert identifier | defaults to undefined|


### Return type

**AlertView**

### Authorization

[Authorization](../README.md#Authorization)

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Updated alert |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)
