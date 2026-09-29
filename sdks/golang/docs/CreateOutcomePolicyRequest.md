# CreateOutcomePolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IsDefault** | Pointer to **bool** |  | [optional]
**Name** | **string** |  | [required]
**Rules** | Pointer to **interface{}** |  | [optional]

## Methods

### NewCreateOutcomePolicyRequest

`func NewCreateOutcomePolicyRequest(name string) *CreateOutcomePolicyRequest`

NewCreateOutcomePolicyRequest instantiates a new CreateOutcomePolicyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateOutcomePolicyRequestWithDefaults

`func NewCreateOutcomePolicyRequestWithDefaults() *CreateOutcomePolicyRequest`

NewCreateOutcomePolicyRequestWithDefaults instantiates a new CreateOutcomePolicyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIsDefault

`func (o *CreateOutcomePolicyRequest) GetIsDefault() bool`

GetIsDefault returns the IsDefault field if non-nil, zero value otherwise.

### GetIsDefaultOk

`func (o *CreateOutcomePolicyRequest) GetIsDefaultOk() (*bool, bool)`

GetIsDefaultOk returns a tuple with the IsDefault field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDefault

`func (o *CreateOutcomePolicyRequest) SetIsDefault(v bool)`

SetIsDefault sets IsDefault field to given value.

### HasIsDefault

`func (o *CreateOutcomePolicyRequest) HasIsDefault() bool`

HasIsDefault returns a boolean if a field has been set.

### GetName

`func (o *CreateOutcomePolicyRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateOutcomePolicyRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateOutcomePolicyRequest) SetName(v string)`

SetName sets Name field to given value.


### GetRules

`func (o *CreateOutcomePolicyRequest) GetRules() interface{}`

GetRules returns the Rules field if non-nil, zero value otherwise.

### GetRulesOk

`func (o *CreateOutcomePolicyRequest) GetRulesOk() (*interface{}, bool)`

GetRulesOk returns a tuple with the Rules field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRules

`func (o *CreateOutcomePolicyRequest) SetRules(v interface{})`

SetRules sets Rules field to given value.

### HasRules

`func (o *CreateOutcomePolicyRequest) HasRules() bool`

HasRules returns a boolean if a field has been set.

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
