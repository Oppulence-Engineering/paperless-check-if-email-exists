# Duration

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Nanos** | **float32** | Duration in nanoseconds. | [required]
**Secs** | **float32** | Duration in seconds. | [required]

## Methods

### NewDuration

`func NewDuration(nanos float32, secs float32) *Duration`

NewDuration instantiates a new Duration object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewDurationWithDefaults

`func NewDurationWithDefaults() *Duration`

NewDurationWithDefaults instantiates a new Duration object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetNanos

`func (o *Duration) GetNanos() float32`

GetNanos returns the Nanos field if non-nil, zero value otherwise.

### GetNanosOk

`func (o *Duration) GetNanosOk() (*float32, bool)`

GetNanosOk returns a tuple with the Nanos field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNanos

`func (o *Duration) SetNanos(v float32)`

SetNanos sets Nanos field to given value.


### GetSecs

`func (o *Duration) GetSecs() float32`

GetSecs returns the Secs field if non-nil, zero value otherwise.

### GetSecsOk

`func (o *Duration) GetSecsOk() (*float32, bool)`

GetSecsOk returns a tuple with the Secs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSecs

`func (o *Duration) SetSecs(v float32)`

SetSecs sets Secs field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


