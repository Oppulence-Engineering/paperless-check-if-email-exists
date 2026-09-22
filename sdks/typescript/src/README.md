## @oppulence/reacher-sdk@4.3.0

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
npm install @oppulence/reacher-sdk@4.3.0 --save
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
*CommentsApi* | [**v1CreateComment**](docs/CommentsApi.md#v1createcomment) | **POST** /v1/comments | POST /v1/comments
*CommentsApi* | [**v1DeleteComment**](docs/CommentsApi.md#v1deletecomment) | **DELETE** /v1/comments/{comment_id} | DELETE /v1/comments/{comment_id}
*CommentsApi* | [**v1ListComments**](docs/CommentsApi.md#v1listcomments) | **GET** /v1/comments | GET /v1/comments
*EventsApi* | [**v1ListEvents**](docs/EventsApi.md#v1listevents) | **GET** /v1/events | GET /v1/events
*HealthApi* | [**healthz**](docs/HealthApi.md#healthz) | **GET** /healthz | GET /healthz
*HealthApi* | [**readyz**](docs/HealthApi.md#readyz) | **GET** /readyz | GET /readyz
*JobsApi* | [**v1CancelJob**](docs/JobsApi.md#v1canceljob) | **POST** /v1/jobs/{job_id}/cancel | POST /v1/jobs/{job_id}/cancel
*JobsApi* | [**v1DownloadJobResults**](docs/JobsApi.md#v1downloadjobresults) | **GET** /v1/jobs/{job_id}/download | GET /v1/jobs/{job_id}/download
*JobsApi* | [**v1GetBulkJobProgress**](docs/JobsApi.md#v1getbulkjobprogress) | **GET** /v1/bulk/{job_id} | GET /v1/bulk/{job_id}
*JobsApi* | [**v1GetBulkJobResults**](docs/JobsApi.md#v1getbulkjobresults) | **GET** /v1/bulk/{job_id}/results | GET /v1/bulk/{job_id}/results
*JobsApi* | [**v1GetJobEvents**](docs/JobsApi.md#v1getjobevents) | **GET** /v1/jobs/{job_id}/events | GET /v1/jobs/{job_id}/events
*JobsApi* | [**v1GetJobResults**](docs/JobsApi.md#v1getjobresults) | **GET** /v1/jobs/{job_id}/results | GET /v1/jobs/{job_id}/results
*JobsApi* | [**v1GetJobStatus**](docs/JobsApi.md#v1getjobstatus) | **GET** /v1/jobs/{job_id} | GET /v1/jobs/{job_id}
*JobsApi* | [**v1JobApprovalChecklist**](docs/JobsApi.md#v1jobapprovalchecklist) | **GET** /v1/jobs/{job_id}/approval | GET /v1/jobs/{job_id}/approval
*JobsApi* | [**v1JobLatency**](docs/JobsApi.md#v1joblatency) | **GET** /v1/jobs/{job_id}/latency | GET /v1/jobs/{job_id}/latency
*JobsApi* | [**v1JobsJobIdFailureCenterGet**](docs/JobsApi.md#v1jobsjobidfailurecenterget) | **GET** /v1/jobs/{job_id}/failure-center | Get job failure center
*JobsApi* | [**v1JobsJobIdFailureReportGet**](docs/JobsApi.md#v1jobsjobidfailurereportget) | **GET** /v1/jobs/{job_id}/failure-report | Download job failure report
*JobsApi* | [**v1RetryJob**](docs/JobsApi.md#v1retryjob) | **POST** /v1/jobs/{job_id}/retry | POST /v1/jobs/{job_id}/retry
*ListsApi* | [**v1ListQuality**](docs/ListsApi.md#v1listquality) | **GET** /v1/lists/{list_id}/quality | GET /v1/lists/{list_id}/quality
*ListsApi* | [**v1ListsListIdRemediationExportsExportIdDownloadGet**](docs/ListsApi.md#v1listslistidremediationexportsexportiddownloadget) | **GET** /v1/lists/{list_id}/remediation-exports/{export_id}/download | Download remediation export
*ListsApi* | [**v1ListsListIdRemediationExportsPost**](docs/ListsApi.md#v1listslistidremediationexportspost) | **POST** /v1/lists/{list_id}/remediation-exports | Create remediation export
*ListsApi* | [**v1ListsListIdRemediationPlanGet**](docs/ListsApi.md#v1listslistidremediationplanget) | **GET** /v1/lists/{list_id}/remediation-plan | Get remediation plan
*ListsApi* | [**v1ListsListIdRemediationPlanPost**](docs/ListsApi.md#v1listslistidremediationplanpost) | **POST** /v1/lists/{list_id}/remediation-plan | Create remediation plan
*OutcomesApi* | [**v1CreateProviderEndpoint**](docs/OutcomesApi.md#v1createproviderendpoint) | **POST** /v1/provider-endpoints |
*OutcomesApi* | [**v1DeleteProviderEndpoint**](docs/OutcomesApi.md#v1deleteproviderendpoint) | **DELETE** /v1/provider-endpoints/{endpoint_id} |
*OutcomesApi* | [**v1IngestOutcomes**](docs/OutcomesApi.md#v1ingestoutcomes) | **POST** /v1/outcomes | POST /v1/outcomes
*OutcomesApi* | [**v1IngestProviderOutcomes**](docs/OutcomesApi.md#v1ingestprovideroutcomes) | **POST** /v1/inbound/providers/{provider}/{endpoint_id}/{delivery_token} |
*OutcomesApi* | [**v1ListOutcomes**](docs/OutcomesApi.md#v1listoutcomes) | **GET** /v1/outcomes | GET /v1/outcomes
*OutcomesApi* | [**v1ListProviderEndpoints**](docs/OutcomesApi.md#v1listproviderendpoints) | **GET** /v1/provider-endpoints |
*OutcomesApi* | [**v1UpdateProviderEndpoint**](docs/OutcomesApi.md#v1updateproviderendpoint) | **PATCH** /v1/provider-endpoints/{endpoint_id} |
_PipelinesApi_ | [__v1CreatePipeline__](docs/PipelinesApi.md#v1createpipeline) | **POST** /v1/pipelines | POST /v1/pipelines
_PipelinesApi_ | [__v1DeletePipeline__](docs/PipelinesApi.md#v1deletepipeline) | **DELETE** /v1/pipelines/{pipeline_id} | DELETE /v1/pipelines/{pipeline_id}
_PipelinesApi_ | [__v1GetPipeline__](docs/PipelinesApi.md#v1getpipeline) | **GET** /v1/pipelines/{pipeline_id} | GET /v1/pipelines/{pipeline_id}
_PipelinesApi_ | [__v1GetPipelineRun__](docs/PipelinesApi.md#v1getpipelinerun) | **GET** /v1/pipelines/{pipeline_id}/runs/{run_id} | GET /v1/pipelines/{pipeline_id}/runs/{run_id}
_PipelinesApi_ | [__v1ListPipelineRuns__](docs/PipelinesApi.md#v1listpipelineruns) | **GET** /v1/pipelines/{pipeline_id}/runs | GET /v1/pipelines/{pipeline_id}/runs
_PipelinesApi_ | [__v1ListPipelines__](docs/PipelinesApi.md#v1listpipelines) | **GET** /v1/pipelines | GET /v1/pipelines
_PipelinesApi_ | [__v1PausePipeline__](docs/PipelinesApi.md#v1pausepipeline) | **POST** /v1/pipelines/{pipeline_id}/pause | POST /v1/pipelines/{pipeline_id}/pause
_PipelinesApi_ | [**v1PushPipeline**](docs/PipelinesApi.md#v1pushpipeline) | **POST** /v1/pipelines/{pipeline_id}/push | POST /v1/pipelines/{pipeline_id}/push
_PipelinesApi_ | [__v1ResumePipeline__](docs/PipelinesApi.md#v1resumepipeline) | **POST** /v1/pipelines/{pipeline_id}/resume | POST /v1/pipelines/{pipeline_id}/resume
_PipelinesApi_ | [__v1TriggerPipeline__](docs/PipelinesApi.md#v1triggerpipeline) | **POST** /v1/pipelines/{pipeline_id}/trigger | POST /v1/pipelines/{pipeline_id}/trigger
_PipelinesApi_ | [__v1UpdatePipeline__](docs/PipelinesApi.md#v1updatepipeline) | **PATCH** /v1/pipelines/{pipeline_id} | PATCH /v1/pipelines/{pipeline_id}
*QueryApi* | [**v1QueryResults**](docs/QueryApi.md#v1queryresults) | **GET** /v1/query | GET /v1/query
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
*V1Api* | [**v1CheckEmailWithOnboard**](docs/V1Api.md#v1checkemailwithonboard) | **POST** /v1/check-email-with-onboard | POST /v1/check-email-with-onboard — Self-service signup + email verification in one call. No authentication required. Creates a tenant, generates an API key, verifies the email, and returns all three.
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
*V1Api* | [**v1ReverificationStatus**](docs/V1Api.md#v1reverificationstatus) | **GET** /v1/reverification/status | GET /v1/reverification/status
*V1Api* | [**v1SourcesQualityGet**](docs/V1Api.md#v1sourcesqualityget) | **GET** /v1/sources/quality | List source quality
*V1Api* | [**v1SuppressionsExportGet**](docs/V1Api.md#v1suppressionsexportget) | **GET** /v1/suppressions/export | Export suppressions
*V1Api* | [**v1SuppressionsIdEventsGet**](docs/V1Api.md#v1suppressionsideventsget) | **GET** /v1/suppressions/{id}/events | List suppression events
*V1Api* | [**v1SuppressionsImportPost**](docs/V1Api.md#v1suppressionsimportpost) | **POST** /v1/suppressions/import | Import suppressions
*VerificationApi* | [**v1EmailHistory**](docs/VerificationApi.md#v1emailhistory) | **GET** /v1/emails/{email}/history | GET /v1/emails/{email}/history


### Documentation For Models

 - [AddSuppressionsRequest](docs/AddSuppressionsRequest.md)
 - [AddSuppressionsResponse](docs/AddSuppressionsResponse.md)
 - [ApprovalCategoryBreakdown](docs/ApprovalCategoryBreakdown.md)
 - [ApprovalChecklistResponse](docs/ApprovalChecklistResponse.md)
 - [ApprovalRiskFlags](docs/ApprovalRiskFlags.md)
 - [BlacklistResult](docs/BlacklistResult.md)
 - [BounceRiskAssessment](docs/BounceRiskAssessment.md)
 - [BounceRiskCategory](docs/BounceRiskCategory.md)
 - [BulkCreateRequest](docs/BulkCreateRequest.md)
 - [BulkCreateResponse](docs/BulkCreateResponse.md)
 - [BulkJobResultsResponse](docs/BulkJobResultsResponse.md)
 - [CheckEmailInputProxy](docs/CheckEmailInputProxy.md)
 - [CheckEmailOutput](docs/CheckEmailOutput.md)
 - [CheckEmailOutputMisc](docs/CheckEmailOutputMisc.md)
 - [CheckEmailOutputMx](docs/CheckEmailOutputMx.md)
 - [CheckEmailOutputSmtp](docs/CheckEmailOutputSmtp.md)
 - [CheckEmailRequest](docs/CheckEmailRequest.md)
 - [ConfidenceExplanation](docs/ConfidenceExplanation.md)
 - [CoreError](docs/CoreError.md)
 - [CreatePipelineInput](docs/CreatePipelineInput.md)
 - [CreateProviderEndpointInput](docs/CreateProviderEndpointInput.md)
 - [DebugDetails](docs/DebugDetails.md)
 - [DebugDetailsSmtp](docs/DebugDetailsSmtp.md)
 - [DeletePipelineResponse](docs/DeletePipelineResponse.md)
 - [DnsRecordResults](docs/DnsRecordResults.md)
 - [DomainInfo](docs/DomainInfo.md)
 - [Duration](docs/Duration.md)
 - [EmailCategory](docs/EmailCategory.md)
 - [EmailScore](docs/EmailScore.md)
 - [ErrorResponse](docs/ErrorResponse.md)
 - [FindEmailAcceptedResponse](docs/FindEmailAcceptedResponse.md)
 - [FindEmailRequest](docs/FindEmailRequest.md)
 - [FindEmailStatusResponse](docs/FindEmailStatusResponse.md)
 - [FinderBestMatch](docs/FinderBestMatch.md)
 - [FinderCandidateResult](docs/FinderCandidateResult.md)
 - [Freshness](docs/Freshness.md)
 - [GmailVerifMethod](docs/GmailVerifMethod.md)
 - [HotmailB2BVerifMethod](docs/HotmailB2BVerifMethod.md)
 - [HotmailB2CVerifMethod](docs/HotmailB2CVerifMethod.md)
 - [InboundOutcomeResponse](docs/InboundOutcomeResponse.md)
 - [JobResultPageResponse](docs/JobResultPageResponse.md)
 - [JobTaskResult](docs/JobTaskResult.md)
 - [ListDeleteResponse](docs/ListDeleteResponse.md)
 - [ListDetailResponse](docs/ListDetailResponse.md)
 - [ListItem](docs/ListItem.md)
 - [ListListResponse](docs/ListListResponse.md)
 - [ListPipelineRunsResponse](docs/ListPipelineRunsResponse.md)
 - [ListPipelinesResponse](docs/ListPipelinesResponse.md)
 - [ListSummary](docs/ListSummary.md)
 - [ListUploadResponse](docs/ListUploadResponse.md)
 - [MiscDetails](docs/MiscDetails.md)
 - [MxDetails](docs/MxDetails.md)
 - [OutcomeIngestRequest](docs/OutcomeIngestRequest.md)
 - [OutcomeIngestResponse](docs/OutcomeIngestResponse.md)
 - [OutcomeInput](docs/OutcomeInput.md)
 - [OutcomeListResponse](docs/OutcomeListResponse.md)
 - [OutcomeView](docs/OutcomeView.md)
 - [PipelineDeliveryConfig](docs/PipelineDeliveryConfig.md)
 - [PipelineDeliveryStatus](docs/PipelineDeliveryStatus.md)
 - [PipelineDeliveryWebhook](docs/PipelineDeliveryWebhook.md)
 - [PipelineErrorResponse](docs/PipelineErrorResponse.md)
 - [PipelinePolicyConfig](docs/PipelinePolicyConfig.md)
 - [PipelineRunResultLocation](docs/PipelineRunResultLocation.md)
 - [PipelineRunStats](docs/PipelineRunStats.md)
 - [PipelineRunStatus](docs/PipelineRunStatus.md)
 - [PipelineRunView](docs/PipelineRunView.md)
 - [PipelineSchedule](docs/PipelineSchedule.md)
 - [PipelineSource](docs/PipelineSource.md)
 - [PipelineSourceOneOf](docs/PipelineSourceOneOf.md)
 - [PipelineSourceOneOf1](docs/PipelineSourceOneOf1.md)
 - [PipelineSourceOneOf2](docs/PipelineSourceOneOf2.md)
 - [PipelineSourceOneOf3](docs/PipelineSourceOneOf3.md)
 - [PipelineStatus](docs/PipelineStatus.md)
 - [PipelineVerificationSettings](docs/PipelineVerificationSettings.md)
 - [PipelineView](docs/PipelineView.md)
 - [Provider](docs/Provider.md)
 - [ProviderConfidence](docs/ProviderConfidence.md)
 - [ProviderDeleteResponse](docs/ProviderDeleteResponse.md)
 - [ProviderEndpointListResponse](docs/ProviderEndpointListResponse.md)
 - [ProviderEndpointView](docs/ProviderEndpointView.md)
 - [ProviderRejectionReason](docs/ProviderRejectionReason.md)
 - [PushPipelineInput](docs/PushPipelineInput.md)
 - [PushPipelineResponse](docs/PushPipelineResponse.md)
 - [Reachable](docs/Reachable.md)
 - [ReasonCode](docs/ReasonCode.md)
 - [RecommendedAction](docs/RecommendedAction.md)
 - [ReputationCheckRequest](docs/ReputationCheckRequest.md)
 - [ReputationCheckResponse](docs/ReputationCheckResponse.md)
 - [RetryJobResponse](docs/RetryJobResponse.md)
 - [ReverificationStatusResponse](docs/ReverificationStatusResponse.md)
 - [RiskDirection](docs/RiskDirection.md)
 - [RiskFactor](docs/RiskFactor.md)
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
 - [TriggerPipelineInput](docs/TriggerPipelineInput.md)
 - [TriggerPipelineResponse](docs/TriggerPipelineResponse.md)
 - [UpdatePipelineInput](docs/UpdatePipelineInput.md)
 - [UpdateProviderEndpointInput](docs/UpdateProviderEndpointInput.md)
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
