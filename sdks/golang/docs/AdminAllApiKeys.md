# AdminAllApiKeys

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ApiKeys** | Pointer to [**[]AdminApiKey**](AdminApiKey.md) |  | [optional]
**Total** | Pointer to **int64** |  | [optional]

## Methods

### NewAdminAllApiKeys

`func NewAdminAllApiKeys() *AdminAllApiKeys`

NewAdminAllApiKeys instantiates a new AdminAllApiKeys object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAdminAllApiKeysWithDefaults

`func NewAdminAllApiKeysWithDefaults() *AdminAllApiKeys`

NewAdminAllApiKeysWithDefaults instantiates a new AdminAllApiKeys object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetApiKeys

`func (o *AdminAllApiKeys) GetApiKeys() []AdminApiKey`

GetApiKeys returns the ApiKeys field if non-nil, zero value otherwise.

### GetApiKeysOk

`func (o *AdminAllApiKeys) GetApiKeysOk() ([]AdminApiKey, bool)`

GetApiKeysOk returns a tuple with the ApiKeys field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetApiKeys

`func (o *AdminAllApiKeys) SetApiKeys(v []AdminApiKey)`

SetApiKeys sets ApiKeys field to given value.

### HasApiKeys

`func (o *AdminAllApiKeys) HasApiKeys() bool`

HasApiKeys returns a boolean if a field has been set.

### GetTotal

`func (o *AdminAllApiKeys) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *AdminAllApiKeys) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *AdminAllApiKeys) SetTotal(v int64)`

SetTotal sets Total field to given value.

### HasTotal

`func (o *AdminAllApiKeys) HasTotal() bool`

HasTotal returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
