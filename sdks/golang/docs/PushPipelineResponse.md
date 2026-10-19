# PushPipelineResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AcceptedRows** | **int32** |  | [required]
**BatchId** | **int64** |  | [required]
**Replayed** | **bool** |  | [required]
**RunId** | **int64** |  | [required]
**Status** | [**PipelineRunStatus**](PipelineRunStatus.md) |  | [required]

## Methods

### NewPushPipelineResponse

`func NewPushPipelineResponse(acceptedRows int32, batchId int64, replayed bool, runId int64, status PipelineRunStatus) *PushPipelineResponse`

NewPushPipelineResponse instantiates a new PushPipelineResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPushPipelineResponseWithDefaults

`func NewPushPipelineResponseWithDefaults() *PushPipelineResponse`

NewPushPipelineResponseWithDefaults instantiates a new PushPipelineResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAcceptedRows

`func (o *PushPipelineResponse) GetAcceptedRows() int32`

GetAcceptedRows returns the AcceptedRows field if non-nil, zero value otherwise.

### GetAcceptedRowsOk

`func (o *PushPipelineResponse) GetAcceptedRowsOk() (*int32, bool)`

GetAcceptedRowsOk returns a tuple with the AcceptedRows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAcceptedRows

`func (o *PushPipelineResponse) SetAcceptedRows(v int32)`

SetAcceptedRows sets AcceptedRows field to given value.


### GetBatchId

`func (o *PushPipelineResponse) GetBatchId() int64`

GetBatchId returns the BatchId field if non-nil, zero value otherwise.

### GetBatchIdOk

`func (o *PushPipelineResponse) GetBatchIdOk() (*int64, bool)`

GetBatchIdOk returns a tuple with the BatchId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBatchId

`func (o *PushPipelineResponse) SetBatchId(v int64)`

SetBatchId sets BatchId field to given value.


### GetReplayed

`func (o *PushPipelineResponse) GetReplayed() bool`

GetReplayed returns the Replayed field if non-nil, zero value otherwise.

### GetReplayedOk

`func (o *PushPipelineResponse) GetReplayedOk() (*bool, bool)`

GetReplayedOk returns a tuple with the Replayed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReplayed

`func (o *PushPipelineResponse) SetReplayed(v bool)`

SetReplayed sets Replayed field to given value.


### GetRunId

`func (o *PushPipelineResponse) GetRunId() int64`

GetRunId returns the RunId field if non-nil, zero value otherwise.

### GetRunIdOk

`func (o *PushPipelineResponse) GetRunIdOk() (*int64, bool)`

GetRunIdOk returns a tuple with the RunId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRunId

`func (o *PushPipelineResponse) SetRunId(v int64)`

SetRunId sets RunId field to given value.


### GetStatus

`func (o *PushPipelineResponse) GetStatus() PipelineRunStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *PushPipelineResponse) GetStatusOk() (*PipelineRunStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *PushPipelineResponse) SetStatus(v PipelineRunStatus)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
