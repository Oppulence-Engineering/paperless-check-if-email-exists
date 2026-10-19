# ScorePolicyListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Policies** | [**[]ScorePolicyView**](ScorePolicyView.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewScorePolicyListResponse

`func NewScorePolicyListResponse(policies []ScorePolicyView, total int64) *ScorePolicyListResponse`

NewScorePolicyListResponse instantiates a new ScorePolicyListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewScorePolicyListResponseWithDefaults

`func NewScorePolicyListResponseWithDefaults() *ScorePolicyListResponse`

NewScorePolicyListResponseWithDefaults instantiates a new ScorePolicyListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetPolicies

`func (o *ScorePolicyListResponse) GetPolicies() []ScorePolicyView`

GetPolicies returns the Policies field if non-nil, zero value otherwise.

### GetPoliciesOk

`func (o *ScorePolicyListResponse) GetPoliciesOk() ([]ScorePolicyView, bool)`

GetPoliciesOk returns a tuple with the Policies field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPolicies

`func (o *ScorePolicyListResponse) SetPolicies(v []ScorePolicyView)`

SetPolicies sets Policies field to given value.


### GetTotal

`func (o *ScorePolicyListResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *ScorePolicyListResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *ScorePolicyListResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
