CURDIR=$(shell pwd)
APP_NAME=$(shell cat Cargo.toml | grep -Po "(?<=name\s=\s\")[a-zA-Z0-9-_]+")
TARGET_FOLDER=build
TARGET_BUILD=${TARGET_FOLDER}/${APP_NAME}
COMMIT_NO=$(shell git rev-list --all --count)

ARGS_SRC=config/args.yaml
ARGS_TRG=.argsprod.yaml


all: copy_args def_test def_build run_compression
build: copy_args def_build run_compression
args: copy_args


# surplus
copy_config_files:
	mkdir -p ${TARGET_FOLDER}
	cp config/${APP_NAME}.yaml ${TARGET_FOLDER}/
	mkdir -p target/debug
	cp config/${APP_NAME}.yaml target/debug/

copy_args:
	cat "${ARGS_SRC}" | sed '/version/s/\.X\"/\.${COMMIT_NO}\"/g' > ${ARGS_TRG}

# default functions
def_test:
	cargo test

def_build:
	maker/build.sh \
		"${APP_NAME}" \
		"${TARGET_FOLDER}" \

run_compression:
	maker/compress.sh "${TARGET_FOLDER}"
