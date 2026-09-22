# CreateProviderEndpointInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedIps** | Pointer to **[]string** |  | [optional]
**Label** | **string** |  | [required]
**Provider** | **string** |  | [required]
**ProviderConfig** | Pointer to **interface{}** |  | [optional]
**Status** | Pointer to **string** |  | [optional]

## Methods

### NewCreateProviderEndpointInput

`func NewCreateProviderEndpointInput(label string, provider string) *CreateProviderEndpointInput`

NewCreateProviderEndpointInput instantiates a new CreateProviderEndpointInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateProviderEndpointInputWithDefaults

`func NewCreateProviderEndpointInputWithDefaults() *CreateProviderEndpointInput`

NewCreateProviderEndpointInputWithDefaults instantiates a new CreateProviderEndpointInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedIps

`func (o *CreateProviderEndpointInput) GetAllowedIps() []string`

GetAllowedIps returns the AllowedIps field if non-nil, zero value otherwise.

### GetAllowedIpsOk

`func (o *CreateProviderEndpointInput) GetAllowedIpsOk() ([]string, bool)`

GetAllowedIpsOk returns a tuple with the AllowedIps field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedIps

`func (o *CreateProviderEndpointInput) SetAllowedIps(v []string)`

SetAllowedIps sets AllowedIps field to given value.

### HasAllowedIps

`func (o *CreateProviderEndpointInput) HasAllowedIps() bool`

HasAllowedIps returns a boolean if a field has been set.

### GetLabel

`func (o *CreateProviderEndpointInput) GetLabel() string`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *CreateProviderEndpointInput) GetLabelOk() (*string, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *CreateProviderEndpointInput) SetLabel(v string)`

SetLabel sets Label field to given value.


### GetProvider

`func (o *CreateProviderEndpointInput) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *CreateProviderEndpointInput) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *CreateProviderEndpointInput) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetProviderConfig

`func (o *CreateProviderEndpointInput) GetProviderConfig() interface{}`

GetProviderConfig returns the ProviderConfig field if non-nil, zero value otherwise.

### GetProviderConfigOk

`func (o *CreateProviderEndpointInput) GetProviderConfigOk() (*interface{}, bool)`

GetProviderConfigOk returns a tuple with the ProviderConfig field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderConfig

`func (o *CreateProviderEndpointInput) SetProviderConfig(v interface{})`

SetProviderConfig sets ProviderConfig field to given value.

### HasProviderConfig

`func (o *CreateProviderEndpointInput) HasProviderConfig() bool`

HasProviderConfig returns a boolean if a field has been set.

### GetStatus

`func (o *CreateProviderEndpointInput) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *CreateProviderEndpointInput) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *CreateProviderEndpointInput) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *CreateProviderEndpointInput) HasStatus() bool`

HasStatus returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
