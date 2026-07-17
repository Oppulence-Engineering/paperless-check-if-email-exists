# OutcomeIngestResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**AutoSuppressed** | **int64** |  | [required]
**Ignored** | **int64** |  | [required]
**Ingested** | **int64** |  | [required]

## Methods

### NewOutcomeIngestResponse

`func NewOutcomeIngestResponse(autoSuppressed int64, ignored int64, ingested int64) *OutcomeIngestResponse`

NewOutcomeIngestResponse instantiates a new OutcomeIngestResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOutcomeIngestResponseWithDefaults

`func NewOutcomeIngestResponseWithDefaults() *OutcomeIngestResponse`

NewOutcomeIngestResponseWithDefaults instantiates a new OutcomeIngestResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAutoSuppressed

`func (o *OutcomeIngestResponse) GetAutoSuppressed() int64`

GetAutoSuppressed returns the AutoSuppressed field if non-nil, zero value otherwise.

### GetAutoSuppressedOk

`func (o *OutcomeIngestResponse) GetAutoSuppressedOk() (*int64, bool)`

GetAutoSuppressedOk returns a tuple with the AutoSuppressed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAutoSuppressed

`func (o *OutcomeIngestResponse) SetAutoSuppressed(v int64)`

SetAutoSuppressed sets AutoSuppressed field to given value.


### GetIgnored

`func (o *OutcomeIngestResponse) GetIgnored() int64`

GetIgnored returns the Ignored field if non-nil, zero value otherwise.

### GetIgnoredOk

`func (o *OutcomeIngestResponse) GetIgnoredOk() (*int64, bool)`

GetIgnoredOk returns a tuple with the Ignored field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIgnored

`func (o *OutcomeIngestResponse) SetIgnored(v int64)`

SetIgnored sets Ignored field to given value.


### GetIngested

`func (o *OutcomeIngestResponse) GetIngested() int64`

GetIngested returns the Ingested field if non-nil, zero value otherwise.

### GetIngestedOk

`func (o *OutcomeIngestResponse) GetIngestedOk() (*int64, bool)`

GetIngestedOk returns a tuple with the Ingested field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIngested

`func (o *OutcomeIngestResponse) SetIngested(v int64)`

SetIngested sets Ingested field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
