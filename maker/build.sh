#!/bin/bash

scriptdir="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
basedir="$(echo "${scriptdir}" | grep -Po ".*(?=\/)")"

conf="${scriptdir}/targets.yaml"

APP_NAME="${1}"
TARGET_FOLDER="${2}"


# copy cargo linker config to the right place
cp -rp "${scriptdir}/cargo_config" "${basedir}/.cargo"


# prepare
rm -rf build/*
    
# process
while read line; do
    arch=$(echo "${line}" | grep -Poi "^[a-z0-9_]+")
    flag=$(echo "${line}" | grep -Poi "[a-z0-9-_]+$")

    tfol="${TARGET_FOLDER}/${arch}"
    tfil="${tfol}/${APP_NAME}"
    
	echo "Build \"${arch}/${APP_NAME}\""
    rustup target add ${flag}
    cargo build --target ${flag} --release
    mkdir -p "${tfol}"
    mv -f "target/${flag}/release/${APP_NAME}" "${tfil}"    
    file "${tfil}"
done < "${conf}"


# aftermath
cargo clean
