# DiffRow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BaseCategory** | Pointer to **NullableString** |  | [optional]
**BaseRowIndex** | Pointer to **NullableInt32** |  | [optional]
**BaseScore** | Pointer to **NullableInt32** |  | [optional]
**BaseTaskId** | Pointer to **NullableInt32** |  | [optional]
**CanonicalEmail** | **string** |  | [required]
**ChangeType** | **string** |  | [required]
**CompareCategory** | Pointer to **NullableString** |  | [optional]
**CompareRowIndex** | Pointer to **NullableInt32** |  | [optional]
**CompareScore** | Pointer to **NullableInt32** |  | [optional]
**CompareTaskId** | Pointer to **NullableInt32** |  | [optional]

## Methods

### NewDiffRow

`func NewDiffRow(canonicalEmail string, changeType string) *DiffRow`

NewDiffRow instantiates a new DiffRow object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDiffRowWithDefaults

`func NewDiffRowWithDefaults() *DiffRow`

NewDiffRowWithDefaults instantiates a new DiffRow object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBaseCategory

`func (o *DiffRow) GetBaseCategory() string`

GetBaseCategory returns the BaseCategory field if non-nil, zero value otherwise.

### GetBaseCategoryOk

`func (o *DiffRow) GetBaseCategoryOk() (*string, bool)`

GetBaseCategoryOk returns a tuple with the BaseCategory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBaseCategory

`func (o *DiffRow) SetBaseCategory(v string)`

SetBaseCategory sets BaseCategory field to given value.

### HasBaseCategory

`func (o *DiffRow) HasBaseCategory() bool`

HasBaseCategory returns a boolean if a field has been set.

### SetBaseCategoryNil

`func (o *DiffRow) SetBaseCategoryNil()`

 SetBaseCategoryNil sets the value for BaseCategory to be an explicit nil

### UnsetBaseCategory
`func (o *DiffRow) UnsetBaseCategory()`

UnsetBaseCategory ensures that no value is present for BaseCategory, not even an explicit nil

### GetBaseRowIndex

`func (o *DiffRow) GetBaseRowIndex() int32`

GetBaseRowIndex returns the BaseRowIndex field if non-nil, zero value otherwise.

### GetBaseRowIndexOk

`func (o *DiffRow) GetBaseRowIndexOk() (*int32, bool)`

GetBaseRowIndexOk returns a tuple with the BaseRowIndex field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBaseRowIndex

`func (o *DiffRow) SetBaseRowIndex(v int32)`

SetBaseRowIndex sets BaseRowIndex field to given value.

### HasBaseRowIndex

`func (o *DiffRow) HasBaseRowIndex() bool`

HasBaseRowIndex returns a boolean if a field has been set.

### SetBaseRowIndexNil

`func (o *DiffRow) SetBaseRowIndexNil()`

 SetBaseRowIndexNil sets the value for BaseRowIndex to be an explicit nil

### UnsetBaseRowIndex
`func (o *DiffRow) UnsetBaseRowIndex()`

UnsetBaseRowIndex ensures that no value is present for BaseRowIndex, not even an explicit nil

### GetBaseScore

`func (o *DiffRow) GetBaseScore() int32`

GetBaseScore returns the BaseScore field if non-nil, zero value otherwise.

### GetBaseScoreOk

`func (o *DiffRow) GetBaseScoreOk() (*int32, bool)`

GetBaseScoreOk returns a tuple with the BaseScore field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBaseScore

`func (o *DiffRow) SetBaseScore(v int32)`

SetBaseScore sets BaseScore field to given value.

### HasBaseScore

`func (o *DiffRow) HasBaseScore() bool`

HasBaseScore returns a boolean if a field has been set.

### SetBaseScoreNil

`func (o *DiffRow) SetBaseScoreNil()`

 SetBaseScoreNil sets the value for BaseScore to be an explicit nil

### UnsetBaseScore
`func (o *DiffRow) UnsetBaseScore()`

UnsetBaseScore ensures that no value is present for BaseScore, not even an explicit nil

### GetBaseTaskId

`func (o *DiffRow) GetBaseTaskId() int32`

GetBaseTaskId returns the BaseTaskId field if non-nil, zero value otherwise.

### GetBaseTaskIdOk

`func (o *DiffRow) GetBaseTaskIdOk() (*int32, bool)`

GetBaseTaskIdOk returns a tuple with the BaseTaskId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBaseTaskId

`func (o *DiffRow) SetBaseTaskId(v int32)`

SetBaseTaskId sets BaseTaskId field to given value.

### HasBaseTaskId

`func (o *DiffRow) HasBaseTaskId() bool`

HasBaseTaskId returns a boolean if a field has been set.

### SetBaseTaskIdNil

`func (o *DiffRow) SetBaseTaskIdNil()`

 SetBaseTaskIdNil sets the value for BaseTaskId to be an explicit nil

### UnsetBaseTaskId
`func (o *DiffRow) UnsetBaseTaskId()`

UnsetBaseTaskId ensures that no value is present for BaseTaskId, not even an explicit nil

