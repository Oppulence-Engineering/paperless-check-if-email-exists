# SuppressionEventListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Events** | [**[]SuppressionEvent**](SuppressionEvent.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewSuppressionEventListResponse

`func NewSuppressionEventListResponse(events []SuppressionEvent, total int64) *SuppressionEventListResponse`

NewSuppressionEventListResponse instantiates a new SuppressionEventListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSuppressionEventListResponseWithDefaults

`func NewSuppressionEventListResponseWithDefaults() *SuppressionEventListResponse`

NewSuppressionEventListResponseWithDefaults instantiates a new SuppressionEventListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetEvents

`func (o *SuppressionEventListResponse) GetEvents() []SuppressionEvent`

GetEvents returns the Events field if non-nil, zero value otherwise.

### GetEventsOk

`func (o *SuppressionEventListResponse) GetEventsOk() ([]SuppressionEvent, bool)`

GetEventsOk returns a tuple with the Events field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEvents

`func (o *SuppressionEventListResponse) SetEvents(v []SuppressionEvent)`

SetEvents sets Events field to given value.


### GetTotal

`func (o *SuppressionEventListResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *SuppressionEventListResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *SuppressionEventListResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
