#!/bin/bash

# s3d should be running on localhost:33333 and then run this script.

export S3D_ENDPOINT='http://localhost:33333'

function LOG() {
    { echo -e "\n----------> $@\n"; } 2>/dev/null
}

function S3CURL() {
    local rc
    LOG "curl $@"
    curl -s -i "${S3D_ENDPOINT}$@"
    rc="$?"
    if [ $rc -ne 0 ]; then
        LOG "Error\n\nError: S3CURL failed\nReturnCode: $rc\nCommand: curl -s -i ${S3D_ENDPOINT}$@"
        exit 1
    fi
}

function S3CLI() {
    LOG "aws s3 $@"
    aws --endpoint ${S3D_ENDPOINT} s3 "$@"
}

function S3API() {
    LOG "aws s3api $@"
    aws --endpoint ${S3D_ENDPOINT} s3api "$@"
}

function s3curl_test() {
    LOG "s3curl_test"
    S3CURL /                                            # ListBuckets
    S3CURL /lala -X PUT                                 # CreateBucket
    S3CURL /lala -I                                     # HeadBucket
    S3CURL /                                            # ListBuckets
    S3CURL /lala -X GET                                 # ListObjects
    S3CURL /lala/README.md -X PUT -d @README.md         # PutObject
    S3CURL /lala/README.md -X GET                       # GetObject
    S3CURL /lala -X DELETE                              # DeleteObject
    S3CURL /                                            # ListBuckets
    LOG "s3curl_test - DONE"
}


function s3cli_test() {
    LOG "s3cli_test"
    S3CLI ls
    S3CLI mb s3://lala
    S3CLI ls
    S3CLI ls s3://lala
    S3CLI cp README.md s3://lala/README.md
    S3CLI cp s3://lala/README.md -
    S3CLI rb s3://lala
    S3CLI ls
    LOG "s3cli_test - DONE"
}

function s3api_test() {
    LOG "s3api_test"
    S3API list-buckets
    S3API list-objects --bucket sanity
    S3API put-object --bucket sanity --key test --body test
    S3API get-object --bucket sanity --key test /dev/null
    LOG "s3api_test - DONE"
}

s3curl_test
