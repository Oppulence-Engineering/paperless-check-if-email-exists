# OverviewSectionOverviewOutcomes

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Data** | Pointer to [**OverviewSectionOverviewOutcomesData**](OverviewSectionOverviewOutcomesData.md) |  | [optional]
**Status** | [**SectionStatus**](SectionStatus.md) |  | [required]

## Methods

### NewOverviewSectionOverviewOutcomes

`func NewOverviewSectionOverviewOutcomes(status SectionStatus) *OverviewSectionOverviewOutcomes`

NewOverviewSectionOverviewOutcomes instantiates a new OverviewSectionOverviewOutcomes object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOverviewSectionOverviewOutcomesWithDefaults

`func NewOverviewSectionOverviewOutcomesWithDefaults() *OverviewSectionOverviewOutcomes`

NewOverviewSectionOverviewOutcomesWithDefaults instantiates a new OverviewSectionOverviewOutcomes object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetData

`func (o *OverviewSectionOverviewOutcomes) GetData() OverviewSectionOverviewOutcomesData`

GetData returns the Data field if non-nil, zero value otherwise.

### GetDataOk

`func (o *OverviewSectionOverviewOutcomes) GetDataOk() (*OverviewSectionOverviewOutcomesData, bool)`

GetDataOk returns a tuple with the Data field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetData

`func (o *OverviewSectionOverviewOutcomes) SetData(v OverviewSectionOverviewOutcomesData)`

SetData sets Data field to given value.

### HasData

`func (o *OverviewSectionOverviewOutcomes) HasData() bool`

HasData returns a boolean if a field has been set.

### GetStatus

`func (o *OverviewSectionOverviewOutcomes) GetStatus() SectionStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *OverviewSectionOverviewOutcomes) GetStatusOk() (*SectionStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *OverviewSectionOverviewOutcomes) SetStatus(v SectionStatus)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
