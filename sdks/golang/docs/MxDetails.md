# MxDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AcceptsMail** | **bool** | Indicates if the mail server accepts emails. | [required]
**Records** | **[]string** | List of Fully Qualified Domain Names (FQDN) of the mail server. | [required]

## Methods

### NewMxDetails

`func NewMxDetails(acceptsMail bool, records []string) *MxDetails`

NewMxDetails instantiates a new MxDetails object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewMxDetailsWithDefaults

`func NewMxDetailsWithDefaults() *MxDetails`

NewMxDetailsWithDefaults instantiates a new MxDetails object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAcceptsMail

`func (o *MxDetails) GetAcceptsMail() bool`

GetAcceptsMail returns the AcceptsMail field if non-nil, zero value otherwise.

### GetAcceptsMailOk

`func (o *MxDetails) GetAcceptsMailOk() (*bool, bool)`

GetAcceptsMailOk returns a tuple with the AcceptsMail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAcceptsMail

`func (o *MxDetails) SetAcceptsMail(v bool)`

SetAcceptsMail sets AcceptsMail field to given value.


### GetRecords

`func (o *MxDetails) GetRecords() []string`

GetRecords returns the Records field if non-nil, zero value otherwise.

### GetRecordsOk

`func (o *MxDetails) GetRecordsOk() ([]string, bool)`

GetRecordsOk returns a tuple with the Records field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRecords

`func (o *MxDetails) SetRecords(v []string)`

SetRecords sets Records field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


