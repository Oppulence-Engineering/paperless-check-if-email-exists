# OutcomePolicyRules

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Click** | Pointer to [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**Complaint** | Pointer to [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**Delivered** | Pointer to [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**HardBounce** | Pointer to [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**Open** | Pointer to [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**OutcomeTtlDays** | Pointer to **int32** |  | [optional]
**SoftBounce** | Pointer to [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]
**Unsubscribe** | Pointer to [**OutcomeRuleAction**](OutcomeRuleAction.md) |  | [optional]

## Methods

### NewOutcomePolicyRules

`func NewOutcomePolicyRules() *OutcomePolicyRules`

NewOutcomePolicyRules instantiates a new OutcomePolicyRules object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOutcomePolicyRulesWithDefaults

`func NewOutcomePolicyRulesWithDefaults() *OutcomePolicyRules`

NewOutcomePolicyRulesWithDefaults instantiates a new OutcomePolicyRules object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetClick

`func (o *OutcomePolicyRules) GetClick() OutcomeRuleAction`

GetClick returns the Click field if non-nil, zero value otherwise.

### GetClickOk

`func (o *OutcomePolicyRules) GetClickOk() (*OutcomeRuleAction, bool)`

GetClickOk returns a tuple with the Click field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClick

`func (o *OutcomePolicyRules) SetClick(v OutcomeRuleAction)`

SetClick sets Click field to given value.

### HasClick

`func (o *OutcomePolicyRules) HasClick() bool`

HasClick returns a boolean if a field has been set.

### GetComplaint

`func (o *OutcomePolicyRules) GetComplaint() OutcomeRuleAction`

GetComplaint returns the Complaint field if non-nil, zero value otherwise.

### GetComplaintOk

`func (o *OutcomePolicyRules) GetComplaintOk() (*OutcomeRuleAction, bool)`

GetComplaintOk returns a tuple with the Complaint field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetComplaint

`func (o *OutcomePolicyRules) SetComplaint(v OutcomeRuleAction)`

SetComplaint sets Complaint field to given value.

### HasComplaint

`func (o *OutcomePolicyRules) HasComplaint() bool`

HasComplaint returns a boolean if a field has been set.

### GetDelivered

`func (o *OutcomePolicyRules) GetDelivered() OutcomeRuleAction`

GetDelivered returns the Delivered field if non-nil, zero value otherwise.

### GetDeliveredOk

`func (o *OutcomePolicyRules) GetDeliveredOk() (*OutcomeRuleAction, bool)`

GetDeliveredOk returns a tuple with the Delivered field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDelivered

`func (o *OutcomePolicyRules) SetDelivered(v OutcomeRuleAction)`

SetDelivered sets Delivered field to given value.

### HasDelivered

`func (o *OutcomePolicyRules) HasDelivered() bool`

HasDelivered returns a boolean if a field has been set.

### GetHardBounce

`func (o *OutcomePolicyRules) GetHardBounce() OutcomeRuleAction`

GetHardBounce returns the HardBounce field if non-nil, zero value otherwise.

### GetHardBounceOk

`func (o *OutcomePolicyRules) GetHardBounceOk() (*OutcomeRuleAction, bool)`

GetHardBounceOk returns a tuple with the HardBounce field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetHardBounce

`func (o *OutcomePolicyRules) SetHardBounce(v OutcomeRuleAction)`

SetHardBounce sets HardBounce field to given value.

### HasHardBounce

`func (o *OutcomePolicyRules) HasHardBounce() bool`

HasHardBounce returns a boolean if a field has been set.

### GetOpen

`func (o *OutcomePolicyRules) GetOpen() OutcomeRuleAction`

GetOpen returns the Open field if non-nil, zero value otherwise.

### GetOpenOk

`func (o *OutcomePolicyRules) GetOpenOk() (*OutcomeRuleAction, bool)`

GetOpenOk returns a tuple with the Open field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpen

`func (o *OutcomePolicyRules) SetOpen(v OutcomeRuleAction)`

SetOpen sets Open field to given value.

### HasOpen

`func (o *OutcomePolicyRules) HasOpen() bool`

HasOpen returns a boolean if a field has been set.

### GetOutcomeTtlDays

`func (o *OutcomePolicyRules) GetOutcomeTtlDays() int32`

GetOutcomeTtlDays returns the OutcomeTtlDays field if non-nil, zero value otherwise.

### GetOutcomeTtlDaysOk

`func (o *OutcomePolicyRules) GetOutcomeTtlDaysOk() (*int32, bool)`

GetOutcomeTtlDaysOk returns a tuple with the OutcomeTtlDays field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcomeTtlDays

`func (o *OutcomePolicyRules) SetOutcomeTtlDays(v int32)`

SetOutcomeTtlDays sets OutcomeTtlDays field to given value.

### HasOutcomeTtlDays

`func (o *OutcomePolicyRules) HasOutcomeTtlDays() bool`

HasOutcomeTtlDays returns a boolean if a field has been set.

### GetSoftBounce

`func (o *OutcomePolicyRules) GetSoftBounce() OutcomeRuleAction`

GetSoftBounce returns the SoftBounce field if non-nil, zero value otherwise.

### GetSoftBounceOk

`func (o *OutcomePolicyRules) GetSoftBounceOk() (*OutcomeRuleAction, bool)`

GetSoftBounceOk returns a tuple with the SoftBounce field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSoftBounce

`func (o *OutcomePolicyRules) SetSoftBounce(v OutcomeRuleAction)`

SetSoftBounce sets SoftBounce field to given value.

### HasSoftBounce

`func (o *OutcomePolicyRules) HasSoftBounce() bool`

HasSoftBounce returns a boolean if a field has been set.

### GetUnsubscribe

`func (o *OutcomePolicyRules) GetUnsubscribe() OutcomeRuleAction`

GetUnsubscribe returns the Unsubscribe field if non-nil, zero value otherwise.

### GetUnsubscribeOk

`func (o *OutcomePolicyRules) GetUnsubscribeOk() (*OutcomeRuleAction, bool)`

GetUnsubscribeOk returns a tuple with the Unsubscribe field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnsubscribe

`func (o *OutcomePolicyRules) SetUnsubscribe(v OutcomeRuleAction)`

SetUnsubscribe sets Unsubscribe field to given value.

### HasUnsubscribe

`func (o *OutcomePolicyRules) HasUnsubscribe() bool`

HasUnsubscribe returns a boolean if a field has been set.


[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
