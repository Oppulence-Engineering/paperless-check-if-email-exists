# OverviewSectionVecOverviewListDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Id** | **int32** |  | [required]
**JobId** | **int32** |  | [required]
**Name** | **string** |  | [required]
**QualityGrade** | Pointer to **NullableString** |  | [optional]
**ReadyToSend** | Pointer to **NullableBool** |  | [optional]
**SafeToSendCount** | Pointer to **NullableInt64** |  | [optional]
**Status** | **string** |  | [required]
**TotalProcessed** | **int64** |  | [required]
**TotalRows** | **int32** |  | [required]

## Methods

### NewOverviewSectionVecOverviewListDataInner

`func NewOverviewSectionVecOverviewListDataInner(id int32, jobId int32, name string, status string, totalProcessed int64, totalRows int32) *OverviewSectionVecOverviewListDataInner`

NewOverviewSectionVecOverviewListDataInner instantiates a new OverviewSectionVecOverviewListDataInner object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOverviewSectionVecOverviewListDataInnerWithDefaults

`func NewOverviewSectionVecOverviewListDataInnerWithDefaults() *OverviewSectionVecOverviewListDataInner`

NewOverviewSectionVecOverviewListDataInnerWithDefaults instantiates a new OverviewSectionVecOverviewListDataInner object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetId

`func (o *OverviewSectionVecOverviewListDataInner) GetId() int32`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *OverviewSectionVecOverviewListDataInner) GetIdOk() (*int32, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *OverviewSectionVecOverviewListDataInner) SetId(v int32)`

SetId sets Id field to given value.


### GetJobId

`func (o *OverviewSectionVecOverviewListDataInner) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *OverviewSectionVecOverviewListDataInner) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *OverviewSectionVecOverviewListDataInner) SetJobId(v int32)`

SetJobId sets JobId field to given value.


### GetName

`func (o *OverviewSectionVecOverviewListDataInner) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *OverviewSectionVecOverviewListDataInner) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *OverviewSectionVecOverviewListDataInner) SetName(v string)`

SetName sets Name field to given value.


### GetQualityGrade

`func (o *OverviewSectionVecOverviewListDataInner) GetQualityGrade() string`

GetQualityGrade returns the QualityGrade field if non-nil, zero value otherwise.

### GetQualityGradeOk

`func (o *OverviewSectionVecOverviewListDataInner) GetQualityGradeOk() (*string, bool)`

GetQualityGradeOk returns a tuple with the QualityGrade field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQualityGrade

`func (o *OverviewSectionVecOverviewListDataInner) SetQualityGrade(v string)`

SetQualityGrade sets QualityGrade field to given value.

### HasQualityGrade

`func (o *OverviewSectionVecOverviewListDataInner) HasQualityGrade() bool`

HasQualityGrade returns a boolean if a field has been set.

### SetQualityGradeNil

`func (o *OverviewSectionVecOverviewListDataInner) SetQualityGradeNil()`

 SetQualityGradeNil sets the value for QualityGrade to be an explicit nil

### UnsetQualityGrade
`func (o *OverviewSectionVecOverviewListDataInner) UnsetQualityGrade()`

UnsetQualityGrade ensures that no value is present for QualityGrade, not even an explicit nil

### GetReadyToSend

`func (o *OverviewSectionVecOverviewListDataInner) GetReadyToSend() bool`

GetReadyToSend returns the ReadyToSend field if non-nil, zero value otherwise.

### GetReadyToSendOk

`func (o *OverviewSectionVecOverviewListDataInner) GetReadyToSendOk() (*bool, bool)`

GetReadyToSendOk returns a tuple with the ReadyToSend field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReadyToSend

`func (o *OverviewSectionVecOverviewListDataInner) SetReadyToSend(v bool)`

SetReadyToSend sets ReadyToSend field to given value.

### HasReadyToSend

`func (o *OverviewSectionVecOverviewListDataInner) HasReadyToSend() bool`

HasReadyToSend returns a boolean if a field has been set.

### SetReadyToSendNil

`func (o *OverviewSectionVecOverviewListDataInner) SetReadyToSendNil()`

 SetReadyToSendNil sets the value for ReadyToSend to be an explicit nil

### UnsetReadyToSend
`func (o *OverviewSectionVecOverviewListDataInner) UnsetReadyToSend()`

UnsetReadyToSend ensures that no value is present for ReadyToSend, not even an explicit nil

### GetSafeToSendCount

`func (o *OverviewSectionVecOverviewListDataInner) GetSafeToSendCount() int64`

GetSafeToSendCount returns the SafeToSendCount field if non-nil, zero value otherwise.

### GetSafeToSendCountOk

`func (o *OverviewSectionVecOverviewListDataInner) GetSafeToSendCountOk() (*int64, bool)`

GetSafeToSendCountOk returns a tuple with the SafeToSendCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSendCount

`func (o *OverviewSectionVecOverviewListDataInner) SetSafeToSendCount(v int64)`

SetSafeToSendCount sets SafeToSendCount field to given value.

### HasSafeToSendCount

`func (o *OverviewSectionVecOverviewListDataInner) HasSafeToSendCount() bool`

HasSafeToSendCount returns a boolean if a field has been set.

### SetSafeToSendCountNil

`func (o *OverviewSectionVecOverviewListDataInner) SetSafeToSendCountNil()`

 SetSafeToSendCountNil sets the value for SafeToSendCount to be an explicit nil

### UnsetSafeToSendCount
`func (o *OverviewSectionVecOverviewListDataInner) UnsetSafeToSendCount()`

UnsetSafeToSendCount ensures that no value is present for SafeToSendCount, not even an explicit nil

### GetStatus

`func (o *OverviewSectionVecOverviewListDataInner) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *OverviewSectionVecOverviewListDataInner) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *OverviewSectionVecOverviewListDataInner) SetStatus(v string)`

SetStatus sets Status field to given value.


### GetTotalProcessed

`func (o *OverviewSectionVecOverviewListDataInner) GetTotalProcessed() int64`

GetTotalProcessed returns the TotalProcessed field if non-nil, zero value otherwise.

### GetTotalProcessedOk

`func (o *OverviewSectionVecOverviewListDataInner) GetTotalProcessedOk() (*int64, bool)`

GetTotalProcessedOk returns a tuple with the TotalProcessed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalProcessed

`func (o *OverviewSectionVecOverviewListDataInner) SetTotalProcessed(v int64)`

SetTotalProcessed sets TotalProcessed field to given value.


### GetTotalRows

`func (o *OverviewSectionVecOverviewListDataInner) GetTotalRows() int32`

GetTotalRows returns the TotalRows field if non-nil, zero value otherwise.

### GetTotalRowsOk

`func (o *OverviewSectionVecOverviewListDataInner) GetTotalRowsOk() (*int32, bool)`

GetTotalRowsOk returns a tuple with the TotalRows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRows

`func (o *OverviewSectionVecOverviewListDataInner) SetTotalRows(v int32)`

SetTotalRows sets TotalRows field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
