#!/usr/bin/env sh
PROTOC_VERSION="3.11.2"
PROTOC_RELEASE="protoc-$PROTOC_VERSION-linux-x86_64.zip"

curl -OL "https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOC_VERSION/$PROTOC_RELEASE"
sudo unzip -o $PROTOC_RELEASE -d /usr/local bin/protoc
sudo unzip -o $PROTOC_RELEASE -d /usr/local 'include/*'