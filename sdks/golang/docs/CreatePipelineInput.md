# CreatePipelineInput

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Delivery** | Pointer to [**PipelineDeliveryConfig**](PipelineDeliveryConfig.md) |  | [optional] 
**Name** | **string** |  | 
**Policy** | Pointer to [**PipelinePolicyConfig**](PipelinePolicyConfig.md) |  | [optional] 
**Schedule** | [**PipelineSchedule**](PipelineSchedule.md) |  | 
**Source** | [**PipelineSource**](PipelineSource.md) |  | 
**Status** | Pointer to [**PipelineStatus**](PipelineStatus.md) |  | [optional] 
**Verification** | Pointer to [**PipelineVerificationSettings**](PipelineVerificationSettings.md) |  | [optional] 

## Methods

### NewCreatePipelineInput

`func NewCreatePipelineInput(name string, schedule PipelineSchedule, source PipelineSource, ) *CreatePipelineInput`

NewCreatePipelineInput instantiates a new CreatePipelineInput object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreatePipelineInputWithDefaults

`func NewCreatePipelineInputWithDefaults() *CreatePipelineInput`

NewCreatePipelineInputWithDefaults instantiates a new CreatePipelineInput object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDelivery

`func (o *CreatePipelineInput) GetDelivery() PipelineDeliveryConfig`

GetDelivery returns the Delivery field if non-nil, zero value otherwise.

### GetDeliveryOk

`func (o *CreatePipelineInput) GetDeliveryOk() (*PipelineDeliveryConfig, bool)`

GetDeliveryOk returns a tuple with the Delivery field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDelivery

`func (o *CreatePipelineInput) SetDelivery(v PipelineDeliveryConfig)`

SetDelivery sets Delivery field to given value.

### HasDelivery

`func (o *CreatePipelineInput) HasDelivery() bool`

HasDelivery returns a boolean if a field has been set.

### GetName

`func (o *CreatePipelineInput) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreatePipelineInput) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreatePipelineInput) SetName(v string)`

SetName sets Name field to given value.


### GetPolicy

`func (o *CreatePipelineInput) GetPolicy() PipelinePolicyConfig`

GetPolicy returns the Policy field if non-nil, zero value otherwise.

### GetPolicyOk

`func (o *CreatePipelineInput) GetPolicyOk() (*PipelinePolicyConfig, bool)`

GetPolicyOk returns a tuple with the Policy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicy

`func (o *CreatePipelineInput) SetPolicy(v PipelinePolicyConfig)`

SetPolicy sets Policy field to given value.

### HasPolicy

`func (o *CreatePipelineInput) HasPolicy() bool`

HasPolicy returns a boolean if a field has been set.

### GetSchedule

`func (o *CreatePipelineInput) GetSchedule() PipelineSchedule`

GetSchedule returns the Schedule field if non-nil, zero value otherwise.

### GetScheduleOk

`func (o *CreatePipelineInput) GetScheduleOk() (*PipelineSchedule, bool)`

GetScheduleOk returns a tuple with the Schedule field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedule

`func (o *CreatePipelineInput) SetSchedule(v PipelineSchedule)`

SetSchedule sets Schedule field to given value.


### GetSource

`func (o *CreatePipelineInput) GetSource() PipelineSource`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *CreatePipelineInput) GetSourceOk() (*PipelineSource, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *CreatePipelineInput) SetSource(v PipelineSource)`

SetSource sets Source field to given value.


### GetStatus

`func (o *CreatePipelineInput) GetStatus() PipelineStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *CreatePipelineInput) GetStatusOk() (*PipelineStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *CreatePipelineInput) SetStatus(v PipelineStatus)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *CreatePipelineInput) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### GetVerification

`func (o *CreatePipelineInput) GetVerification() PipelineVerificationSettings`

GetVerification returns the Verification field if non-nil, zero value otherwise.

### GetVerificationOk

`func (o *CreatePipelineInput) GetVerificationOk() (*PipelineVerificationSettings, bool)`

GetVerificationOk returns a tuple with the Verification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVerification

`func (o *CreatePipelineInput) SetVerification(v PipelineVerificationSettings)`

SetVerification sets Verification field to given value.

### HasVerification

`func (o *CreatePipelineInput) HasVerification() bool`

HasVerification returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


