# SourceQualityRow

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**BouncedCount** | **int64** |  | [required]
**ClickedCount** | **int64** |  | [required]
**ComplainedCount** | **int64** |  | [required]
**DeliveredCount** | **int64** |  | [required]
**DropCount** | **int64** |  | [required]
**InvalidCount** | **int64** |  | [required]
**InvalidPct** | **float64** |  | [required]
**NegativeOutcomePct** | **float64** |  | [required]
**OpenedCount** | **int64** |  | [required]
**OutcomeCount** | **int64** |  | [required]
**ProcessedRecords** | **int64** |  | [required]
**QualityGrade** | **string** |  | [required]
**ReviewCount** | **int64** |  | [required]
**RiskyCount** | **int64** |  | [required]
**RiskyPct** | **float64** |  | [required]
**SafeToSendCount** | **int64** |  | [required]
**SendCount** | **int64** |  | [required]
**SendWithCautionCount** | **int64** |  | [required]
**SourceKey** | **string** |  | [required]
**Summary** | **string** |  | [required]
**SuppressCount** | **int64** |  | [required]
**TotalRecords** | **int64** |  | [required]
**UnknownCount** | **int64** |  | [required]
**UnsafeRecommendationPct** | **float64** |  | [required]
**UnsubscribedCount** | **int64** |  | [required]
**ValidCount** | **int64** |  | [required]

## Methods

### NewSourceQualityRow

`func NewSourceQualityRow(bouncedCount int64, clickedCount int64, complainedCount int64, deliveredCount int64, dropCount int64, invalidCount int64, invalidPct float64, negativeOutcomePct float64, openedCount int64, outcomeCount int64, processedRecords int64, qualityGrade string, reviewCount int64, riskyCount int64, riskyPct float64, safeToSendCount int64, sendCount int64, sendWithCautionCount int64, sourceKey string, summary string, suppressCount int64, totalRecords int64, unknownCount int64, unsafeRecommendationPct float64, unsubscribedCount int64, validCount int64) *SourceQualityRow`

NewSourceQualityRow instantiates a new SourceQualityRow object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSourceQualityRowWithDefaults

`func NewSourceQualityRowWithDefaults() *SourceQualityRow`

NewSourceQualityRowWithDefaults instantiates a new SourceQualityRow object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetBouncedCount

`func (o *SourceQualityRow) GetBouncedCount() int64`

GetBouncedCount returns the BouncedCount field if non-nil, zero value otherwise.

### GetBouncedCountOk

`func (o *SourceQualityRow) GetBouncedCountOk() (*int64, bool)`

GetBouncedCountOk returns a tuple with the BouncedCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetBouncedCount

`func (o *SourceQualityRow) SetBouncedCount(v int64)`

SetBouncedCount sets BouncedCount field to given value.


### GetClickedCount

`func (o *SourceQualityRow) GetClickedCount() int64`

GetClickedCount returns the ClickedCount field if non-nil, zero value otherwise.

### GetClickedCountOk

`func (o *SourceQualityRow) GetClickedCountOk() (*int64, bool)`

GetClickedCountOk returns a tuple with the ClickedCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetClickedCount

`func (o *SourceQualityRow) SetClickedCount(v int64)`

SetClickedCount sets ClickedCount field to given value.


### GetComplainedCount

`func (o *SourceQualityRow) GetComplainedCount() int64`

GetComplainedCount returns the ComplainedCount field if non-nil, zero value otherwise.

### GetComplainedCountOk

`func (o *SourceQualityRow) GetComplainedCountOk() (*int64, bool)`

GetComplainedCountOk returns a tuple with the ComplainedCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetComplainedCount

`func (o *SourceQualityRow) SetComplainedCount(v int64)`

SetComplainedCount sets ComplainedCount field to given value.


### GetDeliveredCount

`func (o *SourceQualityRow) GetDeliveredCount() int64`

GetDeliveredCount returns the DeliveredCount field if non-nil, zero value otherwise.

