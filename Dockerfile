# multistage docker build
FROM rust:1.87-slim AS builder
ENV SQLX_OFFLINE=true
WORKDIR /app

# Install dependencies
RUN apt-get update && apt-get install -y protobuf-compiler pkg-config libssl-dev

# copy relevant source code
COPY ./chat ./chat
COPY ./protos ./protos

# build and show build result
RUN cd chat && cargo build --release
RUN ls /app/chat/target/release

# final stage
FROM alpine:3.20

WORKDIR /app

# Create a non-root user and group
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Set permissions for /app
RUN chown -R appuser:appgroup /app

# Switch to the non-root user
USER appuser

ARG APP_NAME
ARG APP_PORT

COPY --from=builder /app/chat/target/release/$APP_NAME /app/$APP_NAME

EXPOSE $APP_PORT
