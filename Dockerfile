# syntax=docker/dockerfile:1

FROM messense/rust-musl-cross:x86_64-musl AS backend-build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
COPY core ./core
COPY cli ./cli
COPY sqs ./sqs
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin reacher_backend --target x86_64-unknown-linux-musl

FROM --platform=linux/amd64 node:24-alpine AS web-deps
WORKDIR /src/web
RUN corepack enable
COPY sdks/typescript/src /src/sdks/typescript/src
RUN npm install --prefix /src/sdks/typescript/src --no-package-lock --ignore-scripts \
 && npm run build --prefix /src/sdks/typescript/src
COPY web/package.json web/pnpm-lock.yaml web/.npmrc ./
COPY web/vendor ./vendor
COPY web/config/fumadocs ./config/fumadocs
COPY web/content ./content
RUN pnpm install --frozen-lockfile

FROM web-deps AS web-build
WORKDIR /src/web
COPY web ./
ENV NEXT_TELEMETRY_DISABLED=1
RUN pnpm build

FROM --platform=linux/amd64 node:24-alpine
RUN apk add --no-cache chromium chromium-chromedriver
WORKDIR /srv/web
ENV NODE_ENV=production \
    NEXT_TELEMETRY_DISABLED=1 \
    HOSTNAME=0.0.0.0 \
    PORT=3000 \
    RCH__HTTP_HOST=127.0.0.1 \
    RCH__HTTP_PORT=8081 \
    RCH__WORKER__ENABLE=true \
    BACKEND_API_URL=http://127.0.0.1:8081
COPY --from=backend-build --chown=node:node /src/target/x86_64-unknown-linux-musl/release/reacher_backend /srv/reacher_backend
COPY --from=backend-build --chown=node:node /src/backend/backend_config.toml /srv/backend_config.toml
COPY --from=web-build --chown=node:node /src/web/.next/standalone ./
COPY --from=web-build --chown=node:node /src/web/.next/static ./.next/static
COPY --from=web-build --chown=node:node /src/web/public ./public
COPY --from=web-build --chown=node:node /src/web/content ./content
COPY --from=web-build --chown=node:node /src/web/config/contracts/backend.openapi.json ./config/contracts/backend.openapi.json
COPY --from=web-build --chown=node:node /src/web/config/contracts/backend.capabilities.json ./config/contracts/backend.capabilities.json
COPY --from=web-build --chown=node:node /src/web/config/auth/migrations ./config/auth/migrations
COPY --from=web-build --chown=node:node /src/web/scripts/migrate-auth.mjs ./scripts/migrate-auth.mjs
COPY --chown=node:node web/config/auth/digitalocean-db-ca.pem /srv/digitalocean-db-ca.pem
COPY --chown=node:node scripts/full-stack-entrypoint.sh /srv/full-stack-entrypoint.sh
USER node
EXPOSE 3000
HEALTHCHECK --interval=30s --timeout=5s --start-period=30s --retries=3 \
  CMD wget -qO- http://127.0.0.1:3000/readyz >/dev/null || exit 1
ENTRYPOINT ["/bin/sh", "/srv/full-stack-entrypoint.sh"]
