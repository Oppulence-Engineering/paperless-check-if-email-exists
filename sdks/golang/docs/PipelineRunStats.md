# PipelineRunStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AcceptedRows** | Pointer to **NullableInt32** |  | [optional]
**BilledEmails** | Pointer to **NullableInt32** |  | [optional]
**ChangedOnlyExport** | Pointer to **NullableBool** |  | [optional]
**DeltaMode** | Pointer to **NullableBool** |  | [optional]
**FreshnessDays** | Pointer to **NullableInt32** |  | [optional]
**JobId** | Pointer to **NullableInt32** |  | [optional]
**ListId** | Pointer to **NullableInt32** |  | [optional]
**SelectedUniqueEmails** | Pointer to **NullableInt32** |  | [optional]
**SkippedUnchanged** | Pointer to **NullableInt32** |  | [optional]
**SourceFilename** | Pointer to **NullableString** |  | [optional]
**SourceName** | Pointer to **NullableString** |  | [optional]
**SourceRowCount** | Pointer to **NullableInt32** |  | [optional]
**SourceUniqueEmails** | Pointer to **NullableInt32** |  | [optional]
**TotalFailed** | Pointer to **NullableInt64** |  | [optional]
**TotalInvalid** | Pointer to **NullableInt64** |  | [optional]
**TotalProcessed** | Pointer to **NullableInt64** |  | [optional]
**TotalRisky** | Pointer to **NullableInt64** |  | [optional]
**TotalUnknown** | Pointer to **NullableInt64** |  | [optional]
**TotalValid** | Pointer to **NullableInt64** |  | [optional]
**TriggerReason** | Pointer to **NullableString** |  | [optional]

## Methods

### NewPipelineRunStats

`func NewPipelineRunStats() *PipelineRunStats`

NewPipelineRunStats instantiates a new PipelineRunStats object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewPipelineRunStatsWithDefaults

`func NewPipelineRunStatsWithDefaults() *PipelineRunStats`

NewPipelineRunStatsWithDefaults instantiates a new PipelineRunStats object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAcceptedRows

`func (o *PipelineRunStats) GetAcceptedRows() int32`

GetAcceptedRows returns the AcceptedRows field if non-nil, zero value otherwise.

### GetAcceptedRowsOk

`func (o *PipelineRunStats) GetAcceptedRowsOk() (*int32, bool)`

GetAcceptedRowsOk returns a tuple with the AcceptedRows field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAcceptedRows

`func (o *PipelineRunStats) SetAcceptedRows(v int32)`

SetAcceptedRows sets AcceptedRows field to given value.

### HasAcceptedRows

`func (o *PipelineRunStats) HasAcceptedRows() bool`

HasAcceptedRows returns a boolean if a field has been set.

### SetAcceptedRowsNil

`func (o *PipelineRunStats) SetAcceptedRowsNil()`

 SetAcceptedRowsNil sets the value for AcceptedRows to be an explicit nil

### UnsetAcceptedRows
`func (o *PipelineRunStats) UnsetAcceptedRows()`

UnsetAcceptedRows ensures that no value is present for AcceptedRows, not even an explicit nil

### GetBilledEmails

`func (o *PipelineRunStats) GetBilledEmails() int32`

GetBilledEmails returns the BilledEmails field if non-nil, zero value otherwise.

### GetBilledEmailsOk

`func (o *PipelineRunStats) GetBilledEmailsOk() (*int32, bool)`

GetBilledEmailsOk returns a tuple with the BilledEmails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBilledEmails

`func (o *PipelineRunStats) SetBilledEmails(v int32)`

SetBilledEmails sets BilledEmails field to given value.

### HasBilledEmails

`func (o *PipelineRunStats) HasBilledEmails() bool`

HasBilledEmails returns a boolean if a field has been set.

### SetBilledEmailsNil

`func (o *PipelineRunStats) SetBilledEmailsNil()`

 SetBilledEmailsNil sets the value for BilledEmails to be an explicit nil

### UnsetBilledEmails
`func (o *PipelineRunStats) UnsetBilledEmails()`

UnsetBilledEmails ensures that no value is present for BilledEmails, not even an explicit nil

### GetChangedOnlyExport

`func (o *PipelineRunStats) GetChangedOnlyExport() bool`

GetChangedOnlyExport returns the ChangedOnlyExport field if non-nil, zero value otherwise.

### GetChangedOnlyExportOk

`func (o *PipelineRunStats) GetChangedOnlyExportOk() (*bool, bool)`

GetChangedOnlyExportOk returns a tuple with the ChangedOnlyExport field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetChangedOnlyExport

