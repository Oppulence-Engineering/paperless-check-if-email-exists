# DnsRecordResultsDkimSelectorsInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Error** | **NullableString** |  | [required]
**Selector** | **string** |  | [required]
**Status** | **string** |  | [required]

## Methods

### NewDnsRecordResultsDkimSelectorsInner

`func NewDnsRecordResultsDkimSelectorsInner(error_ NullableString, selector string, status string) *DnsRecordResultsDkimSelectorsInner`

NewDnsRecordResultsDkimSelectorsInner instantiates a new DnsRecordResultsDkimSelectorsInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDnsRecordResultsDkimSelectorsInnerWithDefaults

`func NewDnsRecordResultsDkimSelectorsInnerWithDefaults() *DnsRecordResultsDkimSelectorsInner`

NewDnsRecordResultsDkimSelectorsInnerWithDefaults instantiates a new DnsRecordResultsDkimSelectorsInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetError

`func (o *DnsRecordResultsDkimSelectorsInner) GetError() string`

GetError returns the Error field if non-nil, zero value otherwise.

### GetErrorOk

`func (o *DnsRecordResultsDkimSelectorsInner) GetErrorOk() (*string, bool)`

GetErrorOk returns a tuple with the Error field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetError

`func (o *DnsRecordResultsDkimSelectorsInner) SetError(v string)`

SetError sets Error field to given value.


### GetSelector

`func (o *DnsRecordResultsDkimSelectorsInner) GetSelector() string`

GetSelector returns the Selector field if non-nil, zero value otherwise.

### GetSelectorOk

`func (o *DnsRecordResultsDkimSelectorsInner) GetSelectorOk() (*string, bool)`

GetSelectorOk returns a tuple with the Selector field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSelector

`func (o *DnsRecordResultsDkimSelectorsInner) SetSelector(v string)`

SetSelector sets Selector field to given value.


### GetStatus

`func (o *DnsRecordResultsDkimSelectorsInner) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *DnsRecordResultsDkimSelectorsInner) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *DnsRecordResultsDkimSelectorsInner) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
