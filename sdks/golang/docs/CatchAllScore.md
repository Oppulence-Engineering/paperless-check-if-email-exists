# CatchAllScore

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Confidence** | **int32** |  | [required]
**Factors** | **[]string** |  | [required]
**Severity** | [**CatchAllSeverity**](CatchAllSeverity.md) |  | [required]

## Methods

### NewCatchAllScore

`func NewCatchAllScore(confidence int32, factors []string, severity CatchAllSeverity) *CatchAllScore`

NewCatchAllScore instantiates a new CatchAllScore object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCatchAllScoreWithDefaults

`func NewCatchAllScoreWithDefaults() *CatchAllScore`

NewCatchAllScoreWithDefaults instantiates a new CatchAllScore object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetConfidence

`func (o *CatchAllScore) GetConfidence() int32`

GetConfidence returns the Confidence field if non-nil, zero value otherwise.

### GetConfidenceOk

`func (o *CatchAllScore) GetConfidenceOk() (*int32, bool)`

GetConfidenceOk returns a tuple with the Confidence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfidence

`func (o *CatchAllScore) SetConfidence(v int32)`

SetConfidence sets Confidence field to given value.


### GetFactors

`func (o *CatchAllScore) GetFactors() []string`

GetFactors returns the Factors field if non-nil, zero value otherwise.

### GetFactorsOk

`func (o *CatchAllScore) GetFactorsOk() ([]string, bool)`

GetFactorsOk returns a tuple with the Factors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFactors

`func (o *CatchAllScore) SetFactors(v []string)`

SetFactors sets Factors field to given value.


### GetSeverity

`func (o *CatchAllScore) GetSeverity() CatchAllSeverity`

GetSeverity returns the Severity field if non-nil, zero value otherwise.

### GetSeverityOk

`func (o *CatchAllScore) GetSeverityOk() (*CatchAllSeverity, bool)`

GetSeverityOk returns a tuple with the Severity field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSeverity

`func (o *CatchAllScore) SetSeverity(v CatchAllSeverity)`

SetSeverity sets Severity field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
