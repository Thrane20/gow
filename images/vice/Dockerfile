ARG BASE_APP_IMAGE

# hadolint ignore=DL3006
FROM ${BASE_APP_IMAGE}

WORKDIR /opt/src/vice
COPY source/ .

# Install the vice binaries (and supporting libs)
RUN apt-get update
RUN apt-get install -y ./gtk3vice_3.7.1+r44049.deb
RUN apt-get install -y libvorbisfile3

# Not sure if this is totally needed
WORKDIR /usr/lib/vice
COPY source/vice-3.7/data/ .

# COPY configs/retroarch.cfg /cfg/retroarch.cfg
COPY --chmod=777 scripts/startup.sh /opt/gow/startup-app.sh

ENV XDG_RUNTIME_DIR=/tmp/.X11-unix

ARG IMAGE_SOURCE