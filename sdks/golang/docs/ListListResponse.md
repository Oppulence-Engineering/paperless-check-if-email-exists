# ListListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Lists** | [**[]ListItem**](ListItem.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewListListResponse

`func NewListListResponse(lists []ListItem, total int64) *ListListResponse`

NewListListResponse instantiates a new ListListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListListResponseWithDefaults

`func NewListListResponseWithDefaults() *ListListResponse`

NewListListResponseWithDefaults instantiates a new ListListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetLists

`func (o *ListListResponse) GetLists() []ListItem`

GetLists returns the Lists field if non-nil, zero value otherwise.

### GetListsOk

`func (o *ListListResponse) GetListsOk() ([]ListItem, bool)`

GetListsOk returns a tuple with the Lists field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLists

`func (o *ListListResponse) SetLists(v []ListItem)`

SetLists sets Lists field to given value.


### GetTotal

`func (o *ListListResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *ListListResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *ListListResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


