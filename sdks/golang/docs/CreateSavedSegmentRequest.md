# CreateSavedSegmentRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Filter** | Pointer to **interface{}** |  | [optional]
**Name** | **string** |  | [required]
**Scope** | Pointer to **string** |  | [optional]

## Methods

### NewCreateSavedSegmentRequest

`func NewCreateSavedSegmentRequest(name string) *CreateSavedSegmentRequest`

NewCreateSavedSegmentRequest instantiates a new CreateSavedSegmentRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCreateSavedSegmentRequestWithDefaults

`func NewCreateSavedSegmentRequestWithDefaults() *CreateSavedSegmentRequest`

NewCreateSavedSegmentRequestWithDefaults instantiates a new CreateSavedSegmentRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetFilter

`func (o *CreateSavedSegmentRequest) GetFilter() interface{}`

GetFilter returns the Filter field if non-nil, zero value otherwise.

### GetFilterOk

`func (o *CreateSavedSegmentRequest) GetFilterOk() (*interface{}, bool)`

GetFilterOk returns a tuple with the Filter field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFilter

`func (o *CreateSavedSegmentRequest) SetFilter(v interface{})`

SetFilter sets Filter field to given value.

### HasFilter

`func (o *CreateSavedSegmentRequest) HasFilter() bool`

HasFilter returns a boolean if a field has been set.

### GetName

`func (o *CreateSavedSegmentRequest) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *CreateSavedSegmentRequest) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *CreateSavedSegmentRequest) SetName(v string)`

SetName sets Name field to given value.


### GetScope

`func (o *CreateSavedSegmentRequest) GetScope() string`

GetScope returns the Scope field if non-nil, zero value otherwise.

### GetScopeOk

`func (o *CreateSavedSegmentRequest) GetScopeOk() (*string, bool)`

GetScopeOk returns a tuple with the Scope field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScope

`func (o *CreateSavedSegmentRequest) SetScope(v string)`

SetScope sets Scope field to given value.

### HasScope

`func (o *CreateSavedSegmentRequest) HasScope() bool`

HasScope returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
