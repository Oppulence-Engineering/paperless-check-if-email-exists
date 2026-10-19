# ProviderEndpointView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedIps** | **[]string** |  | [required]
**CreatedAt** | **time.Time** |  | [required]
**DeliveryToken** | Pointer to **NullableString** |  | [optional]
**EndpointId** | **string** |  | [required]
**Label** | **string** |  | [required]
**Provider** | **string** |  | [required]
**ProviderConfigured** | **bool** |  | [required]
**Status** | **string** |  | [required]
**UpdatedAt** | **time.Time** |  | [required]
**WebhookPath** | **string** |  | [required]

## Methods

### NewProviderEndpointView

`func NewProviderEndpointView(allowedIps []string, createdAt time.Time, endpointId string, label string, provider string, providerConfigured bool, status string, updatedAt time.Time, webhookPath string) *ProviderEndpointView`

NewProviderEndpointView instantiates a new ProviderEndpointView object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewProviderEndpointViewWithDefaults

`func NewProviderEndpointViewWithDefaults() *ProviderEndpointView`

NewProviderEndpointViewWithDefaults instantiates a new ProviderEndpointView object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedIps

`func (o *ProviderEndpointView) GetAllowedIps() []string`

GetAllowedIps returns the AllowedIps field if non-nil, zero value otherwise.

### GetAllowedIpsOk

`func (o *ProviderEndpointView) GetAllowedIpsOk() ([]string, bool)`

GetAllowedIpsOk returns a tuple with the AllowedIps field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedIps

`func (o *ProviderEndpointView) SetAllowedIps(v []string)`

SetAllowedIps sets AllowedIps field to given value.


### GetCreatedAt

`func (o *ProviderEndpointView) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ProviderEndpointView) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ProviderEndpointView) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetDeliveryToken

`func (o *ProviderEndpointView) GetDeliveryToken() string`

GetDeliveryToken returns the DeliveryToken field if non-nil, zero value otherwise.

### GetDeliveryTokenOk

`func (o *ProviderEndpointView) GetDeliveryTokenOk() (*string, bool)`

GetDeliveryTokenOk returns a tuple with the DeliveryToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeliveryToken

`func (o *ProviderEndpointView) SetDeliveryToken(v string)`

SetDeliveryToken sets DeliveryToken field to given value.

### HasDeliveryToken

`func (o *ProviderEndpointView) HasDeliveryToken() bool`

HasDeliveryToken returns a boolean if a field has been set.

### SetDeliveryTokenNil

`func (o *ProviderEndpointView) SetDeliveryTokenNil()`

 SetDeliveryTokenNil sets the value for DeliveryToken to be an explicit nil

### UnsetDeliveryToken
`func (o *ProviderEndpointView) UnsetDeliveryToken()`

UnsetDeliveryToken ensures that no value is present for DeliveryToken, not even an explicit nil

### GetEndpointId

`func (o *ProviderEndpointView) GetEndpointId() string`

GetEndpointId returns the EndpointId field if non-nil, zero value otherwise.

### GetEndpointIdOk

`func (o *ProviderEndpointView) GetEndpointIdOk() (*string, bool)`

GetEndpointIdOk returns a tuple with the EndpointId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEndpointId

`func (o *ProviderEndpointView) SetEndpointId(v string)`

SetEndpointId sets EndpointId field to given value.


### GetLabel

`func (o *ProviderEndpointView) GetLabel() string`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *ProviderEndpointView) GetLabelOk() (*string, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *ProviderEndpointView) SetLabel(v string)`

SetLabel sets Label field to given value.


### GetProvider

`func (o *ProviderEndpointView) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *ProviderEndpointView) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *ProviderEndpointView) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetProviderConfigured

`func (o *ProviderEndpointView) GetProviderConfigured() bool`

GetProviderConfigured returns the ProviderConfigured field if non-nil, zero value otherwise.

### GetProviderConfiguredOk

`func (o *ProviderEndpointView) GetProviderConfiguredOk() (*bool, bool)`

GetProviderConfiguredOk returns a tuple with the ProviderConfigured field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderConfigured

`func (o *ProviderEndpointView) SetProviderConfigured(v bool)`

SetProviderConfigured sets ProviderConfigured field to given value.


### GetStatus

`func (o *ProviderEndpointView) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ProviderEndpointView) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ProviderEndpointView) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetUpdatedAt

`func (o *ProviderEndpointView) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *ProviderEndpointView) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *ProviderEndpointView) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.


### GetWebhookPath

`func (o *ProviderEndpointView) GetWebhookPath() string`

GetWebhookPath returns the WebhookPath field if non-nil, zero value otherwise.

### GetWebhookPathOk

`func (o *ProviderEndpointView) GetWebhookPathOk() (*string, bool)`

GetWebhookPathOk returns a tuple with the WebhookPath field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetWebhookPath

`func (o *ProviderEndpointView) SetWebhookPath(v string)`

SetWebhookPath sets WebhookPath field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
