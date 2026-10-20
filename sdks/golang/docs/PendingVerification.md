# PendingVerification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**OperationId** | **string** |  | [required]
**Status** | **string** |  | [required]
**StatusUrl** | **string** |  | [required]

## Methods

### NewPendingVerification

`func NewPendingVerification(operationId string, status string, statusUrl string) *PendingVerification`

NewPendingVerification instantiates a new PendingVerification object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPendingVerificationWithDefaults

`func NewPendingVerificationWithDefaults() *PendingVerification`

NewPendingVerificationWithDefaults instantiates a new PendingVerification object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetOperationId

`func (o *PendingVerification) GetOperationId() string`

GetOperationId returns the OperationId field if non-nil, zero value otherwise.

### GetOperationIdOk

`func (o *PendingVerification) GetOperationIdOk() (*string, bool)`

GetOperationIdOk returns a tuple with the OperationId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOperationId

`func (o *PendingVerification) SetOperationId(v string)`

SetOperationId sets OperationId field to given value.


### GetStatus

`func (o *PendingVerification) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *PendingVerification) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *PendingVerification) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetStatusUrl

`func (o *PendingVerification) GetStatusUrl() string`

GetStatusUrl returns the StatusUrl field if non-nil, zero value otherwise.

### GetStatusUrlOk

`func (o *PendingVerification) GetStatusUrlOk() (*string, bool)`

GetStatusUrlOk returns a tuple with the StatusUrl field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatusUrl

`func (o *PendingVerification) SetStatusUrl(v string)`

SetStatusUrl sets StatusUrl field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
