# ResearchExportResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Csv** | **string** |  | [required]
**ExcludedRowKeys** | **[]string** |  | [required]
**ExportedRows** | **int32** |  | [required]
**Filename** | **string** |  | [required]

## Methods

### NewResearchExportResponse

`func NewResearchExportResponse(csv string, excludedRowKeys []string, exportedRows int32, filename string) *ResearchExportResponse`

NewResearchExportResponse instantiates a new ResearchExportResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResearchExportResponseWithDefaults

`func NewResearchExportResponseWithDefaults() *ResearchExportResponse`

NewResearchExportResponseWithDefaults instantiates a new ResearchExportResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCsv

`func (o *ResearchExportResponse) GetCsv() string`

GetCsv returns the Csv field if non-nil, zero value otherwise.

### GetCsvOk

`func (o *ResearchExportResponse) GetCsvOk() (*string, bool)`

GetCsvOk returns a tuple with the Csv field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCsv

`func (o *ResearchExportResponse) SetCsv(v string)`

SetCsv sets Csv field to given value.


### GetExcludedRowKeys

`func (o *ResearchExportResponse) GetExcludedRowKeys() []string`

GetExcludedRowKeys returns the ExcludedRowKeys field if non-nil, zero value otherwise.

### GetExcludedRowKeysOk

`func (o *ResearchExportResponse) GetExcludedRowKeysOk() ([]string, bool)`

GetExcludedRowKeysOk returns a tuple with the ExcludedRowKeys field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExcludedRowKeys

`func (o *ResearchExportResponse) SetExcludedRowKeys(v []string)`

SetExcludedRowKeys sets ExcludedRowKeys field to given value.


### GetExportedRows

`func (o *ResearchExportResponse) GetExportedRows() int32`

GetExportedRows returns the ExportedRows field if non-nil, zero value otherwise.

### GetExportedRowsOk

`func (o *ResearchExportResponse) GetExportedRowsOk() (*int32, bool)`

GetExportedRowsOk returns a tuple with the ExportedRows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetExportedRows

`func (o *ResearchExportResponse) SetExportedRows(v int32)`

SetExportedRows sets ExportedRows field to given value.


### GetFilename

`func (o *ResearchExportResponse) GetFilename() string`

GetFilename returns the Filename field if non-nil, zero value otherwise.

### GetFilenameOk

`func (o *ResearchExportResponse) GetFilenameOk() (*string, bool)`

GetFilenameOk returns a tuple with the Filename field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFilename

`func (o *ResearchExportResponse) SetFilename(v string)`

SetFilename sets Filename field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
