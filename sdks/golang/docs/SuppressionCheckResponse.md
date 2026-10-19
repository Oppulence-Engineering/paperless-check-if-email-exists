# SuppressionCheckResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CanonicalEmail** | Pointer to **NullableString** |  | [optional]
**CreatedAt** | Pointer to **NullableTime** |  | [optional]
**CreatedBy** | Pointer to **NullableString** |  | [optional]
**ExpiresAt** | Pointer to **NullableTime** |  | [optional]
**Id** | Pointer to **NullableInt32** |  | [optional]
**Metadata** | Pointer to **map[string]interface{}** |  | [optional]
**Reason** | Pointer to [**NullableSuppressionReason**](SuppressionReason.md) |  | [optional]
**ReasonCode** | Pointer to **NullableString** |  | [optional]
**ReasonDetail** | Pointer to **NullableString** |  | [optional]
**Source** | Pointer to **NullableString** |  | [optional]
**SourceRef** | Pointer to **NullableString** |  | [optional]
**SourceType** | Pointer to **NullableString** |  | [optional]
**Status** | Pointer to **NullableString** |  | [optional]
**Suppressed** | **bool** |  | [required]

## Methods

### NewSuppressionCheckResponse

`func NewSuppressionCheckResponse(suppressed bool) *SuppressionCheckResponse`

NewSuppressionCheckResponse instantiates a new SuppressionCheckResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSuppressionCheckResponseWithDefaults

`func NewSuppressionCheckResponseWithDefaults() *SuppressionCheckResponse`

NewSuppressionCheckResponseWithDefaults instantiates a new SuppressionCheckResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCanonicalEmail

`func (o *SuppressionCheckResponse) GetCanonicalEmail() string`

GetCanonicalEmail returns the CanonicalEmail field if non-nil, zero value otherwise.

### GetCanonicalEmailOk

`func (o *SuppressionCheckResponse) GetCanonicalEmailOk() (*string, bool)`

GetCanonicalEmailOk returns a tuple with the CanonicalEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanonicalEmail

`func (o *SuppressionCheckResponse) SetCanonicalEmail(v string)`

SetCanonicalEmail sets CanonicalEmail field to given value.

### HasCanonicalEmail

`func (o *SuppressionCheckResponse) HasCanonicalEmail() bool`

HasCanonicalEmail returns a boolean if a field has been set.

### SetCanonicalEmailNil

`func (o *SuppressionCheckResponse) SetCanonicalEmailNil()`

 SetCanonicalEmailNil sets the value for CanonicalEmail to be an explicit nil

### UnsetCanonicalEmail
`func (o *SuppressionCheckResponse) UnsetCanonicalEmail()`

UnsetCanonicalEmail ensures that no value is present for CanonicalEmail, not even an explicit nil

### GetCreatedAt

`func (o *SuppressionCheckResponse) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *SuppressionCheckResponse) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *SuppressionCheckResponse) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *SuppressionCheckResponse) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *SuppressionCheckResponse) SetCreatedAtNil()`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *SuppressionCheckResponse) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil

### GetCreatedBy

`func (o *SuppressionCheckResponse) GetCreatedBy() string`

GetCreatedBy returns the CreatedBy field if non-nil, zero value otherwise.

### GetCreatedByOk

`func (o *SuppressionCheckResponse) GetCreatedByOk() (*string, bool)`

GetCreatedByOk returns a tuple with the CreatedBy field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedBy

`func (o *SuppressionCheckResponse) SetCreatedBy(v string)`

SetCreatedBy sets CreatedBy field to given value.

### HasCreatedBy

`func (o *SuppressionCheckResponse) HasCreatedBy() bool`

HasCreatedBy returns a boolean if a field has been set.

### SetCreatedByNil

`func (o *SuppressionCheckResponse) SetCreatedByNil()`

 SetCreatedByNil sets the value for CreatedBy to be an explicit nil

### UnsetCreatedBy
`func (o *SuppressionCheckResponse) UnsetCreatedBy()`

UnsetCreatedBy ensures that no value is present for CreatedBy, not even an explicit nil

### GetExpiresAt

