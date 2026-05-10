# OutcomePolicyListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Policies** | [**[]OutcomePolicyView**](OutcomePolicyView.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewOutcomePolicyListResponse

`func NewOutcomePolicyListResponse(policies []OutcomePolicyView, total int64) *OutcomePolicyListResponse`

NewOutcomePolicyListResponse instantiates a new OutcomePolicyListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOutcomePolicyListResponseWithDefaults

`func NewOutcomePolicyListResponseWithDefaults() *OutcomePolicyListResponse`

NewOutcomePolicyListResponseWithDefaults instantiates a new OutcomePolicyListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPolicies

`func (o *OutcomePolicyListResponse) GetPolicies() []OutcomePolicyView`

GetPolicies returns the Policies field if non-nil, zero value otherwise.

### GetPoliciesOk

`func (o *OutcomePolicyListResponse) GetPoliciesOk() ([]OutcomePolicyView, bool)`

GetPoliciesOk returns a tuple with the Policies field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicies

`func (o *OutcomePolicyListResponse) SetPolicies(v []OutcomePolicyView)`

SetPolicies sets Policies field to given value.


### GetTotal

`func (o *OutcomePolicyListResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *OutcomePolicyListResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *OutcomePolicyListResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
