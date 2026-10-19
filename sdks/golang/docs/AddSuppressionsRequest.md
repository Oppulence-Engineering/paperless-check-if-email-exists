# AddSuppressionsRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Emails** | **[]string** |  | [required]
**ExpiresAt** | Pointer to **NullableTime** |  | [optional]
**Metadata** | Pointer to **map[string]interface{}** |  | [optional]
**Notes** | Pointer to **NullableString** |  | [optional]
**Reason** | Pointer to [**SuppressionReason**](SuppressionReason.md) |  | [optional]
**ReasonDetail** | Pointer to **NullableString** |  | [optional]
**Source** | Pointer to **NullableString** |  | [optional]
**SourceRef** | Pointer to **NullableString** |  | [optional]
**SourceType** | Pointer to **NullableString** |  | [optional]

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


### GetExpiresAt

`func (o *AddSuppressionsRequest) GetExpiresAt() time.Time`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *AddSuppressionsRequest) GetExpiresAtOk() (*time.Time, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *AddSuppressionsRequest) SetExpiresAt(v time.Time)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *AddSuppressionsRequest) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *AddSuppressionsRequest) SetExpiresAtNil()`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *AddSuppressionsRequest) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil

### GetMetadata

`func (o *AddSuppressionsRequest) GetMetadata() map[string]interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *AddSuppressionsRequest) GetMetadataOk() (map[string]interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *AddSuppressionsRequest) SetMetadata(v map[string]interface{})`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *AddSuppressionsRequest) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

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

### GetReasonDetail

`func (o *AddSuppressionsRequest) GetReasonDetail() string`

GetReasonDetail returns the ReasonDetail field if non-nil, zero value otherwise.

### GetReasonDetailOk

`func (o *AddSuppressionsRequest) GetReasonDetailOk() (*string, bool)`

GetReasonDetailOk returns a tuple with the ReasonDetail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonDetail

`func (o *AddSuppressionsRequest) SetReasonDetail(v string)`

SetReasonDetail sets ReasonDetail field to given value.

### HasReasonDetail

`func (o *AddSuppressionsRequest) HasReasonDetail() bool`

HasReasonDetail returns a boolean if a field has been set.

### SetReasonDetailNil

`func (o *AddSuppressionsRequest) SetReasonDetailNil()`

 SetReasonDetailNil sets the value for ReasonDetail to be an explicit nil

### UnsetReasonDetail
`func (o *AddSuppressionsRequest) UnsetReasonDetail()`

UnsetReasonDetail ensures that no value is present for ReasonDetail, not even an explicit nil

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

### GetSourceRef

`func (o *AddSuppressionsRequest) GetSourceRef() string`

GetSourceRef returns the SourceRef field if non-nil, zero value otherwise.

### GetSourceRefOk

`func (o *AddSuppressionsRequest) GetSourceRefOk() (*string, bool)`

GetSourceRefOk returns a tuple with the SourceRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceRef

`func (o *AddSuppressionsRequest) SetSourceRef(v string)`

SetSourceRef sets SourceRef field to given value.

### HasSourceRef

`func (o *AddSuppressionsRequest) HasSourceRef() bool`

HasSourceRef returns a boolean if a field has been set.

### SetSourceRefNil

`func (o *AddSuppressionsRequest) SetSourceRefNil()`

 SetSourceRefNil sets the value for SourceRef to be an explicit nil

### UnsetSourceRef
`func (o *AddSuppressionsRequest) UnsetSourceRef()`

UnsetSourceRef ensures that no value is present for SourceRef, not even an explicit nil

### GetSourceType

`func (o *AddSuppressionsRequest) GetSourceType() string`

GetSourceType returns the SourceType field if non-nil, zero value otherwise.

### GetSourceTypeOk

`func (o *AddSuppressionsRequest) GetSourceTypeOk() (*string, bool)`

GetSourceTypeOk returns a tuple with the SourceType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceType

`func (o *AddSuppressionsRequest) SetSourceType(v string)`

SetSourceType sets SourceType field to given value.

### HasSourceType

`func (o *AddSuppressionsRequest) HasSourceType() bool`

HasSourceType returns a boolean if a field has been set.

### SetSourceTypeNil

`func (o *AddSuppressionsRequest) SetSourceTypeNil()`

 SetSourceTypeNil sets the value for SourceType to be an explicit nil

### UnsetSourceType
`func (o *AddSuppressionsRequest) UnsetSourceType()`

UnsetSourceType ensures that no value is present for SourceType, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
