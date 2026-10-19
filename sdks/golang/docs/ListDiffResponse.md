# ListDiffResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Added** | [**DiffGroup**](DiffGroup.md) |  | [required]
**BaseListId** | **int32** |  | [required]
**CompareListId** | **int32** |  | [required]
**Degraded** | [**DiffGroup**](DiffGroup.md) |  | [required]
**Improved** | [**DiffGroup**](DiffGroup.md) |  | [required]
**NewlyInvalid** | [**DiffGroup**](DiffGroup.md) |  | [required]
**NewlyRisky** | [**DiffGroup**](DiffGroup.md) |  | [required]
**NewlySafe** | [**DiffGroup**](DiffGroup.md) |  | [required]
**Removed** | [**DiffGroup**](DiffGroup.md) |  | [required]
**Unchanged** | [**DiffGroup**](DiffGroup.md) |  | [required]

## Methods

### NewListDiffResponse

`func NewListDiffResponse(added DiffGroup, baseListId int32, compareListId int32, degraded DiffGroup, improved DiffGroup, newlyInvalid DiffGroup, newlyRisky DiffGroup, newlySafe DiffGroup, removed DiffGroup, unchanged DiffGroup) *ListDiffResponse`

NewListDiffResponse instantiates a new ListDiffResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewListDiffResponseWithDefaults

`func NewListDiffResponseWithDefaults() *ListDiffResponse`

NewListDiffResponseWithDefaults instantiates a new ListDiffResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAdded

`func (o *ListDiffResponse) GetAdded() DiffGroup`

GetAdded returns the Added field if non-nil, zero value otherwise.

### GetAddedOk

`func (o *ListDiffResponse) GetAddedOk() (*DiffGroup, bool)`

GetAddedOk returns a tuple with the Added field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAdded

`func (o *ListDiffResponse) SetAdded(v DiffGroup)`

SetAdded sets Added field to given value.


### GetBaseListId

`func (o *ListDiffResponse) GetBaseListId() int32`

GetBaseListId returns the BaseListId field if non-nil, zero value otherwise.

### GetBaseListIdOk

`func (o *ListDiffResponse) GetBaseListIdOk() (*int32, bool)`

GetBaseListIdOk returns a tuple with the BaseListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBaseListId

`func (o *ListDiffResponse) SetBaseListId(v int32)`

SetBaseListId sets BaseListId field to given value.


### GetCompareListId

`func (o *ListDiffResponse) GetCompareListId() int32`

GetCompareListId returns the CompareListId field if non-nil, zero value otherwise.

### GetCompareListIdOk

`func (o *ListDiffResponse) GetCompareListIdOk() (*int32, bool)`

GetCompareListIdOk returns a tuple with the CompareListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompareListId

`func (o *ListDiffResponse) SetCompareListId(v int32)`

SetCompareListId sets CompareListId field to given value.


### GetDegraded

`func (o *ListDiffResponse) GetDegraded() DiffGroup`

GetDegraded returns the Degraded field if non-nil, zero value otherwise.

### GetDegradedOk

`func (o *ListDiffResponse) GetDegradedOk() (*DiffGroup, bool)`

GetDegradedOk returns a tuple with the Degraded field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDegraded

`func (o *ListDiffResponse) SetDegraded(v DiffGroup)`

SetDegraded sets Degraded field to given value.


### GetImproved

`func (o *ListDiffResponse) GetImproved() DiffGroup`

GetImproved returns the Improved field if non-nil, zero value otherwise.

### GetImprovedOk

`func (o *ListDiffResponse) GetImprovedOk() (*DiffGroup, bool)`

GetImprovedOk returns a tuple with the Improved field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImproved

`func (o *ListDiffResponse) SetImproved(v DiffGroup)`

SetImproved sets Improved field to given value.


### GetNewlyInvalid

`func (o *ListDiffResponse) GetNewlyInvalid() DiffGroup`

GetNewlyInvalid returns the NewlyInvalid field if non-nil, zero value otherwise.

### GetNewlyInvalidOk

`func (o *ListDiffResponse) GetNewlyInvalidOk() (*DiffGroup, bool)`

GetNewlyInvalidOk returns a tuple with the NewlyInvalid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNewlyInvalid

`func (o *ListDiffResponse) SetNewlyInvalid(v DiffGroup)`

SetNewlyInvalid sets NewlyInvalid field to given value.


### GetNewlyRisky

`func (o *ListDiffResponse) GetNewlyRisky() DiffGroup`

GetNewlyRisky returns the NewlyRisky field if non-nil, zero value otherwise.

### GetNewlyRiskyOk

`func (o *ListDiffResponse) GetNewlyRiskyOk() (*DiffGroup, bool)`

GetNewlyRiskyOk returns a tuple with the NewlyRisky field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNewlyRisky

`func (o *ListDiffResponse) SetNewlyRisky(v DiffGroup)`

SetNewlyRisky sets NewlyRisky field to given value.


### GetNewlySafe

`func (o *ListDiffResponse) GetNewlySafe() DiffGroup`

GetNewlySafe returns the NewlySafe field if non-nil, zero value otherwise.

### GetNewlySafeOk

`func (o *ListDiffResponse) GetNewlySafeOk() (*DiffGroup, bool)`

GetNewlySafeOk returns a tuple with the NewlySafe field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNewlySafe

`func (o *ListDiffResponse) SetNewlySafe(v DiffGroup)`

SetNewlySafe sets NewlySafe field to given value.


### GetRemoved

`func (o *ListDiffResponse) GetRemoved() DiffGroup`

GetRemoved returns the Removed field if non-nil, zero value otherwise.

### GetRemovedOk

`func (o *ListDiffResponse) GetRemovedOk() (*DiffGroup, bool)`

GetRemovedOk returns a tuple with the Removed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRemoved

`func (o *ListDiffResponse) SetRemoved(v DiffGroup)`

SetRemoved sets Removed field to given value.


### GetUnchanged

`func (o *ListDiffResponse) GetUnchanged() DiffGroup`

GetUnchanged returns the Unchanged field if non-nil, zero value otherwise.

### GetUnchangedOk

`func (o *ListDiffResponse) GetUnchangedOk() (*DiffGroup, bool)`

GetUnchangedOk returns a tuple with the Unchanged field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnchanged

`func (o *ListDiffResponse) SetUnchanged(v DiffGroup)`

SetUnchanged sets Unchanged field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
