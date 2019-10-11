APP_NAME=$(shell cat Cargo.toml | grep -Po "(?<=name\s=\s\")[a-zA-Z0-9-_]+")
TARGET_FOLDER=build
TARGET_BUILD=${TARGET_FOLDER}/${APP_NAME}
COMMIT_NO=$(shell git rev-list --all --count)

ARGS_SRC=config/args.yaml
ARGS_TRG=.argsprod.yaml

all: make_args copy_config_files do_test do_build
args: copy_config_files make_args
build: make_args copy_config_files do_build
test: copy_config_files do_test

copy_config_files:
	echo nothing to see here
	# mkdir -p ${TARGET_FOLDER}
	# cp config/${APP_NAME}.yaml ${TARGET_FOLDER}/
	# mkdir -p target/debug
	# cp config/${APP_NAME}.yaml target/debug/

make_args:
	cat "${ARGS_SRC}" | sed '/version/s/\.X\"/\.${COMMIT_NO}\"/g' > ${ARGS_TRG}

do_benchmark:
	hyperfine "${TARGET_BUILD} -o ${TEMPFILE} testdata/set1/without-toc.md"
	rm -f ${TEMPFILE}

do_build:
	cargo build --target x86_64-unknown-linux-musl --release
	mkdir -p ${TARGET_FOLDER}
	mv "target/x86_64-unknown-linux-musl/release/${APP_NAME}" "${TARGET_BUILD}"
	strip "${TARGET_BUILD}"

do_test:
	cargo test
