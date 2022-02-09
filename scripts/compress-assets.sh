#!/usr/bin/env bash

OS="$1"
TARGET="$2"
RELEASE_VERSION="$3"

if [ "$OS" = "windows-2022" ]; then
  7z a -tzip "tun-$RELEASE_VERSION-$TARGET.zip" tun-"$RELEASE_VERSION"/
else
  tar -czvf tun-"$RELEASE_VERSION"-"$TARGET".tar.gz tun-"$RELEASE_VERSION"/
  shasum -a 512 tun-"$RELEASE_VERSION"-"$TARGET".tar.gz >tun-"$RELEASE_VERSION"-"$TARGET".tar.gz.sha512
fi
