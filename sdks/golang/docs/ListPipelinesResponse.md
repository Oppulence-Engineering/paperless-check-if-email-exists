# ListPipelinesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Pipelines** | [**[]PipelineView**](PipelineView.md) |  | 
**Total** | **int64** |  | 

## Methods

### NewListPipelinesResponse

`func NewListPipelinesResponse(pipelines []PipelineView, total int64, ) *ListPipelinesResponse`

NewListPipelinesResponse instantiates a new ListPipelinesResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListPipelinesResponseWithDefaults

`func NewListPipelinesResponseWithDefaults() *ListPipelinesResponse`

NewListPipelinesResponseWithDefaults instantiates a new ListPipelinesResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPipelines

`func (o *ListPipelinesResponse) GetPipelines() []PipelineView`

GetPipelines returns the Pipelines field if non-nil, zero value otherwise.

### GetPipelinesOk

`func (o *ListPipelinesResponse) GetPipelinesOk() (*[]PipelineView, bool)`

GetPipelinesOk returns a tuple with the Pipelines field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPipelines

`func (o *ListPipelinesResponse) SetPipelines(v []PipelineView)`

SetPipelines sets Pipelines field to given value.


### GetTotal

`func (o *ListPipelinesResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *ListPipelinesResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *ListPipelinesResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


