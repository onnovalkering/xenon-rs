#!/usr/bin/env sh
if [ `uname` = 'Darwin' ]; then
    OS='osx'
else
    OS='linux'
fi

PROTOC_VERSION="3.11.4"
PROTOC_RELEASE="protoc-$PROTOC_VERSION-$OS-x86_64.zip"

curl -OL "https://github.com/protocolbuffers/protobuf/releases/download/v$PROTOC_VERSION/$PROTOC_RELEASE"
sudo unzip -o $PROTOC_RELEASE -d /usr/local bin/protoc
sudo unzip -o $PROTOC_RELEASE -d /usr/local 'include/*'