`func (o *PipelineRunStats) SetChangedOnlyExport(v bool)`

SetChangedOnlyExport sets ChangedOnlyExport field to given value.

### HasChangedOnlyExport

`func (o *PipelineRunStats) HasChangedOnlyExport() bool`

HasChangedOnlyExport returns a boolean if a field has been set.

### SetChangedOnlyExportNil

`func (o *PipelineRunStats) SetChangedOnlyExportNil()`

 SetChangedOnlyExportNil sets the value for ChangedOnlyExport to be an explicit nil

### UnsetChangedOnlyExport
`func (o *PipelineRunStats) UnsetChangedOnlyExport()`

UnsetChangedOnlyExport ensures that no value is present for ChangedOnlyExport, not even an explicit nil

### GetDeltaMode

`func (o *PipelineRunStats) GetDeltaMode() bool`

GetDeltaMode returns the DeltaMode field if non-nil, zero value otherwise.

### GetDeltaModeOk

`func (o *PipelineRunStats) GetDeltaModeOk() (*bool, bool)`

GetDeltaModeOk returns a tuple with the DeltaMode field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeltaMode

`func (o *PipelineRunStats) SetDeltaMode(v bool)`

SetDeltaMode sets DeltaMode field to given value.

### HasDeltaMode

`func (o *PipelineRunStats) HasDeltaMode() bool`

HasDeltaMode returns a boolean if a field has been set.

### SetDeltaModeNil

`func (o *PipelineRunStats) SetDeltaModeNil()`

 SetDeltaModeNil sets the value for DeltaMode to be an explicit nil

### UnsetDeltaMode
`func (o *PipelineRunStats) UnsetDeltaMode()`

UnsetDeltaMode ensures that no value is present for DeltaMode, not even an explicit nil

### GetFreshnessDays

`func (o *PipelineRunStats) GetFreshnessDays() int32`

GetFreshnessDays returns the FreshnessDays field if non-nil, zero value otherwise.

### GetFreshnessDaysOk

`func (o *PipelineRunStats) GetFreshnessDaysOk() (*int32, bool)`

GetFreshnessDaysOk returns a tuple with the FreshnessDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetFreshnessDays

`func (o *PipelineRunStats) SetFreshnessDays(v int32)`

SetFreshnessDays sets FreshnessDays field to given value.

### HasFreshnessDays

`func (o *PipelineRunStats) HasFreshnessDays() bool`

HasFreshnessDays returns a boolean if a field has been set.

### SetFreshnessDaysNil

`func (o *PipelineRunStats) SetFreshnessDaysNil()`

 SetFreshnessDaysNil sets the value for FreshnessDays to be an explicit nil

### UnsetFreshnessDays
`func (o *PipelineRunStats) UnsetFreshnessDays()`

UnsetFreshnessDays ensures that no value is present for FreshnessDays, not even an explicit nil

### GetJobId

`func (o *PipelineRunStats) GetJobId() int32`

GetJobId returns the JobId field if non-nil, zero value otherwise.

### GetJobIdOk

`func (o *PipelineRunStats) GetJobIdOk() (*int32, bool)`

GetJobIdOk returns a tuple with the JobId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetJobId

`func (o *PipelineRunStats) SetJobId(v int32)`

SetJobId sets JobId field to given value.

### HasJobId

`func (o *PipelineRunStats) HasJobId() bool`

HasJobId returns a boolean if a field has been set.

### SetJobIdNil

`func (o *PipelineRunStats) SetJobIdNil()`

 SetJobIdNil sets the value for JobId to be an explicit nil

### UnsetJobId
`func (o *PipelineRunStats) UnsetJobId()`

UnsetJobId ensures that no value is present for JobId, not even an explicit nil

### GetListId

`func (o *PipelineRunStats) GetListId() int32`

GetListId returns the ListId field if non-nil, zero value otherwise.

### GetListIdOk

`func (o *PipelineRunStats) GetListIdOk() (*int32, bool)`

GetListIdOk returns a tuple with the ListId field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListId

`func (o *PipelineRunStats) SetListId(v int32)`

SetListId sets ListId field to given value.

### HasListId

`func (o *PipelineRunStats) HasListId() bool`

HasListId returns a boolean if a field has been set.

### SetListIdNil

`func (o *PipelineRunStats) SetListIdNil()`

 SetListIdNil sets the value for ListId to be an explicit nil

### UnsetListId
`func (o *PipelineRunStats) UnsetListId()`

UnsetListId ensures that no value is present for ListId, not even an explicit nil

### GetSelectedUniqueEmails

`func (o *PipelineRunStats) GetSelectedUniqueEmails() int32`

