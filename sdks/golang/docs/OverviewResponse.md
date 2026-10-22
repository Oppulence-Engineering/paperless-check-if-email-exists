# OverviewResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Alerts** | [**OverviewSectionAlertListResponse**](OverviewSectionAlertListResponse.md) |  | [required]
**Domains** | [**OverviewSectionOverviewDomains**](OverviewSectionOverviewDomains.md) |  | [required]
**Lists** | [**OverviewSectionVecOverviewList**](OverviewSectionVecOverviewList.md) |  | [required]
**Outcomes** | [**OverviewSectionOverviewOutcomes**](OverviewSectionOverviewOutcomes.md) |  | [required]
**Pipelines** | [**OverviewSectionVecOverviewPipeline**](OverviewSectionVecOverviewPipeline.md) |  | [required]
**Reverification** | [**OverviewSectionOverviewReverification**](OverviewSectionOverviewReverification.md) |  | [required]
**Suppressions** | [**OverviewSectionI64**](OverviewSectionI64.md) |  | [required]
**Usage** | [**TenantUsageResponse**](TenantUsageResponse.md) |  | [required]

## Methods

### NewOverviewResponse

`func NewOverviewResponse(alerts OverviewSectionAlertListResponse, domains OverviewSectionOverviewDomains, lists OverviewSectionVecOverviewList, outcomes OverviewSectionOverviewOutcomes, pipelines OverviewSectionVecOverviewPipeline, reverification OverviewSectionOverviewReverification, suppressions OverviewSectionI64, usage TenantUsageResponse) *OverviewResponse`

NewOverviewResponse instantiates a new OverviewResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOverviewResponseWithDefaults

`func NewOverviewResponseWithDefaults() *OverviewResponse`

NewOverviewResponseWithDefaults instantiates a new OverviewResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAlerts

`func (o *OverviewResponse) GetAlerts() OverviewSectionAlertListResponse`

GetAlerts returns the Alerts field if non-nil, zero value otherwise.

### GetAlertsOk

`func (o *OverviewResponse) GetAlertsOk() (*OverviewSectionAlertListResponse, bool)`

GetAlertsOk returns a tuple with the Alerts field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAlerts

`func (o *OverviewResponse) SetAlerts(v OverviewSectionAlertListResponse)`

SetAlerts sets Alerts field to given value.


### GetDomains

`func (o *OverviewResponse) GetDomains() OverviewSectionOverviewDomains`

GetDomains returns the Domains field if non-nil, zero value otherwise.

### GetDomainsOk

`func (o *OverviewResponse) GetDomainsOk() (*OverviewSectionOverviewDomains, bool)`

GetDomainsOk returns a tuple with the Domains field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomains

`func (o *OverviewResponse) SetDomains(v OverviewSectionOverviewDomains)`

SetDomains sets Domains field to given value.


### GetLists

`func (o *OverviewResponse) GetLists() OverviewSectionVecOverviewList`

GetLists returns the Lists field if non-nil, zero value otherwise.

### GetListsOk

`func (o *OverviewResponse) GetListsOk() (*OverviewSectionVecOverviewList, bool)`

GetListsOk returns a tuple with the Lists field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLists

`func (o *OverviewResponse) SetLists(v OverviewSectionVecOverviewList)`

SetLists sets Lists field to given value.


### GetOutcomes

`func (o *OverviewResponse) GetOutcomes() OverviewSectionOverviewOutcomes`

GetOutcomes returns the Outcomes field if non-nil, zero value otherwise.

### GetOutcomesOk

`func (o *OverviewResponse) GetOutcomesOk() (*OverviewSectionOverviewOutcomes, bool)`

GetOutcomesOk returns a tuple with the Outcomes field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcomes

`func (o *OverviewResponse) SetOutcomes(v OverviewSectionOverviewOutcomes)`

SetOutcomes sets Outcomes field to given value.


### GetPipelines

`func (o *OverviewResponse) GetPipelines() OverviewSectionVecOverviewPipeline`

GetPipelines returns the Pipelines field if non-nil, zero value otherwise.

### GetPipelinesOk

`func (o *OverviewResponse) GetPipelinesOk() (*OverviewSectionVecOverviewPipeline, bool)`

GetPipelinesOk returns a tuple with the Pipelines field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPipelines

`func (o *OverviewResponse) SetPipelines(v OverviewSectionVecOverviewPipeline)`

SetPipelines sets Pipelines field to given value.


### GetReverification

`func (o *OverviewResponse) GetReverification() OverviewSectionOverviewReverification`

GetReverification returns the Reverification field if non-nil, zero value otherwise.

### GetReverificationOk

`func (o *OverviewResponse) GetReverificationOk() (*OverviewSectionOverviewReverification, bool)`

GetReverificationOk returns a tuple with the Reverification field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReverification

`func (o *OverviewResponse) SetReverification(v OverviewSectionOverviewReverification)`

SetReverification sets Reverification field to given value.


### GetSuppressions

`func (o *OverviewResponse) GetSuppressions() OverviewSectionI64`

GetSuppressions returns the Suppressions field if non-nil, zero value otherwise.

### GetSuppressionsOk

`func (o *OverviewResponse) GetSuppressionsOk() (*OverviewSectionI64, bool)`

GetSuppressionsOk returns a tuple with the Suppressions field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSuppressions

`func (o *OverviewResponse) SetSuppressions(v OverviewSectionI64)`

SetSuppressions sets Suppressions field to given value.


### GetUsage

`func (o *OverviewResponse) GetUsage() TenantUsageResponse`

GetUsage returns the Usage field if non-nil, zero value otherwise.

### GetUsageOk

`func (o *OverviewResponse) GetUsageOk() (*TenantUsageResponse, bool)`

GetUsageOk returns a tuple with the Usage field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsage

`func (o *OverviewResponse) SetUsage(v TenantUsageResponse)`

SetUsage sets Usage field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
