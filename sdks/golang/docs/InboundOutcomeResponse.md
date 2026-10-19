# InboundOutcomeResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Accepted** | **int64** |  | [required]
**Duplicates** | **int64** |  | [required]
**Provider** | **string** |  | [required]
**ReceiptId** | **string** |  | [required]
**Rejected** | **int64** |  | [required]
**Unmatched** | **int64** |  | [required]

## Methods

### NewInboundOutcomeResponse

`func NewInboundOutcomeResponse(accepted int64, duplicates int64, provider string, receiptId string, rejected int64, unmatched int64) *InboundOutcomeResponse`

NewInboundOutcomeResponse instantiates a new InboundOutcomeResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewInboundOutcomeResponseWithDefaults

`func NewInboundOutcomeResponseWithDefaults() *InboundOutcomeResponse`

NewInboundOutcomeResponseWithDefaults instantiates a new InboundOutcomeResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAccepted

`func (o *InboundOutcomeResponse) GetAccepted() int64`

GetAccepted returns the Accepted field if non-nil, zero value otherwise.

### GetAcceptedOk

`func (o *InboundOutcomeResponse) GetAcceptedOk() (*int64, bool)`

GetAcceptedOk returns a tuple with the Accepted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAccepted

`func (o *InboundOutcomeResponse) SetAccepted(v int64)`

SetAccepted sets Accepted field to given value.


### GetDuplicates

`func (o *InboundOutcomeResponse) GetDuplicates() int64`

GetDuplicates returns the Duplicates field if non-nil, zero value otherwise.

### GetDuplicatesOk

`func (o *InboundOutcomeResponse) GetDuplicatesOk() (*int64, bool)`

GetDuplicatesOk returns a tuple with the Duplicates field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDuplicates

`func (o *InboundOutcomeResponse) SetDuplicates(v int64)`

SetDuplicates sets Duplicates field to given value.


### GetProvider

`func (o *InboundOutcomeResponse) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *InboundOutcomeResponse) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *InboundOutcomeResponse) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetReceiptId

`func (o *InboundOutcomeResponse) GetReceiptId() string`

GetReceiptId returns the ReceiptId field if non-nil, zero value otherwise.

### GetReceiptIdOk

`func (o *InboundOutcomeResponse) GetReceiptIdOk() (*string, bool)`

GetReceiptIdOk returns a tuple with the ReceiptId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReceiptId

`func (o *InboundOutcomeResponse) SetReceiptId(v string)`

SetReceiptId sets ReceiptId field to given value.


### GetRejected

`func (o *InboundOutcomeResponse) GetRejected() int64`

GetRejected returns the Rejected field if non-nil, zero value otherwise.

### GetRejectedOk

`func (o *InboundOutcomeResponse) GetRejectedOk() (*int64, bool)`

GetRejectedOk returns a tuple with the Rejected field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRejected

`func (o *InboundOutcomeResponse) SetRejected(v int64)`

SetRejected sets Rejected field to given value.


### GetUnmatched

`func (o *InboundOutcomeResponse) GetUnmatched() int64`

GetUnmatched returns the Unmatched field if non-nil, zero value otherwise.

### GetUnmatchedOk

`func (o *InboundOutcomeResponse) GetUnmatchedOk() (*int64, bool)`

GetUnmatchedOk returns a tuple with the Unmatched field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnmatched

`func (o *InboundOutcomeResponse) SetUnmatched(v int64)`

SetUnmatched sets Unmatched field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