### GetDeliveredCountOk

`func (o *SourceQualityRow) GetDeliveredCountOk() (*int64, bool)`

GetDeliveredCountOk returns a tuple with the DeliveredCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDeliveredCount

`func (o *SourceQualityRow) SetDeliveredCount(v int64)`

SetDeliveredCount sets DeliveredCount field to given value.


### GetDropCount

`func (o *SourceQualityRow) GetDropCount() int64`

GetDropCount returns the DropCount field if non-nil, zero value otherwise.

### GetDropCountOk

`func (o *SourceQualityRow) GetDropCountOk() (*int64, bool)`

GetDropCountOk returns a tuple with the DropCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDropCount

`func (o *SourceQualityRow) SetDropCount(v int64)`

SetDropCount sets DropCount field to given value.


### GetInvalidCount

`func (o *SourceQualityRow) GetInvalidCount() int64`

GetInvalidCount returns the InvalidCount field if non-nil, zero value otherwise.

### GetInvalidCountOk

`func (o *SourceQualityRow) GetInvalidCountOk() (*int64, bool)`

GetInvalidCountOk returns a tuple with the InvalidCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvalidCount

`func (o *SourceQualityRow) SetInvalidCount(v int64)`

SetInvalidCount sets InvalidCount field to given value.


### GetInvalidPct

`func (o *SourceQualityRow) GetInvalidPct() float64`

GetInvalidPct returns the InvalidPct field if non-nil, zero value otherwise.

### GetInvalidPctOk

`func (o *SourceQualityRow) GetInvalidPctOk() (*float64, bool)`

GetInvalidPctOk returns a tuple with the InvalidPct field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetInvalidPct

`func (o *SourceQualityRow) SetInvalidPct(v float64)`

SetInvalidPct sets InvalidPct field to given value.


### GetNegativeOutcomePct

`func (o *SourceQualityRow) GetNegativeOutcomePct() float64`

GetNegativeOutcomePct returns the NegativeOutcomePct field if non-nil, zero value otherwise.

### GetNegativeOutcomePctOk

`func (o *SourceQualityRow) GetNegativeOutcomePctOk() (*float64, bool)`

GetNegativeOutcomePctOk returns a tuple with the NegativeOutcomePct field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetNegativeOutcomePct

`func (o *SourceQualityRow) SetNegativeOutcomePct(v float64)`

SetNegativeOutcomePct sets NegativeOutcomePct field to given value.


### GetOpenedCount

`func (o *SourceQualityRow) GetOpenedCount() int64`

GetOpenedCount returns the OpenedCount field if non-nil, zero value otherwise.

### GetOpenedCountOk

`func (o *SourceQualityRow) GetOpenedCountOk() (*int64, bool)`

GetOpenedCountOk returns a tuple with the OpenedCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOpenedCount

`func (o *SourceQualityRow) SetOpenedCount(v int64)`

SetOpenedCount sets OpenedCount field to given value.


### GetOutcomeCount

`func (o *SourceQualityRow) GetOutcomeCount() int64`

GetOutcomeCount returns the OutcomeCount field if non-nil, zero value otherwise.

### GetOutcomeCountOk

`func (o *SourceQualityRow) GetOutcomeCountOk() (*int64, bool)`

GetOutcomeCountOk returns a tuple with the OutcomeCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetOutcomeCount

`func (o *SourceQualityRow) SetOutcomeCount(v int64)`

SetOutcomeCount sets OutcomeCount field to given value.


### GetProcessedRecords

`func (o *SourceQualityRow) GetProcessedRecords() int64`

GetProcessedRecords returns the ProcessedRecords field if non-nil, zero value otherwise.

### GetProcessedRecordsOk

`func (o *SourceQualityRow) GetProcessedRecordsOk() (*int64, bool)`

GetProcessedRecordsOk returns a tuple with the ProcessedRecords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetProcessedRecords

