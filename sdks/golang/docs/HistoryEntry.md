# HistoryEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Category** | Pointer to **NullableString** |  | [optional]
**CompletedAt** | Pointer to **NullableTime** |  | [optional]
**IsReachable** | Pointer to **NullableString** |  | [optional]
**JobId** | Pointer to **NullableInt32** |  | [optional]
**PolicyDecision** | Pointer to **NullableString** |  | [optional]
**PolicyEvaluation** | Pointer to **interface{}** |  | [optional]
**PolicyMode** | Pointer to **NullableString** |  | [optional]
**ReasonCodes** | Pointer to **[]string** |  | [optional]
**Recommendation** | Pointer to **interface{}** |  | [optional]
**RecommendationAction** | Pointer to **NullableString** |  | [optional]
**SafeToSend** | Pointer to **NullableBool** |  | [optional]
**Score** | Pointer to **NullableInt32** |  | [optional]
**SubReason** | Pointer to **NullableString** |  | [optional]

## Methods

### NewHistoryEntry

`func NewHistoryEntry() *HistoryEntry`

NewHistoryEntry instantiates a new HistoryEntry object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewHistoryEntryWithDefaults

`func NewHistoryEntryWithDefaults() *HistoryEntry`

NewHistoryEntryWithDefaults instantiates a new HistoryEntry object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCategory

