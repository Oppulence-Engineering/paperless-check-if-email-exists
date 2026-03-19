# DebugDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Duration** | [**Duration**](Duration.md) |  | 
**EndTime** | **string** | The timestamp when the email verification ended. | 
**ServerName** | **string** | The name of the server that performed the verification. | 
**Smtp** | [**DebugDetailsSmtp**](DebugDetailsSmtp.md) |  | 
**StartTime** | **string** | The timestamp when the email verification started. | 

## Methods

### NewDebugDetails

`func NewDebugDetails(duration Duration, endTime string, serverName string, smtp DebugDetailsSmtp, startTime string, ) *DebugDetails`

NewDebugDetails instantiates a new DebugDetails object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDebugDetailsWithDefaults

`func NewDebugDetailsWithDefaults() *DebugDetails`

NewDebugDetailsWithDefaults instantiates a new DebugDetails object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDuration

`func (o *DebugDetails) GetDuration() Duration`

GetDuration returns the Duration field if non-nil, zero value otherwise.

### GetDurationOk

`func (o *DebugDetails) GetDurationOk() (*Duration, bool)`

GetDurationOk returns a tuple with the Duration field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDuration

`func (o *DebugDetails) SetDuration(v Duration)`

SetDuration sets Duration field to given value.


### GetEndTime

`func (o *DebugDetails) GetEndTime() string`

GetEndTime returns the EndTime field if non-nil, zero value otherwise.

### GetEndTimeOk

`func (o *DebugDetails) GetEndTimeOk() (*string, bool)`

GetEndTimeOk returns a tuple with the EndTime field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEndTime

`func (o *DebugDetails) SetEndTime(v string)`

SetEndTime sets EndTime field to given value.


### GetServerName

`func (o *DebugDetails) GetServerName() string`

GetServerName returns the ServerName field if non-nil, zero value otherwise.

### GetServerNameOk

`func (o *DebugDetails) GetServerNameOk() (*string, bool)`

GetServerNameOk returns a tuple with the ServerName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetServerName

`func (o *DebugDetails) SetServerName(v string)`

SetServerName sets ServerName field to given value.


### GetSmtp

`func (o *DebugDetails) GetSmtp() DebugDetailsSmtp`

GetSmtp returns the Smtp field if non-nil, zero value otherwise.

### GetSmtpOk

`func (o *DebugDetails) GetSmtpOk() (*DebugDetailsSmtp, bool)`

GetSmtpOk returns a tuple with the Smtp field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSmtp

`func (o *DebugDetails) SetSmtp(v DebugDetailsSmtp)`

SetSmtp sets Smtp field to given value.


### GetStartTime

`func (o *DebugDetails) GetStartTime() string`

GetStartTime returns the StartTime field if non-nil, zero value otherwise.

### GetStartTimeOk

`func (o *DebugDetails) GetStartTimeOk() (*string, bool)`

GetStartTimeOk returns a tuple with the StartTime field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStartTime

`func (o *DebugDetails) SetStartTime(v string)`

SetStartTime sets StartTime field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


