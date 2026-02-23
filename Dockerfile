# ── Stage 1: Rust build ──────────────────────────────────────────────────────
FROM rust:1.82-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# Python dev headers required by PyO3
RUN apt-get update && apt-get install -y python3-dev && rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# ── Stage 2: Runtime ─────────────────────────────────────────────────────────
FROM debian:bookworm-slim

# System runtimes available in apt
RUN apt-get update && apt-get install -y --no-install-recommends \
    python3 python3-pip \
    golang-go \
    nodejs npm \
    lua5.4 \
    r-base \
    gfortran \
    ghc \
    nim \
    curl ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Deno
RUN curl -fsSL https://deno.land/install.sh | sh
ENV DENO_INSTALL="/root/.deno"
ENV PATH="${DENO_INSTALL}/bin:${PATH}"

# Zig 0.12
RUN curl -fsSL https://ziglang.org/download/0.12.0/zig-linux-x86_64-0.12.0.tar.xz \
    | tar -xJ -C /usr/local && \
    ln -s /usr/local/zig-linux-x86_64-0.12.0/zig /usr/local/bin/zig

# wasmtime
RUN curl -fsSL https://wasmtime.dev/install.sh | bash
ENV PATH="/root/.wasmtime/bin:${PATH}"

# Julia (juliac requires 1.12+; install latest stable)
RUN curl -fsSL https://install.julialang.org | sh -s -- --yes
ENV PATH="/root/.juliaup/bin:${PATH}"

# Swift (Linux)
RUN curl -fsSL https://download.swift.org/swift-6.0.3-release/debian12/swift-6.0.3-RELEASE/swift-6.0.3-RELEASE-debian12.tar.gz \
    | tar -xz -C /usr/local --strip-components=1

# Kotlin via SDKMAN
RUN curl -s "https://get.sdkman.io" | bash && \
    bash -c "source /root/.sdkman/bin/sdkman-init.sh && sdk install kotlin"
ENV PATH="/root/.sdkman/candidates/kotlin/current/bin:${PATH}"

# Copy polyscript binary
COPY --from=builder /app/target/release/polyscript /usr/local/bin/polyscript

WORKDIR /workspace
ENTRYPOINT ["polyscript"]