GetSelectedUniqueEmails returns the SelectedUniqueEmails field if non-nil, zero value otherwise.

### GetSelectedUniqueEmailsOk

`func (o *PipelineRunStats) GetSelectedUniqueEmailsOk() (*int32, bool)`

GetSelectedUniqueEmailsOk returns a tuple with the SelectedUniqueEmails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSelectedUniqueEmails

`func (o *PipelineRunStats) SetSelectedUniqueEmails(v int32)`

SetSelectedUniqueEmails sets SelectedUniqueEmails field to given value.

### HasSelectedUniqueEmails

`func (o *PipelineRunStats) HasSelectedUniqueEmails() bool`

HasSelectedUniqueEmails returns a boolean if a field has been set.

### SetSelectedUniqueEmailsNil

`func (o *PipelineRunStats) SetSelectedUniqueEmailsNil()`

 SetSelectedUniqueEmailsNil sets the value for SelectedUniqueEmails to be an explicit nil

### UnsetSelectedUniqueEmails
`func (o *PipelineRunStats) UnsetSelectedUniqueEmails()`

UnsetSelectedUniqueEmails ensures that no value is present for SelectedUniqueEmails, not even an explicit nil

### GetSkippedUnchanged

`func (o *PipelineRunStats) GetSkippedUnchanged() int32`

GetSkippedUnchanged returns the SkippedUnchanged field if non-nil, zero value otherwise.

### GetSkippedUnchangedOk

`func (o *PipelineRunStats) GetSkippedUnchangedOk() (*int32, bool)`

GetSkippedUnchangedOk returns a tuple with the SkippedUnchanged field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSkippedUnchanged

`func (o *PipelineRunStats) SetSkippedUnchanged(v int32)`

SetSkippedUnchanged sets SkippedUnchanged field to given value.

### HasSkippedUnchanged

`func (o *PipelineRunStats) HasSkippedUnchanged() bool`

HasSkippedUnchanged returns a boolean if a field has been set.

### SetSkippedUnchangedNil

`func (o *PipelineRunStats) SetSkippedUnchangedNil()`

 SetSkippedUnchangedNil sets the value for SkippedUnchanged to be an explicit nil

### UnsetSkippedUnchanged
`func (o *PipelineRunStats) UnsetSkippedUnchanged()`

UnsetSkippedUnchanged ensures that no value is present for SkippedUnchanged, not even an explicit nil

### GetSourceFilename

`func (o *PipelineRunStats) GetSourceFilename() string`

GetSourceFilename returns the SourceFilename field if non-nil, zero value otherwise.

### GetSourceFilenameOk

`func (o *PipelineRunStats) GetSourceFilenameOk() (*string, bool)`

GetSourceFilenameOk returns a tuple with the SourceFilename field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceFilename

`func (o *PipelineRunStats) SetSourceFilename(v string)`

SetSourceFilename sets SourceFilename field to given value.

### HasSourceFilename

`func (o *PipelineRunStats) HasSourceFilename() bool`

HasSourceFilename returns a boolean if a field has been set.

### SetSourceFilenameNil

`func (o *PipelineRunStats) SetSourceFilenameNil()`

 SetSourceFilenameNil sets the value for SourceFilename to be an explicit nil

### UnsetSourceFilename
`func (o *PipelineRunStats) UnsetSourceFilename()`

UnsetSourceFilename ensures that no value is present for SourceFilename, not even an explicit nil

### GetSourceName

`func (o *PipelineRunStats) GetSourceName() string`

GetSourceName returns the SourceName field if non-nil, zero value otherwise.

### GetSourceNameOk

`func (o *PipelineRunStats) GetSourceNameOk() (*string, bool)`

GetSourceNameOk returns a tuple with the SourceName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceName

`func (o *PipelineRunStats) SetSourceName(v string)`

SetSourceName sets SourceName field to given value.

### HasSourceName

`func (o *PipelineRunStats) HasSourceName() bool`

HasSourceName returns a boolean if a field has been set.

### SetSourceNameNil

`func (o *PipelineRunStats) SetSourceNameNil()`

 SetSourceNameNil sets the value for SourceName to be an explicit nil

### UnsetSourceName
`func (o *PipelineRunStats) UnsetSourceName()`

UnsetSourceName ensures that no value is present for SourceName, not even an explicit nil

### GetSourceRowCount

`func (o *PipelineRunStats) GetSourceRowCount() int32`

GetSourceRowCount returns the SourceRowCount field if non-nil, zero value otherwise.

### GetSourceRowCountOk

`func (o *PipelineRunStats) GetSourceRowCountOk() (*int32, bool)`

