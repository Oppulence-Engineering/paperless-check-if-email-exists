# ResearchSummary

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**CreatedAt** | Pointer to **NullableString** |  | [optional]
**Id** | **string** |  | [required]
**ImportedRows** | **int32** |  | [required]
**Name** | **string** |  | [required]

## Methods

### NewResearchSummary

`func NewResearchSummary(id string, importedRows int32, name string) *ResearchSummary`

NewResearchSummary instantiates a new ResearchSummary object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResearchSummaryWithDefaults

`func NewResearchSummaryWithDefaults() *ResearchSummary`

NewResearchSummaryWithDefaults instantiates a new ResearchSummary object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetCreatedAt

`func (o *ResearchSummary) GetCreatedAt() string`

GetCreatedAt returns the CreatedAt field if non-nil, zero value otherwise.

### GetCreatedAtOk

`func (o *ResearchSummary) GetCreatedAtOk() (*string, bool)`

GetCreatedAtOk returns a tuple with the CreatedAt field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCreatedAt

`func (o *ResearchSummary) SetCreatedAt(v string)`

SetCreatedAt sets CreatedAt field to given value.

### HasCreatedAt

`func (o *ResearchSummary) HasCreatedAt() bool`

HasCreatedAt returns a boolean if a field has been set.

### SetCreatedAtNil

`func (o *ResearchSummary) SetCreatedAtNil()`

 SetCreatedAtNil sets the value for CreatedAt to be an explicit nil

### UnsetCreatedAt
`func (o *ResearchSummary) UnsetCreatedAt()`

UnsetCreatedAt ensures that no value is present for CreatedAt, not even an explicit nil

### GetId

`func (o *ResearchSummary) GetId() string`

GetId returns the Id field if non-nil, zero value otherwise.

### GetIdOk

`func (o *ResearchSummary) GetIdOk() (*string, bool)`

GetIdOk returns a tuple with the Id field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetId

`func (o *ResearchSummary) SetId(v string)`

SetId sets Id field to given value.


### GetImportedRows

`func (o *ResearchSummary) GetImportedRows() int32`

GetImportedRows returns the ImportedRows field if non-nil, zero value otherwise.

### GetImportedRowsOk

`func (o *ResearchSummary) GetImportedRowsOk() (*int32, bool)`

GetImportedRowsOk returns a tuple with the ImportedRows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetImportedRows

`func (o *ResearchSummary) SetImportedRows(v int32)`

SetImportedRows sets ImportedRows field to given value.


### GetName

`func (o *ResearchSummary) GetName() string`

GetName returns the Name field if non-nil, zero value otherwise.

### GetNameOk

`func (o *ResearchSummary) GetNameOk() (*string, bool)`

GetNameOk returns a tuple with the Name field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetName

`func (o *ResearchSummary) SetName(v string)`

SetName sets Name field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
