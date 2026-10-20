# ResearchExportRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Mode** | [**ResearchExportMode**](ResearchExportMode.md) |  | [required]
**RowKeys** | **[]string** |  | [required]

## Methods

### NewResearchExportRequest

`func NewResearchExportRequest(mode ResearchExportMode, rowKeys []string) *ResearchExportRequest`

NewResearchExportRequest instantiates a new ResearchExportRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResearchExportRequestWithDefaults

`func NewResearchExportRequestWithDefaults() *ResearchExportRequest`

NewResearchExportRequestWithDefaults instantiates a new ResearchExportRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetMode

`func (o *ResearchExportRequest) GetMode() ResearchExportMode`

GetMode returns the Mode field if non-nil, zero value otherwise.

### GetModeOk

`func (o *ResearchExportRequest) GetModeOk() (*ResearchExportMode, bool)`

GetModeOk returns a tuple with the Mode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetMode

`func (o *ResearchExportRequest) SetMode(v ResearchExportMode)`

SetMode sets Mode field to given value.


### GetRowKeys

`func (o *ResearchExportRequest) GetRowKeys() []string`

GetRowKeys returns the RowKeys field if non-nil, zero value otherwise.

### GetRowKeysOk

`func (o *ResearchExportRequest) GetRowKeysOk() ([]string, bool)`

GetRowKeysOk returns a tuple with the RowKeys field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRowKeys

`func (o *ResearchExportRequest) SetRowKeys(v []string)`

SetRowKeys sets RowKeys field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
