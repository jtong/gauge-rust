#!/bin/sh

set -e

PWD=`pwd`
BUILD_DIR="$PWD/build"
DEPLOY_RELATIVE_DIR="deploy"
DEPLOY_DIR="$PWD/$DEPLOY_RELATIVE_DIR"
EXAMPLE_DIR="$PWD/example"

sh ./scripts/install.sh

echo "\nExecute Specs:"

cd "$EXAMPLE_DIR"

gauge run specs