GetSourceRowCountOk returns a tuple with the SourceRowCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceRowCount

`func (o *PipelineRunStats) SetSourceRowCount(v int32)`

SetSourceRowCount sets SourceRowCount field to given value.

### HasSourceRowCount

`func (o *PipelineRunStats) HasSourceRowCount() bool`

HasSourceRowCount returns a boolean if a field has been set.

### SetSourceRowCountNil

`func (o *PipelineRunStats) SetSourceRowCountNil()`

 SetSourceRowCountNil sets the value for SourceRowCount to be an explicit nil

### UnsetSourceRowCount
`func (o *PipelineRunStats) UnsetSourceRowCount()`

UnsetSourceRowCount ensures that no value is present for SourceRowCount, not even an explicit nil

### GetSourceUniqueEmails

`func (o *PipelineRunStats) GetSourceUniqueEmails() int32`

GetSourceUniqueEmails returns the SourceUniqueEmails field if non-nil, zero value otherwise.

### GetSourceUniqueEmailsOk

`func (o *PipelineRunStats) GetSourceUniqueEmailsOk() (*int32, bool)`

GetSourceUniqueEmailsOk returns a tuple with the SourceUniqueEmails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceUniqueEmails

`func (o *PipelineRunStats) SetSourceUniqueEmails(v int32)`

SetSourceUniqueEmails sets SourceUniqueEmails field to given value.

### HasSourceUniqueEmails

`func (o *PipelineRunStats) HasSourceUniqueEmails() bool`

HasSourceUniqueEmails returns a boolean if a field has been set.

### SetSourceUniqueEmailsNil

`func (o *PipelineRunStats) SetSourceUniqueEmailsNil()`

 SetSourceUniqueEmailsNil sets the value for SourceUniqueEmails to be an explicit nil

### UnsetSourceUniqueEmails
`func (o *PipelineRunStats) UnsetSourceUniqueEmails()`

UnsetSourceUniqueEmails ensures that no value is present for SourceUniqueEmails, not even an explicit nil

### GetTotalFailed

`func (o *PipelineRunStats) GetTotalFailed() int64`

GetTotalFailed returns the TotalFailed field if non-nil, zero value otherwise.

### GetTotalFailedOk

`func (o *PipelineRunStats) GetTotalFailedOk() (*int64, bool)`

GetTotalFailedOk returns a tuple with the TotalFailed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalFailed

`func (o *PipelineRunStats) SetTotalFailed(v int64)`

SetTotalFailed sets TotalFailed field to given value.

### HasTotalFailed

`func (o *PipelineRunStats) HasTotalFailed() bool`

HasTotalFailed returns a boolean if a field has been set.

### SetTotalFailedNil

`func (o *PipelineRunStats) SetTotalFailedNil()`

 SetTotalFailedNil sets the value for TotalFailed to be an explicit nil

### UnsetTotalFailed
`func (o *PipelineRunStats) UnsetTotalFailed()`

UnsetTotalFailed ensures that no value is present for TotalFailed, not even an explicit nil

### GetTotalInvalid

`func (o *PipelineRunStats) GetTotalInvalid() int64`

GetTotalInvalid returns the TotalInvalid field if non-nil, zero value otherwise.

### GetTotalInvalidOk

`func (o *PipelineRunStats) GetTotalInvalidOk() (*int64, bool)`

GetTotalInvalidOk returns a tuple with the TotalInvalid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalInvalid

`func (o *PipelineRunStats) SetTotalInvalid(v int64)`

SetTotalInvalid sets TotalInvalid field to given value.

### HasTotalInvalid

`func (o *PipelineRunStats) HasTotalInvalid() bool`

HasTotalInvalid returns a boolean if a field has been set.

### SetTotalInvalidNil

`func (o *PipelineRunStats) SetTotalInvalidNil()`

 SetTotalInvalidNil sets the value for TotalInvalid to be an explicit nil

### UnsetTotalInvalid
`func (o *PipelineRunStats) UnsetTotalInvalid()`

UnsetTotalInvalid ensures that no value is present for TotalInvalid, not even an explicit nil

### GetTotalProcessed

`func (o *PipelineRunStats) GetTotalProcessed() int64`

GetTotalProcessed returns the TotalProcessed field if non-nil, zero value otherwise.

### GetTotalProcessedOk

`func (o *PipelineRunStats) GetTotalProcessedOk() (*int64, bool)`

GetTotalProcessedOk returns a tuple with the TotalProcessed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalProcessed

`func (o *PipelineRunStats) SetTotalProcessed(v int64)`

SetTotalProcessed sets TotalProcessed field to given value.

