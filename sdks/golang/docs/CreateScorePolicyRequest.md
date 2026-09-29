# CreateScorePolicyRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**IsDefault** | Pointer to **bool** |  | [optional]
**Name** | **string** |  | [required]
**Rules** | Pointer to **interface{}** |  | [optional]

## Methods

### NewCreateScorePolicyRequest

`func NewCreateScorePolicyRequest(name string) *CreateScorePolicyRequest`

NewCreateScorePolicyRequest instantiates a new CreateScorePolicyRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateScorePolicyRequestWithDefaults

`func NewCreateScorePolicyRequestWithDefaults() *CreateScorePolicyRequest`

NewCreateScorePolicyRequestWithDefaults instantiates a new CreateScorePolicyRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetIsDefault

`func (o *CreateScorePolicyRequest) GetIsDefault() bool`

GetIsDefault returns the IsDefault field if non-nil, zero value otherwise.

### GetIsDefaultOk

`func (o *CreateScorePolicyRequest) GetIsDefaultOk() (*bool, bool)`

GetIsDefaultOk returns a tuple with the IsDefault field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDefault

`func (o *CreateScorePolicyRequest) SetIsDefault(v bool)`

SetIsDefault sets IsDefault field to given value.

### HasIsDefault

`func (o *CreateScorePolicyRequest) HasIsDefault() bool`

HasIsDefault returns a boolean if a field has been set.

### GetName

`func (o *CreateScorePolicyRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateScorePolicyRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateScorePolicyRequest) SetName(v string)`

SetName sets Name field to given value.


### GetRules

`func (o *CreateScorePolicyRequest) GetRules() interface{}`

GetRules returns the Rules field if non-nil, zero value otherwise.

### GetRulesOk

`func (o *CreateScorePolicyRequest) GetRulesOk() (*interface{}, bool)`

GetRulesOk returns a tuple with the Rules field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRules

`func (o *CreateScorePolicyRequest) SetRules(v interface{})`

SetRules sets Rules field to given value.

### HasRules

`func (o *CreateScorePolicyRequest) HasRules() bool`

HasRules returns a boolean if a field has been set.

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
