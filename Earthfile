# https://earthly.dev/
# Fast, repeatable CI/CD with an instantly familiar syntax – like Dockerfile and Makefile had a baby.

# Set the version
VERSION 0.6

# We use our devcontainer as it has all the tools we need
FROM tobkle/toby-dev-env:latest

ARG APP_NAME=app
ARG APP_FOLDER=app
ARG IMAGE_PREFIX=ghcr.io/tobkle
ARG APP_EXE_NAME=web-server

# Build target (override via `.arg` or `--arg-file-path`, e.g. `RUST_TARGET=aarch64-unknown-linux-musl`)
ARG RUST_TARGET=x86_64-unknown-linux-musl

# dbmate release arch string (override via `.arg` or `--arg-file-path`, e.g. `DBMATE_ARCH=arm64`)
ARG DBMATE_ARCH=amd64

# Version of software
ARG DBMATE_VERSION=2.28.0

# OCI image metadata (used by GHCR to show the package source repo)
ARG SOURCE_REPO_URL=https://github.com/tobkle/rust-example-app

# Folders
ARG AXUM_FOLDER=crates/web-server
ARG DB_FOLDER=crates/db
ARG GRPC_API_FOLDER=crates/grpc-api
ARG PIPELINE_FOLDER=crates/asset-pipeline

# This file builds the following containers
ARG APP_IMAGE_NAME=$IMAGE_PREFIX/$APP_NAME:latest
ARG MIGRATIONS_IMAGE_NAME=$IMAGE_PREFIX/$APP_NAME-migrations:latest

WORKDIR /build

USER root

# Set up for docker in docker https://github.com/earthly/earthly/issues/1225
DO github.com/earthly/lib+INSTALL_DIND

USER vscode

all:
    BUILD +migration-container
    BUILD +app-container

npm-deps:
    COPY $PIPELINE_FOLDER/package.json $PIPELINE_FOLDER/package.json
    COPY $PIPELINE_FOLDER/package-lock.json $PIPELINE_FOLDER/package-lock.json
    RUN cd $PIPELINE_FOLDER && npm install
    SAVE ARTIFACT $PIPELINE_FOLDER/node_modules

npm-build:
    FROM +npm-deps
    COPY $PIPELINE_FOLDER $PIPELINE_FOLDER
    COPY --if-exists $GRPC_API_FOLDER $GRPC_API_FOLDER
    COPY +npm-deps/node_modules $PIPELINE_FOLDER/node_modules
    RUN cd $PIPELINE_FOLDER && npm run release
    SAVE ARTIFACT $PIPELINE_FOLDER/dist

prepare-cache:
    # Copy in all our crates
    COPY --dir crates crates
    COPY Cargo.lock Cargo.toml .
    RUN cargo chef prepare --recipe-path recipe.json --bin $APP_EXE_NAME
    SAVE ARTIFACT recipe.json

build-cache:
    COPY +prepare-cache/recipe.json ./
    RUN cargo chef cook --release --target $RUST_TARGET
    SAVE ARTIFACT target
    SAVE ARTIFACT $CARGO_HOME cargo_home
    SAVE IMAGE --cache-hint

build:
    # Copy in all our crates
    COPY --dir crates crates
    COPY --dir Cargo.lock Cargo.toml .
    COPY +build-cache/cargo_home $CARGO_HOME
    COPY +build-cache/target target
    COPY --dir +npm-build/dist $PIPELINE_FOLDER/
    # We need to run inside docker as we need postgres running for cornucopia
    ARG DATABASE_URL=postgresql://postgres:testpassword@localhost:5432/postgres?sslmode=disable
    USER root
    WITH DOCKER \
        --pull postgres:alpine
        RUN docker run -d --rm --network=host -e POSTGRES_PASSWORD=testpassword postgres:alpine \
            && while ! pg_isready --host=localhost --port=5432 --username=postgres; do sleep 1; done ;\
                dbmate --migrations-dir $DB_FOLDER/migrations up \
            && cargo build --release --target $RUST_TARGET --bin $APP_EXE_NAME
    END
    SAVE ARTIFACT target/$RUST_TARGET/release/$APP_EXE_NAME

# This is our migrations sidecar
migration-container:
    FROM alpine
    LABEL org.opencontainers.image.source=$SOURCE_REPO_URL
    RUN apk add --no-cache \
        curl \
        postgresql-client \
        tzdata
    RUN curl -OL https://github.com/amacneil/dbmate/releases/download/v$DBMATE_VERSION/dbmate-linux-$DBMATE_ARCH \
        && mv ./dbmate-linux-$DBMATE_ARCH /usr/bin/dbmate \
        && chmod +x /usr/bin/dbmate
    COPY --dir $DB_FOLDER .
    CMD dbmate up
    SAVE IMAGE $MIGRATIONS_IMAGE_NAME
    SAVE IMAGE --push $MIGRATIONS_IMAGE_NAME

# Our axum server
app-container:
    FROM scratch
    LABEL org.opencontainers.image.source=$SOURCE_REPO_URL
    COPY +build/$APP_EXE_NAME web-server
    # Place assets in a build folder as that's where statics is expecting them.
    COPY --dir +npm-build/dist /build/$PIPELINE_FOLDER/
    COPY --dir $PIPELINE_FOLDER/images /build/$PIPELINE_FOLDER/images
    ENTRYPOINT ["./web-server"]
    SAVE IMAGE $APP_IMAGE_NAME
    SAVE IMAGE --push $APP_IMAGE_NAME