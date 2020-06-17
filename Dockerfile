FROM zilliqa/scilla:v0.7.0

RUN apt-get update \
    && apt-get install -y software-properties-common \
    && add-apt-repository ppa:tah83/secp256k1 -y \
    && apt-get update && apt-get install -y --no-install-recommends \
    autoconf \
    build-essential \
    ca-certificates \
    cmake \
    # curl is not a build dependency
    curl \
    git \
    golang \
    libboost-filesystem-dev \
    libboost-program-options-dev \
    libboost-system-dev \
    libboost-test-dev \
    libboost-python-dev \
    libcurl4-openssl-dev \
    libevent-dev \
    python3-dev \
    libjsoncpp-dev \
    libjsonrpccpp-dev \
    libleveldb-dev \
    libmicrohttpd-dev \
    libminiupnpc-dev \
    libsnappy-dev \
    libssl-dev \
    libtool \
    ocl-icd-opencl-dev \
    pkg-config \
    python \
    python-pip \
    python3-setuptools \
    python3-pip \
    libsecp256k1-dev \
    && rm -rf /var/lib/apt/lists/* \
    && pip install 'setuptools<45' \
    && pip install  request requests clint futures \
    && pip3 install requests clint future

ARG ZILLIQA_VERSION=master
ARG REPO=https://github.com/Zilliqa/Zilliqa.git
ARG SOURCE_DIR=/zilliqa
ARG BUILD_DIR=/zilliqa/build
ARG INSTALL_DIR=/usr/local
ARG BUILD_TYPE=RelWithDebInfo
ARG EXTRA_CMAKE_ARGS=

RUN git clone -b ${ZILLIQA_VERSION} --depth 1 ${REPO} ${SOURCE_DIR} \
    && cmake -H${SOURCE_DIR} -B${BUILD_DIR} -DCMAKE_BUILD_TYPE=${BUILD_TYPE} \
    -DCMAKE_INSTALL_PREFIX=${INSTALL_DIR} ${EXTRA_CMAKE_ARGS} \
    && cmake --build ${BUILD_DIR} -- -j$(nproc) \
    && cmake --build ${BUILD_DIR} --target install

WORKDIR ${SOURCE_DIR}

COPY boot.json ./boot.json
COPY constants.xml ./constants.xml
EXPOSE 5555

ENTRYPOINT ["/zilliqa/build/bin/isolatedServer", "-f", "boot.json", "&>> /zilliqa/logs/isolated-server.logs"]
