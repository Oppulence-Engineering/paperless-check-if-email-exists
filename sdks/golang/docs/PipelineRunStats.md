# PipelineRunStats

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BilledEmails** | Pointer to **NullableInt32** |  | [optional]
**ChangedOnlyExport** | Pointer to **NullableBool** |  | [optional]
**CompletedTasks** | Pointer to **NullableInt32** |  | [optional]
**DeltaMode** | Pointer to **NullableBool** |  | [optional]
**FreshnessDays** | Pointer to **NullableInt32** |  | [optional]
**Invalid** | Pointer to **NullableInt32** |  | [optional]
**PublishedTasks** | Pointer to **NullableInt32** |  | [optional]
**QueuedEmails** | Pointer to **NullableInt32** |  | [optional]
**Risky** | Pointer to **NullableInt32** |  | [optional]
**SelectedUniqueEmails** | Pointer to **NullableInt32** |  | [optional]
**SkippedUnchanged** | Pointer to **NullableInt32** |  | [optional]
**SourceFilename** | Pointer to **NullableString** |  | [optional]
**SourceName** | Pointer to **NullableString** |  | [optional]
**SourceRowCount** | Pointer to **NullableInt32** |  | [optional]
**SourceUniqueEmails** | Pointer to **NullableInt32** |  | [optional]
**TriggerReason** | Pointer to **NullableString** |  | [optional]
**Unknown** | Pointer to **NullableInt32** |  | [optional]
**Valid** | Pointer to **NullableInt32** |  | [optional]

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

### GetCompletedTasks

`func (o *PipelineRunStats) GetCompletedTasks() int32`

GetCompletedTasks returns the CompletedTasks field if non-nil, zero value otherwise.

### GetCompletedTasksOk

`func (o *PipelineRunStats) GetCompletedTasksOk() (*int32, bool)`

GetCompletedTasksOk returns a tuple with the CompletedTasks field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCompletedTasks

`func (o *PipelineRunStats) SetCompletedTasks(v int32)`

SetCompletedTasks sets CompletedTasks field to given value.

### HasCompletedTasks

`func (o *PipelineRunStats) HasCompletedTasks() bool`

HasCompletedTasks returns a boolean if a field has been set.

### SetCompletedTasksNil

`func (o *PipelineRunStats) SetCompletedTasksNil()`

 SetCompletedTasksNil sets the value for CompletedTasks to be an explicit nil

### UnsetCompletedTasks
`func (o *PipelineRunStats) UnsetCompletedTasks()`

UnsetCompletedTasks ensures that no value is present for CompletedTasks, not even an explicit nil

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

### GetInvalid

`func (o *PipelineRunStats) GetInvalid() int32`

GetInvalid returns the Invalid field if non-nil, zero value otherwise.

### GetInvalidOk

`func (o *PipelineRunStats) GetInvalidOk() (*int32, bool)`

GetInvalidOk returns a tuple with the Invalid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvalid

`func (o *PipelineRunStats) SetInvalid(v int32)`

SetInvalid sets Invalid field to given value.

### HasInvalid

`func (o *PipelineRunStats) HasInvalid() bool`

HasInvalid returns a boolean if a field has been set.

### SetInvalidNil

`func (o *PipelineRunStats) SetInvalidNil()`

 SetInvalidNil sets the value for Invalid to be an explicit nil

### UnsetInvalid
`func (o *PipelineRunStats) UnsetInvalid()`

UnsetInvalid ensures that no value is present for Invalid, not even an explicit nil

### GetPublishedTasks

`func (o *PipelineRunStats) GetPublishedTasks() int32`

GetPublishedTasks returns the PublishedTasks field if non-nil, zero value otherwise.

### GetPublishedTasksOk

`func (o *PipelineRunStats) GetPublishedTasksOk() (*int32, bool)`

GetPublishedTasksOk returns a tuple with the PublishedTasks field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPublishedTasks

`func (o *PipelineRunStats) SetPublishedTasks(v int32)`

SetPublishedTasks sets PublishedTasks field to given value.

### HasPublishedTasks

`func (o *PipelineRunStats) HasPublishedTasks() bool`