`func (o *SuppressionCheckResponse) GetExpiresAt() time.Time`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *SuppressionCheckResponse) GetExpiresAtOk() (*time.Time, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *SuppressionCheckResponse) SetExpiresAt(v time.Time)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *SuppressionCheckResponse) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *SuppressionCheckResponse) SetExpiresAtNil()`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *SuppressionCheckResponse) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil

### GetId

`func (o *SuppressionCheckResponse) GetId() int32`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *SuppressionCheckResponse) GetIdOk() (*int32, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *SuppressionCheckResponse) SetId(v int32)`

SetId sets Id field to given value.

### HasId

`func (o *SuppressionCheckResponse) HasId() bool`

HasId returns a boolean if a field has been set.

### SetIdNil

`func (o *SuppressionCheckResponse) SetIdNil()`

 SetIdNil sets the value for Id to be an explicit nil

### UnsetId
`func (o *SuppressionCheckResponse) UnsetId()`

UnsetId ensures that no value is present for Id, not even an explicit nil

### GetMetadata

`func (o *SuppressionCheckResponse) GetMetadata() map[string]interface{}`

GetMetadata returns the Metadata field if non-nil, zero value otherwise.

### GetMetadataOk

`func (o *SuppressionCheckResponse) GetMetadataOk() (map[string]interface{}, bool)`

GetMetadataOk returns a tuple with the Metadata field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMetadata

`func (o *SuppressionCheckResponse) SetMetadata(v map[string]interface{})`

SetMetadata sets Metadata field to given value.

### HasMetadata

`func (o *SuppressionCheckResponse) HasMetadata() bool`

HasMetadata returns a boolean if a field has been set.

### GetReason

