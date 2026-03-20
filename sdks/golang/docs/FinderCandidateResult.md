# FinderCandidateResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Category** | [**EmailCategory**](EmailCategory.md) |  | 
**Confidence** | Pointer to [**NullableConfidenceExplanation**](ConfidenceExplanation.md) |  | [optional] 
**Email** | **string** |  | 
**IsReachable** | [**Reachable**](Reachable.md) |  | 
**Pattern** | **string** |  | 
**Result** | Pointer to [**NullableCheckEmailOutput**](CheckEmailOutput.md) |  | [optional] 
**Score** | **int32** |  | 
**SubReason** | [**SubReason**](SubReason.md) |  | 

## Methods

### NewFinderCandidateResult

`func NewFinderCandidateResult(category EmailCategory, email string, isReachable Reachable, pattern string, score int32, subReason SubReason, ) *FinderCandidateResult`

NewFinderCandidateResult instantiates a new FinderCandidateResult object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewFinderCandidateResultWithDefaults

`func NewFinderCandidateResultWithDefaults() *FinderCandidateResult`

NewFinderCandidateResultWithDefaults instantiates a new FinderCandidateResult object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCategory

`func (o *FinderCandidateResult) GetCategory() EmailCategory`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *FinderCandidateResult) GetCategoryOk() (*EmailCategory, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *FinderCandidateResult) SetCategory(v EmailCategory)`

SetCategory sets Category field to given value.


### GetConfidence

`func (o *FinderCandidateResult) GetConfidence() ConfidenceExplanation`

GetConfidence returns the Confidence field if non-nil, zero value otherwise.

### GetConfidenceOk

`func (o *FinderCandidateResult) GetConfidenceOk() (*ConfidenceExplanation, bool)`

GetConfidenceOk returns a tuple with the Confidence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfidence

`func (o *FinderCandidateResult) SetConfidence(v ConfidenceExplanation)`

SetConfidence sets Confidence field to given value.

### HasConfidence

`func (o *FinderCandidateResult) HasConfidence() bool`

HasConfidence returns a boolean if a field has been set.

### SetConfidenceNil

`func (o *FinderCandidateResult) SetConfidenceNil(b bool)`

 SetConfidenceNil sets the value for Confidence to be an explicit nil

### UnsetConfidence
`func (o *FinderCandidateResult) UnsetConfidence()`

UnsetConfidence ensures that no value is present for Confidence, not even an explicit nil
### GetEmail

`func (o *FinderCandidateResult) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *FinderCandidateResult) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *FinderCandidateResult) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetIsReachable

`func (o *FinderCandidateResult) GetIsReachable() Reachable`

GetIsReachable returns the IsReachable field if non-nil, zero value otherwise.

### GetIsReachableOk

`func (o *FinderCandidateResult) GetIsReachableOk() (*Reachable, bool)`

GetIsReachableOk returns a tuple with the IsReachable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsReachable

`func (o *FinderCandidateResult) SetIsReachable(v Reachable)`

SetIsReachable sets IsReachable field to given value.


### GetPattern

`func (o *FinderCandidateResult) GetPattern() string`

GetPattern returns the Pattern field if non-nil, zero value otherwise.

### GetPatternOk

`func (o *FinderCandidateResult) GetPatternOk() (*string, bool)`

GetPatternOk returns a tuple with the Pattern field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPattern

`func (o *FinderCandidateResult) SetPattern(v string)`

SetPattern sets Pattern field to given value.


### GetResult

`func (o *FinderCandidateResult) GetResult() CheckEmailOutput`

GetResult returns the Result field if non-nil, zero value otherwise.

### GetResultOk

`func (o *FinderCandidateResult) GetResultOk() (*CheckEmailOutput, bool)`

GetResultOk returns a tuple with the Result field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResult

`func (o *FinderCandidateResult) SetResult(v CheckEmailOutput)`

SetResult sets Result field to given value.

### HasResult

`func (o *FinderCandidateResult) HasResult() bool`

HasResult returns a boolean if a field has been set.

### SetResultNil

`func (o *FinderCandidateResult) SetResultNil(b bool)`

 SetResultNil sets the value for Result to be an explicit nil

### UnsetResult
`func (o *FinderCandidateResult) UnsetResult()`

UnsetResult ensures that no value is present for Result, not even an explicit nil
### GetScore

`func (o *FinderCandidateResult) GetScore() int32`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *FinderCandidateResult) GetScoreOk() (*int32, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *FinderCandidateResult) SetScore(v int32)`

SetScore sets Score field to given value.


### GetSubReason

`func (o *FinderCandidateResult) GetSubReason() SubReason`

GetSubReason returns the SubReason field if non-nil, zero value otherwise.

### GetSubReasonOk

`func (o *FinderCandidateResult) GetSubReasonOk() (*SubReason, bool)`

GetSubReasonOk returns a tuple with the SubReason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSubReason

`func (o *FinderCandidateResult) SetSubReason(v SubReason)`

SetSubReason sets SubReason field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


