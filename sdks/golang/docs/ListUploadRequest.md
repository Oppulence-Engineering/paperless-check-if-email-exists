# ListUploadRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**EmailColumn** | Pointer to **NullableString** |  | [optional] 
**File** | ***os.File** |  | 
**Name** | Pointer to **NullableString** |  | [optional] 

## Methods

### NewListUploadRequest

`func NewListUploadRequest(file *os.File, ) *ListUploadRequest`

NewListUploadRequest instantiates a new ListUploadRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListUploadRequestWithDefaults

`func NewListUploadRequestWithDefaults() *ListUploadRequest`

NewListUploadRequestWithDefaults instantiates a new ListUploadRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmailColumn

`func (o *ListUploadRequest) GetEmailColumn() string`

GetEmailColumn returns the EmailColumn field if non-nil, zero value otherwise.

### GetEmailColumnOk

`func (o *ListUploadRequest) GetEmailColumnOk() (*string, bool)`

GetEmailColumnOk returns a tuple with the EmailColumn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailColumn

`func (o *ListUploadRequest) SetEmailColumn(v string)`

SetEmailColumn sets EmailColumn field to given value.

### HasEmailColumn

`func (o *ListUploadRequest) HasEmailColumn() bool`

HasEmailColumn returns a boolean if a field has been set.

### SetEmailColumnNil

`func (o *ListUploadRequest) SetEmailColumnNil(b bool)`

 SetEmailColumnNil sets the value for EmailColumn to be an explicit nil

### UnsetEmailColumn
`func (o *ListUploadRequest) UnsetEmailColumn()`

UnsetEmailColumn ensures that no value is present for EmailColumn, not even an explicit nil
### GetFile

`func (o *ListUploadRequest) GetFile() *os.File`

GetFile returns the File field if non-nil, zero value otherwise.

### GetFileOk

`func (o *ListUploadRequest) GetFileOk() (**os.File, bool)`

GetFileOk returns a tuple with the File field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFile

`func (o *ListUploadRequest) SetFile(v *os.File)`

SetFile sets File field to given value.


### GetName

`func (o *ListUploadRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ListUploadRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ListUploadRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *ListUploadRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *ListUploadRequest) SetNameNil(b bool)`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *ListUploadRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


