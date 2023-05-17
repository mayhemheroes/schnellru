# Build Stage
FROM --platform=linux/amd64 rustlang/rust:nightly as builder

ENV DEBIAN_FRONTEND=noninteractive
## Install build dependencies.
RUN apt-get update 
RUN apt-get install -y binutils-dev libunwind-dev libblocksruntime-dev cmake clang
RUN cargo install honggfuzz

## Add source code to the build stage.
ADD . /schnellru/

WORKDIR /schnellru/fuzz/

RUN cargo hfuzz build

FROM --platform=linux/amd64 rustlang/rust:nightly

COPY --from=builder /schnellru/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz-behavior-matches-lru /

COPY --from=builder /schnellru/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz-by-memory-usage-works /

COPY --from=builder /schnellru/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz-internal-invariants /

COPY --from=builder /schnellru/fuzz/hfuzz_target/x86_64-unknown-linux-gnu/release/fuzz-insert /