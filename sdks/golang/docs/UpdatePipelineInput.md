# UpdatePipelineInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Delivery** | Pointer to [**NullablePipelineDeliveryConfig**](PipelineDeliveryConfig.md) |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Policy** | Pointer to [**NullablePipelinePolicyConfig**](PipelinePolicyConfig.md) |  | [optional]
**Schedule** | Pointer to [**NullablePipelineSchedule**](PipelineSchedule.md) |  | [optional]
**Source** | Pointer to [**NullablePipelineSource**](PipelineSource.md) |  | [optional]
**Status** | Pointer to [**NullablePipelineStatus**](PipelineStatus.md) |  | [optional]
**Verification** | Pointer to [**NullablePipelineVerificationSettings**](PipelineVerificationSettings.md) |  | [optional]

## Methods

### NewUpdatePipelineInput

`func NewUpdatePipelineInput() *UpdatePipelineInput`

NewUpdatePipelineInput instantiates a new UpdatePipelineInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdatePipelineInputWithDefaults

`func NewUpdatePipelineInputWithDefaults() *UpdatePipelineInput`

NewUpdatePipelineInputWithDefaults instantiates a new UpdatePipelineInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDelivery

`func (o *UpdatePipelineInput) GetDelivery() PipelineDeliveryConfig`

GetDelivery returns the Delivery field if non-nil, zero value otherwise.

### GetDeliveryOk

`func (o *UpdatePipelineInput) GetDeliveryOk() (*PipelineDeliveryConfig, bool)`

GetDeliveryOk returns a tuple with the Delivery field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDelivery

`func (o *UpdatePipelineInput) SetDelivery(v PipelineDeliveryConfig)`

SetDelivery sets Delivery field to given value.

### HasDelivery

`func (o *UpdatePipelineInput) HasDelivery() bool`

HasDelivery returns a boolean if a field has been set.

### SetDeliveryNil

`func (o *UpdatePipelineInput) SetDeliveryNil()`

 SetDeliveryNil sets the value for Delivery to be an explicit nil

### UnsetDelivery
`func (o *UpdatePipelineInput) UnsetDelivery()`

UnsetDelivery ensures that no value is present for Delivery, not even an explicit nil

### GetName

`func (o *UpdatePipelineInput) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdatePipelineInput) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdatePipelineInput) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *UpdatePipelineInput) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdatePipelineInput) SetNameNil()`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdatePipelineInput) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil

### GetPolicy

`func (o *UpdatePipelineInput) GetPolicy() PipelinePolicyConfig`

GetPolicy returns the Policy field if non-nil, zero value otherwise.

### GetPolicyOk

`func (o *UpdatePipelineInput) GetPolicyOk() (*PipelinePolicyConfig, bool)`

GetPolicyOk returns a tuple with the Policy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicy

`func (o *UpdatePipelineInput) SetPolicy(v PipelinePolicyConfig)`

SetPolicy sets Policy field to given value.

### HasPolicy

`func (o *UpdatePipelineInput) HasPolicy() bool`

HasPolicy returns a boolean if a field has been set.

### SetPolicyNil

`func (o *UpdatePipelineInput) SetPolicyNil()`

 SetPolicyNil sets the value for Policy to be an explicit nil

### UnsetPolicy
`func (o *UpdatePipelineInput) UnsetPolicy()`

UnsetPolicy ensures that no value is present for Policy, not even an explicit nil

### GetSchedule

`func (o *UpdatePipelineInput) GetSchedule() PipelineSchedule`

GetSchedule returns the Schedule field if non-nil, zero value otherwise.

### GetScheduleOk

`func (o *UpdatePipelineInput) GetScheduleOk() (*PipelineSchedule, bool)`

GetScheduleOk returns a tuple with the Schedule field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedule

`func (o *UpdatePipelineInput) SetSchedule(v PipelineSchedule)`

SetSchedule sets Schedule field to given value.

### HasSchedule

`func (o *UpdatePipelineInput) HasSchedule() bool`

HasSchedule returns a boolean if a field has been set.

### SetScheduleNil

`func (o *UpdatePipelineInput) SetScheduleNil()`

 SetScheduleNil sets the value for Schedule to be an explicit nil

### UnsetSchedule
`func (o *UpdatePipelineInput) UnsetSchedule()`

UnsetSchedule ensures that no value is present for Schedule, not even an explicit nil

### GetSource

`func (o *UpdatePipelineInput) GetSource() PipelineSource`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *UpdatePipelineInput) GetSourceOk() (*PipelineSource, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *UpdatePipelineInput) SetSource(v PipelineSource)`

SetSource sets Source field to given value.

### HasSource

`func (o *UpdatePipelineInput) HasSource() bool`

HasSource returns a boolean if a field has been set.

### SetSourceNil

`func (o *UpdatePipelineInput) SetSourceNil()`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *UpdatePipelineInput) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

### GetStatus

`func (o *UpdatePipelineInput) GetStatus() PipelineStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *UpdatePipelineInput) GetStatusOk() (*PipelineStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *UpdatePipelineInput) SetStatus(v PipelineStatus)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *UpdatePipelineInput) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### SetStatusNil

`func (o *UpdatePipelineInput) SetStatusNil()`

 SetStatusNil sets the value for Status to be an explicit nil

### UnsetStatus
`func (o *UpdatePipelineInput) UnsetStatus()`

UnsetStatus ensures that no value is present for Status, not even an explicit nil

### GetVerification

`func (o *UpdatePipelineInput) GetVerification() PipelineVerificationSettings`

GetVerification returns the Verification field if non-nil, zero value otherwise.

### GetVerificationOk

`func (o *UpdatePipelineInput) GetVerificationOk() (*PipelineVerificationSettings, bool)`

GetVerificationOk returns a tuple with the Verification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVerification

`func (o *UpdatePipelineInput) SetVerification(v PipelineVerificationSettings)`

SetVerification sets Verification field to given value.

### HasVerification

`func (o *UpdatePipelineInput) HasVerification() bool`

HasVerification returns a boolean if a field has been set.

### SetVerificationNil

`func (o *UpdatePipelineInput) SetVerificationNil()`

 SetVerificationNil sets the value for Verification to be an explicit nil

### UnsetVerification
`func (o *UpdatePipelineInput) UnsetVerification()`

UnsetVerification ensures that no value is present for Verification, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
