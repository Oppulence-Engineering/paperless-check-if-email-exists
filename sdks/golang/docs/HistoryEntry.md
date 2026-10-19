# HistoryEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Category** | Pointer to **NullableString** |  | [optional]
**ChangeType** | Pointer to **NullableString** |  | [optional]
**CompletedAt** | Pointer to **NullableTime** |  | [optional]
**IsReachable** | Pointer to **NullableString** |  | [optional]
**JobId** | Pointer to **NullableInt32** |  | [optional]
**ListId** | Pointer to **NullableInt32** |  | [optional]
**PipelineRunId** | Pointer to **NullableInt64** |  | [optional]
**PolicyDecision** | Pointer to **NullableString** |  | [optional]
**PolicyEvaluation** | Pointer to **interface{}** |  | [optional]
**PolicyMode** | Pointer to **NullableString** |  | [optional]
**PreviousCategory** | Pointer to **NullableString** |  | [optional]
**PreviousListId** | Pointer to **NullableInt32** |  | [optional]
**PreviousPipelineRunId** | Pointer to **NullableInt64** |  | [optional]
**PreviousReasonCodes** | Pointer to **[]string** |  | [optional]
**PreviousSafeToSend** | Pointer to **NullableBool** |  | [optional]
**PreviousScore** | Pointer to **NullableInt32** |  | [optional]
**PreviousTaskId** | Pointer to **NullableInt32** |  | [optional]
**ReasonCodes** | Pointer to **[]string** |  | [optional]
**Recommendation** | Pointer to **interface{}** |  | [optional]
**RecommendationAction** | Pointer to **NullableString** |  | [optional]
**SafeToSend** | Pointer to **NullableBool** |  | [optional]
**Score** | Pointer to **NullableInt32** |  | [optional]
**SubReason** | Pointer to **NullableString** |  | [optional]
**TaskId** | **int32** |  | [required]

## Methods

### NewHistoryEntry

`func NewHistoryEntry(taskId int32) *HistoryEntry`

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

### GetChangeType

`func (o *HistoryEntry) GetChangeType() string`

GetChangeType returns the ChangeType field if non-nil, zero value otherwise.

### GetChangeTypeOk

`func (o *HistoryEntry) GetChangeTypeOk() (*string, bool)`

GetChangeTypeOk returns a tuple with the ChangeType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChangeType

`func (o *HistoryEntry) SetChangeType(v string)`

SetChangeType sets ChangeType field to given value.

### HasChangeType

`func (o *HistoryEntry) HasChangeType() bool`

HasChangeType returns a boolean if a field has been set.

### SetChangeTypeNil

`func (o *HistoryEntry) SetChangeTypeNil()`

 SetChangeTypeNil sets the value for ChangeType to be an explicit nil

### UnsetChangeType
`func (o *HistoryEntry) UnsetChangeType()`

UnsetChangeType ensures that no value is present for ChangeType, not even an explicit nil

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

### GetListId

`func (o *HistoryEntry) GetListId() int32`

GetListId returns the ListId field if non-nil, zero value otherwise.

### GetListIdOk

`func (o *HistoryEntry) GetListIdOk() (*int32, bool)`

GetListIdOk returns a tuple with the ListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListId

`func (o *HistoryEntry) SetListId(v int32)`

SetListId sets ListId field to given value.

### HasListId

`func (o *HistoryEntry) HasListId() bool`

HasListId returns a boolean if a field has been set.

### SetListIdNil

`func (o *HistoryEntry) SetListIdNil()`

 SetListIdNil sets the value for ListId to be an explicit nil

### UnsetListId
`func (o *HistoryEntry) UnsetListId()`

UnsetListId ensures that no value is present for ListId, not even an explicit nil

### GetPipelineRunId

`func (o *HistoryEntry) GetPipelineRunId() int64`

GetPipelineRunId returns the PipelineRunId field if non-nil, zero value otherwise.

### GetPipelineRunIdOk

`func (o *HistoryEntry) GetPipelineRunIdOk() (*int64, bool)`

GetPipelineRunIdOk returns a tuple with the PipelineRunId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPipelineRunId

`func (o *HistoryEntry) SetPipelineRunId(v int64)`

SetPipelineRunId sets PipelineRunId field to given value.

