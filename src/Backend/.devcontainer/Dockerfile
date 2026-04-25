FROM rust:1.95-alpine

RUN apk add --no-cache \
    git curl wget build-base pkgconfig \
    openssl-dev musl-dev postgresql-client

RUN rustup component add rustfmt clippy rust-analyzer

RUN adduser -D -s /bin/sh vscode

USER vscode
WORKDIR /workspace/src/Backend

CMD ["sleep", "infinity"]
