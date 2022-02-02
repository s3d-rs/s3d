#!/bin/bash

# s3d should be running on localhost:33333 and then run this script.

EP="http://localhost:33333"
BKT="${1:-s3d-test-bucket}"

function LOG() {
    { echo -e "\n----------> sanity: $@\n"; } 2>/dev/null
}

function S3DCLI() {
    LOG "🚀 s3d $@"
    eval "./target/debug/s3d $@"
}

function S3CURL() {
    local rc
    LOG "🚀 curl $@"
    curl -s -i "${EP}$@"
    rc="$?"
    if [ $rc -ne 0 ]; then
        LOG "Error\n\nError: S3CURL failed\nReturnCode: $rc\nCommand: curl -s -i ${EP}$@"
        exit 1
    fi
}

function S3CLI() {
    LOG "🚀 aws s3 $@"
    aws --endpoint $EP s3 "$@"
}

function S3API() {
    LOG "🚀 aws s3api $@"
    aws --endpoint $EP s3api "$@"
}

function s3d_client_test() {
    LOG "▶️ s3d_client_test ..."
    S3DCLI ls
    S3DCLI ls $BKT
    S3DCLI put $BKT/README.md "<README.md"
    sleep 10
    S3DCLI get $BKT/README.md ">/dev/null"
    S3DCLI ls $BKT
    LOG "✅ s3d_client_test done"
}

function s3curl_test() {
    LOG "▶️ s3curl_test ..."
    S3CURL /                                            # ListBuckets
    S3CURL /$BKT -X PUT                                 # CreateBucket
    S3CURL /                                            # ListBuckets
    S3CURL /$BKT -I                                     # HeadBucket
    S3CURL /$BKT -X GET                                 # ListObjects
    S3CURL /$BKT/README.md -X PUT -d @README.md         # PutObject
    S3CURL /$BKT/README.md -I                           # HeadObject
    S3CURL /$BKT/README.md -X GET                       # GetObject
    S3CURL /$BKT/README.md -X DELETE                    # DeleteObject
    S3CURL /$BKT -X DELETE                              # DeleteBucket
    S3CURL /                                            # ListBuckets
    LOG "✅ s3curl_test done"
}

function s3cli_test() {
    LOG "▶️ s3cli_test ..."
    S3CLI ls
    S3CLI ls s3://$BKT
    S3CLI cp README.md s3://$BKT/README.md
    S3CLI cp s3://$BKT/README.md -
    S3CLI rm s3://$BKT/README.md
    S3CLI rb s3://$BKT
    S3CLI ls
    LOG "✅ s3cli_test done"
}

function s3api_test() {
    LOG "▶️ s3api_test ..."
    S3API list-buckets
    S3API list-objects --bucket $BKT
    S3API put-object --bucket $BKT --key README.md --body README.md
    S3API get-object --bucket $BKT --key README.md /dev/null
    S3API delete-object --bucket $BKT --key README.md
    S3API list-objects --bucket $BKT
    LOG "✅ s3api_test done"
}


#s3curl_test
#s3cli_test
#s3api_test
s3d_client_test