`func (o *SourceQualityRow) SetProcessedRecords(v int64)`

SetProcessedRecords sets ProcessedRecords field to given value.


### GetQualityGrade

`func (o *SourceQualityRow) GetQualityGrade() string`

GetQualityGrade returns the QualityGrade field if non-nil, zero value otherwise.

### GetQualityGradeOk

`func (o *SourceQualityRow) GetQualityGradeOk() (*string, bool)`

GetQualityGradeOk returns a tuple with the QualityGrade field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetQualityGrade

`func (o *SourceQualityRow) SetQualityGrade(v string)`

SetQualityGrade sets QualityGrade field to given value.


### GetReviewCount

`func (o *SourceQualityRow) GetReviewCount() int64`

GetReviewCount returns the ReviewCount field if non-nil, zero value otherwise.

### GetReviewCountOk

`func (o *SourceQualityRow) GetReviewCountOk() (*int64, bool)`

GetReviewCountOk returns a tuple with the ReviewCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetReviewCount

`func (o *SourceQualityRow) SetReviewCount(v int64)`

SetReviewCount sets ReviewCount field to given value.


### GetRiskyCount

`func (o *SourceQualityRow) GetRiskyCount() int64`

GetRiskyCount returns the RiskyCount field if non-nil, zero value otherwise.

### GetRiskyCountOk

`func (o *SourceQualityRow) GetRiskyCountOk() (*int64, bool)`

GetRiskyCountOk returns a tuple with the RiskyCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRiskyCount

`func (o *SourceQualityRow) SetRiskyCount(v int64)`

SetRiskyCount sets RiskyCount field to given value.


### GetRiskyPct

`func (o *SourceQualityRow) GetRiskyPct() float64`

GetRiskyPct returns the RiskyPct field if non-nil, zero value otherwise.

### GetRiskyPctOk

`func (o *SourceQualityRow) GetRiskyPctOk() (*float64, bool)`

GetRiskyPctOk returns a tuple with the RiskyPct field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetRiskyPct

`func (o *SourceQualityRow) SetRiskyPct(v float64)`

SetRiskyPct sets RiskyPct field to given value.


### GetSafeToSendCount

`func (o *SourceQualityRow) GetSafeToSendCount() int64`

GetSafeToSendCount returns the SafeToSendCount field if non-nil, zero value otherwise.

### GetSafeToSendCountOk

`func (o *SourceQualityRow) GetSafeToSendCountOk() (*int64, bool)`

GetSafeToSendCountOk returns a tuple with the SafeToSendCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSafeToSendCount

`func (o *SourceQualityRow) SetSafeToSendCount(v int64)`

SetSafeToSendCount sets SafeToSendCount field to given value.


### GetSendCount

`func (o *SourceQualityRow) GetSendCount() int64`

GetSendCount returns the SendCount field if non-nil, zero value otherwise.

### GetSendCountOk

`func (o *SourceQualityRow) GetSendCountOk() (*int64, bool)`

GetSendCountOk returns a tuple with the SendCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSendCount

`func (o *SourceQualityRow) SetSendCount(v int64)`

SetSendCount sets SendCount field to given value.


### GetSendWithCautionCount

`func (o *SourceQualityRow) GetSendWithCautionCount() int64`

GetSendWithCautionCount returns the SendWithCautionCount field if non-nil, zero value otherwise.

### GetSendWithCautionCountOk

`func (o *SourceQualityRow) GetSendWithCautionCountOk() (*int64, bool)`

GetSendWithCautionCountOk returns a tuple with the SendWithCautionCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSendWithCautionCount

`func (o *SourceQualityRow) SetSendWithCautionCount(v int64)`

SetSendWithCautionCount sets SendWithCautionCount field to given value.


### GetSourceKey

`func (o *SourceQualityRow) GetSourceKey() string`

GetSourceKey returns the SourceKey field if non-nil, zero value otherwise.

### GetSourceKeyOk

