ARG BASE_APP_IMAGE

# hadolint ignore=DL3006
FROM ${BASE_APP_IMAGE}
ARG DEBIAN_FRONTEND=noninteractive

# Wine relies on 32-bit architecture being available
# RUN add-apt-repository -y ppa:graphics-drivers/ppa
RUN dpkg --add-architecture i386
RUN apt-get update
RUN apt-get install -y nvidia-driver-515 libvulkan1 libvulkan1:i386

# Get the latest wine.
# The following is based on gloriouseggroll's advice on how to get out of wine dependency hell
RUN apt-get install -y wget gnupg 
RUN wget -O /etc/apt/keyrings/winehq-archive.key https://dl.winehq.org/wine-builds/winehq.key
RUN wget -NP /etc/apt/sources.list.d/ "https://dl.winehq.org/wine-builds/ubuntu/dists/jammy/winehq-jammy.sources"
RUN apt-get update
RUN apt-get install -y --install-recommends winehq-staging
RUN apt-get install -y winetricks

USER ${UNAME}

# Copy over the wine-ge-custom binaries
WORKDIR /opt/gow/wine-ge-custom
COPY assets/wine-lutris-GE-Proton7-35-x86_64.tar.xz .
RUN tar -xvf wine-lutris-GE-Proton7-35-x86_64.tar.xz

RUN ls
COPY --chmod=777 scripts/startup.sh /opt/gow/startup-app.sh

ENV XDG_RUNTIME_DIR=/tmp/.X11-unix

ARG IMAGE_SOURCE
LABEL org.opencontainers.image.source $IMAGE_SOURCE