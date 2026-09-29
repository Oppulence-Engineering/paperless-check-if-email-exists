# UpdateSavedSegmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Filter** | Pointer to **interface{}** |  | [optional]
**Name** | Pointer to **NullableString** |  | [optional]
**Scope** | Pointer to **NullableString** |  | [optional]

## Methods

### NewUpdateSavedSegmentRequest

`func NewUpdateSavedSegmentRequest() *UpdateSavedSegmentRequest`

NewUpdateSavedSegmentRequest instantiates a new UpdateSavedSegmentRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewUpdateSavedSegmentRequestWithDefaults

`func NewUpdateSavedSegmentRequestWithDefaults() *UpdateSavedSegmentRequest`

NewUpdateSavedSegmentRequestWithDefaults instantiates a new UpdateSavedSegmentRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFilter

`func (o *UpdateSavedSegmentRequest) GetFilter() interface{}`

GetFilter returns the Filter field if non-nil, zero value otherwise.

### GetFilterOk

`func (o *UpdateSavedSegmentRequest) GetFilterOk() (*interface{}, bool)`

GetFilterOk returns a tuple with the Filter field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFilter

`func (o *UpdateSavedSegmentRequest) SetFilter(v interface{})`

SetFilter sets Filter field to given value.

### HasFilter

`func (o *UpdateSavedSegmentRequest) HasFilter() bool`

HasFilter returns a boolean if a field has been set.

### GetName

`func (o *UpdateSavedSegmentRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *UpdateSavedSegmentRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *UpdateSavedSegmentRequest) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *UpdateSavedSegmentRequest) HasName() bool`

HasName returns a boolean if a field has been set.

### SetNameNil

`func (o *UpdateSavedSegmentRequest) SetNameNil()`

 SetNameNil sets the value for Name to be an explicit nil

### UnsetName
`func (o *UpdateSavedSegmentRequest) UnsetName()`

UnsetName ensures that no value is present for Name, not even an explicit nil

### GetScope

`func (o *UpdateSavedSegmentRequest) GetScope() string`

GetScope returns the Scope field if non-nil, zero value otherwise.

### GetScopeOk

`func (o *UpdateSavedSegmentRequest) GetScopeOk() (*string, bool)`

GetScopeOk returns a tuple with the Scope field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScope

`func (o *UpdateSavedSegmentRequest) SetScope(v string)`

SetScope sets Scope field to given value.

### HasScope

`func (o *UpdateSavedSegmentRequest) HasScope() bool`

HasScope returns a boolean if a field has been set.

### SetScopeNil

`func (o *UpdateSavedSegmentRequest) SetScopeNil()`

 SetScopeNil sets the value for Scope to be an explicit nil

### UnsetScope
`func (o *UpdateSavedSegmentRequest) UnsetScope()`

UnsetScope ensures that no value is present for Scope, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
