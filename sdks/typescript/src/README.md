## @oppulence/reacher-sdk@0.11.0

This generator creates TypeScript/JavaScript client that utilizes [axios](https://github.com/axios/axios). The generated Node module can be used in the following environments:

Environment
* Node.js
* Webpack
* Browserify

Language level
* ES5 - you must have a Promises/A+ library installed
* ES6

Module system
* CommonJS
* ES6 module system

It can be used in both TypeScript and JavaScript. In TypeScript, the definition will be automatically resolved via `package.json`. ([Reference](https://www.typescriptlang.org/docs/handbook/declaration-files/consumption.html))

### Building

To build and compile the typescript sources to javascript use:
```
npm install
npm run build
```

### Publishing

First build the package then run `npm publish`

### Consuming

navigate to the folder of your consuming project and run one of the following commands.

_published:_

```
npm install @oppulence/reacher-sdk@0.11.0 --save
```

_unPublished (not recommended):_

```
npm install PATH_TO_GENERATED_PACKAGE --save
```

### Documentation for API Endpoints

All URIs are relative to *https://api.reacher.email*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AccountApi* | [**createTenantApiKey**](docs/AccountApi.md#createtenantapikey) | **POST** /v1/me/api-keys | POST /v1/me/api-keys
*AccountApi* | [**getTenantApiKey**](docs/AccountApi.md#gettenantapikey) | **GET** /v1/me/api-keys/{key_id} | GET /v1/me/api-keys/{key_id}
*AccountApi* | [**listTenantApiKeys**](docs/AccountApi.md#listtenantapikeys) | **GET** /v1/me/api-keys | GET /v1/me/api-keys
*AccountApi* | [**revokeTenantApiKey**](docs/AccountApi.md#revoketenantapikey) | **DELETE** /v1/me/api-keys/{key_id} | DELETE /v1/me/api-keys/{key_id}
*AccountApi* | [**updateTenantApiKey**](docs/AccountApi.md#updatetenantapikey) | **PATCH** /v1/me/api-keys/{key_id} | PATCH /v1/me/api-keys/{key_id}
*AccountApi* | [**v1Me**](docs/AccountApi.md#v1me) | **GET** /v1/me | GET /v1/me
*AdminApi* | [**createApiKey**](docs/AdminApi.md#createapikey) | **POST** /v1/admin/tenants/{tenant_id}/api-keys | POST /v1/admin/tenants/{tenant_id}/api-keys
*AdminApi* | [**createTenant**](docs/AdminApi.md#createtenant) | **POST** /v1/admin/tenants | POST /v1/admin/tenants
*AdminApi* | [**deleteTenant**](docs/AdminApi.md#deletetenant) | **DELETE** /v1/admin/tenants/{tenant_id} | DELETE /v1/admin/tenants/{tenant_id}
*AdminApi* | [**getApiKey**](docs/AdminApi.md#getapikey) | **GET** /v1/admin/tenants/{tenant_id}/api-keys/{key_id} | GET /v1/admin/tenants/{tenant_id}/api-keys/{key_id}
*AdminApi* | [**getTenant**](docs/AdminApi.md#gettenant) | **GET** /v1/admin/tenants/{tenant_id} | GET /v1/admin/tenants/{tenant_id}
*AdminApi* | [**getTenantQuota**](docs/AdminApi.md#gettenantquota) | **GET** /v1/admin/tenants/{tenant_id}/quota | GET /v1/admin/tenants/{tenant_id}/quota
*AdminApi* | [**listAllApiKeys**](docs/AdminApi.md#listallapikeys) | **GET** /v1/admin/api-keys | GET /v1/admin/api-keys
*AdminApi* | [**listApiKeys**](docs/AdminApi.md#listapikeys) | **GET** /v1/admin/tenants/{tenant_id}/api-keys | GET /v1/admin/tenants/{tenant_id}/api-keys
*AdminApi* | [**listTenants**](docs/AdminApi.md#listtenants) | **GET** /v1/admin/tenants | GET /v1/admin/tenants
*AdminApi* | [**reactivateApiKey**](docs/AdminApi.md#reactivateapikey) | **POST** /v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate | POST /v1/admin/tenants/{tenant_id}/api-keys/{key_id}/reactivate
*AdminApi* | [**resetTenantQuota**](docs/AdminApi.md#resettenantquota) | **POST** /v1/admin/tenants/{tenant_id}/quota/reset | POST /v1/admin/tenants/{tenant_id}/quota/reset
*AdminApi* | [**revokeApiKey**](docs/AdminApi.md#revokeapikey) | **DELETE** /v1/admin/tenants/{tenant_id}/api-keys/{key_id} | DELETE /v1/admin/tenants/{tenant_id}/api-keys/{key_id}
*AdminApi* | [**updateApiKey**](docs/AdminApi.md#updateapikey) | **PATCH** /v1/admin/tenants/{tenant_id}/api-keys/{key_id} | PATCH /v1/admin/tenants/{tenant_id}/api-keys/{key_id}
*AdminApi* | [**updateTenant**](docs/AdminApi.md#updatetenant) | **PUT** /v1/admin/tenants/{tenant_id} | PUT /v1/admin/tenants/{tenant_id}
*AdminApi* | [**updateTenantQuota**](docs/AdminApi.md#updatetenantquota) | **PATCH** /v1/admin/tenants/{tenant_id}/quota | PATCH /v1/admin/tenants/{tenant_id}/quota
*AdminJobsApi* | [**getJob**](docs/AdminJobsApi.md#getjob) | **GET** /v1/admin/jobs/{job_id} | GET /v1/admin/jobs/{job_id}
*AdminJobsApi* | [**getJobEvents**](docs/AdminJobsApi.md#getjobevents) | **GET** /v1/admin/jobs/{job_id}/events | GET /v1/admin/jobs/{job_id}/events
*AdminJobsApi* | [**getJobResults**](docs/AdminJobsApi.md#getjobresults) | **GET** /v1/admin/jobs/{job_id}/results | GET /v1/admin/jobs/{job_id}/results
*AdminJobsApi* | [**listJobs**](docs/AdminJobsApi.md#listjobs) | **GET** /v1/admin/jobs | GET /v1/admin/jobs
*AdminJobsApi* | [**listTenantJobs**](docs/AdminJobsApi.md#listtenantjobs) | **GET** /v1/admin/tenants/{tenant_id}/jobs | GET /v1/admin/tenants/{tenant_id}/jobs
*HealthApi* | [**healthz**](docs/HealthApi.md#healthz) | **GET** /healthz | GET /healthz
*HealthApi* | [**readyz**](docs/HealthApi.md#readyz) | **GET** /readyz | GET /readyz
*JobsApi* | [**v1CancelJob**](docs/JobsApi.md#v1canceljob) | **POST** /v1/jobs/{job_id}/cancel | POST /v1/jobs/{job_id}/cancel
*JobsApi* | [**v1DownloadJobResults**](docs/JobsApi.md#v1downloadjobresults) | **GET** /v1/jobs/{job_id}/download | GET /v1/jobs/{job_id}/download
*JobsApi* | [**v1GetBulkJobProgress**](docs/JobsApi.md#v1getbulkjobprogress) | **GET** /v1/bulk/{job_id} | GET /v1/bulk/{job_id}
*JobsApi* | [**v1GetBulkJobResults**](docs/JobsApi.md#v1getbulkjobresults) | **GET** /v1/bulk/{job_id}/results | GET /v1/bulk/{job_id}/results
*JobsApi* | [**v1GetJobEvents**](docs/JobsApi.md#v1getjobevents) | **GET** /v1/jobs/{job_id}/events | GET /v1/jobs/{job_id}/events
*JobsApi* | [**v1GetJobResults**](docs/JobsApi.md#v1getjobresults) | **GET** /v1/jobs/{job_id}/results | GET /v1/jobs/{job_id}/results
*JobsApi* | [**v1GetJobStatus**](docs/JobsApi.md#v1getjobstatus) | **GET** /v1/jobs/{job_id} | GET /v1/jobs/{job_id}
*SystemApi* | [**getVersion**](docs/SystemApi.md#getversion) | **GET** /version | GET /version
*SystemApi* | [**openapiSpec**](docs/SystemApi.md#openapispec) | **GET** /openapi.json | Serve the merged OpenAPI document for all documented REST endpoints.
*TenantApi* | [**v1ClearTenantWebhook**](docs/TenantApi.md#v1cleartenantwebhook) | **DELETE** /v1/me/webhook | DELETE /v1/me/webhook
*TenantApi* | [**v1CreateTenantDomain**](docs/TenantApi.md#v1createtenantdomain) | **POST** /v1/me/domains | POST /v1/me/domains
*TenantApi* | [**v1DeleteTenantDomain**](docs/TenantApi.md#v1deletetenantdomain) | **DELETE** /v1/me/domains/{domain} | DELETE /v1/me/domains/{domain}
*TenantApi* | [**v1GetTenantDomain**](docs/TenantApi.md#v1gettenantdomain) | **GET** /v1/me/domains/{domain} | GET /v1/me/domains/{domain}
*TenantApi* | [**v1GetTenantSettings**](docs/TenantApi.md#v1gettenantsettings) | **GET** /v1/me/settings | GET /v1/me/settings
*TenantApi* | [**v1GetTenantUsage**](docs/TenantApi.md#v1gettenantusage) | **GET** /v1/me/usage | GET /v1/me/usage
*TenantApi* | [**v1GetTenantWebhook**](docs/TenantApi.md#v1gettenantwebhook) | **GET** /v1/me/webhook | GET /v1/me/webhook
*TenantApi* | [**v1ListTenantDomains**](docs/TenantApi.md#v1listtenantdomains) | **GET** /v1/me/domains | GET /v1/me/domains
*TenantApi* | [**v1UpdateTenantDomain**](docs/TenantApi.md#v1updatetenantdomain) | **PATCH** /v1/me/domains/{domain} | PATCH /v1/me/domains/{domain}
*TenantApi* | [**v1UpdateTenantSettings**](docs/TenantApi.md#v1updatetenantsettings) | **PATCH** /v1/me/settings | PATCH /v1/me/settings
*TenantApi* | [**v1UpdateTenantWebhook**](docs/TenantApi.md#v1updatetenantwebhook) | **PATCH** /v1/me/webhook | PATCH /v1/me/webhook
*V0Api* | [**createBulkJob**](docs/V0Api.md#createbulkjob) | **POST** /v0/bulk | POST /v0/bulk
*V0Api* | [**getBulkJobResult**](docs/V0Api.md#getbulkjobresult) | **GET** /v0/bulk/{job_id}/results | GET /v0/bulk/{job_id}/results
*V0Api* | [**getBulkJobStatus**](docs/V0Api.md#getbulkjobstatus) | **GET** /v0/bulk/{job_id} | GET /v0/bulk/{job_id}
*V0Api* | [**postCheckEmail**](docs/V0Api.md#postcheckemail) | **POST** /v0/check_email | POST /v0/check_email
*V1Api* | [**v1AddSuppressions**](docs/V1Api.md#v1addsuppressions) | **POST** /v1/suppressions | POST /v1/suppressions
*V1Api* | [**v1CheckEmail**](docs/V1Api.md#v1checkemail) | **POST** /v1/check_email | POST /v1/check_email
*V1Api* | [**v1CheckReputation**](docs/V1Api.md#v1checkreputation) | **POST** /v1/reputation/check | POST /v1/reputation/check
*V1Api* | [**v1CheckSuppression**](docs/V1Api.md#v1checksuppression) | **GET** /v1/suppressions/check | GET /v1/suppressions/check
*V1Api* | [**v1CreateBulkJob**](docs/V1Api.md#v1createbulkjob) | **POST** /v1/bulk | Create the v1 bulk endpoint.
*V1Api* | [**v1CreateList**](docs/V1Api.md#v1createlist) | **POST** /v1/lists | POST /v1/lists
*V1Api* | [**v1DeleteList**](docs/V1Api.md#v1deletelist) | **DELETE** /v1/lists/{list_id} | DELETE /v1/lists/{list_id}
*V1Api* | [**v1DeleteSuppression**](docs/V1Api.md#v1deletesuppression) | **DELETE** /v1/suppressions/{id} | DELETE /v1/suppressions/{id}
*V1Api* | [**v1DownloadList**](docs/V1Api.md#v1downloadlist) | **GET** /v1/lists/{list_id}/download | GET /v1/lists/{list_id}/download
*V1Api* | [**v1FindEmail**](docs/V1Api.md#v1findemail) | **POST** /v1/find_email | POST /v1/find_email
*V1Api* | [**v1GetFindEmail**](docs/V1Api.md#v1getfindemail) | **GET** /v1/find_email/{job_id} | GET /v1/find_email/{job_id}
*V1Api* | [**v1GetList**](docs/V1Api.md#v1getlist) | **GET** /v1/lists/{list_id} | GET /v1/lists/{list_id}
*V1Api* | [**v1ListLists**](docs/V1Api.md#v1listlists) | **GET** /v1/lists | GET /v1/lists
*V1Api* | [**v1ListSuppressions**](docs/V1Api.md#v1listsuppressions) | **GET** /v1/suppressions | GET /v1/suppressions


### Documentation For Models

 - [AddSuppressionsRequest](docs/AddSuppressionsRequest.md)
 - [AddSuppressionsResponse](docs/AddSuppressionsResponse.md)
 - [BlacklistResult](docs/BlacklistResult.md)
 - [BulkJobResultsResponse](docs/BulkJobResultsResponse.md)
 - [CheckEmailInputProxy](docs/CheckEmailInputProxy.md)
 - [CheckEmailOutput](docs/CheckEmailOutput.md)
 - [CheckEmailOutputMisc](docs/CheckEmailOutputMisc.md)
 - [CheckEmailOutputMx](docs/CheckEmailOutputMx.md)
 - [CheckEmailOutputSmtp](docs/CheckEmailOutputSmtp.md)
 - [CheckEmailRequest](docs/CheckEmailRequest.md)
 - [CoreError](docs/CoreError.md)
 - [DebugDetails](docs/DebugDetails.md)
 - [DebugDetailsSmtp](docs/DebugDetailsSmtp.md)
 - [DnsRecordResults](docs/DnsRecordResults.md)
 - [DomainInfo](docs/DomainInfo.md)
 - [Duration](docs/Duration.md)
 - [EmailCategory](docs/EmailCategory.md)
 - [EmailScore](docs/EmailScore.md)
 - [FindEmailAcceptedResponse](docs/FindEmailAcceptedResponse.md)
 - [FindEmailRequest](docs/FindEmailRequest.md)
 - [FindEmailStatusResponse](docs/FindEmailStatusResponse.md)
 - [FinderBestMatch](docs/FinderBestMatch.md)
 - [FinderCandidateResult](docs/FinderCandidateResult.md)
 - [GmailVerifMethod](docs/GmailVerifMethod.md)
 - [HotmailB2BVerifMethod](docs/HotmailB2BVerifMethod.md)
 - [HotmailB2CVerifMethod](docs/HotmailB2CVerifMethod.md)
 - [JobResultPageResponse](docs/JobResultPageResponse.md)
 - [JobTaskResult](docs/JobTaskResult.md)
 - [ListDeleteResponse](docs/ListDeleteResponse.md)
 - [ListDetailResponse](docs/ListDetailResponse.md)
 - [ListItem](docs/ListItem.md)
 - [ListListResponse](docs/ListListResponse.md)
 - [ListSummary](docs/ListSummary.md)
 - [ListUploadResponse](docs/ListUploadResponse.md)
 - [MiscDetails](docs/MiscDetails.md)
 - [MxDetails](docs/MxDetails.md)
 - [Reachable](docs/Reachable.md)
 - [ReasonCode](docs/ReasonCode.md)
 - [ReputationCheckRequest](docs/ReputationCheckRequest.md)
 - [ReputationCheckResponse](docs/ReputationCheckResponse.md)
 - [ScoringSignals](docs/ScoringSignals.md)
 - [SmtpDetails](docs/SmtpDetails.md)
 - [SubReason](docs/SubReason.md)
 - [SuppressionCheckResponse](docs/SuppressionCheckResponse.md)
 - [SuppressionDeleteResponse](docs/SuppressionDeleteResponse.md)
 - [SuppressionEntry](docs/SuppressionEntry.md)
 - [SuppressionListResponse](docs/SuppressionListResponse.md)
 - [SuppressionReason](docs/SuppressionReason.md)
 - [SyntaxDetails](docs/SyntaxDetails.md)
 - [TaskWebhook](docs/TaskWebhook.md)
 - [VerifMethod](docs/VerifMethod.md)
 - [Webhook](docs/Webhook.md)
 - [YahooVerifMethod](docs/YahooVerifMethod.md)


<a id="documentation-for-authorization"></a>
## Documentation For Authorization


Authentication schemes defined for the API:
<a id="Authorization"></a>
### Authorization

- **Type**: API key
- **API key parameter name**: Authorization
- **Location**: HTTP header

