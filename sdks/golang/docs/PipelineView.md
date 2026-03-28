# PipelineView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | **time.Time** |  | 
**Delivery** | [**PipelineDeliveryConfig**](PipelineDeliveryConfig.md) |  | 
**Id** | **int64** |  | 
**LastRunId** | Pointer to **NullableInt64** |  | [optional] 
**LastScheduledAt** | Pointer to **NullableTime** |  | [optional] 
**Name** | **string** |  | 
**NextRunAt** | Pointer to **NullableTime** |  | [optional] 
**Policy** | [**PipelinePolicyConfig**](PipelinePolicyConfig.md) |  | 
**Schedule** | [**PipelineSchedule**](PipelineSchedule.md) |  | 
**Source** | [**PipelineSource**](PipelineSource.md) |  | 
**Status** | [**PipelineStatus**](PipelineStatus.md) |  | 
**TenantId** | **string** |  | 
**UpdatedAt** | **time.Time** |  | 
**Verification** | [**PipelineVerificationSettings**](PipelineVerificationSettings.md) |  | 

## Methods

### NewPipelineView

`func NewPipelineView(createdAt time.Time, delivery PipelineDeliveryConfig, id int64, name string, policy PipelinePolicyConfig, schedule PipelineSchedule, source PipelineSource, status PipelineStatus, tenantId string, updatedAt time.Time, verification PipelineVerificationSettings, ) *PipelineView`

NewPipelineView instantiates a new PipelineView object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineViewWithDefaults

`func NewPipelineViewWithDefaults() *PipelineView`

NewPipelineViewWithDefaults instantiates a new PipelineView object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *PipelineView) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *PipelineView) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *PipelineView) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetDelivery

`func (o *PipelineView) GetDelivery() PipelineDeliveryConfig`

GetDelivery returns the Delivery field if non-nil, zero value otherwise.

### GetDeliveryOk

`func (o *PipelineView) GetDeliveryOk() (*PipelineDeliveryConfig, bool)`

GetDeliveryOk returns a tuple with the Delivery field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDelivery

`func (o *PipelineView) SetDelivery(v PipelineDeliveryConfig)`

SetDelivery sets Delivery field to given value.


### GetId

`func (o *PipelineView) GetId() int64`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *PipelineView) GetIdOk() (*int64, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *PipelineView) SetId(v int64)`

SetId sets Id field to given value.


### GetLastRunId

`func (o *PipelineView) GetLastRunId() int64`

GetLastRunId returns the LastRunId field if non-nil, zero value otherwise.

### GetLastRunIdOk

`func (o *PipelineView) GetLastRunIdOk() (*int64, bool)`

GetLastRunIdOk returns a tuple with the LastRunId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastRunId

`func (o *PipelineView) SetLastRunId(v int64)`

SetLastRunId sets LastRunId field to given value.

### HasLastRunId

`func (o *PipelineView) HasLastRunId() bool`

HasLastRunId returns a boolean if a field has been set.

### SetLastRunIdNil

`func (o *PipelineView) SetLastRunIdNil(b bool)`

 SetLastRunIdNil sets the value for LastRunId to be an explicit nil

### UnsetLastRunId
`func (o *PipelineView) UnsetLastRunId()`

UnsetLastRunId ensures that no value is present for LastRunId, not even an explicit nil
### GetLastScheduledAt

`func (o *PipelineView) GetLastScheduledAt() time.Time`

GetLastScheduledAt returns the LastScheduledAt field if non-nil, zero value otherwise.

### GetLastScheduledAtOk

`func (o *PipelineView) GetLastScheduledAtOk() (*time.Time, bool)`

GetLastScheduledAtOk returns a tuple with the LastScheduledAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastScheduledAt

`func (o *PipelineView) SetLastScheduledAt(v time.Time)`

SetLastScheduledAt sets LastScheduledAt field to given value.

### HasLastScheduledAt

`func (o *PipelineView) HasLastScheduledAt() bool`

HasLastScheduledAt returns a boolean if a field has been set.

### SetLastScheduledAtNil

`func (o *PipelineView) SetLastScheduledAtNil(b bool)`

 SetLastScheduledAtNil sets the value for LastScheduledAt to be an explicit nil

### UnsetLastScheduledAt
`func (o *PipelineView) UnsetLastScheduledAt()`

UnsetLastScheduledAt ensures that no value is present for LastScheduledAt, not even an explicit nil
### GetName

`func (o *PipelineView) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *PipelineView) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *PipelineView) SetName(v string)`

SetName sets Name field to given value.


### GetNextRunAt

`func (o *PipelineView) GetNextRunAt() time.Time`

GetNextRunAt returns the NextRunAt field if non-nil, zero value otherwise.

### GetNextRunAtOk

`func (o *PipelineView) GetNextRunAtOk() (*time.Time, bool)`

GetNextRunAtOk returns a tuple with the NextRunAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextRunAt

`func (o *PipelineView) SetNextRunAt(v time.Time)`

SetNextRunAt sets NextRunAt field to given value.

### HasNextRunAt

`func (o *PipelineView) HasNextRunAt() bool`

HasNextRunAt returns a boolean if a field has been set.

### SetNextRunAtNil

`func (o *PipelineView) SetNextRunAtNil(b bool)`

 SetNextRunAtNil sets the value for NextRunAt to be an explicit nil

### UnsetNextRunAt
`func (o *PipelineView) UnsetNextRunAt()`

UnsetNextRunAt ensures that no value is present for NextRunAt, not even an explicit nil
### GetPolicy

`func (o *PipelineView) GetPolicy() PipelinePolicyConfig`

GetPolicy returns the Policy field if non-nil, zero value otherwise.

### GetPolicyOk

`func (o *PipelineView) GetPolicyOk() (*PipelinePolicyConfig, bool)`

GetPolicyOk returns a tuple with the Policy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicy

`func (o *PipelineView) SetPolicy(v PipelinePolicyConfig)`

SetPolicy sets Policy field to given value.


### GetSchedule

`func (o *PipelineView) GetSchedule() PipelineSchedule`

GetSchedule returns the Schedule field if non-nil, zero value otherwise.

### GetScheduleOk

`func (o *PipelineView) GetScheduleOk() (*PipelineSchedule, bool)`

GetScheduleOk returns a tuple with the Schedule field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSchedule

`func (o *PipelineView) SetSchedule(v PipelineSchedule)`

SetSchedule sets Schedule field to given value.


### GetSource

`func (o *PipelineView) GetSource() PipelineSource`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *PipelineView) GetSourceOk() (*PipelineSource, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *PipelineView) SetSource(v PipelineSource)`

SetSource sets Source field to given value.


### GetStatus

`func (o *PipelineView) GetStatus() PipelineStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *PipelineView) GetStatusOk() (*PipelineStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *PipelineView) SetStatus(v PipelineStatus)`

SetStatus sets Status field to given value.


### GetTenantId

`func (o *PipelineView) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *PipelineView) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *PipelineView) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.


### GetUpdatedAt

`func (o *PipelineView) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *PipelineView) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *PipelineView) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.


### GetVerification

`func (o *PipelineView) GetVerification() PipelineVerificationSettings`

GetVerification returns the Verification field if non-nil, zero value otherwise.

### GetVerificationOk

`func (o *PipelineView) GetVerificationOk() (*PipelineVerificationSettings, bool)`

GetVerificationOk returns a tuple with the Verification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVerification

`func (o *PipelineView) SetVerification(v PipelineVerificationSettings)`

SetVerification sets Verification field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