### GetCanonicalEmail

`func (o *DiffRow) GetCanonicalEmail() string`

GetCanonicalEmail returns the CanonicalEmail field if non-nil, zero value otherwise.

### GetCanonicalEmailOk

`func (o *DiffRow) GetCanonicalEmailOk() (*string, bool)`

GetCanonicalEmailOk returns a tuple with the CanonicalEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanonicalEmail

`func (o *DiffRow) SetCanonicalEmail(v string)`

SetCanonicalEmail sets CanonicalEmail field to given value.


### GetChangeType

`func (o *DiffRow) GetChangeType() string`

GetChangeType returns the ChangeType field if non-nil, zero value otherwise.

### GetChangeTypeOk

`func (o *DiffRow) GetChangeTypeOk() (*string, bool)`

GetChangeTypeOk returns a tuple with the ChangeType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChangeType

`func (o *DiffRow) SetChangeType(v string)`

SetChangeType sets ChangeType field to given value.


### GetCompareCategory

`func (o *DiffRow) GetCompareCategory() string`

GetCompareCategory returns the CompareCategory field if non-nil, zero value otherwise.

### GetCompareCategoryOk

`func (o *DiffRow) GetCompareCategoryOk() (*string, bool)`

GetCompareCategoryOk returns a tuple with the CompareCategory field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompareCategory

`func (o *DiffRow) SetCompareCategory(v string)`

SetCompareCategory sets CompareCategory field to given value.

### HasCompareCategory

`func (o *DiffRow) HasCompareCategory() bool`

HasCompareCategory returns a boolean if a field has been set.

### SetCompareCategoryNil

`func (o *DiffRow) SetCompareCategoryNil()`

 SetCompareCategoryNil sets the value for CompareCategory to be an explicit nil

### UnsetCompareCategory
`func (o *DiffRow) UnsetCompareCategory()`

UnsetCompareCategory ensures that no value is present for CompareCategory, not even an explicit nil

### GetCompareRowIndex

`func (o *DiffRow) GetCompareRowIndex() int32`

GetCompareRowIndex returns the CompareRowIndex field if non-nil, zero value otherwise.

### GetCompareRowIndexOk

`func (o *DiffRow) GetCompareRowIndexOk() (*int32, bool)`

GetCompareRowIndexOk returns a tuple with the CompareRowIndex field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompareRowIndex

`func (o *DiffRow) SetCompareRowIndex(v int32)`

SetCompareRowIndex sets CompareRowIndex field to given value.

### HasCompareRowIndex

`func (o *DiffRow) HasCompareRowIndex() bool`

HasCompareRowIndex returns a boolean if a field has been set.

### SetCompareRowIndexNil

`func (o *DiffRow) SetCompareRowIndexNil()`

 SetCompareRowIndexNil sets the value for CompareRowIndex to be an explicit nil

### UnsetCompareRowIndex
`func (o *DiffRow) UnsetCompareRowIndex()`

UnsetCompareRowIndex ensures that no value is present for CompareRowIndex, not even an explicit nil

### GetCompareScore

`func (o *DiffRow) GetCompareScore() int32`

GetCompareScore returns the CompareScore field if non-nil, zero value otherwise.

### GetCompareScoreOk

`func (o *DiffRow) GetCompareScoreOk() (*int32, bool)`

GetCompareScoreOk returns a tuple with the CompareScore field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompareScore

`func (o *DiffRow) SetCompareScore(v int32)`

SetCompareScore sets CompareScore field to given value.

### HasCompareScore

`func (o *DiffRow) HasCompareScore() bool`

HasCompareScore returns a boolean if a field has been set.

### SetCompareScoreNil

`func (o *DiffRow) SetCompareScoreNil()`

 SetCompareScoreNil sets the value for CompareScore to be an explicit nil

### UnsetCompareScore
`func (o *DiffRow) UnsetCompareScore()`

UnsetCompareScore ensures that no value is present for CompareScore, not even an explicit nil

### GetCompareTaskId

`func (o *DiffRow) GetCompareTaskId() int32`

GetCompareTaskId returns the CompareTaskId field if non-nil, zero value otherwise.

### GetCompareTaskIdOk

`func (o *DiffRow) GetCompareTaskIdOk() (*int32, bool)`

GetCompareTaskIdOk returns a tuple with the CompareTaskId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompareTaskId

`func (o *DiffRow) SetCompareTaskId(v int32)`

SetCompareTaskId sets CompareTaskId field to given value.

### HasCompareTaskId

`func (o *DiffRow) HasCompareTaskId() bool`

HasCompareTaskId returns a boolean if a field has been set.

### SetCompareTaskIdNil

`func (o *DiffRow) SetCompareTaskIdNil()`

 SetCompareTaskIdNil sets the value for CompareTaskId to be an explicit nil

### UnsetCompareTaskId
`func (o *DiffRow) UnsetCompareTaskId()`

UnsetCompareTaskId ensures that no value is present for CompareTaskId, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
