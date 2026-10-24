# RejectedRow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Reason** | **string** |  | [required]
**RowNumber** | **int32** |  | [required]
**Value** | **string** |  | [required]

## Methods

### NewRejectedRow

`func NewRejectedRow(reason string, rowNumber int32, value string) *RejectedRow`

NewRejectedRow instantiates a new RejectedRow object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewRejectedRowWithDefaults

`func NewRejectedRowWithDefaults() *RejectedRow`

NewRejectedRowWithDefaults instantiates a new RejectedRow object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetReason

`func (o *RejectedRow) GetReason() string`

GetReason returns the Reason field if non-nil, zero value otherwise.

### GetReasonOk

`func (o *RejectedRow) GetReasonOk() (*string, bool)`

GetReasonOk returns a tuple with the Reason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReason

`func (o *RejectedRow) SetReason(v string)`

SetReason sets Reason field to given value.


### GetRowNumber

`func (o *RejectedRow) GetRowNumber() int32`

GetRowNumber returns the RowNumber field if non-nil, zero value otherwise.

### GetRowNumberOk

`func (o *RejectedRow) GetRowNumberOk() (*int32, bool)`

GetRowNumberOk returns a tuple with the RowNumber field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRowNumber

`func (o *RejectedRow) SetRowNumber(v int32)`

SetRowNumber sets RowNumber field to given value.


### GetValue

`func (o *RejectedRow) GetValue() string`

GetValue returns the Value field if non-nil, zero value otherwise.

### GetValueOk

`func (o *RejectedRow) GetValueOk() (*string, bool)`

GetValueOk returns a tuple with the Value field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValue

`func (o *RejectedRow) SetValue(v string)`

SetValue sets Value field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
