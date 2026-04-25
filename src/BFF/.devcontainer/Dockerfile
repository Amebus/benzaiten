FROM oven/bun:1.3.11-alpine
WORKDIR /usr/src/app

RUN apk add --no-cache \
    ca-certificates \
    git \
    bash \
    curl \
    openssh-client

WORKDIR /workspace

USER bun
