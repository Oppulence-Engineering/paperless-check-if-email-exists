# ConfidenceExplanation

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Factors** | **[]string** |  | [required]
**Level** | **string** |  | [required]
**Score** | **int32** |  | [required]

## Methods

### NewConfidenceExplanation

`func NewConfidenceExplanation(factors []string, level string, score int32) *ConfidenceExplanation`

NewConfidenceExplanation instantiates a new ConfidenceExplanation object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewConfidenceExplanationWithDefaults

`func NewConfidenceExplanationWithDefaults() *ConfidenceExplanation`

NewConfidenceExplanationWithDefaults instantiates a new ConfidenceExplanation object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFactors

`func (o *ConfidenceExplanation) GetFactors() []string`

GetFactors returns the Factors field if non-nil, zero value otherwise.

### GetFactorsOk

`func (o *ConfidenceExplanation) GetFactorsOk() ([]string, bool)`

GetFactorsOk returns a tuple with the Factors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFactors

`func (o *ConfidenceExplanation) SetFactors(v []string)`

SetFactors sets Factors field to given value.


### GetLevel

`func (o *ConfidenceExplanation) GetLevel() string`

GetLevel returns the Level field if non-nil, zero value otherwise.

### GetLevelOk

`func (o *ConfidenceExplanation) GetLevelOk() (*string, bool)`

GetLevelOk returns a tuple with the Level field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLevel

`func (o *ConfidenceExplanation) SetLevel(v string)`

SetLevel sets Level field to given value.


### GetScore

`func (o *ConfidenceExplanation) GetScore() int32`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *ConfidenceExplanation) GetScoreOk() (*int32, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *ConfidenceExplanation) SetScore(v int32)`

SetScore sets Score field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
