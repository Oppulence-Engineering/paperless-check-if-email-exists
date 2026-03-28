# SuppressionCheckResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableTime** |  | [optional]
**Reason** | Pointer to [**SuppressionReason**](SuppressionReason.md) |  | [optional]
**Source** | Pointer to **NullableString** |  | [optional]
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
