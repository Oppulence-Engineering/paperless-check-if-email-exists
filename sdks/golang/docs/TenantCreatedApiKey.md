# TenantCreatedApiKey

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | **time.Time** |  | [required]
**ExpiresAt** | Pointer to **NullableTime** |  | [optional]
**Id** | **string** |  | [required]
**Key** | **string** | The full key. The API shows it only once. | [required]
**KeyPrefix** | **string** |  | [required]
**Name** | **string** |  | [required]
**Scopes** | **[]string** |  | [required]
**Status** | **string** |  | [required]
**TenantId** | **string** |  | [required]

## Methods

### NewTenantCreatedApiKey

`func NewTenantCreatedApiKey(createdAt time.Time, id string, key string, keyPrefix string, name string, scopes []string, status string, tenantId string) *TenantCreatedApiKey`

NewTenantCreatedApiKey instantiates a new TenantCreatedApiKey object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTenantCreatedApiKeyWithDefaults

`func NewTenantCreatedApiKeyWithDefaults() *TenantCreatedApiKey`

NewTenantCreatedApiKeyWithDefaults instantiates a new TenantCreatedApiKey object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *TenantCreatedApiKey) GetCreatedAt() time.Time`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *TenantCreatedApiKey) GetCreatedAtOk() (*time.Time, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *TenantCreatedApiKey) SetCreatedAt(v time.Time)`

SetCreatedAt sets CreatedAt field to given value.


### GetExpiresAt

`func (o *TenantCreatedApiKey) GetExpiresAt() time.Time`

GetExpiresAt returns the ExpiresAt field if non-nil, zero value otherwise.

### GetExpiresAtOk

`func (o *TenantCreatedApiKey) GetExpiresAtOk() (*time.Time, bool)`

GetExpiresAtOk returns a tuple with the ExpiresAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExpiresAt

`func (o *TenantCreatedApiKey) SetExpiresAt(v time.Time)`

SetExpiresAt sets ExpiresAt field to given value.

### HasExpiresAt

`func (o *TenantCreatedApiKey) HasExpiresAt() bool`

HasExpiresAt returns a boolean if a field has been set.

### SetExpiresAtNil

`func (o *TenantCreatedApiKey) SetExpiresAtNil()`

 SetExpiresAtNil sets the value for ExpiresAt to be an explicit nil

### UnsetExpiresAt
`func (o *TenantCreatedApiKey) UnsetExpiresAt()`

UnsetExpiresAt ensures that no value is present for ExpiresAt, not even an explicit nil

### GetId

`func (o *TenantCreatedApiKey) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *TenantCreatedApiKey) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *TenantCreatedApiKey) SetId(v string)`

SetId sets Id field to given value.


### GetKey

`func (o *TenantCreatedApiKey) GetKey() string`

GetKey returns the Key field if non-nil, zero value otherwise.

### GetKeyOk

`func (o *TenantCreatedApiKey) GetKeyOk() (*string, bool)`

GetKeyOk returns a tuple with the Key field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKey

`func (o *TenantCreatedApiKey) SetKey(v string)`

SetKey sets Key field to given value.


### GetKeyPrefix

`func (o *TenantCreatedApiKey) GetKeyPrefix() string`

GetKeyPrefix returns the KeyPrefix field if non-nil, zero value otherwise.

### GetKeyPrefixOk

`func (o *TenantCreatedApiKey) GetKeyPrefixOk() (*string, bool)`

GetKeyPrefixOk returns a tuple with the KeyPrefix field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKeyPrefix

`func (o *TenantCreatedApiKey) SetKeyPrefix(v string)`

SetKeyPrefix sets KeyPrefix field to given value.


### GetName

`func (o *TenantCreatedApiKey) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *TenantCreatedApiKey) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *TenantCreatedApiKey) SetName(v string)`

SetName sets Name field to given value.


### GetScopes

`func (o *TenantCreatedApiKey) GetScopes() []string`

GetScopes returns the Scopes field if non-nil, zero value otherwise.

### GetScopesOk

`func (o *TenantCreatedApiKey) GetScopesOk() ([]string, bool)`

GetScopesOk returns a tuple with the Scopes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScopes

`func (o *TenantCreatedApiKey) SetScopes(v []string)`

SetScopes sets Scopes field to given value.


### GetStatus

`func (o *TenantCreatedApiKey) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *TenantCreatedApiKey) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *TenantCreatedApiKey) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetTenantId

`func (o *TenantCreatedApiKey) GetTenantId() string`

GetTenantId returns the TenantId field if non-nil, zero value otherwise.

### GetTenantIdOk

`func (o *TenantCreatedApiKey) GetTenantIdOk() (*string, bool)`

GetTenantIdOk returns a tuple with the TenantId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantId

`func (o *TenantCreatedApiKey) SetTenantId(v string)`

SetTenantId sets TenantId field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
