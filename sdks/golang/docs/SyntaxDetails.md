# SyntaxDetails

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**Domain** | **string** | The domain part of the email address. | [required]
**IsValidSyntax** | **bool** | Indicates if the email address syntax is valid. | [required]
**Username** | **string** | The username part of the email address. | [required]

## Methods

### NewSyntaxDetails

`func NewSyntaxDetails(domain string, isValidSyntax bool, username string) *SyntaxDetails`

NewSyntaxDetails instantiates a new SyntaxDetails object
This constructor will assign default values to properties that have it defined,
and makes sure properties required by API are set, but the set of arguments
will change when the set of required properties is changed

### NewSyntaxDetailsWithDefaults

`func NewSyntaxDetailsWithDefaults() *SyntaxDetails`

NewSyntaxDetailsWithDefaults instantiates a new SyntaxDetails object
This constructor will only assign default values to properties that have it defined,
but it doesn't guarantee that properties required by API are set

### GetDomain

`func (o *SyntaxDetails) GetDomain() string`

GetDomain returns the Domain field if non-nil, zero value otherwise.

### GetDomainOk

`func (o *SyntaxDetails) GetDomainOk() (*string, bool)`

GetDomainOk returns a tuple with the Domain field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetDomain

`func (o *SyntaxDetails) SetDomain(v string)`

SetDomain sets Domain field to given value.


### GetIsValidSyntax

`func (o *SyntaxDetails) GetIsValidSyntax() bool`

GetIsValidSyntax returns the IsValidSyntax field if non-nil, zero value otherwise.

### GetIsValidSyntaxOk

`func (o *SyntaxDetails) GetIsValidSyntaxOk() (*bool, bool)`

GetIsValidSyntaxOk returns a tuple with the IsValidSyntax field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetIsValidSyntax

`func (o *SyntaxDetails) SetIsValidSyntax(v bool)`

SetIsValidSyntax sets IsValidSyntax field to given value.


### GetUsername

`func (o *SyntaxDetails) GetUsername() string`

GetUsername returns the Username field if non-nil, zero value otherwise.

### GetUsernameOk

`func (o *SyntaxDetails) GetUsernameOk() (*string, bool)`

GetUsernameOk returns a tuple with the Username field if it's non-nil, zero value otherwise
and a boolean to check if the value has been set.

### SetUsername

`func (o *SyntaxDetails) SetUsername(v string)`

SetUsername sets Username field to given value.



[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)
