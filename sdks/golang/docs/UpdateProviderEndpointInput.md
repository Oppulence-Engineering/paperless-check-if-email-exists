# UpdateProviderEndpointInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AllowedIps** | Pointer to **[]string** |  | [optional]
**Label** | Pointer to **NullableString** |  | [optional]
**ProviderConfig** | Pointer to **interface{}** |  | [optional]
**RotateDeliveryToken** | Pointer to **bool** |  | [optional]
**Status** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateProviderEndpointInput

`func NewUpdateProviderEndpointInput() *UpdateProviderEndpointInput`

NewUpdateProviderEndpointInput instantiates a new UpdateProviderEndpointInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateProviderEndpointInputWithDefaults

`func NewUpdateProviderEndpointInputWithDefaults() *UpdateProviderEndpointInput`

NewUpdateProviderEndpointInputWithDefaults instantiates a new UpdateProviderEndpointInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAllowedIps

`func (o *UpdateProviderEndpointInput) GetAllowedIps() []string`

GetAllowedIps returns the AllowedIps field if non-nil, zero value otherwise.

### GetAllowedIpsOk

`func (o *UpdateProviderEndpointInput) GetAllowedIpsOk() ([]string, bool)`

GetAllowedIpsOk returns a tuple with the AllowedIps field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAllowedIps

`func (o *UpdateProviderEndpointInput) SetAllowedIps(v []string)`

SetAllowedIps sets AllowedIps field to given value.

### HasAllowedIps

`func (o *UpdateProviderEndpointInput) HasAllowedIps() bool`

HasAllowedIps returns a boolean if a field has been set.

### GetLabel

`func (o *UpdateProviderEndpointInput) GetLabel() string`

GetLabel returns the Label field if non-nil, zero value otherwise.

### GetLabelOk

`func (o *UpdateProviderEndpointInput) GetLabelOk() (*string, bool)`

GetLabelOk returns a tuple with the Label field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLabel

`func (o *UpdateProviderEndpointInput) SetLabel(v string)`

SetLabel sets Label field to given value.

### HasLabel

`func (o *UpdateProviderEndpointInput) HasLabel() bool`

HasLabel returns a boolean if a field has been set.

### SetLabelNil

`func (o *UpdateProviderEndpointInput) SetLabelNil()`

 SetLabelNil sets the value for Label to be an explicit nil

### UnsetLabel
`func (o *UpdateProviderEndpointInput) UnsetLabel()`

UnsetLabel ensures that no value is present for Label, not even an explicit nil

### GetProviderConfig

`func (o *UpdateProviderEndpointInput) GetProviderConfig() interface{}`

GetProviderConfig returns the ProviderConfig field if non-nil, zero value otherwise.

### GetProviderConfigOk

`func (o *UpdateProviderEndpointInput) GetProviderConfigOk() (*interface{}, bool)`

GetProviderConfigOk returns a tuple with the ProviderConfig field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProviderConfig

`func (o *UpdateProviderEndpointInput) SetProviderConfig(v interface{})`

SetProviderConfig sets ProviderConfig field to given value.

### HasProviderConfig

`func (o *UpdateProviderEndpointInput) HasProviderConfig() bool`

HasProviderConfig returns a boolean if a field has been set.

### GetRotateDeliveryToken

`func (o *UpdateProviderEndpointInput) GetRotateDeliveryToken() bool`

GetRotateDeliveryToken returns the RotateDeliveryToken field if non-nil, zero value otherwise.

### GetRotateDeliveryTokenOk

`func (o *UpdateProviderEndpointInput) GetRotateDeliveryTokenOk() (*bool, bool)`

GetRotateDeliveryTokenOk returns a tuple with the RotateDeliveryToken field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRotateDeliveryToken

`func (o *UpdateProviderEndpointInput) SetRotateDeliveryToken(v bool)`

SetRotateDeliveryToken sets RotateDeliveryToken field to given value.

### HasRotateDeliveryToken

`func (o *UpdateProviderEndpointInput) HasRotateDeliveryToken() bool`

HasRotateDeliveryToken returns a boolean if a field has been set.

### GetStatus

`func (o *UpdateProviderEndpointInput) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *UpdateProviderEndpointInput) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *UpdateProviderEndpointInput) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *UpdateProviderEndpointInput) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### SetStatusNil

`func (o *UpdateProviderEndpointInput) SetStatusNil()`

 SetStatusNil sets the value for Status to be an explicit nil

### UnsetStatus
`func (o *UpdateProviderEndpointInput) UnsetStatus()`

UnsetStatus ensures that no value is present for Status, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
