# QueryResultRow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Category** | Pointer to **NullableString** |  | [optional]
**CompletedAt** | Pointer to **NullableTime** |  | [optional]
**CreatedAt** | **time.Time** |  | [required]
**Email** | Pointer to **NullableString** |  | [optional]
**Id** | **int32** |  | [required]
**JobId** | Pointer to **NullableInt32** |  | [optional]
**PolicyDecision** | Pointer to **NullableString** |  | [optional]
**PolicyEvaluation** | Pointer to **map[string]interface{}** |  | [optional]
**PolicyMode** | Pointer to **NullableString** |  | [optional]
**ReasonCodes** | Pointer to **[]string** |  | [optional]
**Recommendation** | Pointer to **map[string]interface{}** |  | [optional]
**RecommendationAction** | Pointer to **NullableString** |  | [optional]
**SafeToSend** | Pointer to **NullableBool** |  | [optional]
**Score** | Pointer to **NullableInt32** |  | [optional]
**SubReason** | Pointer to **NullableString** |  | [optional]
**TaskState** | **string** |  | [required]

## Methods

### NewQueryResultRow

`func NewQueryResultRow(createdAt time.Time, id int32, taskState string) *QueryResultRow`

NewQueryResultRow instantiates a new QueryResultRow object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewQueryResultRowWithDefaults

`func NewQueryResultRowWithDefaults() *QueryResultRow`

NewQueryResultRowWithDefaults instantiates a new QueryResultRow object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCategory

`func (o *QueryResultRow) GetCategory() string`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *QueryResultRow) GetCategoryOk() (*string, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *QueryResultRow) SetCategory(v string)`

SetCategory sets Category field to given value.

### HasCategory

`func (o *QueryResultRow) HasCategory() bool`

HasCategory returns a boolean if a field has been set.

### SetCategoryNil

`func (o *QueryResultRow) SetCategoryNil()`

 SetCategoryNil sets the value for Category to be an explicit nil

### UnsetCategory
`func (o *QueryResultRow) UnsetCategory()`

UnsetCategory ensures that no value is present for Category, not even an explicit nil

### GetCompletedAt

`func (o *QueryResultRow) GetCompletedAt() time.Time`

GetCompletedAt returns the CompletedAt field if non-nil, zero value otherwise.

### GetCompletedAtOk

`func (o *QueryResultRow) GetCompletedAtOk() (*time.Time, bool)`

GetCompletedAtOk returns a tuple with the CompletedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompletedAt

`func (o *QueryResultRow) SetCompletedAt(v time.Time)`

SetCompletedAt sets CompletedAt field to given value.

### HasCompletedAt

`func (o *QueryResultRow) HasCompletedAt() bool`

HasCompletedAt returns a boolean if a field has been set.

### SetCompletedAtNil

`func (o *QueryResultRow) SetCompletedAtNil()`

 SetCompletedAtNil sets the value for CompletedAt to be an explicit nil

### UnsetCompletedAt
`func (o *QueryResultRow) UnsetCompletedAt()`

UnsetCompletedAt ensures that no value is present for CompletedAt, not even an explicit nil

### GetCreatedAt

`func (o *QueryResultRow) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *QueryResultRow) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *QueryResultRow) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetEmail

`func (o *QueryResultRow) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *QueryResultRow) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *QueryResultRow) SetEmail(v string)`

SetEmail sets Email field to given value.

### HasEmail

`func (o *QueryResultRow) HasEmail() bool`

HasEmail returns a boolean if a field has been set.

### SetEmailNil

`func (o *QueryResultRow) SetEmailNil()`

 SetEmailNil sets the value for Email to be an explicit nil

### UnsetEmail
`func (o *QueryResultRow) UnsetEmail()`

UnsetEmail ensures that no value is present for Email, not even an explicit nil

### GetId

