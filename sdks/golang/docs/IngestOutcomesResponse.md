# IngestOutcomesResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Accepted** | **int32** |  | [required]
**Errors** | [**[]IngestRowError**](IngestRowError.md) |  | [required]
**PolicyId** | **int64** |  | [required]
**Rejected** | **int32** |  | [required]
**Suppressed** | **int32** |  | [required]

## Methods

### NewIngestOutcomesResponse

`func NewIngestOutcomesResponse(accepted int32, errors []IngestRowError, policyId int64, rejected int32, suppressed int32) *IngestOutcomesResponse`

NewIngestOutcomesResponse instantiates a new IngestOutcomesResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewIngestOutcomesResponseWithDefaults

`func NewIngestOutcomesResponseWithDefaults() *IngestOutcomesResponse`

NewIngestOutcomesResponseWithDefaults instantiates a new IngestOutcomesResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAccepted

`func (o *IngestOutcomesResponse) GetAccepted() int32`

GetAccepted returns the Accepted field if non-nil, zero value otherwise.

### GetAcceptedOk

`func (o *IngestOutcomesResponse) GetAcceptedOk() (*int32, bool)`

GetAcceptedOk returns a tuple with the Accepted field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAccepted

`func (o *IngestOutcomesResponse) SetAccepted(v int32)`

SetAccepted sets Accepted field to given value.


### GetErrors

`func (o *IngestOutcomesResponse) GetErrors() []IngestRowError`

GetErrors returns the Errors field if non-nil, zero value otherwise.

### GetErrorsOk

`func (o *IngestOutcomesResponse) GetErrorsOk() ([]IngestRowError, bool)`

GetErrorsOk returns a tuple with the Errors field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetErrors

`func (o *IngestOutcomesResponse) SetErrors(v []IngestRowError)`

SetErrors sets Errors field to given value.


### GetPolicyId

`func (o *IngestOutcomesResponse) GetPolicyId() int64`

GetPolicyId returns the PolicyId field if non-nil, zero value otherwise.

### GetPolicyIdOk

`func (o *IngestOutcomesResponse) GetPolicyIdOk() (*int64, bool)`

GetPolicyIdOk returns a tuple with the PolicyId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicyId

`func (o *IngestOutcomesResponse) SetPolicyId(v int64)`

SetPolicyId sets PolicyId field to given value.


### GetRejected

`func (o *IngestOutcomesResponse) GetRejected() int32`

GetRejected returns the Rejected field if non-nil, zero value otherwise.

### GetRejectedOk

`func (o *IngestOutcomesResponse) GetRejectedOk() (*int32, bool)`

GetRejectedOk returns a tuple with the Rejected field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRejected

`func (o *IngestOutcomesResponse) SetRejected(v int32)`

SetRejected sets Rejected field to given value.


### GetSuppressed

`func (o *IngestOutcomesResponse) GetSuppressed() int32`

GetSuppressed returns the Suppressed field if non-nil, zero value otherwise.

### GetSuppressedOk

`func (o *IngestOutcomesResponse) GetSuppressedOk() (*int32, bool)`

GetSuppressedOk returns a tuple with the Suppressed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSuppressed

`func (o *IngestOutcomesResponse) SetSuppressed(v int32)`

SetSuppressed sets Suppressed field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