### HasPipelineRunId

`func (o *HistoryEntry) HasPipelineRunId() bool`

HasPipelineRunId returns a boolean if a field has been set.

### SetPipelineRunIdNil

`func (o *HistoryEntry) SetPipelineRunIdNil()`

 SetPipelineRunIdNil sets the value for PipelineRunId to be an explicit nil

### UnsetPipelineRunId
`func (o *HistoryEntry) UnsetPipelineRunId()`

UnsetPipelineRunId ensures that no value is present for PipelineRunId, not even an explicit nil

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

### GetPreviousCategory

`func (o *HistoryEntry) GetPreviousCategory() string`

GetPreviousCategory returns the PreviousCategory field if non-nil, zero value otherwise.

### GetPreviousCategoryOk

`func (o *HistoryEntry) GetPreviousCategoryOk() (*string, bool)`

GetPreviousCategoryOk returns a tuple with the PreviousCategory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreviousCategory

`func (o *HistoryEntry) SetPreviousCategory(v string)`

SetPreviousCategory sets PreviousCategory field to given value.

### HasPreviousCategory

`func (o *HistoryEntry) HasPreviousCategory() bool`

HasPreviousCategory returns a boolean if a field has been set.

### SetPreviousCategoryNil

`func (o *HistoryEntry) SetPreviousCategoryNil()`

 SetPreviousCategoryNil sets the value for PreviousCategory to be an explicit nil

### UnsetPreviousCategory
`func (o *HistoryEntry) UnsetPreviousCategory()`

UnsetPreviousCategory ensures that no value is present for PreviousCategory, not even an explicit nil

### GetPreviousListId

`func (o *HistoryEntry) GetPreviousListId() int32`

GetPreviousListId returns the PreviousListId field if non-nil, zero value otherwise.

### GetPreviousListIdOk

`func (o *HistoryEntry) GetPreviousListIdOk() (*int32, bool)`

GetPreviousListIdOk returns a tuple with the PreviousListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreviousListId

`func (o *HistoryEntry) SetPreviousListId(v int32)`

SetPreviousListId sets PreviousListId field to given value.

### HasPreviousListId

`func (o *HistoryEntry) HasPreviousListId() bool`

HasPreviousListId returns a boolean if a field has been set.

### SetPreviousListIdNil

`func (o *HistoryEntry) SetPreviousListIdNil()`

 SetPreviousListIdNil sets the value for PreviousListId to be an explicit nil

### UnsetPreviousListId
`func (o *HistoryEntry) UnsetPreviousListId()`

UnsetPreviousListId ensures that no value is present for PreviousListId, not even an explicit nil

### GetPreviousPipelineRunId

`func (o *HistoryEntry) GetPreviousPipelineRunId() int64`

GetPreviousPipelineRunId returns the PreviousPipelineRunId field if non-nil, zero value otherwise.

### GetPreviousPipelineRunIdOk

`func (o *HistoryEntry) GetPreviousPipelineRunIdOk() (*int64, bool)`

GetPreviousPipelineRunIdOk returns a tuple with the PreviousPipelineRunId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreviousPipelineRunId

`func (o *HistoryEntry) SetPreviousPipelineRunId(v int64)`

SetPreviousPipelineRunId sets PreviousPipelineRunId field to given value.

### HasPreviousPipelineRunId

`func (o *HistoryEntry) HasPreviousPipelineRunId() bool`

HasPreviousPipelineRunId returns a boolean if a field has been set.

### SetPreviousPipelineRunIdNil

`func (o *HistoryEntry) SetPreviousPipelineRunIdNil()`

 SetPreviousPipelineRunIdNil sets the value for PreviousPipelineRunId to be an explicit nil

### UnsetPreviousPipelineRunId
`func (o *HistoryEntry) UnsetPreviousPipelineRunId()`

UnsetPreviousPipelineRunId ensures that no value is present for PreviousPipelineRunId, not even an explicit nil

### GetPreviousReasonCodes

`func (o *HistoryEntry) GetPreviousReasonCodes() []string`

GetPreviousReasonCodes returns the PreviousReasonCodes field if non-nil, zero value otherwise.

### GetPreviousReasonCodesOk

`func (o *HistoryEntry) GetPreviousReasonCodesOk() ([]string, bool)`

