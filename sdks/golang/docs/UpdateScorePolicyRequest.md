# UpdateScorePolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IsDefault** | Pointer to **NullableBool** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Rules** | Pointer to **interface{}** |  | [optional]

## Methods

### NewUpdateScorePolicyRequest

`func NewUpdateScorePolicyRequest() *UpdateScorePolicyRequest`

NewUpdateScorePolicyRequest instantiates a new UpdateScorePolicyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateScorePolicyRequestWithDefaults

`func NewUpdateScorePolicyRequestWithDefaults() *UpdateScorePolicyRequest`

NewUpdateScorePolicyRequestWithDefaults instantiates a new UpdateScorePolicyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIsDefault

`func (o *UpdateScorePolicyRequest) GetIsDefault() bool`

GetIsDefault returns the IsDefault field if non-nil, zero value otherwise.

### GetIsDefaultOk

`func (o *UpdateScorePolicyRequest) GetIsDefaultOk() (*bool, bool)`

GetIsDefaultOk returns a tuple with the IsDefault field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDefault

`func (o *UpdateScorePolicyRequest) SetIsDefault(v bool)`

SetIsDefault sets IsDefault field to given value.

### HasIsDefault

`func (o *UpdateScorePolicyRequest) HasIsDefault() bool`

HasIsDefault returns a boolean if a field has been set.

### SetIsDefaultNil

`func (o *UpdateScorePolicyRequest) SetIsDefaultNil()`

 SetIsDefaultNil sets the value for IsDefault to be an explicit nil

### UnsetIsDefault
`func (o *UpdateScorePolicyRequest) UnsetIsDefault()`

UnsetIsDefault ensures that no value is present for IsDefault, not even an explicit nil

### GetName

`func (o *UpdateScorePolicyRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateScorePolicyRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateScorePolicyRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *UpdateScorePolicyRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdateScorePolicyRequest) SetNameNil()`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdateScorePolicyRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil

### GetRules

`func (o *UpdateScorePolicyRequest) GetRules() interface{}`

GetRules returns the Rules field if non-nil, zero value otherwise.

### GetRulesOk

`func (o *UpdateScorePolicyRequest) GetRulesOk() (*interface{}, bool)`

GetRulesOk returns a tuple with the Rules field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRules

`func (o *UpdateScorePolicyRequest) SetRules(v interface{})`

SetRules sets Rules field to given value.

### HasRules

`func (o *UpdateScorePolicyRequest) HasRules() bool`

HasRules returns a boolean if a field has been set.

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
