# AdminCreatedApiKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **time.Time** |  | [optional]
**ExpiresAt** | Pointer to **NullableTime** |  | [optional]
**Id** | Pointer to **string** |  | [optional]
**KeyPrefix** | Pointer to **string** |  | [optional]
**LastUsedAt** | Pointer to **NullableTime** |  | [optional]
**Name** | Pointer to **string** |  | [optional]
**Scopes** | Pointer to **[]string** |  | [optional]
**Status** | Pointer to **string** |  | [optional]
**TenantId** | Pointer to **string** |  | [optional]
**Key** | **string** | Plaintext key returned only at creation. | [required]

## Methods

### NewAdminCreatedApiKey

`func NewAdminCreatedApiKey(key string) *AdminCreatedApiKey`

NewAdminCreatedApiKey instantiates a new AdminCreatedApiKey object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdminCreatedApiKeyWithDefaults

`func NewAdminCreatedApiKeyWithDefaults() *AdminCreatedApiKey`

NewAdminCreatedApiKeyWithDefaults instantiates a new AdminCreatedApiKey object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *AdminCreatedApiKey) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *AdminCreatedApiKey) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *AdminCreatedApiKey) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *AdminCreatedApiKey) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### GetExpiresAt

`func (o *AdminCreatedApiKey) GetExpiresAt() time.Time`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *AdminCreatedApiKey) GetExpiresAtOk() (*time.Time, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *AdminCreatedApiKey) SetExpiresAt(v time.Time)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *AdminCreatedApiKey) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *AdminCreatedApiKey) SetExpiresAtNil()`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *AdminCreatedApiKey) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil

### GetId

`func (o *AdminCreatedApiKey) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *AdminCreatedApiKey) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *AdminCreatedApiKey) SetId(v string)`

SetId sets Id field to given value.

### HasId

`func (o *AdminCreatedApiKey) HasId() bool`

HasId returns a boolean if a field has been set.

### GetKeyPrefix

`func (o *AdminCreatedApiKey) GetKeyPrefix() string`

GetKeyPrefix returns the KeyPrefix field if non-nil, zero value otherwise.

### GetKeyPrefixOk

`func (o *AdminCreatedApiKey) GetKeyPrefixOk() (*string, bool)`

GetKeyPrefixOk returns a tuple with the KeyPrefix field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKeyPrefix

`func (o *AdminCreatedApiKey) SetKeyPrefix(v string)`

SetKeyPrefix sets KeyPrefix field to given value.

### HasKeyPrefix

`func (o *AdminCreatedApiKey) HasKeyPrefix() bool`

HasKeyPrefix returns a boolean if a field has been set.

### GetLastUsedAt

`func (o *AdminCreatedApiKey) GetLastUsedAt() time.Time`

GetLastUsedAt returns the LastUsedAt field if non-nil, zero value otherwise.

### GetLastUsedAtOk

`func (o *AdminCreatedApiKey) GetLastUsedAtOk() (*time.Time, bool)`

GetLastUsedAtOk returns a tuple with the LastUsedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLastUsedAt

`func (o *AdminCreatedApiKey) SetLastUsedAt(v time.Time)`

SetLastUsedAt sets LastUsedAt field to given value.

### HasLastUsedAt

`func (o *AdminCreatedApiKey) HasLastUsedAt() bool`

HasLastUsedAt returns a boolean if a field has been set.

### SetLastUsedAtNil

`func (o *AdminCreatedApiKey) SetLastUsedAtNil()`

 SetLastUsedAtNil sets the value for LastUsedAt to be an explicit nil

### UnsetLastUsedAt
`func (o *AdminCreatedApiKey) UnsetLastUsedAt()`

UnsetLastUsedAt ensures that no value is present for LastUsedAt, not even an explicit nil

### GetName

`func (o *AdminCreatedApiKey) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *AdminCreatedApiKey) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *AdminCreatedApiKey) SetName(v string)`

SetName sets Name field to given value.

### HasName

`func (o *AdminCreatedApiKey) HasName() bool`

HasName returns a boolean if a field has been set.

### GetScopes

`func (o *AdminCreatedApiKey) GetScopes() []string`

GetScopes returns the Scopes field if non-nil, zero value otherwise.

### GetScopesOk

`func (o *AdminCreatedApiKey) GetScopesOk() ([]string, bool)`

GetScopesOk returns a tuple with the Scopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopes

`func (o *AdminCreatedApiKey) SetScopes(v []string)`

SetScopes sets Scopes field to given value.

### HasScopes

`func (o *AdminCreatedApiKey) HasScopes() bool`

HasScopes returns a boolean if a field has been set.

### GetStatus

`func (o *AdminCreatedApiKey) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *AdminCreatedApiKey) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *AdminCreatedApiKey) SetStatus(v string)`

SetStatus sets Status field to given value.

### HasStatus

`func (o *AdminCreatedApiKey) HasStatus() bool`

HasStatus returns a boolean if a field has been set.

### GetTenantId

`func (o *AdminCreatedApiKey) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *AdminCreatedApiKey) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *AdminCreatedApiKey) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.

### HasTenantId

`func (o *AdminCreatedApiKey) HasTenantId() bool`

HasTenantId returns a boolean if a field has been set.

### GetKey

`func (o *AdminCreatedApiKey) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *AdminCreatedApiKey) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *AdminCreatedApiKey) SetKey(v string)`

SetKey sets Key field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
