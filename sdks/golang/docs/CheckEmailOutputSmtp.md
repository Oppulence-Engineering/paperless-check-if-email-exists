# CheckEmailOutputSmtp

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CanConnectSmtp** | **bool** | Indicates if the mail exchanger can be contacted successfully. | [optional]
**HasFullInbox** | **bool** | Indicates if the mailbox is full. | [optional]
**IsCatchAll** | **bool** | Indicates if the email address is a catch-all address. | [optional]
**IsDeliverable** | **bool** | Indicates if an email sent to this address is deliverable. | [optional]
**IsDisabled** | **bool** | Indicates if the email address has been disabled by the provider. | [optional]
**Message** | **string** | A human-readable description of the error. | [optional]
**Type** | **string** | The type of error. | [optional]

## Methods

### NewCheckEmailOutputSmtp

`func NewCheckEmailOutputSmtp(canConnectSmtp bool, hasFullInbox bool, isCatchAll bool, isDeliverable bool, isDisabled bool, message string, type_ string) *CheckEmailOutputSmtp`

NewCheckEmailOutputSmtp instantiates a new CheckEmailOutputSmtp object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCheckEmailOutputSmtpWithDefaults

`func NewCheckEmailOutputSmtpWithDefaults() *CheckEmailOutputSmtp`

NewCheckEmailOutputSmtpWithDefaults instantiates a new CheckEmailOutputSmtp object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCanConnectSmtp

`func (o *CheckEmailOutputSmtp) GetCanConnectSmtp() bool`

GetCanConnectSmtp returns the CanConnectSmtp field if non-nil, zero value otherwise.

### GetCanConnectSmtpOk

`func (o *CheckEmailOutputSmtp) GetCanConnectSmtpOk() (*bool, bool)`

GetCanConnectSmtpOk returns a tuple with the CanConnectSmtp field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCanConnectSmtp

`func (o *CheckEmailOutputSmtp) SetCanConnectSmtp(v bool)`

SetCanConnectSmtp sets CanConnectSmtp field to given value.


### GetHasFullInbox

`func (o *CheckEmailOutputSmtp) GetHasFullInbox() bool`

GetHasFullInbox returns the HasFullInbox field if non-nil, zero value otherwise.

### GetHasFullInboxOk

`func (o *CheckEmailOutputSmtp) GetHasFullInboxOk() (*bool, bool)`

GetHasFullInboxOk returns a tuple with the HasFullInbox field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHasFullInbox

`func (o *CheckEmailOutputSmtp) SetHasFullInbox(v bool)`

SetHasFullInbox sets HasFullInbox field to given value.


### GetIsCatchAll

`func (o *CheckEmailOutputSmtp) GetIsCatchAll() bool`

GetIsCatchAll returns the IsCatchAll field if non-nil, zero value otherwise.

### GetIsCatchAllOk

`func (o *CheckEmailOutputSmtp) GetIsCatchAllOk() (*bool, bool)`

GetIsCatchAllOk returns a tuple with the IsCatchAll field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsCatchAll

`func (o *CheckEmailOutputSmtp) SetIsCatchAll(v bool)`

SetIsCatchAll sets IsCatchAll field to given value.


### GetIsDeliverable

`func (o *CheckEmailOutputSmtp) GetIsDeliverable() bool`

GetIsDeliverable returns the IsDeliverable field if non-nil, zero value otherwise.

### GetIsDeliverableOk

`func (o *CheckEmailOutputSmtp) GetIsDeliverableOk() (*bool, bool)`

GetIsDeliverableOk returns a tuple with the IsDeliverable field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDeliverable

`func (o *CheckEmailOutputSmtp) SetIsDeliverable(v bool)`

SetIsDeliverable sets IsDeliverable field to given value.


### GetIsDisabled

`func (o *CheckEmailOutputSmtp) GetIsDisabled() bool`

GetIsDisabled returns the IsDisabled field if non-nil, zero value otherwise.

### GetIsDisabledOk

`func (o *CheckEmailOutputSmtp) GetIsDisabledOk() (*bool, bool)`

GetIsDisabledOk returns a tuple with the IsDisabled field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsDisabled

`func (o *CheckEmailOutputSmtp) SetIsDisabled(v bool)`

SetIsDisabled sets IsDisabled field to given value.


### GetMessage

`func (o *CheckEmailOutputSmtp) GetMessage() string`

GetMessage returns the Message field if non-nil, zero value otherwise.

### GetMessageOk

`func (o *CheckEmailOutputSmtp) GetMessageOk() (*string, bool)`

GetMessageOk returns a tuple with the Message field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMessage

`func (o *CheckEmailOutputSmtp) SetMessage(v string)`

SetMessage sets Message field to given value.


### GetType

`func (o *CheckEmailOutputSmtp) GetType() string`

GetType returns the Type field if non-nil, zero value otherwise.

### GetTypeOk

`func (o *CheckEmailOutputSmtp) GetTypeOk() (*string, bool)`

GetTypeOk returns a tuple with the Type field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetType

`func (o *CheckEmailOutputSmtp) SetType(v string)`

SetType sets Type field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


