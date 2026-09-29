# PartialConfidence

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Classification** | [**SmtpUncertaintyClass**](SmtpUncertaintyClass.md) |  | [required]
**Confidence** | **int32** |  | [required]
**Factors** | **[]string** |  | [required]

## Methods

### NewPartialConfidence

`func NewPartialConfidence(classification SmtpUncertaintyClass, confidence int32, factors []string) *PartialConfidence`

NewPartialConfidence instantiates a new PartialConfidence object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPartialConfidenceWithDefaults

`func NewPartialConfidenceWithDefaults() *PartialConfidence`

NewPartialConfidenceWithDefaults instantiates a new PartialConfidence object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetClassification

`func (o *PartialConfidence) GetClassification() SmtpUncertaintyClass`

GetClassification returns the Classification field if non-nil, zero value otherwise.

### GetClassificationOk

`func (o *PartialConfidence) GetClassificationOk() (*SmtpUncertaintyClass, bool)`

GetClassificationOk returns a tuple with the Classification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClassification

`func (o *PartialConfidence) SetClassification(v SmtpUncertaintyClass)`

SetClassification sets Classification field to given value.


### GetConfidence

`func (o *PartialConfidence) GetConfidence() int32`

GetConfidence returns the Confidence field if non-nil, zero value otherwise.

### GetConfidenceOk

`func (o *PartialConfidence) GetConfidenceOk() (*int32, bool)`

GetConfidenceOk returns a tuple with the Confidence field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetConfidence

`func (o *PartialConfidence) SetConfidence(v int32)`

SetConfidence sets Confidence field to given value.


### GetFactors

`func (o *PartialConfidence) GetFactors() []string`

GetFactors returns the Factors field if non-nil, zero value otherwise.

### GetFactorsOk

`func (o *PartialConfidence) GetFactorsOk() ([]string, bool)`

GetFactorsOk returns a tuple with the Factors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFactors

`func (o *PartialConfidence) SetFactors(v []string)`

SetFactors sets Factors field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
