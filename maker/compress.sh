#!/bin/bash

target_folder="${1}"

arr=($(
    find "${target_folder}" -mindepth 1 -type f -executable | sort
))

for bin in "${arr[@]}"; do
    echo "Compress \"${bin}\""
    upx -qqq --lzma --best --overlay=strip "${bin}"
done
echo ""
