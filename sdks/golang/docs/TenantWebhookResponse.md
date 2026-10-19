# TenantWebhookResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DefaultWebhookUrl** | Pointer to **NullableString** |  | [optional]
**TenantId** | **string** |  | [required]
**TenantName** | **string** |  | [required]
**WebhookSigningSecretConfigured** | **bool** |  | [required]

## Methods

### NewTenantWebhookResponse

`func NewTenantWebhookResponse(tenantId string, tenantName string, webhookSigningSecretConfigured bool) *TenantWebhookResponse`

NewTenantWebhookResponse instantiates a new TenantWebhookResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTenantWebhookResponseWithDefaults

`func NewTenantWebhookResponseWithDefaults() *TenantWebhookResponse`

NewTenantWebhookResponseWithDefaults instantiates a new TenantWebhookResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDefaultWebhookUrl

`func (o *TenantWebhookResponse) GetDefaultWebhookUrl() string`

GetDefaultWebhookUrl returns the DefaultWebhookUrl field if non-nil, zero value otherwise.

### GetDefaultWebhookUrlOk

`func (o *TenantWebhookResponse) GetDefaultWebhookUrlOk() (*string, bool)`

GetDefaultWebhookUrlOk returns a tuple with the DefaultWebhookUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDefaultWebhookUrl

`func (o *TenantWebhookResponse) SetDefaultWebhookUrl(v string)`

SetDefaultWebhookUrl sets DefaultWebhookUrl field to given value.

### HasDefaultWebhookUrl

`func (o *TenantWebhookResponse) HasDefaultWebhookUrl() bool`

HasDefaultWebhookUrl returns a boolean if a field has been set.

### SetDefaultWebhookUrlNil

`func (o *TenantWebhookResponse) SetDefaultWebhookUrlNil()`

 SetDefaultWebhookUrlNil sets the value for DefaultWebhookUrl to be an explicit nil

### UnsetDefaultWebhookUrl
`func (o *TenantWebhookResponse) UnsetDefaultWebhookUrl()`

UnsetDefaultWebhookUrl ensures that no value is present for DefaultWebhookUrl, not even an explicit nil

### GetTenantId

`func (o *TenantWebhookResponse) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *TenantWebhookResponse) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *TenantWebhookResponse) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.


### GetTenantName

`func (o *TenantWebhookResponse) GetTenantName() string`

GetTenantName returns the TenantName field if non-nil, zero value otherwise.

### GetTenantNameOk

`func (o *TenantWebhookResponse) GetTenantNameOk() (*string, bool)`

GetTenantNameOk returns a tuple with the TenantName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantName

`func (o *TenantWebhookResponse) SetTenantName(v string)`

SetTenantName sets TenantName field to given value.


### GetWebhookSigningSecretConfigured

`func (o *TenantWebhookResponse) GetWebhookSigningSecretConfigured() bool`

GetWebhookSigningSecretConfigured returns the WebhookSigningSecretConfigured field if non-nil, zero value otherwise.

### GetWebhookSigningSecretConfiguredOk

`func (o *TenantWebhookResponse) GetWebhookSigningSecretConfiguredOk() (*bool, bool)`

GetWebhookSigningSecretConfiguredOk returns a tuple with the WebhookSigningSecretConfigured field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhookSigningSecretConfigured

`func (o *TenantWebhookResponse) SetWebhookSigningSecretConfigured(v bool)`

SetWebhookSigningSecretConfigured sets WebhookSigningSecretConfigured field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