HasPublishedTasks returns a boolean if a field has been set.

### SetPublishedTasksNil

`func (o *PipelineRunStats) SetPublishedTasksNil()`

 SetPublishedTasksNil sets the value for PublishedTasks to be an explicit nil

### UnsetPublishedTasks
`func (o *PipelineRunStats) UnsetPublishedTasks()`

UnsetPublishedTasks ensures that no value is present for PublishedTasks, not even an explicit nil

### GetQueuedEmails

`func (o *PipelineRunStats) GetQueuedEmails() int32`

GetQueuedEmails returns the QueuedEmails field if non-nil, zero value otherwise.

### GetQueuedEmailsOk

`func (o *PipelineRunStats) GetQueuedEmailsOk() (*int32, bool)`

GetQueuedEmailsOk returns a tuple with the QueuedEmails field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQueuedEmails

`func (o *PipelineRunStats) SetQueuedEmails(v int32)`

SetQueuedEmails sets QueuedEmails field to given value.

### HasQueuedEmails

`func (o *PipelineRunStats) HasQueuedEmails() bool`

HasQueuedEmails returns a boolean if a field has been set.

### SetQueuedEmailsNil

`func (o *PipelineRunStats) SetQueuedEmailsNil()`

 SetQueuedEmailsNil sets the value for QueuedEmails to be an explicit nil

### UnsetQueuedEmails
`func (o *PipelineRunStats) UnsetQueuedEmails()`

UnsetQueuedEmails ensures that no value is present for QueuedEmails, not even an explicit nil

### GetRisky

`func (o *PipelineRunStats) GetRisky() int32`

GetRisky returns the Risky field if non-nil, zero value otherwise.

### GetRiskyOk

`func (o *PipelineRunStats) GetRiskyOk() (*int32, bool)`

GetRiskyOk returns a tuple with the Risky field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRisky

`func (o *PipelineRunStats) SetRisky(v int32)`

SetRisky sets Risky field to given value.

### HasRisky

`func (o *PipelineRunStats) HasRisky() bool`

HasRisky returns a boolean if a field has been set.

### SetRiskyNil

`func (o *PipelineRunStats) SetRiskyNil()`

 SetRiskyNil sets the value for Risky to be an explicit nil

### UnsetRisky
`func (o *PipelineRunStats) UnsetRisky()`

UnsetRisky ensures that no value is present for Risky, not even an explicit nil

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

### GetUnknown

`func (o *PipelineRunStats) GetUnknown() int32`

GetUnknown returns the Unknown field if non-nil, zero value otherwise.

### GetUnknownOk

`func (o *PipelineRunStats) GetUnknownOk() (*int32, bool)`

GetUnknownOk returns a tuple with the Unknown field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnknown

`func (o *PipelineRunStats) SetUnknown(v int32)`

SetUnknown sets Unknown field to given value.

### HasUnknown

`func (o *PipelineRunStats) HasUnknown() bool`

HasUnknown returns a boolean if a field has been set.

### SetUnknownNil

`func (o *PipelineRunStats) SetUnknownNil()`

 SetUnknownNil sets the value for Unknown to be an explicit nil

### UnsetUnknown
`func (o *PipelineRunStats) UnsetUnknown()`

UnsetUnknown ensures that no value is present for Unknown, not even an explicit nil

### GetValid

`func (o *PipelineRunStats) GetValid() int32`

GetValid returns the Valid field if non-nil, zero value otherwise.

### GetValidOk

`func (o *PipelineRunStats) GetValidOk() (*int32, bool)`

GetValidOk returns a tuple with the Valid field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValid

`func (o *PipelineRunStats) SetValid(v int32)`

SetValid sets Valid field to given value.

### HasValid

`func (o *PipelineRunStats) HasValid() bool`

HasValid returns a boolean if a field has been set.

### SetValidNil

`func (o *PipelineRunStats) SetValidNil()`

 SetValidNil sets the value for Valid to be an explicit nil

### UnsetValid
`func (o *PipelineRunStats) UnsetValid()`

UnsetValid ensures that no value is present for Valid, not even an explicit nil

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
