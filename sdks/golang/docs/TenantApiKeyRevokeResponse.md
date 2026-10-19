# TenantApiKeyRevokeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**KeyId** | **string** |  | [required]
**Revoked** | **bool** |  | [required]

## Methods

### NewTenantApiKeyRevokeResponse

`func NewTenantApiKeyRevokeResponse(keyId string, revoked bool) *TenantApiKeyRevokeResponse`

NewTenantApiKeyRevokeResponse instantiates a new TenantApiKeyRevokeResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewTenantApiKeyRevokeResponseWithDefaults

`func NewTenantApiKeyRevokeResponseWithDefaults() *TenantApiKeyRevokeResponse`

NewTenantApiKeyRevokeResponseWithDefaults instantiates a new TenantApiKeyRevokeResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetKeyId

`func (o *TenantApiKeyRevokeResponse) GetKeyId() string`

GetKeyId returns the KeyId field if non-nil, zero value otherwise.

### GetKeyIdOk

`func (o *TenantApiKeyRevokeResponse) GetKeyIdOk() (*string, bool)`

GetKeyIdOk returns a tuple with the KeyId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetKeyId

`func (o *TenantApiKeyRevokeResponse) SetKeyId(v string)`

SetKeyId sets KeyId field to given value.


### GetRevoked

`func (o *TenantApiKeyRevokeResponse) GetRevoked() bool`

GetRevoked returns the Revoked field if non-nil, zero value otherwise.

### GetRevokedOk

`func (o *TenantApiKeyRevokeResponse) GetRevokedOk() (*bool, bool)`

GetRevokedOk returns a tuple with the Revoked field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRevoked

`func (o *TenantApiKeyRevokeResponse) SetRevoked(v bool)`

SetRevoked sets Revoked field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
