#!/usr/bin/env sh
if [ `uname` = 'Darwin' ]; then
    OS='osx'
else
    OS='linux'
fi

# Use same `protoc` version as Xenon
PROTOC_VERSION="3.7.1"
PROTOC_RELEASE="protoc-$PROTOC_VERSION-$OS-x86_64.zip"

curl -OL "https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOC_VERSION/$PROTOC_RELEASE"
sudo unzip -o $PROTOC_RELEASE -d /usr/local bin/protoc
sudo unzip -o $PROTOC_RELEASE -d /usr/local 'include/*'
