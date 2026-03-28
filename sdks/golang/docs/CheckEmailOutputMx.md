# CheckEmailOutputMx

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AcceptsMail** | **bool** | Indicates if the mail server accepts emails. | [optional]
**Records** | **[]string** | List of Fully Qualified Domain Names (FQDN) of the mail server. | [optional]
**Message** | **string** | A human-readable description of the error. | [optional]
**Type** | **string** | The type of error. | [optional]

## Methods

### NewCheckEmailOutputMx

`func NewCheckEmailOutputMx(acceptsMail bool, records []string, message string, type_ string) *CheckEmailOutputMx`

NewCheckEmailOutputMx instantiates a new CheckEmailOutputMx object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCheckEmailOutputMxWithDefaults

`func NewCheckEmailOutputMxWithDefaults() *CheckEmailOutputMx`

NewCheckEmailOutputMxWithDefaults instantiates a new CheckEmailOutputMx object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAcceptsMail

`func (o *CheckEmailOutputMx) GetAcceptsMail() bool`

GetAcceptsMail returns the AcceptsMail field if non-nil, zero value otherwise.

### GetAcceptsMailOk

`func (o *CheckEmailOutputMx) GetAcceptsMailOk() (*bool, bool)`

GetAcceptsMailOk returns a tuple with the AcceptsMail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAcceptsMail

`func (o *CheckEmailOutputMx) SetAcceptsMail(v bool)`

SetAcceptsMail sets AcceptsMail field to given value.


### GetRecords

`func (o *CheckEmailOutputMx) GetRecords() []string`

GetRecords returns the Records field if non-nil, zero value otherwise.

### GetRecordsOk

`func (o *CheckEmailOutputMx) GetRecordsOk() ([]string, bool)`

GetRecordsOk returns a tuple with the Records field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecords

`func (o *CheckEmailOutputMx) SetRecords(v []string)`

SetRecords sets Records field to given value.


### GetMessage

`func (o *CheckEmailOutputMx) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *CheckEmailOutputMx) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *CheckEmailOutputMx) SetMessage(v string)`

SetMessage sets Message field to given value.


### GetType

`func (o *CheckEmailOutputMx) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *CheckEmailOutputMx) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *CheckEmailOutputMx) SetType(v string)`

SetType sets Type field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


