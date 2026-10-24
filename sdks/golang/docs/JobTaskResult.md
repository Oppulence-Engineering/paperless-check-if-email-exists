# JobTaskResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CanonicalTaskId** | Pointer to **NullableInt32** |  | [optional]
**Error** | Pointer to **NullableString** |  | [optional]
**Id** | **int64** |  | [required]
**IsDuplicate** | Pointer to **bool** |  | [optional]
**OriginalEmail** | Pointer to **NullableString** |  | [optional]
**Result** | Pointer to [**NullableCheckEmailOutput**](CheckEmailOutput.md) |  | [optional]
**RetryCount** | **int32** |  | [required]
**RowNumber** | Pointer to **NullableInt32** |  | [optional]
**TaskState** | **string** |  | [required]

## Methods

### NewJobTaskResult

`func NewJobTaskResult(id int64, retryCount int32, taskState string) *JobTaskResult`

NewJobTaskResult instantiates a new JobTaskResult object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewJobTaskResultWithDefaults

`func NewJobTaskResultWithDefaults() *JobTaskResult`

NewJobTaskResultWithDefaults instantiates a new JobTaskResult object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCanonicalTaskId

`func (o *JobTaskResult) GetCanonicalTaskId() int32`

GetCanonicalTaskId returns the CanonicalTaskId field if non-nil, zero value otherwise.

### GetCanonicalTaskIdOk

`func (o *JobTaskResult) GetCanonicalTaskIdOk() (*int32, bool)`

GetCanonicalTaskIdOk returns a tuple with the CanonicalTaskId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanonicalTaskId

`func (o *JobTaskResult) SetCanonicalTaskId(v int32)`

SetCanonicalTaskId sets CanonicalTaskId field to given value.

### HasCanonicalTaskId

`func (o *JobTaskResult) HasCanonicalTaskId() bool`

HasCanonicalTaskId returns a boolean if a field has been set.

### SetCanonicalTaskIdNil

`func (o *JobTaskResult) SetCanonicalTaskIdNil()`

 SetCanonicalTaskIdNil sets the value for CanonicalTaskId to be an explicit nil

### UnsetCanonicalTaskId
`func (o *JobTaskResult) UnsetCanonicalTaskId()`

UnsetCanonicalTaskId ensures that no value is present for CanonicalTaskId, not even an explicit nil

### GetError

`func (o *JobTaskResult) GetError() string`

GetError returns the Error field if non-nil, zero value otherwise.

### GetErrorOk

`func (o *JobTaskResult) GetErrorOk() (*string, bool)`

GetErrorOk returns a tuple with the Error field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetError

`func (o *JobTaskResult) SetError(v string)`

SetError sets Error field to given value.

### HasError

`func (o *JobTaskResult) HasError() bool`

HasError returns a boolean if a field has been set.

### SetErrorNil

`func (o *JobTaskResult) SetErrorNil()`

 SetErrorNil sets the value for Error to be an explicit nil

### UnsetError
`func (o *JobTaskResult) UnsetError()`

UnsetError ensures that no value is present for Error, not even an explicit nil

### GetId

`func (o *JobTaskResult) GetId() int64`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *JobTaskResult) GetIdOk() (*int64, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *JobTaskResult) SetId(v int64)`

SetId sets Id field to given value.


### GetIsDuplicate

`func (o *JobTaskResult) GetIsDuplicate() bool`

GetIsDuplicate returns the IsDuplicate field if non-nil, zero value otherwise.

### GetIsDuplicateOk

`func (o *JobTaskResult) GetIsDuplicateOk() (*bool, bool)`

GetIsDuplicateOk returns a tuple with the IsDuplicate field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDuplicate

`func (o *JobTaskResult) SetIsDuplicate(v bool)`

SetIsDuplicate sets IsDuplicate field to given value.

### HasIsDuplicate

`func (o *JobTaskResult) HasIsDuplicate() bool`

HasIsDuplicate returns a boolean if a field has been set.

### GetOriginalEmail

`func (o *JobTaskResult) GetOriginalEmail() string`

GetOriginalEmail returns the OriginalEmail field if non-nil, zero value otherwise.

### GetOriginalEmailOk

`func (o *JobTaskResult) GetOriginalEmailOk() (*string, bool)`

GetOriginalEmailOk returns a tuple with the OriginalEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOriginalEmail

`func (o *JobTaskResult) SetOriginalEmail(v string)`

SetOriginalEmail sets OriginalEmail field to given value.

### HasOriginalEmail

`func (o *JobTaskResult) HasOriginalEmail() bool`

HasOriginalEmail returns a boolean if a field has been set.

### SetOriginalEmailNil

`func (o *JobTaskResult) SetOriginalEmailNil()`

 SetOriginalEmailNil sets the value for OriginalEmail to be an explicit nil

### UnsetOriginalEmail
`func (o *JobTaskResult) UnsetOriginalEmail()`

UnsetOriginalEmail ensures that no value is present for OriginalEmail, not even an explicit nil

### GetResult

`func (o *JobTaskResult) GetResult() CheckEmailOutput`

GetResult returns the Result field if non-nil, zero value otherwise.

### GetResultOk

`func (o *JobTaskResult) GetResultOk() (*CheckEmailOutput, bool)`

GetResultOk returns a tuple with the Result field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResult

`func (o *JobTaskResult) SetResult(v CheckEmailOutput)`

SetResult sets Result field to given value.

### HasResult

`func (o *JobTaskResult) HasResult() bool`

HasResult returns a boolean if a field has been set.

### SetResultNil

`func (o *JobTaskResult) SetResultNil()`

 SetResultNil sets the value for Result to be an explicit nil

### UnsetResult
`func (o *JobTaskResult) UnsetResult()`

UnsetResult ensures that no value is present for Result, not even an explicit nil

### GetRetryCount

`func (o *JobTaskResult) GetRetryCount() int32`

GetRetryCount returns the RetryCount field if non-nil, zero value otherwise.

### GetRetryCountOk

`func (o *JobTaskResult) GetRetryCountOk() (*int32, bool)`

GetRetryCountOk returns a tuple with the RetryCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRetryCount

`func (o *JobTaskResult) SetRetryCount(v int32)`

SetRetryCount sets RetryCount field to given value.


### GetRowNumber

`func (o *JobTaskResult) GetRowNumber() int32`

GetRowNumber returns the RowNumber field if non-nil, zero value otherwise.

### GetRowNumberOk

`func (o *JobTaskResult) GetRowNumberOk() (*int32, bool)`

GetRowNumberOk returns a tuple with the RowNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRowNumber

`func (o *JobTaskResult) SetRowNumber(v int32)`

SetRowNumber sets RowNumber field to given value.

### HasRowNumber

`func (o *JobTaskResult) HasRowNumber() bool`

HasRowNumber returns a boolean if a field has been set.

### SetRowNumberNil

`func (o *JobTaskResult) SetRowNumberNil()`

 SetRowNumberNil sets the value for RowNumber to be an explicit nil

### UnsetRowNumber
`func (o *JobTaskResult) UnsetRowNumber()`

UnsetRowNumber ensures that no value is present for RowNumber, not even an explicit nil

### GetTaskState

`func (o *JobTaskResult) GetTaskState() string`

GetTaskState returns the TaskState field if non-nil, zero value otherwise.

### GetTaskStateOk

`func (o *JobTaskResult) GetTaskStateOk() (*string, bool)`

GetTaskStateOk returns a tuple with the TaskState field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTaskState

`func (o *JobTaskResult) SetTaskState(v string)`

SetTaskState sets TaskState field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
