###############################################################################
# Global tooling
###############################################################################

DOCKER_COMPOSE ?= docker-compose
COMPOSE_FILE := $(CURDIR)/rabbitmq/docker-compose.yaml
TEST_COMPOSE_FILE := $(CURDIR)/docker-compose.yml
TEST_EMAIL_SCRIPT := ./scripts/test_email.go
TEST_DATABASE_URL ?= postgres://postgres:postgres@127.0.0.1:25432/reacher_test
TEST_AMQP_URL ?= amqp://guest:guest@127.0.0.1:35672

###############################################################################
# Run
###############################################################################

# Run the backend without worker mode, i.e. only enabling single-shot
# verifications via the /v1/check_email endpoint.
.PHONY: run
run:
	cd backend && cargo run --bin reacher_backend

# Run the backend with worker mode on. This enables the /v1/bulk endpoints.
# Make sure to have a Postgres DB and a RabbitMQ instance running.
.PHONY: run-with-worker
run-with-worker: export RCH__WORKER__ENABLE=true
run-with-worker: export RCH__WORKER__RABBITMQ__URL=amqp://guest:guest@localhost:5672
run-with-worker: export RCH__STORAGE__POSTGRES__DB_URL=postgresql://localhost/reacherdb
run-with-worker: run

.PHONY: run-with-commercial-license-trial
run-with-commercial-license-trial: export RCH__COMMERCIAL_LICENSE_TRIAL__URL=http://localhost:3000/api/v1/commercial_license_trial
run-with-commercial-license-trial: run

# Bring up the full Docker Compose stack and run an end-to-end email verification.
.PHONY: deploy-and-test
deploy-and-test:
	$(DOCKER_COMPOSE) -f $(COMPOSE_FILE) up -d --build
	go run $(TEST_EMAIL_SCRIPT)

.PHONY: test-services-up
test-services-up:
	$(DOCKER_COMPOSE) -f $(TEST_COMPOSE_FILE) up -d postgres rabbitmq
	$(DOCKER_COMPOSE) -f $(TEST_COMPOSE_FILE) exec -T postgres sh -lc "until pg_isready -U postgres -d postgres >/dev/null 2>&1; do sleep 1; done"
	$(DOCKER_COMPOSE) -f $(TEST_COMPOSE_FILE) exec -T postgres psql -U postgres -d postgres -tc "SELECT 1 FROM pg_database WHERE datname = 'reacher_test'" | grep -q 1 || \
		$(DOCKER_COMPOSE) -f $(TEST_COMPOSE_FILE) exec -T postgres psql -U postgres -d postgres -c "CREATE DATABASE reacher_test"

.PHONY: test-services-down
test-services-down:
	$(DOCKER_COMPOSE) -f $(TEST_COMPOSE_FILE) down

.PHONY: test-pipelines-db
test-pipelines-db: test-services-up
	cd backend && TEST_DATABASE_URL=$(TEST_DATABASE_URL) TEST_AMQP_URL=$(TEST_AMQP_URL) cargo test --test e2e_pipelines -- --nocapture

# Generate the changelog using the conventional-changelog tool.
# As a hack, we delete all tags that are not beta tags, so that the changelog
# only contains the vX.X.X tags. See:
# https://github.com/conventional-changelog/standard-version/issues/818
#
# To have those tags back locally, run `git fetch --tags`.
.PHONY: changelog
changelog:
	git tag | grep -E '(beta|backend|worker)' | xargs git tag -d
	echo "# Changelog" > CHANGELOG.md
	echo "" >> CHANGELOG.md
	echo "All notable changes to this project will be documented in this file. The changes in this project follow [Convention Commits](https://www.conventionalcommits.org/en/v1.0.0/)." >> CHANGELOG.md
	echo "" >> CHANGELOG.md
	conventional-changelog -p angular -r 0 >> CHANGELOG.md

###############################################################################
# Update lists
###############################################################################

.PHONY: update-role-accounts
update-role-accounts:
# License is MIT.
	curl https://raw.githubusercontent.com/mixmaxhq/role-based-email-addresses/refs/heads/master/index.js -o core/src/misc/roles.txt
# Remove first line, last line, and all ' and , characters
	sed -i.bak '1d' core/src/misc/roles.txt && rm core/src/misc/roles.txt.bak
	sed -i.bak '$$d' core/src/misc/roles.txt && rm core/src/misc/roles.txt.bak
	sed -i.bak 's/['\'', ]//g' core/src/misc/roles.txt && rm core/src/misc/roles.txt.bak


.PHONY: update-free-email-providers
update-free-email-providers:
# License is MIT.
	curl https://raw.githubusercontent.com/ihmpavel/free-email-domains-list/refs/heads/master/data/data.txt -o core/src/misc/b2c.txt

###############################################################################
# SDK Generation
###############################################################################

OPENAPI_GENERATOR_VERSION ?= 7.14.0
OPENAPI_SPEC := $(CURDIR)/backend/openapi.json
SDK_DIR := $(CURDIR)/sdks

.PHONY: sdk-install-generator
sdk-install-generator:
	@which openapi-generator-cli > /dev/null || npm install -g @openapitools/openapi-generator-cli
	openapi-generator-cli version-manager set $(OPENAPI_GENERATOR_VERSION)

.PHONY: sdk-generate-typescript
sdk-generate-typescript: sdk-install-generator
	openapi-generator-cli generate \
		-i $(OPENAPI_SPEC) \
		-g typescript-axios \
		-o $(SDK_DIR)/typescript/src \
		--additional-properties=npmName=@oppulence/reacher-sdk,supportsES6=true,withInterfaces=true,withSeparateModelsAndApi=true,apiPackage=api,modelPackage=models,useSingleRequestParameter=true

.PHONY: sdk-generate-golang
sdk-generate-golang: sdk-install-generator
	openapi-generator-cli generate \
		-i $(OPENAPI_SPEC) \
		-g go \
		-o $(SDK_DIR)/golang \
		--additional-properties=packageName=reacher,isGoSubmodule=true,generateInterfaces=true,structPrefix=true,enumClassPrefix=true \
		--global-property=skipFormModel=false
	rm -rf $(SDK_DIR)/golang/test
	cd $(SDK_DIR)/golang && go mod tidy

.PHONY: sdk-generate-all
sdk-generate-all: sdk-generate-typescript sdk-generate-golang
	@echo "SDKs generated successfully!"

.PHONY: sdk-build-typescript
sdk-build-typescript:
	cd $(SDK_DIR)/typescript && npm install && npm run build

.PHONY: sdk-test-typescript
sdk-test-typescript:
	cd $(SDK_DIR)/typescript && npm test

.PHONY: sdk-test-golang
sdk-test-golang:
	cd $(SDK_DIR)/golang && go test ./...

.PHONY: sdk-clean
sdk-clean:
	rm -rf $(SDK_DIR)/typescript/dist
	rm -rf $(SDK_DIR)/typescript/node_modules
	rm -rf $(SDK_DIR)/golang/docs
	find $(SDK_DIR)/golang -name "*.go" ! -name "*_test.go" -delete
