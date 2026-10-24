# OverviewSectionVecOverviewList

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Data** | Pointer to [**[]OverviewSectionVecOverviewListDataInner**](OverviewSectionVecOverviewListDataInner.md) |  | [optional]
**Status** | [**SectionStatus**](SectionStatus.md) |  | [required]

## Methods

### NewOverviewSectionVecOverviewList

`func NewOverviewSectionVecOverviewList(status SectionStatus) *OverviewSectionVecOverviewList`

NewOverviewSectionVecOverviewList instantiates a new OverviewSectionVecOverviewList object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOverviewSectionVecOverviewListWithDefaults

`func NewOverviewSectionVecOverviewListWithDefaults() *OverviewSectionVecOverviewList`

NewOverviewSectionVecOverviewListWithDefaults instantiates a new OverviewSectionVecOverviewList object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetData

`func (o *OverviewSectionVecOverviewList) GetData() []OverviewSectionVecOverviewListDataInner`

GetData returns the Data field if non-nil, zero value otherwise.

### GetDataOk

`func (o *OverviewSectionVecOverviewList) GetDataOk() ([]OverviewSectionVecOverviewListDataInner, bool)`

GetDataOk returns a tuple with the Data field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetData

`func (o *OverviewSectionVecOverviewList) SetData(v []OverviewSectionVecOverviewListDataInner)`

SetData sets Data field to given value.

### HasData

`func (o *OverviewSectionVecOverviewList) HasData() bool`

HasData returns a boolean if a field has been set.

### GetStatus

`func (o *OverviewSectionVecOverviewList) GetStatus() SectionStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *OverviewSectionVecOverviewList) GetStatusOk() (*SectionStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *OverviewSectionVecOverviewList) SetStatus(v SectionStatus)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
