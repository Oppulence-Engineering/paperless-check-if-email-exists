# PipelineVerificationSettings

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**DeltaMode** | Pointer to **bool** |  | [optional]
**FreshnessDays** | Pointer to **NullableInt32** |  | [optional]

## Methods

### NewPipelineVerificationSettings

`func NewPipelineVerificationSettings() *PipelineVerificationSettings`

NewPipelineVerificationSettings instantiates a new PipelineVerificationSettings object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineVerificationSettingsWithDefaults

`func NewPipelineVerificationSettingsWithDefaults() *PipelineVerificationSettings`

NewPipelineVerificationSettingsWithDefaults instantiates a new PipelineVerificationSettings object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDeltaMode

`func (o *PipelineVerificationSettings) GetDeltaMode() bool`

GetDeltaMode returns the DeltaMode field if non-nil, zero value otherwise.

### GetDeltaModeOk

`func (o *PipelineVerificationSettings) GetDeltaModeOk() (*bool, bool)`

GetDeltaModeOk returns a tuple with the DeltaMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeltaMode

`func (o *PipelineVerificationSettings) SetDeltaMode(v bool)`

SetDeltaMode sets DeltaMode field to given value.

### HasDeltaMode

`func (o *PipelineVerificationSettings) HasDeltaMode() bool`

HasDeltaMode returns a boolean if a field has been set.

### GetFreshnessDays

`func (o *PipelineVerificationSettings) GetFreshnessDays() int32`

GetFreshnessDays returns the FreshnessDays field if non-nil, zero value otherwise.

### GetFreshnessDaysOk

`func (o *PipelineVerificationSettings) GetFreshnessDaysOk() (*int32, bool)`

GetFreshnessDaysOk returns a tuple with the FreshnessDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFreshnessDays

`func (o *PipelineVerificationSettings) SetFreshnessDays(v int32)`

SetFreshnessDays sets FreshnessDays field to given value.

### HasFreshnessDays

`func (o *PipelineVerificationSettings) HasFreshnessDays() bool`

HasFreshnessDays returns a boolean if a field has been set.

### SetFreshnessDaysNil

`func (o *PipelineVerificationSettings) SetFreshnessDaysNil()`

 SetFreshnessDaysNil sets the value for FreshnessDays to be an explicit nil

### UnsetFreshnessDays
`func (o *PipelineVerificationSettings) UnsetFreshnessDays()`

UnsetFreshnessDays ensures that no value is present for FreshnessDays, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
