# ScorePolicyRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Review** | Pointer to [**NullableScorePolicyCondition**](ScorePolicyCondition.md) |  | [optional]
**Send** | Pointer to [**NullableScorePolicyCondition**](ScorePolicyCondition.md) |  | [optional]
**Suppress** | Pointer to [**NullableScorePolicyCondition**](ScorePolicyCondition.md) |  | [optional]

## Methods

### NewScorePolicyRules

`func NewScorePolicyRules() *ScorePolicyRules`

NewScorePolicyRules instantiates a new ScorePolicyRules object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewScorePolicyRulesWithDefaults

`func NewScorePolicyRulesWithDefaults() *ScorePolicyRules`

NewScorePolicyRulesWithDefaults instantiates a new ScorePolicyRules object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetReview

`func (o *ScorePolicyRules) GetReview() ScorePolicyCondition`

GetReview returns the Review field if non-nil, zero value otherwise.

### GetReviewOk

`func (o *ScorePolicyRules) GetReviewOk() (*ScorePolicyCondition, bool)`

GetReviewOk returns a tuple with the Review field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReview

`func (o *ScorePolicyRules) SetReview(v ScorePolicyCondition)`

SetReview sets Review field to given value.

### HasReview

`func (o *ScorePolicyRules) HasReview() bool`

HasReview returns a boolean if a field has been set.

### SetReviewNil

`func (o *ScorePolicyRules) SetReviewNil()`

 SetReviewNil sets the value for Review to be an explicit nil

### UnsetReview
`func (o *ScorePolicyRules) UnsetReview()`

UnsetReview ensures that no value is present for Review, not even an explicit nil

### GetSend

`func (o *ScorePolicyRules) GetSend() ScorePolicyCondition`

GetSend returns the Send field if non-nil, zero value otherwise.

### GetSendOk

`func (o *ScorePolicyRules) GetSendOk() (*ScorePolicyCondition, bool)`

GetSendOk returns a tuple with the Send field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSend

`func (o *ScorePolicyRules) SetSend(v ScorePolicyCondition)`

SetSend sets Send field to given value.

### HasSend

`func (o *ScorePolicyRules) HasSend() bool`

HasSend returns a boolean if a field has been set.

### SetSendNil

`func (o *ScorePolicyRules) SetSendNil()`

 SetSendNil sets the value for Send to be an explicit nil

### UnsetSend
`func (o *ScorePolicyRules) UnsetSend()`

UnsetSend ensures that no value is present for Send, not even an explicit nil

### GetSuppress

`func (o *ScorePolicyRules) GetSuppress() ScorePolicyCondition`

GetSuppress returns the Suppress field if non-nil, zero value otherwise.

### GetSuppressOk

`func (o *ScorePolicyRules) GetSuppressOk() (*ScorePolicyCondition, bool)`

GetSuppressOk returns a tuple with the Suppress field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSuppress

`func (o *ScorePolicyRules) SetSuppress(v ScorePolicyCondition)`

SetSuppress sets Suppress field to given value.

### HasSuppress

`func (o *ScorePolicyRules) HasSuppress() bool`

HasSuppress returns a boolean if a field has been set.

### SetSuppressNil

`func (o *ScorePolicyRules) SetSuppressNil()`

 SetSuppressNil sets the value for Suppress to be an explicit nil

### UnsetSuppress
`func (o *ScorePolicyRules) UnsetSuppress()`

UnsetSuppress ensures that no value is present for Suppress, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
