# TenantDomainResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | **string** |  | [required]
**Domain** | **string** |  | [required]
**Id** | **string** |  | [required]
**IsActive** | **bool** |  | [required]
**IsVerified** | **bool** |  | [required]
**Notes** | Pointer to **NullableString** |  | [optional]
**TenantId** | **string** |  | [required]
**UpdatedAt** | **string** |  | [required]
**VerificationRecord** | **string** |  | [required]
**VerificationValue** | **string** |  | [required]

## Methods

### NewTenantDomainResponse

`func NewTenantDomainResponse(createdAt string, domain string, id string, isActive bool, isVerified bool, tenantId string, updatedAt string, verificationRecord string, verificationValue string) *TenantDomainResponse`

NewTenantDomainResponse instantiates a new TenantDomainResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTenantDomainResponseWithDefaults

`func NewTenantDomainResponseWithDefaults() *TenantDomainResponse`

NewTenantDomainResponseWithDefaults instantiates a new TenantDomainResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *TenantDomainResponse) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *TenantDomainResponse) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *TenantDomainResponse) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.


### GetDomain

`func (o *TenantDomainResponse) GetDomain() string`

GetDomain returns the Domain field if non-nil, zero value otherwise.

### GetDomainOk

`func (o *TenantDomainResponse) GetDomainOk() (*string, bool)`

GetDomainOk returns a tuple with the Domain field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomain

`func (o *TenantDomainResponse) SetDomain(v string)`

SetDomain sets Domain field to given value.


### GetId

`func (o *TenantDomainResponse) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *TenantDomainResponse) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *TenantDomainResponse) SetId(v string)`

SetId sets Id field to given value.


### GetIsActive

`func (o *TenantDomainResponse) GetIsActive() bool`

GetIsActive returns the IsActive field if non-nil, zero value otherwise.

### GetIsActiveOk

`func (o *TenantDomainResponse) GetIsActiveOk() (*bool, bool)`

GetIsActiveOk returns a tuple with the IsActive field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsActive

`func (o *TenantDomainResponse) SetIsActive(v bool)`

SetIsActive sets IsActive field to given value.


### GetIsVerified

`func (o *TenantDomainResponse) GetIsVerified() bool`

GetIsVerified returns the IsVerified field if non-nil, zero value otherwise.

### GetIsVerifiedOk

`func (o *TenantDomainResponse) GetIsVerifiedOk() (*bool, bool)`

GetIsVerifiedOk returns a tuple with the IsVerified field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsVerified

`func (o *TenantDomainResponse) SetIsVerified(v bool)`

SetIsVerified sets IsVerified field to given value.


### GetNotes

`func (o *TenantDomainResponse) GetNotes() string`

GetNotes returns the Notes field if non-nil, zero value otherwise.

### GetNotesOk

`func (o *TenantDomainResponse) GetNotesOk() (*string, bool)`

GetNotesOk returns a tuple with the Notes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNotes

`func (o *TenantDomainResponse) SetNotes(v string)`

SetNotes sets Notes field to given value.

### HasNotes

`func (o *TenantDomainResponse) HasNotes() bool`

HasNotes returns a boolean if a field has been set.

### SetNotesNil

`func (o *TenantDomainResponse) SetNotesNil()`

 SetNotesNil sets the value for Notes to be an explicit nil

### UnsetNotes
`func (o *TenantDomainResponse) UnsetNotes()`

UnsetNotes ensures that no value is present for Notes, not even an explicit nil

### GetTenantId

`func (o *TenantDomainResponse) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *TenantDomainResponse) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *TenantDomainResponse) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.


### GetUpdatedAt

`func (o *TenantDomainResponse) GetUpdatedAt() string`

GetUpdatedAt returns the UpdatedAt field if non-nil, zero value otherwise.

### GetUpdatedAtOk

`func (o *TenantDomainResponse) GetUpdatedAtOk() (*string, bool)`

GetUpdatedAtOk returns a tuple with the UpdatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUpdatedAt

`func (o *TenantDomainResponse) SetUpdatedAt(v string)`

SetUpdatedAt sets UpdatedAt field to given value.


### GetVerificationRecord

`func (o *TenantDomainResponse) GetVerificationRecord() string`

GetVerificationRecord returns the VerificationRecord field if non-nil, zero value otherwise.

### GetVerificationRecordOk

`func (o *TenantDomainResponse) GetVerificationRecordOk() (*string, bool)`

GetVerificationRecordOk returns a tuple with the VerificationRecord field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVerificationRecord

`func (o *TenantDomainResponse) SetVerificationRecord(v string)`

SetVerificationRecord sets VerificationRecord field to given value.


### GetVerificationValue

`func (o *TenantDomainResponse) GetVerificationValue() string`

GetVerificationValue returns the VerificationValue field if non-nil, zero value otherwise.

### GetVerificationValueOk

`func (o *TenantDomainResponse) GetVerificationValueOk() (*string, bool)`

GetVerificationValueOk returns a tuple with the VerificationValue field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetVerificationValue

`func (o *TenantDomainResponse) SetVerificationValue(v string)`

SetVerificationValue sets VerificationValue field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
