# PipelineSchedule

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Cron** | **string** |  | 
**Timezone** | **string** |  | 

## Methods

### NewPipelineSchedule

`func NewPipelineSchedule(cron string, timezone string, ) *PipelineSchedule`

NewPipelineSchedule instantiates a new PipelineSchedule object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineScheduleWithDefaults

`func NewPipelineScheduleWithDefaults() *PipelineSchedule`

NewPipelineScheduleWithDefaults instantiates a new PipelineSchedule object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCron

`func (o *PipelineSchedule) GetCron() string`

GetCron returns the Cron field if non-nil, zero value otherwise.

### GetCronOk

`func (o *PipelineSchedule) GetCronOk() (*string, bool)`

GetCronOk returns a tuple with the Cron field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCron

`func (o *PipelineSchedule) SetCron(v string)`

SetCron sets Cron field to given value.


### GetTimezone

`func (o *PipelineSchedule) GetTimezone() string`

GetTimezone returns the Timezone field if non-nil, zero value otherwise.

### GetTimezoneOk

`func (o *PipelineSchedule) GetTimezoneOk() (*string, bool)`

GetTimezoneOk returns a tuple with the Timezone field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTimezone

`func (o *PipelineSchedule) SetTimezone(v string)`

SetTimezone sets Timezone field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