`func (o *QueryResultRow) GetId() int32`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *QueryResultRow) GetIdOk() (*int32, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *QueryResultRow) SetId(v int32)`

SetId sets Id field to given value.


### GetJobId

`func (o *QueryResultRow) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *QueryResultRow) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *QueryResultRow) SetJobId(v int32)`

SetJobId sets JobId field to given value.

### HasJobId

`func (o *QueryResultRow) HasJobId() bool`

HasJobId returns a boolean if a field has been set.

### SetJobIdNil

`func (o *QueryResultRow) SetJobIdNil()`

 SetJobIdNil sets the value for JobId to be an explicit nil

### UnsetJobId
`func (o *QueryResultRow) UnsetJobId()`

UnsetJobId ensures that no value is present for JobId, not even an explicit nil

### GetPolicyDecision

`func (o *QueryResultRow) GetPolicyDecision() string`

GetPolicyDecision returns the PolicyDecision field if non-nil, zero value otherwise.

### GetPolicyDecisionOk

`func (o *QueryResultRow) GetPolicyDecisionOk() (*string, bool)`

GetPolicyDecisionOk returns a tuple with the PolicyDecision field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyDecision

`func (o *QueryResultRow) SetPolicyDecision(v string)`

SetPolicyDecision sets PolicyDecision field to given value.

### HasPolicyDecision

`func (o *QueryResultRow) HasPolicyDecision() bool`

HasPolicyDecision returns a boolean if a field has been set.

### SetPolicyDecisionNil

`func (o *QueryResultRow) SetPolicyDecisionNil()`

 SetPolicyDecisionNil sets the value for PolicyDecision to be an explicit nil

### UnsetPolicyDecision
`func (o *QueryResultRow) UnsetPolicyDecision()`

UnsetPolicyDecision ensures that no value is present for PolicyDecision, not even an explicit nil

### GetPolicyEvaluation

`func (o *QueryResultRow) GetPolicyEvaluation() map[string]interface{}`

GetPolicyEvaluation returns the PolicyEvaluation field if non-nil, zero value otherwise.

### GetPolicyEvaluationOk

`func (o *QueryResultRow) GetPolicyEvaluationOk() (map[string]interface{}, bool)`

GetPolicyEvaluationOk returns a tuple with the PolicyEvaluation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyEvaluation

`func (o *QueryResultRow) SetPolicyEvaluation(v map[string]interface{})`

SetPolicyEvaluation sets PolicyEvaluation field to given value.

### HasPolicyEvaluation

`func (o *QueryResultRow) HasPolicyEvaluation() bool`

HasPolicyEvaluation returns a boolean if a field has been set.

### GetPolicyMode

`func (o *QueryResultRow) GetPolicyMode() string`

GetPolicyMode returns the PolicyMode field if non-nil, zero value otherwise.

### GetPolicyModeOk

`func (o *QueryResultRow) GetPolicyModeOk() (*string, bool)`

GetPolicyModeOk returns a tuple with the PolicyMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyMode

`func (o *QueryResultRow) SetPolicyMode(v string)`

SetPolicyMode sets PolicyMode field to given value.

### HasPolicyMode

`func (o *QueryResultRow) HasPolicyMode() bool`

HasPolicyMode returns a boolean if a field has been set.

### SetPolicyModeNil

`func (o *QueryResultRow) SetPolicyModeNil()`

 SetPolicyModeNil sets the value for PolicyMode to be an explicit nil

### UnsetPolicyMode
`func (o *QueryResultRow) UnsetPolicyMode()`

UnsetPolicyMode ensures that no value is present for PolicyMode, not even an explicit nil

### GetReasonCodes

`func (o *QueryResultRow) GetReasonCodes() []string`

GetReasonCodes returns the ReasonCodes field if non-nil, zero value otherwise.

### GetReasonCodesOk

`func (o *QueryResultRow) GetReasonCodesOk() ([]string, bool)`

GetReasonCodesOk returns a tuple with the ReasonCodes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonCodes

`func (o *QueryResultRow) SetReasonCodes(v []string)`

SetReasonCodes sets ReasonCodes field to given value.

### HasReasonCodes

`func (o *QueryResultRow) HasReasonCodes() bool`

HasReasonCodes returns a boolean if a field has been set.

### GetRecommendation

`func (o *QueryResultRow) GetRecommendation() map[string]interface{}`

GetRecommendation returns the Recommendation field if non-nil, zero value otherwise.

### GetRecommendationOk

`func (o *QueryResultRow) GetRecommendationOk() (map[string]interface{}, bool)`

GetRecommendationOk returns a tuple with the Recommendation field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendation

`func (o *QueryResultRow) SetRecommendation(v map[string]interface{})`

SetRecommendation sets Recommendation field to given value.

### HasRecommendation

`func (o *QueryResultRow) HasRecommendation() bool`

HasRecommendation returns a boolean if a field has been set.

### GetRecommendationAction

`func (o *QueryResultRow) GetRecommendationAction() string`

GetRecommendationAction returns the RecommendationAction field if non-nil, zero value otherwise.

### GetRecommendationActionOk

`func (o *QueryResultRow) GetRecommendationActionOk() (*string, bool)`

GetRecommendationActionOk returns a tuple with the RecommendationAction field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecommendationAction

`func (o *QueryResultRow) SetRecommendationAction(v string)`

SetRecommendationAction sets RecommendationAction field to given value.

### HasRecommendationAction

`func (o *QueryResultRow) HasRecommendationAction() bool`

HasRecommendationAction returns a boolean if a field has been set.

### SetRecommendationActionNil

`func (o *QueryResultRow) SetRecommendationActionNil()`

 SetRecommendationActionNil sets the value for RecommendationAction to be an explicit nil

### UnsetRecommendationAction
`func (o *QueryResultRow) UnsetRecommendationAction()`

UnsetRecommendationAction ensures that no value is present for RecommendationAction, not even an explicit nil

### GetSafeToSend

`func (o *QueryResultRow) GetSafeToSend() bool`

GetSafeToSend returns the SafeToSend field if non-nil, zero value otherwise.

### GetSafeToSendOk

`func (o *QueryResultRow) GetSafeToSendOk() (*bool, bool)`

GetSafeToSendOk returns a tuple with the SafeToSend field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSend

`func (o *QueryResultRow) SetSafeToSend(v bool)`

SetSafeToSend sets SafeToSend field to given value.

### HasSafeToSend

`func (o *QueryResultRow) HasSafeToSend() bool`

HasSafeToSend returns a boolean if a field has been set.

### SetSafeToSendNil

`func (o *QueryResultRow) SetSafeToSendNil()`

 SetSafeToSendNil sets the value for SafeToSend to be an explicit nil

### UnsetSafeToSend
`func (o *QueryResultRow) UnsetSafeToSend()`

UnsetSafeToSend ensures that no value is present for SafeToSend, not even an explicit nil

### GetScore

`func (o *QueryResultRow) GetScore() int32`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *QueryResultRow) GetScoreOk() (*int32, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *QueryResultRow) SetScore(v int32)`

