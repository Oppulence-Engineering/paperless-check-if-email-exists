# ReputationCheckRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Domain** | **string** |  | [required]
**ForceRefresh** | Pointer to **bool** |  | [optional]

## Methods

### NewReputationCheckRequest

`func NewReputationCheckRequest(domain string) *ReputationCheckRequest`

NewReputationCheckRequest instantiates a new ReputationCheckRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewReputationCheckRequestWithDefaults

`func NewReputationCheckRequestWithDefaults() *ReputationCheckRequest`

NewReputationCheckRequestWithDefaults instantiates a new ReputationCheckRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDomain

`func (o *ReputationCheckRequest) GetDomain() string`

GetDomain returns the Domain field if non-nil, zero value otherwise.

### GetDomainOk

`func (o *ReputationCheckRequest) GetDomainOk() (*string, bool)`

GetDomainOk returns a tuple with the Domain field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomain

`func (o *ReputationCheckRequest) SetDomain(v string)`

SetDomain sets Domain field to given value.


### GetForceRefresh

`func (o *ReputationCheckRequest) GetForceRefresh() bool`

GetForceRefresh returns the ForceRefresh field if non-nil, zero value otherwise.

### GetForceRefreshOk

`func (o *ReputationCheckRequest) GetForceRefreshOk() (*bool, bool)`

GetForceRefreshOk returns a tuple with the ForceRefresh field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetForceRefresh

`func (o *ReputationCheckRequest) SetForceRefresh(v bool)`

SetForceRefresh sets ForceRefresh field to given value.

### HasForceRefresh

`func (o *ReputationCheckRequest) HasForceRefresh() bool`

HasForceRefresh returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
