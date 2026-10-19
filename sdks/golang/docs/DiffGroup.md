# DiffGroup

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Count** | **int64** |  | [required]
**Rows** | [**[]DiffRow**](DiffRow.md) |  | [required]

## Methods

### NewDiffGroup

`func NewDiffGroup(count int64, rows []DiffRow) *DiffGroup`

NewDiffGroup instantiates a new DiffGroup object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDiffGroupWithDefaults

`func NewDiffGroupWithDefaults() *DiffGroup`

NewDiffGroupWithDefaults instantiates a new DiffGroup object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCount

`func (o *DiffGroup) GetCount() int64`

GetCount returns the Count field if non-nil, zero value otherwise.

### GetCountOk

`func (o *DiffGroup) GetCountOk() (*int64, bool)`

GetCountOk returns a tuple with the Count field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCount

`func (o *DiffGroup) SetCount(v int64)`

SetCount sets Count field to given value.


### GetRows

`func (o *DiffGroup) GetRows() []DiffRow`

GetRows returns the Rows field if non-nil, zero value otherwise.

### GetRowsOk

`func (o *DiffGroup) GetRowsOk() ([]DiffRow, bool)`

GetRowsOk returns a tuple with the Rows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRows

`func (o *DiffGroup) SetRows(v []DiffRow)`

SetRows sets Rows field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
