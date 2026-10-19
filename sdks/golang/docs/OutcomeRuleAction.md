# OutcomeRuleAction

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Action** | **string** |  | [required]
**Boost** | Pointer to **int32** |  | [optional]
**ScoreOverride** | Pointer to **string** |  | [optional]
**ThresholdCount** | Pointer to **int64** |  | [optional]
**ThresholdWindowDays** | Pointer to **int32** |  | [optional]

## Methods

### NewOutcomeRuleAction

`func NewOutcomeRuleAction(action string) *OutcomeRuleAction`

NewOutcomeRuleAction instantiates a new OutcomeRuleAction object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOutcomeRuleActionWithDefaults

`func NewOutcomeRuleActionWithDefaults() *OutcomeRuleAction`

NewOutcomeRuleActionWithDefaults instantiates a new OutcomeRuleAction object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAction

`func (o *OutcomeRuleAction) GetAction() string`

GetAction returns the Action field if non-nil, zero value otherwise.

### GetActionOk

`func (o *OutcomeRuleAction) GetActionOk() (*string, bool)`

GetActionOk returns a tuple with the Action field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAction

`func (o *OutcomeRuleAction) SetAction(v string)`

SetAction sets Action field to given value.


### GetBoost

`func (o *OutcomeRuleAction) GetBoost() int32`

GetBoost returns the Boost field if non-nil, zero value otherwise.

### GetBoostOk

`func (o *OutcomeRuleAction) GetBoostOk() (*int32, bool)`

GetBoostOk returns a tuple with the Boost field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBoost

`func (o *OutcomeRuleAction) SetBoost(v int32)`

SetBoost sets Boost field to given value.

### HasBoost

`func (o *OutcomeRuleAction) HasBoost() bool`

HasBoost returns a boolean if a field has been set.

### GetScoreOverride

`func (o *OutcomeRuleAction) GetScoreOverride() string`

GetScoreOverride returns the ScoreOverride field if non-nil, zero value otherwise.

### GetScoreOverrideOk

`func (o *OutcomeRuleAction) GetScoreOverrideOk() (*string, bool)`

GetScoreOverrideOk returns a tuple with the ScoreOverride field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScoreOverride

`func (o *OutcomeRuleAction) SetScoreOverride(v string)`

SetScoreOverride sets ScoreOverride field to given value.

### HasScoreOverride

`func (o *OutcomeRuleAction) HasScoreOverride() bool`

HasScoreOverride returns a boolean if a field has been set.

### GetThresholdCount

`func (o *OutcomeRuleAction) GetThresholdCount() int64`

GetThresholdCount returns the ThresholdCount field if non-nil, zero value otherwise.

### GetThresholdCountOk

`func (o *OutcomeRuleAction) GetThresholdCountOk() (*int64, bool)`

GetThresholdCountOk returns a tuple with the ThresholdCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetThresholdCount

`func (o *OutcomeRuleAction) SetThresholdCount(v int64)`

SetThresholdCount sets ThresholdCount field to given value.

### HasThresholdCount

`func (o *OutcomeRuleAction) HasThresholdCount() bool`

HasThresholdCount returns a boolean if a field has been set.

### GetThresholdWindowDays

`func (o *OutcomeRuleAction) GetThresholdWindowDays() int32`

GetThresholdWindowDays returns the ThresholdWindowDays field if non-nil, zero value otherwise.

### GetThresholdWindowDaysOk

`func (o *OutcomeRuleAction) GetThresholdWindowDaysOk() (*int32, bool)`

GetThresholdWindowDaysOk returns a tuple with the ThresholdWindowDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetThresholdWindowDays

`func (o *OutcomeRuleAction) SetThresholdWindowDays(v int32)`

SetThresholdWindowDays sets ThresholdWindowDays field to given value.

### HasThresholdWindowDays

`func (o *OutcomeRuleAction) HasThresholdWindowDays() bool`

HasThresholdWindowDays returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
