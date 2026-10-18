# ScorePolicyCondition

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Category** | Pointer to **[]string** |  | [optional]
**ReasonCodesAny** | Pointer to **[]string** |  | [optional]
**SafeToSend** | Pointer to **NullableBool** |  | [optional]
**ScoreMax** | Pointer to **NullableInt32** |  | [optional]
**ScoreMin** | Pointer to **NullableInt32** |  | [optional]

## Methods

### NewScorePolicyCondition

`func NewScorePolicyCondition() *ScorePolicyCondition`

NewScorePolicyCondition instantiates a new ScorePolicyCondition object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewScorePolicyConditionWithDefaults

`func NewScorePolicyConditionWithDefaults() *ScorePolicyCondition`

NewScorePolicyConditionWithDefaults instantiates a new ScorePolicyCondition object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCategory

`func (o *ScorePolicyCondition) GetCategory() []string`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *ScorePolicyCondition) GetCategoryOk() ([]string, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *ScorePolicyCondition) SetCategory(v []string)`

SetCategory sets Category field to given value.

### HasCategory

`func (o *ScorePolicyCondition) HasCategory() bool`

HasCategory returns a boolean if a field has been set.

### GetReasonCodesAny

`func (o *ScorePolicyCondition) GetReasonCodesAny() []string`

GetReasonCodesAny returns the ReasonCodesAny field if non-nil, zero value otherwise.

### GetReasonCodesAnyOk

`func (o *ScorePolicyCondition) GetReasonCodesAnyOk() ([]string, bool)`

GetReasonCodesAnyOk returns a tuple with the ReasonCodesAny field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonCodesAny

`func (o *ScorePolicyCondition) SetReasonCodesAny(v []string)`

SetReasonCodesAny sets ReasonCodesAny field to given value.

### HasReasonCodesAny

`func (o *ScorePolicyCondition) HasReasonCodesAny() bool`

HasReasonCodesAny returns a boolean if a field has been set.

### GetSafeToSend

`func (o *ScorePolicyCondition) GetSafeToSend() bool`

GetSafeToSend returns the SafeToSend field if non-nil, zero value otherwise.

### GetSafeToSendOk

`func (o *ScorePolicyCondition) GetSafeToSendOk() (*bool, bool)`

GetSafeToSendOk returns a tuple with the SafeToSend field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSend

`func (o *ScorePolicyCondition) SetSafeToSend(v bool)`

SetSafeToSend sets SafeToSend field to given value.

### HasSafeToSend

`func (o *ScorePolicyCondition) HasSafeToSend() bool`

HasSafeToSend returns a boolean if a field has been set.

### SetSafeToSendNil

`func (o *ScorePolicyCondition) SetSafeToSendNil()`

 SetSafeToSendNil sets the value for SafeToSend to be an explicit nil

### UnsetSafeToSend
`func (o *ScorePolicyCondition) UnsetSafeToSend()`

UnsetSafeToSend ensures that no value is present for SafeToSend, not even an explicit nil

### GetScoreMax

`func (o *ScorePolicyCondition) GetScoreMax() int32`

GetScoreMax returns the ScoreMax field if non-nil, zero value otherwise.

### GetScoreMaxOk

`func (o *ScorePolicyCondition) GetScoreMaxOk() (*int32, bool)`

GetScoreMaxOk returns a tuple with the ScoreMax field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScoreMax

`func (o *ScorePolicyCondition) SetScoreMax(v int32)`

SetScoreMax sets ScoreMax field to given value.

### HasScoreMax

`func (o *ScorePolicyCondition) HasScoreMax() bool`

HasScoreMax returns a boolean if a field has been set.

### SetScoreMaxNil

`func (o *ScorePolicyCondition) SetScoreMaxNil()`

 SetScoreMaxNil sets the value for ScoreMax to be an explicit nil

### UnsetScoreMax
`func (o *ScorePolicyCondition) UnsetScoreMax()`

UnsetScoreMax ensures that no value is present for ScoreMax, not even an explicit nil

### GetScoreMin

`func (o *ScorePolicyCondition) GetScoreMin() int32`

GetScoreMin returns the ScoreMin field if non-nil, zero value otherwise.

### GetScoreMinOk

`func (o *ScorePolicyCondition) GetScoreMinOk() (*int32, bool)`

GetScoreMinOk returns a tuple with the ScoreMin field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScoreMin

`func (o *ScorePolicyCondition) SetScoreMin(v int32)`

SetScoreMin sets ScoreMin field to given value.

### HasScoreMin

`func (o *ScorePolicyCondition) HasScoreMin() bool`

HasScoreMin returns a boolean if a field has been set.

### SetScoreMinNil

`func (o *ScorePolicyCondition) SetScoreMinNil()`

 SetScoreMinNil sets the value for ScoreMin to be an explicit nil

### UnsetScoreMin
`func (o *ScorePolicyCondition) UnsetScoreMin()`

UnsetScoreMin ensures that no value is present for ScoreMin, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
