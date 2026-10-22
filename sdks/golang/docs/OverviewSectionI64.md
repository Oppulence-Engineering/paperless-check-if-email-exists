# OverviewSectionI64

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Data** | Pointer to **int64** |  | [optional]
**Status** | [**SectionStatus**](SectionStatus.md) |  | [required]

## Methods

### NewOverviewSectionI64

`func NewOverviewSectionI64(status SectionStatus) *OverviewSectionI64`

NewOverviewSectionI64 instantiates a new OverviewSectionI64 object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOverviewSectionI64WithDefaults

`func NewOverviewSectionI64WithDefaults() *OverviewSectionI64`

NewOverviewSectionI64WithDefaults instantiates a new OverviewSectionI64 object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetData

`func (o *OverviewSectionI64) GetData() int64`

GetData returns the Data field if non-nil, zero value otherwise.

### GetDataOk

`func (o *OverviewSectionI64) GetDataOk() (*int64, bool)`

GetDataOk returns a tuple with the Data field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetData

`func (o *OverviewSectionI64) SetData(v int64)`

SetData sets Data field to given value.

### HasData

`func (o *OverviewSectionI64) HasData() bool`

HasData returns a boolean if a field has been set.

### GetStatus

`func (o *OverviewSectionI64) GetStatus() SectionStatus`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *OverviewSectionI64) GetStatusOk() (*SectionStatus, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *OverviewSectionI64) SetStatus(v SectionStatus)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
