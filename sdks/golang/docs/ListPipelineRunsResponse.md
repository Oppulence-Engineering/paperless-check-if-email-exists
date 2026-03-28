# ListPipelineRunsResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Runs** | [**[]PipelineRunView**](PipelineRunView.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewListPipelineRunsResponse

`func NewListPipelineRunsResponse(runs []PipelineRunView, total int64) *ListPipelineRunsResponse`

NewListPipelineRunsResponse instantiates a new ListPipelineRunsResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListPipelineRunsResponseWithDefaults

`func NewListPipelineRunsResponseWithDefaults() *ListPipelineRunsResponse`

NewListPipelineRunsResponseWithDefaults instantiates a new ListPipelineRunsResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetRuns

`func (o *ListPipelineRunsResponse) GetRuns() []PipelineRunView`

GetRuns returns the Runs field if non-nil, zero value otherwise.

### GetRunsOk

`func (o *ListPipelineRunsResponse) GetRunsOk() ([]PipelineRunView, bool)`

GetRunsOk returns a tuple with the Runs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRuns

`func (o *ListPipelineRunsResponse) SetRuns(v []PipelineRunView)`

SetRuns sets Runs field to given value.


### GetTotal

`func (o *ListPipelineRunsResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *ListPipelineRunsResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *ListPipelineRunsResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