`func (o *HistoryEntry) GetCategory() string`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *HistoryEntry) GetCategoryOk() (*string, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *HistoryEntry) SetCategory(v string)`

SetCategory sets Category field to given value.

### HasCategory

`func (o *HistoryEntry) HasCategory() bool`

HasCategory returns a boolean if a field has been set.

### SetCategoryNil

`func (o *HistoryEntry) SetCategoryNil()`

 SetCategoryNil sets the value for Category to be an explicit nil

### UnsetCategory
`func (o *HistoryEntry) UnsetCategory()`

UnsetCategory ensures that no value is present for Category, not even an explicit nil

### GetCompletedAt

`func (o *HistoryEntry) GetCompletedAt() time.Time`

GetCompletedAt returns the CompletedAt field if non-nil, zero value otherwise.

### GetCompletedAtOk

`func (o *HistoryEntry) GetCompletedAtOk() (*time.Time, bool)`

GetCompletedAtOk returns a tuple with the CompletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompletedAt

`func (o *HistoryEntry) SetCompletedAt(v time.Time)`

SetCompletedAt sets CompletedAt field to given value.

### HasCompletedAt

`func (o *HistoryEntry) HasCompletedAt() bool`

HasCompletedAt returns a boolean if a field has been set.

### SetCompletedAtNil

`func (o *HistoryEntry) SetCompletedAtNil()`

 SetCompletedAtNil sets the value for CompletedAt to be an explicit nil

### UnsetCompletedAt
`func (o *HistoryEntry) UnsetCompletedAt()`

UnsetCompletedAt ensures that no value is present for CompletedAt, not even an explicit nil

### GetIsReachable

`func (o *HistoryEntry) GetIsReachable() string`

GetIsReachable returns the IsReachable field if non-nil, zero value otherwise.

### GetIsReachableOk

`func (o *HistoryEntry) GetIsReachableOk() (*string, bool)`

GetIsReachableOk returns a tuple with the IsReachable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsReachable

`func (o *HistoryEntry) SetIsReachable(v string)`

SetIsReachable sets IsReachable field to given value.

### HasIsReachable

`func (o *HistoryEntry) HasIsReachable() bool`

HasIsReachable returns a boolean if a field has been set.

### SetIsReachableNil

`func (o *HistoryEntry) SetIsReachableNil()`

 SetIsReachableNil sets the value for IsReachable to be an explicit nil

### UnsetIsReachable
`func (o *HistoryEntry) UnsetIsReachable()`

UnsetIsReachable ensures that no value is present for IsReachable, not even an explicit nil

### GetJobId

`func (o *HistoryEntry) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *HistoryEntry) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *HistoryEntry) SetJobId(v int32)`

SetJobId sets JobId field to given value.

### HasJobId

`func (o *HistoryEntry) HasJobId() bool`

HasJobId returns a boolean if a field has been set.

### SetJobIdNil

`func (o *HistoryEntry) SetJobIdNil()`

 SetJobIdNil sets the value for JobId to be an explicit nil

### UnsetJobId
`func (o *HistoryEntry) UnsetJobId()`

UnsetJobId ensures that no value is present for JobId, not even an explicit nil

### GetPolicyDecision

`func (o *HistoryEntry) GetPolicyDecision() string`

GetPolicyDecision returns the PolicyDecision field if non-nil, zero value otherwise.

### GetPolicyDecisionOk

`func (o *HistoryEntry) GetPolicyDecisionOk() (*string, bool)`

GetPolicyDecisionOk returns a tuple with the PolicyDecision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyDecision

`func (o *HistoryEntry) SetPolicyDecision(v string)`

SetPolicyDecision sets PolicyDecision field to given value.

### HasPolicyDecision

`func (o *HistoryEntry) HasPolicyDecision() bool`

HasPolicyDecision returns a boolean if a field has been set.

### SetPolicyDecisionNil

`func (o *HistoryEntry) SetPolicyDecisionNil()`

 SetPolicyDecisionNil sets the value for PolicyDecision to be an explicit nil

### UnsetPolicyDecision
`func (o *HistoryEntry) UnsetPolicyDecision()`

UnsetPolicyDecision ensures that no value is present for PolicyDecision, not even an explicit nil

### GetPolicyEvaluation

`func (o *HistoryEntry) GetPolicyEvaluation() interface{}`

GetPolicyEvaluation returns the PolicyEvaluation field if non-nil, zero value otherwise.

### GetPolicyEvaluationOk

`func (o *HistoryEntry) GetPolicyEvaluationOk() (*interface{}, bool)`

GetPolicyEvaluationOk returns a tuple with the PolicyEvaluation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyEvaluation

`func (o *HistoryEntry) SetPolicyEvaluation(v interface{})`

SetPolicyEvaluation sets PolicyEvaluation field to given value.

### HasPolicyEvaluation

`func (o *HistoryEntry) HasPolicyEvaluation() bool`

HasPolicyEvaluation returns a boolean if a field has been set.

### GetPolicyMode

`func (o *HistoryEntry) GetPolicyMode() string`

GetPolicyMode returns the PolicyMode field if non-nil, zero value otherwise.

### GetPolicyModeOk

`func (o *HistoryEntry) GetPolicyModeOk() (*string, bool)`

GetPolicyModeOk returns a tuple with the PolicyMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyMode

`func (o *HistoryEntry) SetPolicyMode(v string)`

SetPolicyMode sets PolicyMode field to given value.

### HasPolicyMode

`func (o *HistoryEntry) HasPolicyMode() bool`

HasPolicyMode returns a boolean if a field has been set.

### SetPolicyModeNil

`func (o *HistoryEntry) SetPolicyModeNil()`

 SetPolicyModeNil sets the value for PolicyMode to be an explicit nil

### UnsetPolicyMode
`func (o *HistoryEntry) UnsetPolicyMode()`

UnsetPolicyMode ensures that no value is present for PolicyMode, not even an explicit nil

### GetReasonCodes

`func (o *HistoryEntry) GetReasonCodes() []string`

GetReasonCodes returns the ReasonCodes field if non-nil, zero value otherwise.

### GetReasonCodesOk

`func (o *HistoryEntry) GetReasonCodesOk() ([]string, bool)`

GetReasonCodesOk returns a tuple with the ReasonCodes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonCodes

`func (o *HistoryEntry) SetReasonCodes(v []string)`

SetReasonCodes sets ReasonCodes field to given value.

### HasReasonCodes

`func (o *HistoryEntry) HasReasonCodes() bool`

HasReasonCodes returns a boolean if a field has been set.

### GetRecommendation

`func (o *HistoryEntry) GetRecommendation() interface{}`

GetRecommendation returns the Recommendation field if non-nil, zero value otherwise.

### GetRecommendationOk

`func (o *HistoryEntry) GetRecommendationOk() (*interface{}, bool)`

GetRecommendationOk returns a tuple with the Recommendation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendation

`func (o *HistoryEntry) SetRecommendation(v interface{})`

SetRecommendation sets Recommendation field to given value.

### HasRecommendation

`func (o *HistoryEntry) HasRecommendation() bool`

HasRecommendation returns a boolean if a field has been set.

### GetRecommendationAction

`func (o *HistoryEntry) GetRecommendationAction() string`

GetRecommendationAction returns the RecommendationAction field if non-nil, zero value otherwise.

### GetRecommendationActionOk

`func (o *HistoryEntry) GetRecommendationActionOk() (*string, bool)`

GetRecommendationActionOk returns a tuple with the RecommendationAction field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendationAction

`func (o *HistoryEntry) SetRecommendationAction(v string)`

SetRecommendationAction sets RecommendationAction field to given value.

### HasRecommendationAction

`func (o *HistoryEntry) HasRecommendationAction() bool`

HasRecommendationAction returns a boolean if a field has been set.

### SetRecommendationActionNil

`func (o *HistoryEntry) SetRecommendationActionNil()`

 SetRecommendationActionNil sets the value for RecommendationAction to be an explicit nil

### UnsetRecommendationAction
`func (o *HistoryEntry) UnsetRecommendationAction()`

UnsetRecommendationAction ensures that no value is present for RecommendationAction, not even an explicit nil

### GetSafeToSend

`func (o *HistoryEntry) GetSafeToSend() bool`

GetSafeToSend returns the SafeToSend field if non-nil, zero value otherwise.

### GetSafeToSendOk

`func (o *HistoryEntry) GetSafeToSendOk() (*bool, bool)`

GetSafeToSendOk returns a tuple with the SafeToSend field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSend

`func (o *HistoryEntry) SetSafeToSend(v bool)`

SetSafeToSend sets SafeToSend field to given value.

### HasSafeToSend

`func (o *HistoryEntry) HasSafeToSend() bool`

HasSafeToSend returns a boolean if a field has been set.

### SetSafeToSendNil

`func (o *HistoryEntry) SetSafeToSendNil()`

 SetSafeToSendNil sets the value for SafeToSend to be an explicit nil

### UnsetSafeToSend
`func (o *HistoryEntry) UnsetSafeToSend()`

UnsetSafeToSend ensures that no value is present for SafeToSend, not even an explicit nil

### GetScore

`func (o *HistoryEntry) GetScore() int32`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *HistoryEntry) GetScoreOk() (*int32, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *HistoryEntry) SetScore(v int32)`

