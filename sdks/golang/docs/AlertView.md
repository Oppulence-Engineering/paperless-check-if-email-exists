# AlertView

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Body** | Pointer to **NullableString** |  | [optional]
**CanonicalEmail** | **string** |  | [required]
**ChangeEventId** | Pointer to **NullableInt64** |  | [optional]
**CreatedAt** | **time.Time** |  | [required]
**Id** | **int64** |  | [required]
**Metadata** | **interface{}** |  | [required]
**Status** | **string** |  | [required]
**Title** | **string** |  | [required]
**Type** | **string** |  | [required]
**UpdatedAt** | **time.Time** |  | [required]

## Methods

### NewAlertView

`func NewAlertView(canonicalEmail string, createdAt time.Time, id int64, metadata interface{}, status string, title string, type_ string, updatedAt time.Time) *AlertView`

NewAlertView instantiates a new AlertView object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAlertViewWithDefaults

`func NewAlertViewWithDefaults() *AlertView`

NewAlertViewWithDefaults instantiates a new AlertView object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBody

`func (o *AlertView) GetBody() string`

GetBody returns the Body field if non-nil, zero value otherwise.

### GetBodyOk

`func (o *AlertView) GetBodyOk() (*string, bool)`

GetBodyOk returns a tuple with the Body field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBody

`func (o *AlertView) SetBody(v string)`

SetBody sets Body field to given value.

### HasBody

`func (o *AlertView) HasBody() bool`

HasBody returns a boolean if a field has been set.

### SetBodyNil

`func (o *AlertView) SetBodyNil()`

 SetBodyNil sets the value for Body to be an explicit nil

### UnsetBody
`func (o *AlertView) UnsetBody()`

UnsetBody ensures that no value is present for Body, not even an explicit nil

### GetCanonicalEmail

`func (o *AlertView) GetCanonicalEmail() string`

GetCanonicalEmail returns the CanonicalEmail field if non-nil, zero value otherwise.

### GetCanonicalEmailOk

`func (o *AlertView) GetCanonicalEmailOk() (*string, bool)`

GetCanonicalEmailOk returns a tuple with the CanonicalEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanonicalEmail

`func (o *AlertView) SetCanonicalEmail(v string)`

SetCanonicalEmail sets CanonicalEmail field to given value.


### GetChangeEventId

`func (o *AlertView) GetChangeEventId() int64`

GetChangeEventId returns the ChangeEventId field if non-nil, zero value otherwise.

### GetChangeEventIdOk

`func (o *AlertView) GetChangeEventIdOk() (*int64, bool)`

GetChangeEventIdOk returns a tuple with the ChangeEventId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChangeEventId

`func (o *AlertView) SetChangeEventId(v int64)`

SetChangeEventId sets ChangeEventId field to given value.

### HasChangeEventId

`func (o *AlertView) HasChangeEventId() bool`

HasChangeEventId returns a boolean if a field has been set.

### SetChangeEventIdNil

`func (o *AlertView) SetChangeEventIdNil()`

 SetChangeEventIdNil sets the value for ChangeEventId to be an explicit nil

### UnsetChangeEventId
`func (o *AlertView) UnsetChangeEventId()`

UnsetChangeEventId ensures that no value is present for ChangeEventId, not even an explicit nil

### GetCreatedAt

`func (o *AlertView) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *AlertView) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *AlertView) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetId

`func (o *AlertView) GetId() int64`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AlertView) GetIdOk() (*int64, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AlertView) SetId(v int64)`

SetId sets Id field to given value.


### GetMetadata

`func (o *AlertView) GetMetadata() interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *AlertView) GetMetadataOk() (*interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *AlertView) SetMetadata(v interface{})`

SetMetadata sets Metadata field to given value.


### GetStatus

`func (o *AlertView) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *AlertView) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *AlertView) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetTitle

`func (o *AlertView) GetTitle() string`

GetTitle returns the Title field if non-nil, zero value otherwise.

### GetTitleOk

`func (o *AlertView) GetTitleOk() (*string, bool)`

GetTitleOk returns a tuple with the Title field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTitle

`func (o *AlertView) SetTitle(v string)`

SetTitle sets Title field to given value.


### GetType

`func (o *AlertView) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *AlertView) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *AlertView) SetType(v string)`

SetType sets Type field to given value.


### GetUpdatedAt

`func (o *AlertView) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *AlertView) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *AlertView) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
