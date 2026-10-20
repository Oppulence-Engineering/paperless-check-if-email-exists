# SingleApiResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**email** | Option<**String**> | The verified email address. | [optional]
**quality** | Option<**String**> | The quality of the email address. | [optional]
**result** | Option<**String**> | The result of email verification. | [optional]
**resultcode** | Option<**i32**> | The result of email verification as an integer. 1 (ok), 2 (catch_all), 3 (unknown), 4 (error), 5 (disposable), 6 (invalid) | [optional]
**subresult** | Option<**String**> | Subresult of email verification. | [optional]
**free** | Option<**bool**> | True if the email address is provided by a free email service such as Gmail or Yahoo. | [optional]
**role** | Option<**bool**> | True if the email address if linked to a role as opposed to a person. For example: admin@, sales@ | [optional]
**didyoumean** | Option<**String**> | Autocorrection suggestion for common typos. | [optional]
**credits** | Option<**i32**> | Remaining credits in your account. | [optional]
**executiontime** | Option<**i32**> | The time in milliseconds for the email verification. | [optional]
**error** | Option<**String**> | Error message displayed here if result is \"error\". | [optional]
**livemode** | Option<**bool**> | Whether the request was made with a real or demo API key | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


