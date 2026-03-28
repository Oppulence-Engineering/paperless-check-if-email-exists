# ListItem

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**EmailColumn** | **string** |  | [required]
**Id** | **int32** |  | [required]
**Name** | **string** |  | [required]
**OriginalFilename** | **string** |  | [required]
**Status** | **string** |  | [required]
**TotalRows** | **int32** |  | [required]

## Methods

### NewListItem

`func NewListItem(emailColumn string, id int32, name string, originalFilename string, status string, totalRows int32) *ListItem`

NewListItem instantiates a new ListItem object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListItemWithDefaults

`func NewListItemWithDefaults() *ListItem`

NewListItemWithDefaults instantiates a new ListItem object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmailColumn

`func (o *ListItem) GetEmailColumn() string`

GetEmailColumn returns the EmailColumn field if non-nil, zero value otherwise.

### GetEmailColumnOk

`func (o *ListItem) GetEmailColumnOk() (*string, bool)`

GetEmailColumnOk returns a tuple with the EmailColumn field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailColumn

`func (o *ListItem) SetEmailColumn(v string)`

SetEmailColumn sets EmailColumn field to given value.


### GetId

`func (o *ListItem) GetId() int32`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ListItem) GetIdOk() (*int32, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ListItem) SetId(v int32)`

SetId sets Id field to given value.


### GetName

`func (o *ListItem) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ListItem) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ListItem) SetName(v string)`

SetName sets Name field to given value.


### GetOriginalFilename

`func (o *ListItem) GetOriginalFilename() string`

GetOriginalFilename returns the OriginalFilename field if non-nil, zero value otherwise.

### GetOriginalFilenameOk

`func (o *ListItem) GetOriginalFilenameOk() (*string, bool)`

GetOriginalFilenameOk returns a tuple with the OriginalFilename field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOriginalFilename

`func (o *ListItem) SetOriginalFilename(v string)`

SetOriginalFilename sets OriginalFilename field to given value.


### GetStatus

`func (o *ListItem) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *ListItem) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *ListItem) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetTotalRows

`func (o *ListItem) GetTotalRows() int32`

GetTotalRows returns the TotalRows field if non-nil, zero value otherwise.

### GetTotalRowsOk

`func (o *ListItem) GetTotalRowsOk() (*int32, bool)`

GetTotalRowsOk returns a tuple with the TotalRows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRows

`func (o *ListItem) SetTotalRows(v int32)`

SetTotalRows sets TotalRows field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


