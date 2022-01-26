#!/bin/bash

if [ "env.sh" = "$(basename $0)" ]
then
    echo "usage:"
    echo "$ source hack/env.sh"
    echo "$ source hack/env.sh clean"
    exit 1
fi

CMD() { echo -n "💬 $* ... "; "$@"; echo -e "\\r✅ $*"; }

if [ "$1" = "clean" ]
then
    CMD unset RUST_LOG
    CMD unset RUST_BACKTRACE
    CMD unset S3D_ENDPOINT
    CMD unalias s3d
    CMD unalias s3api
    CMD unalias s3
else
    CMD export RUST_LOG=info,s3d:trace
    CMD export RUST_BACKTRACE=1
    CMD export S3D_ENDPOINT="http://localhost:33333"
    CMD alias s3d='cargo -q run --'
    CMD alias s3api='aws --endpoint ${S3D_ENDPOINT} s3api'
    CMD alias s3='aws --endpoint ${S3D_ENDPOINT} s3'
fi
echo
echo "RUST_LOG=$RUST_LOG"
echo "RUST_BACKTRACE=$RUST_BACKTRACE"
echo "S3D_ENDPOINT=$S3D_ENDPOINT"
alias s3d s3api s3
