# SuppressionEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | **time.Time** |  | 
**Email** | **string** |  | 
**Id** | **int32** |  | 
**Notes** | Pointer to **NullableString** |  | [optional] 
**Reason** | [**SuppressionReason**](SuppressionReason.md) |  | 
**Source** | Pointer to **NullableString** |  | [optional] 

## Methods

### NewSuppressionEntry

`func NewSuppressionEntry(createdAt time.Time, email string, id int32, reason SuppressionReason, ) *SuppressionEntry`

NewSuppressionEntry instantiates a new SuppressionEntry object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSuppressionEntryWithDefaults

`func NewSuppressionEntryWithDefaults() *SuppressionEntry`

NewSuppressionEntryWithDefaults instantiates a new SuppressionEntry object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *SuppressionEntry) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *SuppressionEntry) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *SuppressionEntry) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetEmail

`func (o *SuppressionEntry) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *SuppressionEntry) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *SuppressionEntry) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetId

`func (o *SuppressionEntry) GetId() int32`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *SuppressionEntry) GetIdOk() (*int32, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *SuppressionEntry) SetId(v int32)`

SetId sets Id field to given value.


### GetNotes

`func (o *SuppressionEntry) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *SuppressionEntry) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *SuppressionEntry) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *SuppressionEntry) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *SuppressionEntry) SetNotesNil(b bool)`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *SuppressionEntry) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil
### GetReason

`func (o *SuppressionEntry) GetReason() SuppressionReason`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *SuppressionEntry) GetReasonOk() (*SuppressionReason, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *SuppressionEntry) SetReason(v SuppressionReason)`

SetReason sets Reason field to given value.


### GetSource

`func (o *SuppressionEntry) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *SuppressionEntry) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *SuppressionEntry) SetSource(v string)`

SetSource sets Source field to given value.

### HasSource

`func (o *SuppressionEntry) HasSource() bool`

HasSource returns a boolean if a field has been set.

### SetSourceNil

`func (o *SuppressionEntry) SetSourceNil(b bool)`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *SuppressionEntry) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


