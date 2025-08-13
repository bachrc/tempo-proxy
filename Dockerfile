# Build stage for web interface
FROM node:20 AS web-builder
WORKDIR /app/web
COPY web/package*.json ./
RUN npm ci
COPY web/ ./
RUN npm run build

# Build stage for Rust binary
FROM rust:1.89 AS rust-builder
WORKDIR /app

# Get target architecture from Docker buildx
ARG TARGETPLATFORM
ARG BUILDPLATFORM

# Set Rust target based on architecture and install target
RUN case "$TARGETPLATFORM" in \
  "linux/amd64") rustup target add x86_64-unknown-linux-gnu && echo "x86_64-unknown-linux-gnu" > /tmp/rust-target ;; \
  "linux/arm64") rustup target add aarch64-unknown-linux-gnu && echo "aarch64-unknown-linux-gnu" > /tmp/rust-target ;; \
  *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
esac

# Copy source code
COPY . .

# Copy web build from previous stage
COPY --from=web-builder /app/web/build ./web/build

# Build the binary
RUN cargo build --release --target $(cat /tmp/rust-target)

# Runtime stage - minimal Alpine
FROM alpine:latest
RUN apk --no-cache add ca-certificates
RUN addgroup -S appgroup && adduser -S appuser -G appgroup

# Copy the binary (use wildcard to match any target architecture)
COPY --from=rust-builder /app/target/*/release/tempo-proxy /usr/local/bin/tempo-proxy
RUN chmod +x /usr/local/bin/tempo-proxy

USER appuser
EXPOSE 8080
CMD ["tempo-proxy", "serve", "--interface", "0.0.0.0:8080"]