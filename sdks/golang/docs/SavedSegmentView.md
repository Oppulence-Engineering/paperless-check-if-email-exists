# SavedSegmentView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | **time.Time** |  | [required]
**Filter** | **interface{}** |  | [required]
**Id** | **int64** |  | [required]
**Name** | **string** |  | [required]
**Scope** | **string** |  | [required]
**UpdatedAt** | **time.Time** |  | [required]

## Methods

### NewSavedSegmentView

`func NewSavedSegmentView(createdAt time.Time, filter interface{}, id int64, name string, scope string, updatedAt time.Time) *SavedSegmentView`

NewSavedSegmentView instantiates a new SavedSegmentView object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSavedSegmentViewWithDefaults

`func NewSavedSegmentViewWithDefaults() *SavedSegmentView`

NewSavedSegmentViewWithDefaults instantiates a new SavedSegmentView object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *SavedSegmentView) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *SavedSegmentView) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *SavedSegmentView) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetFilter

`func (o *SavedSegmentView) GetFilter() interface{}`

GetFilter returns the Filter field if non-nil, zero value otherwise.

### GetFilterOk

`func (o *SavedSegmentView) GetFilterOk() (*interface{}, bool)`

GetFilterOk returns a tuple with the Filter field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFilter

`func (o *SavedSegmentView) SetFilter(v interface{})`

SetFilter sets Filter field to given value.


### GetId

`func (o *SavedSegmentView) GetId() int64`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *SavedSegmentView) GetIdOk() (*int64, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *SavedSegmentView) SetId(v int64)`

SetId sets Id field to given value.


### GetName

`func (o *SavedSegmentView) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *SavedSegmentView) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *SavedSegmentView) SetName(v string)`

SetName sets Name field to given value.


### GetScope

`func (o *SavedSegmentView) GetScope() string`

GetScope returns the Scope field if non-nil, zero value otherwise.

### GetScopeOk

`func (o *SavedSegmentView) GetScopeOk() (*string, bool)`

GetScopeOk returns a tuple with the Scope field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScope

`func (o *SavedSegmentView) SetScope(v string)`

SetScope sets Scope field to given value.


### GetUpdatedAt

`func (o *SavedSegmentView) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *SavedSegmentView) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *SavedSegmentView) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
