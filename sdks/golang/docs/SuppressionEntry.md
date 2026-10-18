# SuppressionEntry

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CanonicalEmail** | **string** |  | [required]
**CreatedAt** | **time.Time** |  | [required]
**CreatedBy** | Pointer to **NullableString** |  | [optional]
**Email** | **string** |  | [required]
**ExpiresAt** | Pointer to **NullableTime** |  | [optional]
**Id** | **int32** |  | [required]
**LastSeenAt** | Pointer to **NullableTime** |  | [optional]
**Metadata** | **map[string]interface{}** |  | [required]
**Notes** | Pointer to **NullableString** |  | [optional]
**Reason** | [**SuppressionReason**](SuppressionReason.md) |  | [required]
**ReasonCode** | Pointer to **NullableString** |  | [optional]
**ReasonDetail** | Pointer to **NullableString** |  | [optional]
**Source** | Pointer to **NullableString** |  | [optional]
**SourceRef** | Pointer to **NullableString** |  | [optional]
**SourceType** | Pointer to **NullableString** |  | [optional]
**Status** | **string** |  | [required]
**UpdatedAt** | **time.Time** |  | [required]

## Methods

### NewSuppressionEntry

`func NewSuppressionEntry(canonicalEmail string, createdAt time.Time, email string, id int32, metadata map[string]interface{}, reason SuppressionReason, status string, updatedAt time.Time) *SuppressionEntry`

NewSuppressionEntry instantiates a new SuppressionEntry object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSuppressionEntryWithDefaults

`func NewSuppressionEntryWithDefaults() *SuppressionEntry`

NewSuppressionEntryWithDefaults instantiates a new SuppressionEntry object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCanonicalEmail

`func (o *SuppressionEntry) GetCanonicalEmail() string`

GetCanonicalEmail returns the CanonicalEmail field if non-nil, zero value otherwise.

### GetCanonicalEmailOk

`func (o *SuppressionEntry) GetCanonicalEmailOk() (*string, bool)`

GetCanonicalEmailOk returns a tuple with the CanonicalEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanonicalEmail

`func (o *SuppressionEntry) SetCanonicalEmail(v string)`

SetCanonicalEmail sets CanonicalEmail field to given value.


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


### GetCreatedBy

`func (o *SuppressionEntry) GetCreatedBy() string`

GetCreatedBy returns the CreatedBy field if non-nil, zero value otherwise.

### GetCreatedByOk

`func (o *SuppressionEntry) GetCreatedByOk() (*string, bool)`

GetCreatedByOk returns a tuple with the CreatedBy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedBy

`func (o *SuppressionEntry) SetCreatedBy(v string)`

SetCreatedBy sets CreatedBy field to given value.

### HasCreatedBy

`func (o *SuppressionEntry) HasCreatedBy() bool`

HasCreatedBy returns a boolean if a field has been set.

### SetCreatedByNil

`func (o *SuppressionEntry) SetCreatedByNil()`

 SetCreatedByNil sets the value for CreatedBy to be an explicit nil

### UnsetCreatedBy
`func (o *SuppressionEntry) UnsetCreatedBy()`

UnsetCreatedBy ensures that no value is present for CreatedBy, not even an explicit nil

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


### GetExpiresAt