SetScore sets Score field to given value.

### HasScore

`func (o *QueryResultRow) HasScore() bool`

HasScore returns a boolean if a field has been set.

### SetScoreNil

`func (o *QueryResultRow) SetScoreNil()`

 SetScoreNil sets the value for Score to be an explicit nil

### UnsetScore
`func (o *QueryResultRow) UnsetScore()`

UnsetScore ensures that no value is present for Score, not even an explicit nil

### GetSubReason

`func (o *QueryResultRow) GetSubReason() string`

GetSubReason returns the SubReason field if non-nil, zero value otherwise.

### GetSubReasonOk

`func (o *QueryResultRow) GetSubReasonOk() (*string, bool)`

GetSubReasonOk returns a tuple with the SubReason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubReason

`func (o *QueryResultRow) SetSubReason(v string)`

SetSubReason sets SubReason field to given value.

### HasSubReason

`func (o *QueryResultRow) HasSubReason() bool`

HasSubReason returns a boolean if a field has been set.

### SetSubReasonNil

`func (o *QueryResultRow) SetSubReasonNil()`

 SetSubReasonNil sets the value for SubReason to be an explicit nil

### UnsetSubReason
`func (o *QueryResultRow) UnsetSubReason()`

UnsetSubReason ensures that no value is present for SubReason, not even an explicit nil

### GetTaskState

`func (o *QueryResultRow) GetTaskState() string`

GetTaskState returns the TaskState field if non-nil, zero value otherwise.

### GetTaskStateOk

`func (o *QueryResultRow) GetTaskStateOk() (*string, bool)`

GetTaskStateOk returns a tuple with the TaskState field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTaskState

`func (o *QueryResultRow) SetTaskState(v string)`

SetTaskState sets TaskState field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
