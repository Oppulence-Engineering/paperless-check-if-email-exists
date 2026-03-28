# JobResultPageResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**HasMore** | **bool** |  | [required]
**NextCursor** | Pointer to **NullableInt64** |  | [optional]
**Results** | [**[]JobTaskResult**](JobTaskResult.md) |  | [required]

## Methods

### NewJobResultPageResponse

`func NewJobResultPageResponse(hasMore bool, results []JobTaskResult) *JobResultPageResponse`

NewJobResultPageResponse instantiates a new JobResultPageResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewJobResultPageResponseWithDefaults

`func NewJobResultPageResponseWithDefaults() *JobResultPageResponse`

NewJobResultPageResponseWithDefaults instantiates a new JobResultPageResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetHasMore

`func (o *JobResultPageResponse) GetHasMore() bool`

GetHasMore returns the HasMore field if non-nil, zero value otherwise.

### GetHasMoreOk

`func (o *JobResultPageResponse) GetHasMoreOk() (*bool, bool)`

GetHasMoreOk returns a tuple with the HasMore field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasMore

`func (o *JobResultPageResponse) SetHasMore(v bool)`

SetHasMore sets HasMore field to given value.


### GetNextCursor

`func (o *JobResultPageResponse) GetNextCursor() int64`

GetNextCursor returns the NextCursor field if non-nil, zero value otherwise.

### GetNextCursorOk

`func (o *JobResultPageResponse) GetNextCursorOk() (*int64, bool)`

GetNextCursorOk returns a tuple with the NextCursor field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNextCursor

`func (o *JobResultPageResponse) SetNextCursor(v int64)`

SetNextCursor sets NextCursor field to given value.

### HasNextCursor

`func (o *JobResultPageResponse) HasNextCursor() bool`

HasNextCursor returns a boolean if a field has been set.

### SetNextCursorNil

`func (o *JobResultPageResponse) SetNextCursorNil()`

 SetNextCursorNil sets the value for NextCursor to be an explicit nil

### UnsetNextCursor
`func (o *JobResultPageResponse) UnsetNextCursor()`

UnsetNextCursor ensures that no value is present for NextCursor, not even an explicit nil

### GetResults

`func (o *JobResultPageResponse) GetResults() []JobTaskResult`

GetResults returns the Results field if non-nil, zero value otherwise.

### GetResultsOk

`func (o *JobResultPageResponse) GetResultsOk() ([]JobTaskResult, bool)`

GetResultsOk returns a tuple with the Results field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResults

`func (o *JobResultPageResponse) SetResults(v []JobTaskResult)`

SetResults sets Results field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
