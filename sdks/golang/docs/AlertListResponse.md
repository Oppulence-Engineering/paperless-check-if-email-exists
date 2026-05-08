# AlertListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Alerts** | [**[]AlertView**](AlertView.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewAlertListResponse

`func NewAlertListResponse(alerts []AlertView, total int64) *AlertListResponse`

NewAlertListResponse instantiates a new AlertListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewAlertListResponseWithDefaults

`func NewAlertListResponseWithDefaults() *AlertListResponse`

NewAlertListResponseWithDefaults instantiates a new AlertListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAlerts

`func (o *AlertListResponse) GetAlerts() []AlertView`

GetAlerts returns the Alerts field if non-nil, zero value otherwise.

### GetAlertsOk

`func (o *AlertListResponse) GetAlertsOk() ([]AlertView, bool)`

GetAlertsOk returns a tuple with the Alerts field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAlerts

`func (o *AlertListResponse) SetAlerts(v []AlertView)`

SetAlerts sets Alerts field to given value.


### GetTotal

`func (o *AlertListResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *AlertListResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *AlertListResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
