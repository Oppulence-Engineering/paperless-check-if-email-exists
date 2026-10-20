# BlacklistResult

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Answers** | **[]string** |  | [required]
**Error** | **NullableString** |  | [required]
**Listed** | **NullableBool** | Null for failed or skipped queries; false only for a completed negative lookup. | [required]
**LookupTimeMs** | **int64** |  | [required]
**Provider** | **string** |  | [required]
**Status** | **string** |  | [required]

## Methods

### NewBlacklistResult

`func NewBlacklistResult(answers []string, error_ NullableString, listed NullableBool, lookupTimeMs int64, provider string, status string) *BlacklistResult`

NewBlacklistResult instantiates a new BlacklistResult object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewBlacklistResultWithDefaults

`func NewBlacklistResultWithDefaults() *BlacklistResult`

NewBlacklistResultWithDefaults instantiates a new BlacklistResult object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAnswers

`func (o *BlacklistResult) GetAnswers() []string`

GetAnswers returns the Answers field if non-nil, zero value otherwise.

### GetAnswersOk

`func (o *BlacklistResult) GetAnswersOk() ([]string, bool)`

GetAnswersOk returns a tuple with the Answers field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAnswers

`func (o *BlacklistResult) SetAnswers(v []string)`

SetAnswers sets Answers field to given value.


### GetError

`func (o *BlacklistResult) GetError() string`

GetError returns the Error field if non-nil, zero value otherwise.

### GetErrorOk

`func (o *BlacklistResult) GetErrorOk() (*string, bool)`

GetErrorOk returns a tuple with the Error field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetError

`func (o *BlacklistResult) SetError(v string)`

SetError sets Error field to given value.


### GetListed

`func (o *BlacklistResult) GetListed() bool`

GetListed returns the Listed field if non-nil, zero value otherwise.

### GetListedOk

`func (o *BlacklistResult) GetListedOk() (*bool, bool)`

GetListedOk returns a tuple with the Listed field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetListed

`func (o *BlacklistResult) SetListed(v bool)`

SetListed sets Listed field to given value.


### GetLookupTimeMs

`func (o *BlacklistResult) GetLookupTimeMs() int64`

GetLookupTimeMs returns the LookupTimeMs field if non-nil, zero value otherwise.

### GetLookupTimeMsOk

`func (o *BlacklistResult) GetLookupTimeMsOk() (*int64, bool)`

GetLookupTimeMsOk returns a tuple with the LookupTimeMs field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetLookupTimeMs

`func (o *BlacklistResult) SetLookupTimeMs(v int64)`

SetLookupTimeMs sets LookupTimeMs field to given value.


### GetProvider

`func (o *BlacklistResult) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *BlacklistResult) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *BlacklistResult) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetStatus

`func (o *BlacklistResult) GetStatus() string`

GetStatus returns the Status field if non-nil, zero value otherwise.

### GetStatusOk

`func (o *BlacklistResult) GetStatusOk() (*string, bool)`

GetStatusOk returns a tuple with the Status field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetStatus

`func (o *BlacklistResult) SetStatus(v string)`

SetStatus sets Status field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
