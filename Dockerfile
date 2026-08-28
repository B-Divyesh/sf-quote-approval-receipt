FROM node:22-bookworm-slim AS frontend
WORKDIR /src
COPY package.json package-lock.json tsconfig.json vite.config.ts ./
COPY frontend ./frontend
COPY public ./public
RUN npm ci && npm run build

FROM rust:1.88-bookworm AS backend
ARG BUILD_SHA=dev
ENV BUILD_SHA=${BUILD_SHA}
WORKDIR /src
COPY Cargo.toml Cargo.lock build.rs ./
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
RUN apt-get update && apt-get install -y --no-install-recommends fonts-dejavu-core ca-certificates \
    && rm -rf /var/lib/apt/lists/* \
    && groupadd --system app && useradd --system --gid app --home-dir /app app \
    && mkdir -p /app/dist /data && chown -R app:app /app /data
WORKDIR /app
COPY --from=backend /src/target/release/quote-approval-receipt /app/server
COPY --from=frontend /src/dist /app/dist
ENV PORT=8080 DATA_DIR=/data STATIC_DIR=/app/dist
EXPOSE 8080
USER app
CMD ["/app/server"]