`func (o *SuppressionEntry) GetExpiresAt() time.Time`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *SuppressionEntry) GetExpiresAtOk() (*time.Time, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *SuppressionEntry) SetExpiresAt(v time.Time)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *SuppressionEntry) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *SuppressionEntry) SetExpiresAtNil()`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *SuppressionEntry) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil

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


### GetLastSeenAt

`func (o *SuppressionEntry) GetLastSeenAt() time.Time`

GetLastSeenAt returns the LastSeenAt field if non-nil, zero value otherwise.

### GetLastSeenAtOk

`func (o *SuppressionEntry) GetLastSeenAtOk() (*time.Time, bool)`

GetLastSeenAtOk returns a tuple with the LastSeenAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastSeenAt

`func (o *SuppressionEntry) SetLastSeenAt(v time.Time)`

SetLastSeenAt sets LastSeenAt field to given value.

### HasLastSeenAt

`func (o *SuppressionEntry) HasLastSeenAt() bool`

HasLastSeenAt returns a boolean if a field has been set.

### SetLastSeenAtNil

`func (o *SuppressionEntry) SetLastSeenAtNil()`

 SetLastSeenAtNil sets the value for LastSeenAt to be an explicit nil

### UnsetLastSeenAt
`func (o *SuppressionEntry) UnsetLastSeenAt()`

UnsetLastSeenAt ensures that no value is present for LastSeenAt, not even an explicit nil

### GetMetadata

`func (o *SuppressionEntry) GetMetadata() map[string]interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *SuppressionEntry) GetMetadataOk() (map[string]interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *SuppressionEntry) SetMetadata(v map[string]interface{})`

SetMetadata sets Metadata field to given value.


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

`func (o *SuppressionEntry) SetNotesNil()`

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


### GetReasonCode

`func (o *SuppressionEntry) GetReasonCode() string`

GetReasonCode returns the ReasonCode field if non-nil, zero value otherwise.

### GetReasonCodeOk

`func (o *SuppressionEntry) GetReasonCodeOk() (*string, bool)`

GetReasonCodeOk returns a tuple with the ReasonCode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonCode

`func (o *SuppressionEntry) SetReasonCode(v string)`

SetReasonCode sets ReasonCode field to given value.

### HasReasonCode

`func (o *SuppressionEntry) HasReasonCode() bool`

HasReasonCode returns a boolean if a field has been set.

### SetReasonCodeNil

`func (o *SuppressionEntry) SetReasonCodeNil()`

 SetReasonCodeNil sets the value for ReasonCode to be an explicit nil

### UnsetReasonCode
`func (o *SuppressionEntry) UnsetReasonCode()`

UnsetReasonCode ensures that no value is present for ReasonCode, not even an explicit nil

### GetReasonDetail

`func (o *SuppressionEntry) GetReasonDetail() string`

GetReasonDetail returns the ReasonDetail field if non-nil, zero value otherwise.

### GetReasonDetailOk

`func (o *SuppressionEntry) GetReasonDetailOk() (*string, bool)`

GetReasonDetailOk returns a tuple with the ReasonDetail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonDetail

`func (o *SuppressionEntry) SetReasonDetail(v string)`

SetReasonDetail sets ReasonDetail field to given value.

### HasReasonDetail

`func (o *SuppressionEntry) HasReasonDetail() bool`

HasReasonDetail returns a boolean if a field has been set.

### SetReasonDetailNil

`func (o *SuppressionEntry) SetReasonDetailNil()`

 SetReasonDetailNil sets the value for ReasonDetail to be an explicit nil

### UnsetReasonDetail
`func (o *SuppressionEntry) UnsetReasonDetail()`

UnsetReasonDetail ensures that no value is present for ReasonDetail, not even an explicit nil

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

`func (o *SuppressionEntry) SetSourceNil()`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *SuppressionEntry) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

### GetSourceRef

`func (o *SuppressionEntry) GetSourceRef() string`

GetSourceRef returns the SourceRef field if non-nil, zero value otherwise.

### GetSourceRefOk

`func (o *SuppressionEntry) GetSourceRefOk() (*string, bool)`

GetSourceRefOk returns a tuple with the SourceRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceRef

`func (o *SuppressionEntry) SetSourceRef(v string)`

SetSourceRef sets SourceRef field to given value.

### HasSourceRef

`func (o *SuppressionEntry) HasSourceRef() bool`

HasSourceRef returns a boolean if a field has been set.

### SetSourceRefNil

`func (o *SuppressionEntry) SetSourceRefNil()`

 SetSourceRefNil sets the value for SourceRef to be an explicit nil

### UnsetSourceRef
`func (o *SuppressionEntry) UnsetSourceRef()`

UnsetSourceRef ensures that no value is present for SourceRef, not even an explicit nil

### GetSourceType

`func (o *SuppressionEntry) GetSourceType() string`

GetSourceType returns the SourceType field if non-nil, zero value otherwise.

### GetSourceTypeOk

`func (o *SuppressionEntry) GetSourceTypeOk() (*string, bool)`

GetSourceTypeOk returns a tuple with the SourceType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceType

`func (o *SuppressionEntry) SetSourceType(v string)`

SetSourceType sets SourceType field to given value.

### HasSourceType

`func (o *SuppressionEntry) HasSourceType() bool`

HasSourceType returns a boolean if a field has been set.

### SetSourceTypeNil

`func (o *SuppressionEntry) SetSourceTypeNil()`

 SetSourceTypeNil sets the value for SourceType to be an explicit nil

### UnsetSourceType
`func (o *SuppressionEntry) UnsetSourceType()`

UnsetSourceType ensures that no value is present for SourceType, not even an explicit nil

### GetStatus

`func (o *SuppressionEntry) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *SuppressionEntry) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *SuppressionEntry) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetUpdatedAt

`func (o *SuppressionEntry) GetUpdatedAt() time.Time`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *SuppressionEntry) GetUpdatedAtOk() (*time.Time, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *SuppressionEntry) SetUpdatedAt(v time.Time)`

SetUpdatedAt sets UpdatedAt field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
