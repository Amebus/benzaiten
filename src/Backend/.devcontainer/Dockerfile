FROM rust:1.75-slim-bookworm

RUN apt-get update && apt-get install -y \
    git curl wget build-essential pkg-config \
    libssl-dev postgresql-client \
    && rm -rf /var/lib/apt/lists/*

RUN rustup component add rustfmt clippy rust-analyzer

RUN useradd -ms /bin/bash vscode

USER vscode
WORKDIR /workspace/src/Backend

CMD ["sleep", "infinity"]
