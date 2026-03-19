# CheckEmailInputProxy

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Host** | **string** | The proxy host address. | 
**Password** | Pointer to **string** | Password for proxy authentication. | [optional] 
**Port** | **int32** | The proxy port number. | 
**Username** | Pointer to **string** | Username for proxy authentication. | [optional] 

## Methods

### NewCheckEmailInputProxy

`func NewCheckEmailInputProxy(host string, port int32, ) *CheckEmailInputProxy`

NewCheckEmailInputProxy instantiates a new CheckEmailInputProxy object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCheckEmailInputProxyWithDefaults

`func NewCheckEmailInputProxyWithDefaults() *CheckEmailInputProxy`

NewCheckEmailInputProxyWithDefaults instantiates a new CheckEmailInputProxy object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetHost

`func (o *CheckEmailInputProxy) GetHost() string`

GetHost returns the Host field if non-nil, zero value otherwise.

### GetHostOk

`func (o *CheckEmailInputProxy) GetHostOk() (*string, bool)`

GetHostOk returns a tuple with the Host field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHost

`func (o *CheckEmailInputProxy) SetHost(v string)`

SetHost sets Host field to given value.


### GetPassword

`func (o *CheckEmailInputProxy) GetPassword() string`

GetPassword returns the Password field if non-nil, zero value otherwise.

### GetPasswordOk

`func (o *CheckEmailInputProxy) GetPasswordOk() (*string, bool)`

GetPasswordOk returns a tuple with the Password field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPassword

`func (o *CheckEmailInputProxy) SetPassword(v string)`

SetPassword sets Password field to given value.

### HasPassword

`func (o *CheckEmailInputProxy) HasPassword() bool`

HasPassword returns a boolean if a field has been set.

### GetPort

`func (o *CheckEmailInputProxy) GetPort() int32`

GetPort returns the Port field if non-nil, zero value otherwise.

### GetPortOk

`func (o *CheckEmailInputProxy) GetPortOk() (*int32, bool)`

GetPortOk returns a tuple with the Port field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPort

`func (o *CheckEmailInputProxy) SetPort(v int32)`

SetPort sets Port field to given value.


### GetUsername

`func (o *CheckEmailInputProxy) GetUsername() string`

GetUsername returns the Username field if non-nil, zero value otherwise.

### GetUsernameOk

`func (o *CheckEmailInputProxy) GetUsernameOk() (*string, bool)`

GetUsernameOk returns a tuple with the Username field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsername

`func (o *CheckEmailInputProxy) SetUsername(v string)`

SetUsername sets Username field to given value.

### HasUsername

`func (o *CheckEmailInputProxy) HasUsername() bool`

HasUsername returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