SetScore sets Score field to given value.

### HasScore

`func (o *HistoryEntry) HasScore() bool`

HasScore returns a boolean if a field has been set.

### SetScoreNil

`func (o *HistoryEntry) SetScoreNil()`

 SetScoreNil sets the value for Score to be an explicit nil

### UnsetScore
`func (o *HistoryEntry) UnsetScore()`

UnsetScore ensures that no value is present for Score, not even an explicit nil

### GetSubReason

`func (o *HistoryEntry) GetSubReason() string`

GetSubReason returns the SubReason field if non-nil, zero value otherwise.

### GetSubReasonOk

`func (o *HistoryEntry) GetSubReasonOk() (*string, bool)`

GetSubReasonOk returns a tuple with the SubReason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubReason

`func (o *HistoryEntry) SetSubReason(v string)`

SetSubReason sets SubReason field to given value.

### HasSubReason

`func (o *HistoryEntry) HasSubReason() bool`

HasSubReason returns a boolean if a field has been set.

### SetSubReasonNil

`func (o *HistoryEntry) SetSubReasonNil()`

 SetSubReasonNil sets the value for SubReason to be an explicit nil

### UnsetSubReason
`func (o *HistoryEntry) UnsetSubReason()`

UnsetSubReason ensures that no value is present for SubReason, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
