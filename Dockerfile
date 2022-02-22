# https://github.com/skerkour/kerkour.com/blob/main/2021/2021_04_06_rust_minimal_docker_image/myip/Dockerfile.scratch

####################################################################################################
## Builder image
####################################################################################################

FROM rust:latest AS builder
WORKDIR /s3d

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev
RUN update-ca-certificates

# Create user
ENV USER=s3d
ENV UID=10001
RUN adduser \
    --uid "${UID}" \
    --home "/s3d" \
    --shell "/sbin/nologin" \
    --gecos "" \
    --no-create-home \
    --disabled-password \
    "${USER}"

COPY ./ .
ENV CARGO_BUILD_TARGET="x86_64-unknown-linux-musl"
RUN make RELEASE=1

####################################################################################################
## Final image
####################################################################################################

FROM scratch
WORKDIR /s3d

# Copy files from builder image
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group
COPY --from=builder /s3d/target/x86_64-unknown-linux-musl/release/s3d ./
COPY --from=builder /s3d/target/x86_64-unknown-linux-musl/release/s3c ./

# Use an unprivileged user
USER s3d:s3d

CMD ["/s3d/s3d"]
