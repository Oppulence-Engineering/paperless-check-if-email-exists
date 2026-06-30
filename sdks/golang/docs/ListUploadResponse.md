# ListUploadResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**EmailColumn** | **string** |  | [required]
**JobId** | **int32** |  | [required]
**ListId** | **int32** |  | [required]
**SourceKey** | Pointer to **NullableString** |  | [optional]
**TotalRows** | **int32** |  | [required]

## Methods

### NewListUploadResponse

`func NewListUploadResponse(emailColumn string, jobId int32, listId int32, totalRows int32) *ListUploadResponse`

NewListUploadResponse instantiates a new ListUploadResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListUploadResponseWithDefaults

`func NewListUploadResponseWithDefaults() *ListUploadResponse`

NewListUploadResponseWithDefaults instantiates a new ListUploadResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmailColumn

`func (o *ListUploadResponse) GetEmailColumn() string`

GetEmailColumn returns the EmailColumn field if non-nil, zero value otherwise.

### GetEmailColumnOk

`func (o *ListUploadResponse) GetEmailColumnOk() (*string, bool)`

GetEmailColumnOk returns a tuple with the EmailColumn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailColumn

`func (o *ListUploadResponse) SetEmailColumn(v string)`

SetEmailColumn sets EmailColumn field to given value.


### GetJobId

`func (o *ListUploadResponse) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *ListUploadResponse) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *ListUploadResponse) SetJobId(v int32)`

SetJobId sets JobId field to given value.


### GetListId

`func (o *ListUploadResponse) GetListId() int32`

GetListId returns the ListId field if non-nil, zero value otherwise.

### GetListIdOk

`func (o *ListUploadResponse) GetListIdOk() (*int32, bool)`

GetListIdOk returns a tuple with the ListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListId

`func (o *ListUploadResponse) SetListId(v int32)`

SetListId sets ListId field to given value.


### GetSourceKey

`func (o *ListUploadResponse) GetSourceKey() string`

GetSourceKey returns the SourceKey field if non-nil, zero value otherwise.

### GetSourceKeyOk

`func (o *ListUploadResponse) GetSourceKeyOk() (*string, bool)`

GetSourceKeyOk returns a tuple with the SourceKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceKey

`func (o *ListUploadResponse) SetSourceKey(v string)`

SetSourceKey sets SourceKey field to given value.

### HasSourceKey

`func (o *ListUploadResponse) HasSourceKey() bool`

HasSourceKey returns a boolean if a field has been set.

### SetSourceKeyNil

`func (o *ListUploadResponse) SetSourceKeyNil()`

 SetSourceKeyNil sets the value for SourceKey to be an explicit nil

### UnsetSourceKey
`func (o *ListUploadResponse) UnsetSourceKey()`

UnsetSourceKey ensures that no value is present for SourceKey, not even an explicit nil

### GetTotalRows

`func (o *ListUploadResponse) GetTotalRows() int32`

GetTotalRows returns the TotalRows field if non-nil, zero value otherwise.

### GetTotalRowsOk

`func (o *ListUploadResponse) GetTotalRowsOk() (*int32, bool)`

GetTotalRowsOk returns a tuple with the TotalRows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRows

`func (o *ListUploadResponse) SetTotalRows(v int32)`

SetTotalRows sets TotalRows field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
