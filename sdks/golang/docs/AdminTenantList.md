# AdminTenantList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Tenants** | Pointer to [**[]AdminTenant**](AdminTenant.md) |  | [optional]
**Total** | Pointer to **int64** |  | [optional]

## Methods

### NewAdminTenantList

`func NewAdminTenantList() *AdminTenantList`

NewAdminTenantList instantiates a new AdminTenantList object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdminTenantListWithDefaults

`func NewAdminTenantListWithDefaults() *AdminTenantList`

NewAdminTenantListWithDefaults instantiates a new AdminTenantList object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetTenants

`func (o *AdminTenantList) GetTenants() []AdminTenant`

GetTenants returns the Tenants field if non-nil, zero value otherwise.

### GetTenantsOk

`func (o *AdminTenantList) GetTenantsOk() ([]AdminTenant, bool)`

GetTenantsOk returns a tuple with the Tenants field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenants

`func (o *AdminTenantList) SetTenants(v []AdminTenant)`

SetTenants sets Tenants field to given value.

### HasTenants

`func (o *AdminTenantList) HasTenants() bool`

HasTenants returns a boolean if a field has been set.

### GetTotal

`func (o *AdminTenantList) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *AdminTenantList) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *AdminTenantList) SetTotal(v int64)`

SetTotal sets Total field to given value.

### HasTotal

`func (o *AdminTenantList) HasTotal() bool`

HasTotal returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
