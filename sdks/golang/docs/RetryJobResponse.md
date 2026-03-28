# RetryJobResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**JobId** | **int32** |  | [required]
**Status** | **string** |  | [required]
**TasksRetried** | **int64** |  | [required]

## Methods

### NewRetryJobResponse

`func NewRetryJobResponse(jobId int32, status string, tasksRetried int64) *RetryJobResponse`

NewRetryJobResponse instantiates a new RetryJobResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRetryJobResponseWithDefaults

`func NewRetryJobResponseWithDefaults() *RetryJobResponse`

NewRetryJobResponseWithDefaults instantiates a new RetryJobResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetJobId

`func (o *RetryJobResponse) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *RetryJobResponse) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *RetryJobResponse) SetJobId(v int32)`

SetJobId sets JobId field to given value.


### GetStatus

`func (o *RetryJobResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *RetryJobResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *RetryJobResponse) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetTasksRetried

`func (o *RetryJobResponse) GetTasksRetried() int64`

GetTasksRetried returns the TasksRetried field if non-nil, zero value otherwise.

### GetTasksRetriedOk

`func (o *RetryJobResponse) GetTasksRetriedOk() (*int64, bool)`

GetTasksRetriedOk returns a tuple with the TasksRetried field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTasksRetried

`func (o *RetryJobResponse) SetTasksRetried(v int64)`

SetTasksRetried sets TasksRetried field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
