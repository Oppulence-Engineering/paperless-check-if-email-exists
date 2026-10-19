# OnboardRequest

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**ContactEmail** | **string** | Contact email for the tenant account (billing, alerts). | [required]
**EmailToVerify** | **string** | Email address to verify. | [required]
**PlanTier** | Pointer to **NullableString** | Optional plan tier (defaults to \&quot;free\&quot;). | [optional]
**Slug** | Pointer to **NullableString** | URL-safe slug (auto-generated from tenant_name if omitted). | [optional]
**TenantName** | **string** | Display name for the new tenant. | [required]

## Methods

### NewOnboardRequest

`func NewOnboardRequest(contactEmail string, emailToVerify string, tenantName string) *OnboardRequest`

NewOnboardRequest instantiates a new OnboardRequest object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewOnboardRequestWithDefaults

`func NewOnboardRequestWithDefaults() *OnboardRequest`

NewOnboardRequestWithDefaults instantiates a new OnboardRequest object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetContactEmail

`func (o *OnboardRequest) GetContactEmail() string`

GetContactEmail returns the ContactEmail field if non-nil, zero value otherwise.

### GetContactEmailOk

`func (o *OnboardRequest) GetContactEmailOk() (*string, bool)`

GetContactEmailOk returns a tuple with the ContactEmail field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetContactEmail

`func (o *OnboardRequest) SetContactEmail(v string)`

SetContactEmail sets ContactEmail field to given value.


### GetEmailToVerify

`func (o *OnboardRequest) GetEmailToVerify() string`

GetEmailToVerify returns the EmailToVerify field if non-nil, zero value otherwise.

### GetEmailToVerifyOk

`func (o *OnboardRequest) GetEmailToVerifyOk() (*string, bool)`

GetEmailToVerifyOk returns a tuple with the EmailToVerify field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetEmailToVerify

`func (o *OnboardRequest) SetEmailToVerify(v string)`

SetEmailToVerify sets EmailToVerify field to given value.


### GetPlanTier

`func (o *OnboardRequest) GetPlanTier() string`

GetPlanTier returns the PlanTier field if non-nil, zero value otherwise.

### GetPlanTierOk

`func (o *OnboardRequest) GetPlanTierOk() (*string, bool)`

GetPlanTierOk returns a tuple with the PlanTier field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetPlanTier

`func (o *OnboardRequest) SetPlanTier(v string)`

SetPlanTier sets PlanTier field to given value.

### HasPlanTier

`func (o *OnboardRequest) HasPlanTier() bool`

HasPlanTier returns a boolean if a field has been set.

### SetPlanTierNil

`func (o *OnboardRequest) SetPlanTierNil()`

 SetPlanTierNil sets the value for PlanTier to be an explicit nil

### UnsetPlanTier
`func (o *OnboardRequest) UnsetPlanTier()`

UnsetPlanTier ensures that no value is present for PlanTier, not even an explicit nil

### GetSlug

`func (o *OnboardRequest) GetSlug() string`

GetSlug returns the Slug field if non-nil, zero value otherwise.

### GetSlugOk

`func (o *OnboardRequest) GetSlugOk() (*string, bool)`

GetSlugOk returns a tuple with the Slug field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetSlug

`func (o *OnboardRequest) SetSlug(v string)`

SetSlug sets Slug field to given value.

### HasSlug

`func (o *OnboardRequest) HasSlug() bool`

HasSlug returns a boolean if a field has been set.

### SetSlugNil

`func (o *OnboardRequest) SetSlugNil()`

 SetSlugNil sets the value for Slug to be an explicit nil

### UnsetSlug
`func (o *OnboardRequest) UnsetSlug()`

UnsetSlug ensures that no value is present for Slug, not even an explicit nil

### GetTenantName

`func (o *OnboardRequest) GetTenantName() string`

GetTenantName returns the TenantName field if non-nil, zero value otherwise.

### GetTenantNameOk

`func (o *OnboardRequest) GetTenantNameOk() (*string, bool)`

GetTenantNameOk returns a tuple with the TenantName field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetTenantName

`func (o *OnboardRequest) SetTenantName(v string)`

SetTenantName sets TenantName field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
