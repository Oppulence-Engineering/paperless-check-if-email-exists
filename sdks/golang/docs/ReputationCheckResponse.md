# ReputationCheckResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BlacklistResults** | [**[]BlacklistResult**](BlacklistResult.md) |  | [required]
**Cached** | **bool** |  | [required]
**DnsRecords** | [**DnsRecordResults**](DnsRecordResults.md) |  | [required]
**Domain** | **string** |  | [required]
**DomainInfo** | [**DomainInfo**](DomainInfo.md) |  | [required]
**RiskLevel** | **string** |  | [required]
**Score** | **int32** |  | [required]

## Methods

### NewReputationCheckResponse

`func NewReputationCheckResponse(blacklistResults []BlacklistResult, cached bool, dnsRecords DnsRecordResults, domain string, domainInfo DomainInfo, riskLevel string, score int32) *ReputationCheckResponse`

NewReputationCheckResponse instantiates a new ReputationCheckResponse object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewReputationCheckResponseWithDefaults

`func NewReputationCheckResponseWithDefaults() *ReputationCheckResponse`

NewReputationCheckResponseWithDefaults instantiates a new ReputationCheckResponse object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBlacklistResults

`func (o *ReputationCheckResponse) GetBlacklistResults() []BlacklistResult`

GetBlacklistResults returns the BlacklistResults field if non-nil, zero value otherwise.

### GetBlacklistResultsOk

`func (o *ReputationCheckResponse) GetBlacklistResultsOk() ([]BlacklistResult, bool)`

GetBlacklistResultsOk returns a tuple with the BlacklistResults field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBlacklistResults

`func (o *ReputationCheckResponse) SetBlacklistResults(v []BlacklistResult)`

SetBlacklistResults sets BlacklistResults field to given value.


### GetCached

`func (o *ReputationCheckResponse) GetCached() bool`

GetCached returns the Cached field if non-nil, zero value otherwise.

### GetCachedOk

`func (o *ReputationCheckResponse) GetCachedOk() (*bool, bool)`

GetCachedOk returns a tuple with the Cached field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCached

`func (o *ReputationCheckResponse) SetCached(v bool)`

SetCached sets Cached field to given value.


### GetDnsRecords

`func (o *ReputationCheckResponse) GetDnsRecords() DnsRecordResults`

GetDnsRecords returns the DnsRecords field if non-nil, zero value otherwise.

### GetDnsRecordsOk

`func (o *ReputationCheckResponse) GetDnsRecordsOk() (*DnsRecordResults, bool)`

GetDnsRecordsOk returns a tuple with the DnsRecords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDnsRecords

`func (o *ReputationCheckResponse) SetDnsRecords(v DnsRecordResults)`

SetDnsRecords sets DnsRecords field to given value.


### GetDomain

`func (o *ReputationCheckResponse) GetDomain() string`

GetDomain returns the Domain field if non-nil, zero value otherwise.

### GetDomainOk

`func (o *ReputationCheckResponse) GetDomainOk() (*string, bool)`

GetDomainOk returns a tuple with the Domain field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomain

`func (o *ReputationCheckResponse) SetDomain(v string)`

SetDomain sets Domain field to given value.


### GetDomainInfo

`func (o *ReputationCheckResponse) GetDomainInfo() DomainInfo`

GetDomainInfo returns the DomainInfo field if non-nil, zero value otherwise.

### GetDomainInfoOk

`func (o *ReputationCheckResponse) GetDomainInfoOk() (*DomainInfo, bool)`

GetDomainInfoOk returns a tuple with the DomainInfo field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomainInfo

`func (o *ReputationCheckResponse) SetDomainInfo(v DomainInfo)`

SetDomainInfo sets DomainInfo field to given value.


### GetRiskLevel

`func (o *ReputationCheckResponse) GetRiskLevel() string`

GetRiskLevel returns the RiskLevel field if non-nil, zero value otherwise.

### GetRiskLevelOk

`func (o *ReputationCheckResponse) GetRiskLevelOk() (*string, bool)`

GetRiskLevelOk returns a tuple with the RiskLevel field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRiskLevel

`func (o *ReputationCheckResponse) SetRiskLevel(v string)`

SetRiskLevel sets RiskLevel field to given value.


### GetScore

`func (o *ReputationCheckResponse) GetScore() int32`

GetScore returns the Score field if non-nil, zero value otherwise.

### GetScoreOk

`func (o *ReputationCheckResponse) GetScoreOk() (*int32, bool)`

GetScoreOk returns a tuple with the Score field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetScore

`func (o *ReputationCheckResponse) SetScore(v int32)`

SetScore sets Score field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