`func (o *SuppressionCheckResponse) GetReason() SuppressionReason`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *SuppressionCheckResponse) GetReasonOk() (*SuppressionReason, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *SuppressionCheckResponse) SetReason(v SuppressionReason)`

SetReason sets Reason field to given value.

### HasReason

`func (o *SuppressionCheckResponse) HasReason() bool`

HasReason returns a boolean if a field has been set.

### SetReasonNil

`func (o *SuppressionCheckResponse) SetReasonNil()`

 SetReasonNil sets the value for Reason to be an explicit nil

### UnsetReason
`func (o *SuppressionCheckResponse) UnsetReason()`

UnsetReason ensures that no value is present for Reason, not even an explicit nil

### GetReasonCode

`func (o *SuppressionCheckResponse) GetReasonCode() string`

GetReasonCode returns the ReasonCode field if non-nil, zero value otherwise.

### GetReasonCodeOk

`func (o *SuppressionCheckResponse) GetReasonCodeOk() (*string, bool)`

GetReasonCodeOk returns a tuple with the ReasonCode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonCode

`func (o *SuppressionCheckResponse) SetReasonCode(v string)`

SetReasonCode sets ReasonCode field to given value.

### HasReasonCode

`func (o *SuppressionCheckResponse) HasReasonCode() bool`

HasReasonCode returns a boolean if a field has been set.

### SetReasonCodeNil

`func (o *SuppressionCheckResponse) SetReasonCodeNil()`

 SetReasonCodeNil sets the value for ReasonCode to be an explicit nil

### UnsetReasonCode
`func (o *SuppressionCheckResponse) UnsetReasonCode()`

UnsetReasonCode ensures that no value is present for ReasonCode, not even an explicit nil

### GetReasonDetail

`func (o *SuppressionCheckResponse) GetReasonDetail() string`

GetReasonDetail returns the ReasonDetail field if non-nil, zero value otherwise.

### GetReasonDetailOk

`func (o *SuppressionCheckResponse) GetReasonDetailOk() (*string, bool)`

GetReasonDetailOk returns a tuple with the ReasonDetail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReasonDetail

`func (o *SuppressionCheckResponse) SetReasonDetail(v string)`

SetReasonDetail sets ReasonDetail field to given value.

### HasReasonDetail

`func (o *SuppressionCheckResponse) HasReasonDetail() bool`

HasReasonDetail returns a boolean if a field has been set.

### SetReasonDetailNil

`func (o *SuppressionCheckResponse) SetReasonDetailNil()`

 SetReasonDetailNil sets the value for ReasonDetail to be an explicit nil

### UnsetReasonDetail
`func (o *SuppressionCheckResponse) UnsetReasonDetail()`

UnsetReasonDetail ensures that no value is present for ReasonDetail, not even an explicit nil

### GetSource

`func (o *SuppressionCheckResponse) GetSource() string`

GetSource returns the Source field if non-nil, zero value otherwise.

### GetSourceOk

`func (o *SuppressionCheckResponse) GetSourceOk() (*string, bool)`

GetSourceOk returns a tuple with the Source field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSource

`func (o *SuppressionCheckResponse) SetSource(v string)`

SetSource sets Source field to given value.

### HasSource

`func (o *SuppressionCheckResponse) HasSource() bool`

HasSource returns a boolean if a field has been set.

### SetSourceNil

`func (o *SuppressionCheckResponse) SetSourceNil()`

 SetSourceNil sets the value for Source to be an explicit nil

### UnsetSource
`func (o *SuppressionCheckResponse) UnsetSource()`

UnsetSource ensures that no value is present for Source, not even an explicit nil

### GetSourceRef

`func (o *SuppressionCheckResponse) GetSourceRef() string`

GetSourceRef returns the SourceRef field if non-nil, zero value otherwise.

### GetSourceRefOk

`func (o *SuppressionCheckResponse) GetSourceRefOk() (*string, bool)`

GetSourceRefOk returns a tuple with the SourceRef field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceRef

`func (o *SuppressionCheckResponse) SetSourceRef(v string)`

SetSourceRef sets SourceRef field to given value.

### HasSourceRef

`func (o *SuppressionCheckResponse) HasSourceRef() bool`

HasSourceRef returns a boolean if a field has been set.

### SetSourceRefNil

`func (o *SuppressionCheckResponse) SetSourceRefNil()`

 SetSourceRefNil sets the value for SourceRef to be an explicit nil

### UnsetSourceRef
`func (o *SuppressionCheckResponse) UnsetSourceRef()`

UnsetSourceRef ensures that no value is present for SourceRef, not even an explicit nil

### GetSourceType

`func (o *SuppressionCheckResponse) GetSourceType() string`

GetSourceType returns the SourceType field if non-nil, zero value otherwise.

### GetSourceTypeOk

`func (o *SuppressionCheckResponse) GetSourceTypeOk() (*string, bool)`

GetSourceTypeOk returns a tuple with the SourceType field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceType

`func (o *SuppressionCheckResponse) SetSourceType(v string)`

SetSourceType sets SourceType field to given value.

### HasSourceType

`func (o *SuppressionCheckResponse) HasSourceType() bool`

HasSourceType returns a boolean if a field has been set.

### SetSourceTypeNil

`func (o *SuppressionCheckResponse) SetSourceTypeNil()`

 SetSourceTypeNil sets the value for SourceType to be an explicit nil

### UnsetSourceType
`func (o *SuppressionCheckResponse) UnsetSourceType()`

UnsetSourceType ensures that no value is present for SourceType, not even an explicit nil

### GetStatus

`func (o *SuppressionCheckResponse) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *SuppressionCheckResponse) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *SuppressionCheckResponse) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *SuppressionCheckResponse) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### SetStatusNil

`func (o *SuppressionCheckResponse) SetStatusNil()`

 SetStatusNil sets the value for Status to be an explicit nil

### UnsetStatus
`func (o *SuppressionCheckResponse) UnsetStatus()`

UnsetStatus ensures that no value is present for Status, not even an explicit nil

### GetSuppressed

`func (o *SuppressionCheckResponse) GetSuppressed() bool`

GetSuppressed returns the Suppressed field if non-nil, zero value otherwise.

### GetSuppressedOk

`func (o *SuppressionCheckResponse) GetSuppressedOk() (*bool, bool)`

GetSuppressedOk returns a tuple with the Suppressed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSuppressed

`func (o *SuppressionCheckResponse) SetSuppressed(v bool)`

SetSuppressed sets Suppressed field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
