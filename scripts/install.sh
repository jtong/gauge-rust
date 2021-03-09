#!/bin/sh

set -e

PWD=`pwd`
BUILD_DIR="$PWD/build"
DEPLOY_RELATIVE_DIR="deploy"
DEPLOY_DIR="$PWD/$DEPLOY_RELATIVE_DIR"

sh ./scripts/package.sh

VERSION=`$BUILD_DIR/bin/gauge_rs --version`
ARCH=`uname -m`
PLATFORM=$(echo `uname` | tr '[:upper:]' '[:lower:]')

OUTPUT_FILE="gauge-rs-$VERSION-$PLATFORM.$ARCH.zip"

echo "\nUninstalling:"

gauge uninstall rust

echo "\nInstalling:"

gauge install rust --file "$DEPLOY_DIR/$OUTPUT_FILE"