GetPreviousReasonCodesOk returns a tuple with the PreviousReasonCodes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreviousReasonCodes

`func (o *HistoryEntry) SetPreviousReasonCodes(v []string)`

SetPreviousReasonCodes sets PreviousReasonCodes field to given value.

### HasPreviousReasonCodes

`func (o *HistoryEntry) HasPreviousReasonCodes() bool`

HasPreviousReasonCodes returns a boolean if a field has been set.

### GetPreviousSafeToSend

`func (o *HistoryEntry) GetPreviousSafeToSend() bool`

GetPreviousSafeToSend returns the PreviousSafeToSend field if non-nil, zero value otherwise.

### GetPreviousSafeToSendOk

`func (o *HistoryEntry) GetPreviousSafeToSendOk() (*bool, bool)`

GetPreviousSafeToSendOk returns a tuple with the PreviousSafeToSend field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreviousSafeToSend

`func (o *HistoryEntry) SetPreviousSafeToSend(v bool)`

SetPreviousSafeToSend sets PreviousSafeToSend field to given value.

### HasPreviousSafeToSend

`func (o *HistoryEntry) HasPreviousSafeToSend() bool`

HasPreviousSafeToSend returns a boolean if a field has been set.

### SetPreviousSafeToSendNil

`func (o *HistoryEntry) SetPreviousSafeToSendNil()`

 SetPreviousSafeToSendNil sets the value for PreviousSafeToSend to be an explicit nil

### UnsetPreviousSafeToSend
`func (o *HistoryEntry) UnsetPreviousSafeToSend()`

UnsetPreviousSafeToSend ensures that no value is present for PreviousSafeToSend, not even an explicit nil

### GetPreviousScore

`func (o *HistoryEntry) GetPreviousScore() int32`

GetPreviousScore returns the PreviousScore field if non-nil, zero value otherwise.

### GetPreviousScoreOk

`func (o *HistoryEntry) GetPreviousScoreOk() (*int32, bool)`

GetPreviousScoreOk returns a tuple with the PreviousScore field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreviousScore

`func (o *HistoryEntry) SetPreviousScore(v int32)`

SetPreviousScore sets PreviousScore field to given value.

### HasPreviousScore

`func (o *HistoryEntry) HasPreviousScore() bool`

HasPreviousScore returns a boolean if a field has been set.

### SetPreviousScoreNil

`func (o *HistoryEntry) SetPreviousScoreNil()`

 SetPreviousScoreNil sets the value for PreviousScore to be an explicit nil

### UnsetPreviousScore
`func (o *HistoryEntry) UnsetPreviousScore()`

UnsetPreviousScore ensures that no value is present for PreviousScore, not even an explicit nil

### GetPreviousTaskId

`func (o *HistoryEntry) GetPreviousTaskId() int32`

GetPreviousTaskId returns the PreviousTaskId field if non-nil, zero value otherwise.

### GetPreviousTaskIdOk

`func (o *HistoryEntry) GetPreviousTaskIdOk() (*int32, bool)`

GetPreviousTaskIdOk returns a tuple with the PreviousTaskId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPreviousTaskId

`func (o *HistoryEntry) SetPreviousTaskId(v int32)`

SetPreviousTaskId sets PreviousTaskId field to given value.

### HasPreviousTaskId

`func (o *HistoryEntry) HasPreviousTaskId() bool`

HasPreviousTaskId returns a boolean if a field has been set.

### SetPreviousTaskIdNil

`func (o *HistoryEntry) SetPreviousTaskIdNil()`

 SetPreviousTaskIdNil sets the value for PreviousTaskId to be an explicit nil

### UnsetPreviousTaskId
`func (o *HistoryEntry) UnsetPreviousTaskId()`

UnsetPreviousTaskId ensures that no value is present for PreviousTaskId, not even an explicit nil

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

### GetTaskId

`func (o *HistoryEntry) GetTaskId() int32`

GetTaskId returns the TaskId field if non-nil, zero value otherwise.

### GetTaskIdOk

`func (o *HistoryEntry) GetTaskIdOk() (*int32, bool)`

GetTaskIdOk returns a tuple with the TaskId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTaskId

`func (o *HistoryEntry) SetTaskId(v int32)`

SetTaskId sets TaskId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
