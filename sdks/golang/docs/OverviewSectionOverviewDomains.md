# OverviewSectionOverviewDomains

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Data** | Pointer to [**OverviewSectionOverviewDomainsData**](OverviewSectionOverviewDomainsData.md) |  | [optional]
**Status** | [**SectionStatus**](SectionStatus.md) |  | [required]

## Methods

### NewOverviewSectionOverviewDomains

`func NewOverviewSectionOverviewDomains(status SectionStatus) *OverviewSectionOverviewDomains`

NewOverviewSectionOverviewDomains instantiates a new OverviewSectionOverviewDomains object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOverviewSectionOverviewDomainsWithDefaults

`func NewOverviewSectionOverviewDomainsWithDefaults() *OverviewSectionOverviewDomains`

NewOverviewSectionOverviewDomainsWithDefaults instantiates a new OverviewSectionOverviewDomains object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetData

`func (o *OverviewSectionOverviewDomains) GetData() OverviewSectionOverviewDomainsData`

GetData returns the Data field if non-nil, zero value otherwise.

### GetDataOk

`func (o *OverviewSectionOverviewDomains) GetDataOk() (*OverviewSectionOverviewDomainsData, bool)`

GetDataOk returns a tuple with the Data field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetData

`func (o *OverviewSectionOverviewDomains) SetData(v OverviewSectionOverviewDomainsData)`

SetData sets Data field to given value.

### HasData

`func (o *OverviewSectionOverviewDomains) HasData() bool`

HasData returns a boolean if a field has been set.

### GetStatus

`func (o *OverviewSectionOverviewDomains) GetStatus() SectionStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *OverviewSectionOverviewDomains) GetStatusOk() (*SectionStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *OverviewSectionOverviewDomains) SetStatus(v SectionStatus)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
