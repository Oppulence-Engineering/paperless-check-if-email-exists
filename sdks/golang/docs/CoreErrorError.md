# CoreErrorError

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Message** | Pointer to **interface{}** | The error details. Usually a string. | [optional]
**Type** | **string** | The type of error. | [required]

## Methods

### NewCoreErrorError

`func NewCoreErrorError(type_ string) *CoreErrorError`

NewCoreErrorError instantiates a new CoreErrorError object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCoreErrorErrorWithDefaults

`func NewCoreErrorErrorWithDefaults() *CoreErrorError`

NewCoreErrorErrorWithDefaults instantiates a new CoreErrorError object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMessage

`func (o *CoreErrorError) GetMessage() interface{}`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *CoreErrorError) GetMessageOk() (*interface{}, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *CoreErrorError) SetMessage(v interface{})`

SetMessage sets Message field to given value.

### HasMessage

`func (o *CoreErrorError) HasMessage() bool`

HasMessage returns a boolean if a field has been set.

### GetType

`func (o *CoreErrorError) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *CoreErrorError) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *CoreErrorError) SetType(v string)`

SetType sets Type field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