### HasTotalProcessed

`func (o *PipelineRunStats) HasTotalProcessed() bool`

HasTotalProcessed returns a boolean if a field has been set.

### SetTotalProcessedNil

`func (o *PipelineRunStats) SetTotalProcessedNil()`

 SetTotalProcessedNil sets the value for TotalProcessed to be an explicit nil

### UnsetTotalProcessed
`func (o *PipelineRunStats) UnsetTotalProcessed()`

UnsetTotalProcessed ensures that no value is present for TotalProcessed, not even an explicit nil

### GetTotalRisky

`func (o *PipelineRunStats) GetTotalRisky() int64`

GetTotalRisky returns the TotalRisky field if non-nil, zero value otherwise.

### GetTotalRiskyOk

`func (o *PipelineRunStats) GetTotalRiskyOk() (*int64, bool)`

GetTotalRiskyOk returns a tuple with the TotalRisky field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRisky

`func (o *PipelineRunStats) SetTotalRisky(v int64)`

SetTotalRisky sets TotalRisky field to given value.

### HasTotalRisky

`func (o *PipelineRunStats) HasTotalRisky() bool`

HasTotalRisky returns a boolean if a field has been set.

### SetTotalRiskyNil

`func (o *PipelineRunStats) SetTotalRiskyNil()`

 SetTotalRiskyNil sets the value for TotalRisky to be an explicit nil

### UnsetTotalRisky
`func (o *PipelineRunStats) UnsetTotalRisky()`

UnsetTotalRisky ensures that no value is present for TotalRisky, not even an explicit nil

### GetTotalUnknown

`func (o *PipelineRunStats) GetTotalUnknown() int64`

GetTotalUnknown returns the TotalUnknown field if non-nil, zero value otherwise.

### GetTotalUnknownOk

`func (o *PipelineRunStats) GetTotalUnknownOk() (*int64, bool)`

GetTotalUnknownOk returns a tuple with the TotalUnknown field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalUnknown

`func (o *PipelineRunStats) SetTotalUnknown(v int64)`

SetTotalUnknown sets TotalUnknown field to given value.

### HasTotalUnknown

`func (o *PipelineRunStats) HasTotalUnknown() bool`

HasTotalUnknown returns a boolean if a field has been set.

### SetTotalUnknownNil

`func (o *PipelineRunStats) SetTotalUnknownNil()`

 SetTotalUnknownNil sets the value for TotalUnknown to be an explicit nil

### UnsetTotalUnknown
`func (o *PipelineRunStats) UnsetTotalUnknown()`

UnsetTotalUnknown ensures that no value is present for TotalUnknown, not even an explicit nil

### GetTotalValid

`func (o *PipelineRunStats) GetTotalValid() int64`

GetTotalValid returns the TotalValid field if non-nil, zero value otherwise.

### GetTotalValidOk

`func (o *PipelineRunStats) GetTotalValidOk() (*int64, bool)`

GetTotalValidOk returns a tuple with the TotalValid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalValid

`func (o *PipelineRunStats) SetTotalValid(v int64)`

SetTotalValid sets TotalValid field to given value.

### HasTotalValid

`func (o *PipelineRunStats) HasTotalValid() bool`

HasTotalValid returns a boolean if a field has been set.

### SetTotalValidNil

`func (o *PipelineRunStats) SetTotalValidNil()`

 SetTotalValidNil sets the value for TotalValid to be an explicit nil

### UnsetTotalValid
`func (o *PipelineRunStats) UnsetTotalValid()`

UnsetTotalValid ensures that no value is present for TotalValid, not even an explicit nil

### GetTriggerReason

`func (o *PipelineRunStats) GetTriggerReason() string`

GetTriggerReason returns the TriggerReason field if non-nil, zero value otherwise.

### GetTriggerReasonOk

`func (o *PipelineRunStats) GetTriggerReasonOk() (*string, bool)`

GetTriggerReasonOk returns a tuple with the TriggerReason field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTriggerReason

`func (o *PipelineRunStats) SetTriggerReason(v string)`

SetTriggerReason sets TriggerReason field to given value.

### HasTriggerReason

`func (o *PipelineRunStats) HasTriggerReason() bool`

HasTriggerReason returns a boolean if a field has been set.

### SetTriggerReasonNil

`func (o *PipelineRunStats) SetTriggerReasonNil()`

 SetTriggerReasonNil sets the value for TriggerReason to be an explicit nil

### UnsetTriggerReason
`func (o *PipelineRunStats) UnsetTriggerReason()`

UnsetTriggerReason ensures that no value is present for TriggerReason, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
