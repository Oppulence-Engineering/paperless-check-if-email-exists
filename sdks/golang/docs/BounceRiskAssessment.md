# BounceRiskAssessment

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Action** | [**RecommendedAction**](RecommendedAction.md) |  | 
**Category** | [**BounceRiskCategory**](BounceRiskCategory.md) |  | 
**Confidence** | **float64** |  | 
**ModelVersion** | **string** |  | 
**RiskFactors** | [**[]RiskFactor**](RiskFactor.md) |  | 
**Score** | **int32** |  | 
**ScoredAt** | **time.Time** |  | 

## Methods

### NewBounceRiskAssessment

`func NewBounceRiskAssessment(action RecommendedAction, category BounceRiskCategory, confidence float64, modelVersion string, riskFactors []RiskFactor, score int32, scoredAt time.Time, ) *BounceRiskAssessment`

NewBounceRiskAssessment instantiates a new BounceRiskAssessment object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBounceRiskAssessmentWithDefaults

`func NewBounceRiskAssessmentWithDefaults() *BounceRiskAssessment`

NewBounceRiskAssessmentWithDefaults instantiates a new BounceRiskAssessment object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAction

`func (o *BounceRiskAssessment) GetAction() RecommendedAction`

GetAction returns the Action field if non-nil, zero value otherwise.

### GetActionOk

`func (o *BounceRiskAssessment) GetActionOk() (*RecommendedAction, bool)`

GetActionOk returns a tuple with the Action field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAction

`func (o *BounceRiskAssessment) SetAction(v RecommendedAction)`

SetAction sets Action field to given value.


### GetCategory

`func (o *BounceRiskAssessment) GetCategory() BounceRiskCategory`

GetCategory returns the Category field if non-nil, zero value otherwise.

### GetCategoryOk

`func (o *BounceRiskAssessment) GetCategoryOk() (*BounceRiskCategory, bool)`

GetCategoryOk returns a tuple with the Category field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCategory

`func (o *BounceRiskAssessment) SetCategory(v BounceRiskCategory)`

SetCategory sets Category field to given value.


### GetConfidence

`func (o *BounceRiskAssessment) GetConfidence() float64`

GetConfidence returns the Confidence field if non-nil, zero value otherwise.

### GetConfidenceOk

`func (o *BounceRiskAssessment) GetConfidenceOk() (*float64, bool)`

GetConfidenceOk returns a tuple with the Confidence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfidence

`func (o *BounceRiskAssessment) SetConfidence(v float64)`

SetConfidence sets Confidence field to given value.


### GetModelVersion

`func (o *BounceRiskAssessment) GetModelVersion() string`

GetModelVersion returns the ModelVersion field if non-nil, zero value otherwise.

### GetModelVersionOk

`func (o *BounceRiskAssessment) GetModelVersionOk() (*string, bool)`

GetModelVersionOk returns a tuple with the ModelVersion field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetModelVersion

`func (o *BounceRiskAssessment) SetModelVersion(v string)`

SetModelVersion sets ModelVersion field to given value.


### GetRiskFactors

`func (o *BounceRiskAssessment) GetRiskFactors() []RiskFactor`

GetRiskFactors returns the RiskFactors field if non-nil, zero value otherwise.

### GetRiskFactorsOk

`func (o *BounceRiskAssessment) GetRiskFactorsOk() (*[]RiskFactor, bool)`

GetRiskFactorsOk returns a tuple with the RiskFactors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRiskFactors

`func (o *BounceRiskAssessment) SetRiskFactors(v []RiskFactor)`

SetRiskFactors sets RiskFactors field to given value.


### GetScore

`func (o *BounceRiskAssessment) GetScore() int32`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *BounceRiskAssessment) GetScoreOk() (*int32, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *BounceRiskAssessment) SetScore(v int32)`

SetScore sets Score field to given value.


### GetScoredAt

`func (o *BounceRiskAssessment) GetScoredAt() time.Time`

GetScoredAt returns the ScoredAt field if non-nil, zero value otherwise.

### GetScoredAtOk

`func (o *BounceRiskAssessment) GetScoredAtOk() (*time.Time, bool)`

GetScoredAtOk returns a tuple with the ScoredAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScoredAt

`func (o *BounceRiskAssessment) SetScoredAt(v time.Time)`

SetScoredAt sets ScoredAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


