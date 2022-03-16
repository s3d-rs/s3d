#!/bin/bash

# s3d should be running on localhost:33333 and then run this script.

EP="http://localhost:33333"
BKT="${1:-s3d-test-bucket}"

function LOG() {
    { echo -e "\n----------> sanity: $@\n"; } 2>/dev/null
}

function S3C() {
    LOG "🚀 s3c $@"
    eval "./target/debug/s3c $@"
}

function CURL() {
    local rc
    LOG "🚀 curl $@"
    curl -s -i "${EP}$@"
    rc="$?"
    if [ $rc -ne 0 ]; then
        LOG "Error\n\nError: CURL failed\nReturnCode: $rc\nCommand: curl -s -i ${EP}$@"
        exit 1
    fi
}

function AWSCLI() {
    LOG "🚀 aws $@"
    aws --endpoint $EP "$@"
}

function test_s3c() {
    LOG "▶️ test_s3c ..."
    S3C ls
    S3C ls $BKT
    S3C put $BKT/README.md "<README.md"
    sleep 10
    S3C get $BKT/README.md ">/dev/null"
    S3C ls $BKT
    LOG "✅ test_s3c done"
}

function test_curl_client() {
    LOG "▶️ test_curl_client ..."
    CURL /                                            # ListBuckets
    CURL /$BKT -X PUT                                 # CreateBucket
    CURL /                                            # ListBuckets
    CURL /$BKT -I                                     # HeadBucket
    CURL /$BKT -X GET                                 # ListObjects
    CURL /$BKT/README.md -X PUT -d @README.md         # PutObject
    CURL /$BKT/README.md -I                           # HeadObject
    CURL /$BKT/README.md -X GET                       # GetObject
    CURL /$BKT/README.md -X DELETE                    # DeleteObject
    CURL /$BKT -X DELETE                              # DeleteBucket
    CURL /                                            # ListBuckets
    LOG "✅ test_curl_client done"
}

function test_awscli_s3() {
    LOG "▶️ test_awscli_s3 ..."
    AWSCLI s3 ls
    AWSCLI s3 ls s3://$BKT
    AWSCLI s3 cp README.md s3://$BKT/README.md
    AWSCLI s3 cp s3://$BKT/README.md -
    AWSCLI s3 rm s3://$BKT/README.md
    AWSCLI s3 rb s3://$BKT
    AWSCLI s3 ls
    LOG "✅ test_awscli_s3 done"
}

function test_awscli_s3api() {
    LOG "▶️ test_awscli_s3api ..."
    AWSCLI s3api list-buckets
    AWSCLI s3api list-objects --bucket $BKT
    AWSCLI s3api put-object --bucket $BKT --key README.md --body README.md
    AWSCLI s3api get-object --bucket $BKT --key README.md /dev/null
    AWSCLI s3api delete-object --bucket $BKT --key README.md
    AWSCLI s3api list-objects --bucket $BKT
    LOG "✅ test_awscli_s3api done"
}


test_s3c
#test_curl_client
#test_awscli_s3
#test_awscli_s3api
