# RiskFactor

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Contribution** | **int32** |  | [required]
**Description** | **string** |  | [required]
**Direction** | [**RiskDirection**](RiskDirection.md) |  | [required]
**Signal** | **string** |  | [required]

## Methods

### NewRiskFactor

`func NewRiskFactor(contribution int32, description string, direction RiskDirection, signal string) *RiskFactor`

NewRiskFactor instantiates a new RiskFactor object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRiskFactorWithDefaults

`func NewRiskFactorWithDefaults() *RiskFactor`

NewRiskFactorWithDefaults instantiates a new RiskFactor object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetContribution

`func (o *RiskFactor) GetContribution() int32`

GetContribution returns the Contribution field if non-nil, zero value otherwise.

### GetContributionOk

`func (o *RiskFactor) GetContributionOk() (*int32, bool)`

GetContributionOk returns a tuple with the Contribution field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContribution

`func (o *RiskFactor) SetContribution(v int32)`

SetContribution sets Contribution field to given value.


### GetDescription

`func (o *RiskFactor) GetDescription() string`

GetDescription returns the Description field if non-nil, zero value otherwise.

### GetDescriptionOk

`func (o *RiskFactor) GetDescriptionOk() (*string, bool)`

GetDescriptionOk returns a tuple with the Description field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDescription

`func (o *RiskFactor) SetDescription(v string)`

SetDescription sets Description field to given value.


### GetDirection

`func (o *RiskFactor) GetDirection() RiskDirection`

GetDirection returns the Direction field if non-nil, zero value otherwise.

### GetDirectionOk

`func (o *RiskFactor) GetDirectionOk() (*RiskDirection, bool)`

GetDirectionOk returns a tuple with the Direction field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDirection

`func (o *RiskFactor) SetDirection(v RiskDirection)`

SetDirection sets Direction field to given value.


### GetSignal

`func (o *RiskFactor) GetSignal() string`

GetSignal returns the Signal field if non-nil, zero value otherwise.

### GetSignalOk

`func (o *RiskFactor) GetSignalOk() (*string, bool)`

GetSignalOk returns a tuple with the Signal field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSignal

`func (o *RiskFactor) SetSignal(v string)`

SetSignal sets Signal field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
