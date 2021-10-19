#!/bin/bash
set -e -x -o pipefail

# update the smithy-rs submodule and pull the latest changes

git submodule init
git submodule status
git submodule update --recursive --remote
git submodule status

set +x
echo ""
echo "---> submodule updated finished."
echo "---> next run: hack/smithy-build.sh"
echo ""
