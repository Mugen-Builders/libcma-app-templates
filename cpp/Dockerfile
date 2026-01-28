# syntax=docker.io/docker/dockerfile:1

# This enforces that the packages downloaded from the repositories are the same
# for the defined date, no matter when the image is built.
ARG UBUNTU_TAG=noble-20250910
ARG APT_UPDATE_SNAPSHOT=20250915T030400Z
ARG MACHINE_GUEST_TOOLS_VERSION=0.17.2
ARG MACHINE_GUEST_TOOLS_SHA256SUM=c077573dbcf0cdc146adf14b480bfe454ca63aa4d3e8408c5487f550a5b77a41
ARG MACHINE_ASSET_TOOLS_VERSION=0.1.0-alpha.4
ARG MACHINE_ASSET_TOOLS_SHA256SUM=dd04ded94fac81b7c3535424edc76599724bac52c338c3bccffffc38e7f084c7

################################################################################
# riscv64 base stage
FROM --platform=linux/riscv64 ubuntu:${UBUNTU_TAG} AS base

ARG APT_UPDATE_SNAPSHOT
ARG DEBIAN_FRONTEND=noninteractive
RUN <<EOF
set -eu
apt-get update
apt-get install -y --no-install-recommends ca-certificates
apt-get update --snapshot=${APT_UPDATE_SNAPSHOT}
apt-get remove -y --purge ca-certificates
apt-get autoremove -y --purge
EOF

ARG MACHINE_GUEST_TOOLS_VERSION
ARG MACHINE_GUEST_TOOLS_SHA256SUM
ADD --checksum=sha256:${MACHINE_GUEST_TOOLS_SHA256SUM} \
    https://github.com/cartesi/machine-guest-tools/releases/download/v${MACHINE_GUEST_TOOLS_VERSION}/machine-guest-tools_riscv64.deb \
    /tmp/machine-guest-tools_riscv64.deb

ARG MACHINE_ASSET_TOOLS_VERSION
ARG MACHINE_ASSET_TOOLS_SHA256SUM
ADD --checksum=sha256:${MACHINE_ASSET_TOOLS_SHA256SUM} \
    https://github.com/Mugen-Builders/machine-asset-tools/releases/download/v${MACHINE_ASSET_TOOLS_VERSION}/machine-asset-tools_riscv64_v${MACHINE_ASSET_TOOLS_VERSION}.deb \
    /tmp/machine-asset-tools_riscv64.deb

RUN <<EOF
set -e
apt-get install -y --no-install-recommends \
  busybox-static \
  /tmp/machine-guest-tools_riscv64.deb \
  /tmp/machine-asset-tools_riscv64.deb

rm /tmp/machine-guest-tools_riscv64.deb /tmp/machine-asset-tools_riscv64.deb
EOF

################################################################################
# riscv64 builder stage
FROM base AS builder

ARG DEBIAN_FRONTEND=noninteractive
RUN <<EOF
set -e
apt-get install -y --no-install-recommends \
  autoconf \
  automake \
  build-essential \
  ca-certificates \
  curl \
  libtool
rm -rf /var/lib/apt/lists/*
EOF

WORKDIR /opt/cartesi/app
COPY . .
RUN make

################################################################################
# runtime stage: produces final image that will be executed
FROM base

WORKDIR /opt/cartesi/app
COPY --from=builder /opt/cartesi/app/app .

RUN rm -rf /var/lib/apt/lists/* /var/log/* /var/cache/*

CMD ["/opt/cartesi/app/app"]
