# OverviewSectionOverviewReverification

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Data** | Pointer to [**OverviewSectionOverviewReverificationData**](OverviewSectionOverviewReverificationData.md) |  | [optional]
**Status** | [**SectionStatus**](SectionStatus.md) |  | [required]

## Methods

### NewOverviewSectionOverviewReverification

`func NewOverviewSectionOverviewReverification(status SectionStatus) *OverviewSectionOverviewReverification`

NewOverviewSectionOverviewReverification instantiates a new OverviewSectionOverviewReverification object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOverviewSectionOverviewReverificationWithDefaults

`func NewOverviewSectionOverviewReverificationWithDefaults() *OverviewSectionOverviewReverification`

NewOverviewSectionOverviewReverificationWithDefaults instantiates a new OverviewSectionOverviewReverification object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetData

`func (o *OverviewSectionOverviewReverification) GetData() OverviewSectionOverviewReverificationData`

GetData returns the Data field if non-nil, zero value otherwise.

### GetDataOk

`func (o *OverviewSectionOverviewReverification) GetDataOk() (*OverviewSectionOverviewReverificationData, bool)`

GetDataOk returns a tuple with the Data field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetData

`func (o *OverviewSectionOverviewReverification) SetData(v OverviewSectionOverviewReverificationData)`

SetData sets Data field to given value.

### HasData

`func (o *OverviewSectionOverviewReverification) HasData() bool`

HasData returns a boolean if a field has been set.

### GetStatus

`func (o *OverviewSectionOverviewReverification) GetStatus() SectionStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *OverviewSectionOverviewReverification) GetStatusOk() (*SectionStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *OverviewSectionOverviewReverification) SetStatus(v SectionStatus)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
