# syntax=docker.io/docker/dockerfile:1

# This enforces that the packages downloaded from the repositories are the same
# for the defined date, no matter when the image is built.
ARG APT_UPDATE_SNAPSHOT=20250915T030400Z
ARG MACHINE_GUEST_TOOLS_VERSION=0.17.2
ARG MACHINE_GUEST_TOOLS_SHA256SUM=c077573dbcf0cdc146adf14b480bfe454ca63aa4d3e8408c5487f550a5b77a41
ARG MACHINE_ASSET_TOOLS_VERSION=0.1.0-alpha.4
ARG MACHINE_ASSET_TOOLS_SHA256SUM=dd04ded94fac81b7c3535424edc76599724bac52c338c3bccffffc38e7f084c7
ARG MACHINE_ASSET_TOOLS_PYMOD_SHA256SUM=b574e472437725c7b46af020435da2b3ee60c37c004fb561d8522462332886ca

################################################################################
# riscv64 base stage
FROM --platform=linux/riscv64 cartesi/python:3.13.2-slim-noble AS base

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

ARG DEBIAN_FRONTEND=noninteractive
RUN <<EOF
set -e
apt-get install -y --no-install-recommends \
  /tmp/machine-guest-tools_riscv64.deb \
  /tmp/machine-asset-tools_riscv64.deb

rm /tmp/machine-guest-tools_riscv64.deb /tmp/machine-asset-tools_riscv64.deb
EOF

ARG MACHINE_ASSET_TOOLS_VERSION
ARG MACHINE_ASSET_TOOLS_PYMOD_SHA256SUM
ADD --checksum=sha256:${MACHINE_ASSET_TOOLS_PYMOD_SHA256SUM} --chmod=644 \
    https://github.com/Mugen-Builders/machine-asset-tools/releases/download/v${MACHINE_ASSET_TOOLS_VERSION}/cmpy.cpython-313-riscv64-linux-gnu.so \
    /usr/local/lib/python3.13/site-packages/.

################################################################################
# runtime stage: produces final image that will be executed

# Here the image's platform MUST be linux/riscv64.
# Give preference to small base images, which lead to better start-up
# performance when loading the Cartesi Machine.
FROM base

WORKDIR /opt/cartesi/app

COPY ./app.py .

RUN rm -rf /var/lib/apt/lists/* /var/log/* /var/cache/*

CMD ["python3", "app.py"]
