# SmtpDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CanConnectSmtp** | **bool** | Indicates if the mail exchanger can be contacted successfully. | [required]
**HasFullInbox** | **bool** | Indicates if the mailbox is full. | [required]
**IsCatchAll** | **bool** | Indicates if the email address is a catch-all address. | [required]
**IsDeliverable** | **bool** | Indicates if an email sent to this address is deliverable. | [required]
**IsDisabled** | **bool** | Indicates if the email address has been disabled by the provider. | [required]

## Methods

### NewSmtpDetails

`func NewSmtpDetails(canConnectSmtp bool, hasFullInbox bool, isCatchAll bool, isDeliverable bool, isDisabled bool) *SmtpDetails`

NewSmtpDetails instantiates a new SmtpDetails object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSmtpDetailsWithDefaults

`func NewSmtpDetailsWithDefaults() *SmtpDetails`

NewSmtpDetailsWithDefaults instantiates a new SmtpDetails object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCanConnectSmtp

`func (o *SmtpDetails) GetCanConnectSmtp() bool`

GetCanConnectSmtp returns the CanConnectSmtp field if non-nil, zero value otherwise.

### GetCanConnectSmtpOk

`func (o *SmtpDetails) GetCanConnectSmtpOk() (*bool, bool)`

GetCanConnectSmtpOk returns a tuple with the CanConnectSmtp field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanConnectSmtp

`func (o *SmtpDetails) SetCanConnectSmtp(v bool)`

SetCanConnectSmtp sets CanConnectSmtp field to given value.


### GetHasFullInbox

`func (o *SmtpDetails) GetHasFullInbox() bool`

GetHasFullInbox returns the HasFullInbox field if non-nil, zero value otherwise.

### GetHasFullInboxOk

`func (o *SmtpDetails) GetHasFullInboxOk() (*bool, bool)`

GetHasFullInboxOk returns a tuple with the HasFullInbox field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasFullInbox

`func (o *SmtpDetails) SetHasFullInbox(v bool)`

SetHasFullInbox sets HasFullInbox field to given value.


### GetIsCatchAll

`func (o *SmtpDetails) GetIsCatchAll() bool`

GetIsCatchAll returns the IsCatchAll field if non-nil, zero value otherwise.

### GetIsCatchAllOk

`func (o *SmtpDetails) GetIsCatchAllOk() (*bool, bool)`

GetIsCatchAllOk returns a tuple with the IsCatchAll field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsCatchAll

`func (o *SmtpDetails) SetIsCatchAll(v bool)`

SetIsCatchAll sets IsCatchAll field to given value.


### GetIsDeliverable

`func (o *SmtpDetails) GetIsDeliverable() bool`

GetIsDeliverable returns the IsDeliverable field if non-nil, zero value otherwise.

### GetIsDeliverableOk

`func (o *SmtpDetails) GetIsDeliverableOk() (*bool, bool)`

GetIsDeliverableOk returns a tuple with the IsDeliverable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDeliverable

`func (o *SmtpDetails) SetIsDeliverable(v bool)`

SetIsDeliverable sets IsDeliverable field to given value.


### GetIsDisabled

`func (o *SmtpDetails) GetIsDisabled() bool`

GetIsDisabled returns the IsDisabled field if non-nil, zero value otherwise.

### GetIsDisabledOk

`func (o *SmtpDetails) GetIsDisabledOk() (*bool, bool)`

GetIsDisabledOk returns a tuple with the IsDisabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDisabled

`func (o *SmtpDetails) SetIsDisabled(v bool)`

SetIsDisabled sets IsDisabled field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
