# ResearchMailbox

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Action** | **string** |  | [required]
**CatchAll** | **bool** |  | [required]
**Provider** | **string** |  | [required]
**Result** | **string** |  | [required]

## Methods

### NewResearchMailbox

`func NewResearchMailbox(action string, catchAll bool, provider string, result string) *ResearchMailbox`

NewResearchMailbox instantiates a new ResearchMailbox object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewResearchMailboxWithDefaults

`func NewResearchMailboxWithDefaults() *ResearchMailbox`

NewResearchMailboxWithDefaults instantiates a new ResearchMailbox object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetAction

`func (o *ResearchMailbox) GetAction() string`

GetAction returns the Action field if non-nil, zero value otherwise.

### GetActionOk

`func (o *ResearchMailbox) GetActionOk() (*string, bool)`

GetActionOk returns a tuple with the Action field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetAction

`func (o *ResearchMailbox) SetAction(v string)`

SetAction sets Action field to given value.


### GetCatchAll

`func (o *ResearchMailbox) GetCatchAll() bool`

GetCatchAll returns the CatchAll field if non-nil, zero value otherwise.

### GetCatchAllOk

`func (o *ResearchMailbox) GetCatchAllOk() (*bool, bool)`

GetCatchAllOk returns a tuple with the CatchAll field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetCatchAll

`func (o *ResearchMailbox) SetCatchAll(v bool)`

SetCatchAll sets CatchAll field to given value.


### GetProvider

`func (o *ResearchMailbox) GetProvider() string`

GetProvider returns the Provider field if non-nil, zero value otherwise.

### GetProviderOk

`func (o *ResearchMailbox) GetProviderOk() (*string, bool)`

GetProviderOk returns a tuple with the Provider field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProvider

`func (o *ResearchMailbox) SetProvider(v string)`

SetProvider sets Provider field to given value.


### GetResult

`func (o *ResearchMailbox) GetResult() string`

GetResult returns the Result field if non-nil, zero value otherwise.

### GetResultOk

`func (o *ResearchMailbox) GetResultOk() (*string, bool)`

GetResultOk returns a tuple with the Result field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetResult

`func (o *ResearchMailbox) SetResult(v string)`

SetResult sets Result field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
