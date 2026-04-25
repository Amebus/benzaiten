FROM oven/bun:1-debian

RUN apt-get update && apt-get install -y \
    git curl wget ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN groupadd --gid 1000 node \
    && useradd --uid 1000 --gid node --shell /bin/bash --create-home node

USER node
WORKDIR /workspace/src/UI

CMD ["sleep", "infinity"]