`func (o *SourceQualityRow) GetSourceKeyOk() (*string, bool)`

GetSourceKeyOk returns a tuple with the SourceKey field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSourceKey

`func (o *SourceQualityRow) SetSourceKey(v string)`

SetSourceKey sets SourceKey field to given value.


### GetSummary

`func (o *SourceQualityRow) GetSummary() string`

GetSummary returns the Summary field if non-nil, zero value otherwise.

### GetSummaryOk

`func (o *SourceQualityRow) GetSummaryOk() (*string, bool)`

GetSummaryOk returns a tuple with the Summary field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSummary

`func (o *SourceQualityRow) SetSummary(v string)`

SetSummary sets Summary field to given value.


### GetSuppressCount

`func (o *SourceQualityRow) GetSuppressCount() int64`

GetSuppressCount returns the SuppressCount field if non-nil, zero value otherwise.

### GetSuppressCountOk

`func (o *SourceQualityRow) GetSuppressCountOk() (*int64, bool)`

GetSuppressCountOk returns a tuple with the SuppressCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSuppressCount

`func (o *SourceQualityRow) SetSuppressCount(v int64)`

SetSuppressCount sets SuppressCount field to given value.


### GetTotalRecords

`func (o *SourceQualityRow) GetTotalRecords() int64`

GetTotalRecords returns the TotalRecords field if non-nil, zero value otherwise.

### GetTotalRecordsOk

`func (o *SourceQualityRow) GetTotalRecordsOk() (*int64, bool)`

GetTotalRecordsOk returns a tuple with the TotalRecords field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTotalRecords

`func (o *SourceQualityRow) SetTotalRecords(v int64)`

SetTotalRecords sets TotalRecords field to given value.


### GetUnknownCount

`func (o *SourceQualityRow) GetUnknownCount() int64`

GetUnknownCount returns the UnknownCount field if non-nil, zero value otherwise.

### GetUnknownCountOk

`func (o *SourceQualityRow) GetUnknownCountOk() (*int64, bool)`

GetUnknownCountOk returns a tuple with the UnknownCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnknownCount

`func (o *SourceQualityRow) SetUnknownCount(v int64)`

SetUnknownCount sets UnknownCount field to given value.


### GetUnsafeRecommendationPct

`func (o *SourceQualityRow) GetUnsafeRecommendationPct() float64`

GetUnsafeRecommendationPct returns the UnsafeRecommendationPct field if non-nil, zero value otherwise.

### GetUnsafeRecommendationPctOk

`func (o *SourceQualityRow) GetUnsafeRecommendationPctOk() (*float64, bool)`

GetUnsafeRecommendationPctOk returns a tuple with the UnsafeRecommendationPct field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnsafeRecommendationPct

`func (o *SourceQualityRow) SetUnsafeRecommendationPct(v float64)`

SetUnsafeRecommendationPct sets UnsafeRecommendationPct field to given value.


### GetUnsubscribedCount

`func (o *SourceQualityRow) GetUnsubscribedCount() int64`

GetUnsubscribedCount returns the UnsubscribedCount field if non-nil, zero value otherwise.

### GetUnsubscribedCountOk

`func (o *SourceQualityRow) GetUnsubscribedCountOk() (*int64, bool)`

GetUnsubscribedCountOk returns a tuple with the UnsubscribedCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUnsubscribedCount

`func (o *SourceQualityRow) SetUnsubscribedCount(v int64)`

SetUnsubscribedCount sets UnsubscribedCount field to given value.


### GetValidCount

`func (o *SourceQualityRow) GetValidCount() int64`

GetValidCount returns the ValidCount field if non-nil, zero value otherwise.

### GetValidCountOk

`func (o *SourceQualityRow) GetValidCountOk() (*int64, bool)`

GetValidCountOk returns a tuple with the ValidCount field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetValidCount

`func (o *SourceQualityRow) SetValidCount(v int64)`

SetValidCount sets ValidCount field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
