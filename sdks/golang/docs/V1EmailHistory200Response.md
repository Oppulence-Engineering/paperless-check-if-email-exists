# V1EmailHistory200Response

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Email** | **string** |  | [required]
**History** | [**[]HistoryEntry**](HistoryEntry.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewV1EmailHistory200Response

`func NewV1EmailHistory200Response(email string, history []HistoryEntry, total int64) *V1EmailHistory200Response`

NewV1EmailHistory200Response instantiates a new V1EmailHistory200Response object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewV1EmailHistory200ResponseWithDefaults

`func NewV1EmailHistory200ResponseWithDefaults() *V1EmailHistory200Response`

NewV1EmailHistory200ResponseWithDefaults instantiates a new V1EmailHistory200Response object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEmail

`func (o *V1EmailHistory200Response) GetEmail() string`

GetEmail returns the Email field if non-nil, zero value otherwise.

### GetEmailOk

`func (o *V1EmailHistory200Response) GetEmailOk() (*string, bool)`

GetEmailOk returns a tuple with the Email field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmail

`func (o *V1EmailHistory200Response) SetEmail(v string)`

SetEmail sets Email field to given value.


### GetHistory

`func (o *V1EmailHistory200Response) GetHistory() []HistoryEntry`

GetHistory returns the History field if non-nil, zero value otherwise.

### GetHistoryOk

`func (o *V1EmailHistory200Response) GetHistoryOk() ([]HistoryEntry, bool)`

GetHistoryOk returns a tuple with the History field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHistory

`func (o *V1EmailHistory200Response) SetHistory(v []HistoryEntry)`

SetHistory sets History field to given value.


### GetTotal

`func (o *V1EmailHistory200Response) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *V1EmailHistory200Response) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *V1EmailHistory200Response) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
