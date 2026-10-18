# CampaignOutcomeListResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Outcomes** | [**[]CampaignOutcomeView**](CampaignOutcomeView.md) |  | [required]
**Total** | **int64** |  | [required]

## Methods

### NewCampaignOutcomeListResponse

`func NewCampaignOutcomeListResponse(outcomes []CampaignOutcomeView, total int64) *CampaignOutcomeListResponse`

NewCampaignOutcomeListResponse instantiates a new CampaignOutcomeListResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewCampaignOutcomeListResponseWithDefaults

`func NewCampaignOutcomeListResponseWithDefaults() *CampaignOutcomeListResponse`

NewCampaignOutcomeListResponseWithDefaults instantiates a new CampaignOutcomeListResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetOutcomes

`func (o *CampaignOutcomeListResponse) GetOutcomes() []CampaignOutcomeView`

GetOutcomes returns the Outcomes field if non-nil, zero value otherwise.

### GetOutcomesOk

`func (o *CampaignOutcomeListResponse) GetOutcomesOk() ([]CampaignOutcomeView, bool)`

GetOutcomesOk returns a tuple with the Outcomes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcomes

`func (o *CampaignOutcomeListResponse) SetOutcomes(v []CampaignOutcomeView)`

SetOutcomes sets Outcomes field to given value.


### GetTotal

`func (o *CampaignOutcomeListResponse) GetTotal() int64`

GetTotal returns the Total field if non-nil, zero value otherwise.

### GetTotalOk

`func (o *CampaignOutcomeListResponse) GetTotalOk() (*int64, bool)`

GetTotalOk returns a tuple with the Total field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotal

`func (o *CampaignOutcomeListResponse) SetTotal(v int64)`

SetTotal sets Total field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
