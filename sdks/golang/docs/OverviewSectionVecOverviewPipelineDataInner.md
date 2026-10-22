# OverviewSectionVecOverviewPipelineDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **int32** |  | [required]
**LastRunId** | Pointer to **NullableInt32** |  | [optional]
**LastRunStatus** | Pointer to **NullableString** |  | [optional]
**Name** | **string** |  | [required]
**NextRunAt** | Pointer to **NullableTime** |  | [optional]
**Status** | **string** |  | [required]

## Methods

### NewOverviewSectionVecOverviewPipelineDataInner

`func NewOverviewSectionVecOverviewPipelineDataInner(id int32, name string, status string) *OverviewSectionVecOverviewPipelineDataInner`

NewOverviewSectionVecOverviewPipelineDataInner instantiates a new OverviewSectionVecOverviewPipelineDataInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOverviewSectionVecOverviewPipelineDataInnerWithDefaults

`func NewOverviewSectionVecOverviewPipelineDataInnerWithDefaults() *OverviewSectionVecOverviewPipelineDataInner`

NewOverviewSectionVecOverviewPipelineDataInnerWithDefaults instantiates a new OverviewSectionVecOverviewPipelineDataInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetId() int32`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetIdOk() (*int32, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *OverviewSectionVecOverviewPipelineDataInner) SetId(v int32)`

SetId sets Id field to given value.


### GetLastRunId

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetLastRunId() int32`

GetLastRunId returns the LastRunId field if non-nil, zero value otherwise.

### GetLastRunIdOk

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetLastRunIdOk() (*int32, bool)`

GetLastRunIdOk returns a tuple with the LastRunId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastRunId

`func (o *OverviewSectionVecOverviewPipelineDataInner) SetLastRunId(v int32)`

SetLastRunId sets LastRunId field to given value.

### HasLastRunId

`func (o *OverviewSectionVecOverviewPipelineDataInner) HasLastRunId() bool`

HasLastRunId returns a boolean if a field has been set.

### SetLastRunIdNil

`func (o *OverviewSectionVecOverviewPipelineDataInner) SetLastRunIdNil()`

 SetLastRunIdNil sets the value for LastRunId to be an explicit nil

### UnsetLastRunId
`func (o *OverviewSectionVecOverviewPipelineDataInner) UnsetLastRunId()`

UnsetLastRunId ensures that no value is present for LastRunId, not even an explicit nil

### GetLastRunStatus

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetLastRunStatus() string`

GetLastRunStatus returns the LastRunStatus field if non-nil, zero value otherwise.

### GetLastRunStatusOk

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetLastRunStatusOk() (*string, bool)`

GetLastRunStatusOk returns a tuple with the LastRunStatus field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastRunStatus

`func (o *OverviewSectionVecOverviewPipelineDataInner) SetLastRunStatus(v string)`

SetLastRunStatus sets LastRunStatus field to given value.

### HasLastRunStatus

`func (o *OverviewSectionVecOverviewPipelineDataInner) HasLastRunStatus() bool`

HasLastRunStatus returns a boolean if a field has been set.

### SetLastRunStatusNil

`func (o *OverviewSectionVecOverviewPipelineDataInner) SetLastRunStatusNil()`

 SetLastRunStatusNil sets the value for LastRunStatus to be an explicit nil

### UnsetLastRunStatus
`func (o *OverviewSectionVecOverviewPipelineDataInner) UnsetLastRunStatus()`

UnsetLastRunStatus ensures that no value is present for LastRunStatus, not even an explicit nil

### GetName

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *OverviewSectionVecOverviewPipelineDataInner) SetName(v string)`

SetName sets Name field to given value.


### GetNextRunAt

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetNextRunAt() time.Time`

GetNextRunAt returns the NextRunAt field if non-nil, zero value otherwise.

### GetNextRunAtOk

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetNextRunAtOk() (*time.Time, bool)`

GetNextRunAtOk returns a tuple with the NextRunAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextRunAt

`func (o *OverviewSectionVecOverviewPipelineDataInner) SetNextRunAt(v time.Time)`

SetNextRunAt sets NextRunAt field to given value.

### HasNextRunAt

`func (o *OverviewSectionVecOverviewPipelineDataInner) HasNextRunAt() bool`

HasNextRunAt returns a boolean if a field has been set.

### SetNextRunAtNil

`func (o *OverviewSectionVecOverviewPipelineDataInner) SetNextRunAtNil()`

 SetNextRunAtNil sets the value for NextRunAt to be an explicit nil

### UnsetNextRunAt
`func (o *OverviewSectionVecOverviewPipelineDataInner) UnsetNextRunAt()`

UnsetNextRunAt ensures that no value is present for NextRunAt, not even an explicit nil

### GetStatus

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *OverviewSectionVecOverviewPipelineDataInner) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *OverviewSectionVecOverviewPipelineDataInner) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
