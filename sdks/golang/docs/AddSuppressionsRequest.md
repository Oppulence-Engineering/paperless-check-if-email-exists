# AddSuppressionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Emails** | **[]string** |  | [required]
**Notes** | Pointer to **NullableString** |  | [optional]
**Reason** | Pointer to [**SuppressionReason**](SuppressionReason.md) |  | [optional]
**Source** | Pointer to **NullableString** |  | [optional]

## Methods

### NewAddSuppressionsRequest

`func NewAddSuppressionsRequest(emails []string) *AddSuppressionsRequest`

NewAddSuppressionsRequest instantiates a new AddSuppressionsRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAddSuppressionsRequestWithDefaults

`func NewAddSuppressionsRequestWithDefaults() *AddSuppressionsRequest`

NewAddSuppressionsRequestWithDefaults instantiates a new AddSuppressionsRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmails

`func (o *AddSuppressionsRequest) GetEmails() []string`

GetEmails returns the Emails field if non-nil, zero value otherwise.

### GetEmailsOk

`func (o *AddSuppressionsRequest) GetEmailsOk() ([]string, bool)`

GetEmailsOk returns a tuple with the Emails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmails

`func (o *AddSuppressionsRequest) SetEmails(v []string)`

SetEmails sets Emails field to given value.


### GetNotes

`func (o *AddSuppressionsRequest) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *AddSuppressionsRequest) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *AddSuppressionsRequest) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *AddSuppressionsRequest) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *AddSuppressionsRequest) SetNotesNil()`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *AddSuppressionsRequest) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil

### GetReason

`func (o *AddSuppressionsRequest) GetReason() SuppressionReason`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *AddSuppressionsRequest) GetReasonOk() (*SuppressionReason, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *AddSuppressionsRequest) SetReason(v SuppressionReason)`

SetReason sets Reason field to given value.

### HasReason

`func (o *AddSuppressionsRequest) HasReason() bool`

HasReason returns a boolean if a field has been set.

### GetSource

`func (o *AddSuppressionsRequest) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *AddSuppressionsRequest) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *AddSuppressionsRequest) SetSource(v string)`

SetSource sets Source field to given value.

### HasSource

`func (o *AddSuppressionsRequest) HasSource() bool`

HasSource returns a boolean if a field has been set.

### SetSourceNil

`func (o *AddSuppressionsRequest) SetSourceNil()`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *AddSuppressionsRequest) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
