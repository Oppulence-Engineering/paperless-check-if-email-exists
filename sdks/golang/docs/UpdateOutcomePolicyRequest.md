# UpdateOutcomePolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IsDefault** | Pointer to **NullableBool** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Rules** | Pointer to **interface{}** |  | [optional]

## Methods

### NewUpdateOutcomePolicyRequest

`func NewUpdateOutcomePolicyRequest() *UpdateOutcomePolicyRequest`

NewUpdateOutcomePolicyRequest instantiates a new UpdateOutcomePolicyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateOutcomePolicyRequestWithDefaults

`func NewUpdateOutcomePolicyRequestWithDefaults() *UpdateOutcomePolicyRequest`

NewUpdateOutcomePolicyRequestWithDefaults instantiates a new UpdateOutcomePolicyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIsDefault

`func (o *UpdateOutcomePolicyRequest) GetIsDefault() bool`

GetIsDefault returns the IsDefault field if non-nil, zero value otherwise.

### GetIsDefaultOk

`func (o *UpdateOutcomePolicyRequest) GetIsDefaultOk() (*bool, bool)`

GetIsDefaultOk returns a tuple with the IsDefault field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDefault

`func (o *UpdateOutcomePolicyRequest) SetIsDefault(v bool)`

SetIsDefault sets IsDefault field to given value.

### HasIsDefault

`func (o *UpdateOutcomePolicyRequest) HasIsDefault() bool`

HasIsDefault returns a boolean if a field has been set.

### SetIsDefaultNil

`func (o *UpdateOutcomePolicyRequest) SetIsDefaultNil()`

 SetIsDefaultNil sets the value for IsDefault to be an explicit nil

### UnsetIsDefault
`func (o *UpdateOutcomePolicyRequest) UnsetIsDefault()`

UnsetIsDefault ensures that no value is present for IsDefault, not even an explicit nil

### GetName

`func (o *UpdateOutcomePolicyRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateOutcomePolicyRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateOutcomePolicyRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *UpdateOutcomePolicyRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdateOutcomePolicyRequest) SetNameNil()`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdateOutcomePolicyRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil

### GetRules

`func (o *UpdateOutcomePolicyRequest) GetRules() interface{}`

GetRules returns the Rules field if non-nil, zero value otherwise.

### GetRulesOk

`func (o *UpdateOutcomePolicyRequest) GetRulesOk() (*interface{}, bool)`

GetRulesOk returns a tuple with the Rules field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRules

`func (o *UpdateOutcomePolicyRequest) SetRules(v interface{})`

SetRules sets Rules field to given value.

### HasRules

`func (o *UpdateOutcomePolicyRequest) HasRules() bool`

HasRules returns a boolean if a field has been set.

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
