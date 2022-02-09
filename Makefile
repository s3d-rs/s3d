TIMER := time
IMAGE := s3d:dev
IMAGE_BUILDER := docker
CODEGEN_CRATES_DIR := smithy-rs/s3d/build/crates
CODEGEN_SERVER_S3 := $(CODEGEN_CRATES_DIR)/s3d-smithy-codegen-server-s3
CARGO_BUILD_FLAGS += -v
ifdef RELEASE
	CARGO_BUILD_FLAGS += --release
endif

#------------------------#
# build - default target #
#------------------------#

build: codegen_if_missing
	@echo "\nMakefile: 👷 running build ... \n"
	$(TIMER) cargo build $(CARGO_BUILD_FLAGS)
	@echo "\nMakefile: ✅ build done \n"
.PHONY: build

#-----------------#
# all - CI target #
#-----------------#

all: codegen build test
	@echo "\nMakefile: ✅ all done \n"
.PHONY: all

#----------------------#
# test - CI validation #
#----------------------#

test:
	@echo "\nMakefile: 👷 running test ... \n"
	$(TIMER) cargo test
	@echo "\nMakefile: ✅ test done \n"
.PHONY: test

#-----------------------------------------------------------------------#
## codegen_if_missing - depend on output dir to trigger only if missing #
#-----------------------------------------------------------------------#

codegen_if_missing: | $(CODEGEN_SERVER_S3)
.PHONY: codegen_if_missing

$(CODEGEN_SERVER_S3):
	@echo "\nMakefile: 👷 calling make codegen ... \n"
	$(TIMER) $(MAKE) codegen

#-----------------------------------#
# codegen - generate with smithy-rs #
#-----------------------------------#

codegen:
	@echo "\nMakefile: 👷 running codegen ... \n"
	git submodule status
	git submodule update --init
	cd smithy-rs && $(TIMER) ./gradlew \
		:rust-runtime:assemble \
		:aws:sdk:assemble \
		:s3d:assemble \
		-Paws.services=+sts,+sso,+s3 \
		-Ps3d.release=false
	@## moving all crates to crates dir
	rm -rf $(CODEGEN_CRATES_DIR)
	mkdir -p $(CODEGEN_CRATES_DIR)
	mv smithy-rs/rust-runtime/build/smithy-rs/rust-runtime/* $(CODEGEN_CRATES_DIR)/
	mv smithy-rs/aws/sdk/build/aws-sdk/sdk/{aws-config,aws-endpoint,aws-http,aws-sig-auth,aws-sigv4,aws-types,s3,sso,sts} $(CODEGEN_CRATES_DIR)/
	mv smithy-rs/s3d/build/smithyprojections/s3d/s3/rust-server-codegen $(CODEGEN_SERVER_S3)
	cd $(CODEGEN_SERVER_S3) && $(TIMER) cargo build
	cd $(CODEGEN_SERVER_S3) && $(TIMER) cargo test
	@echo "\nMakefile: ✅ codegen done \n"
.PHONY: codegen

#---------------------#
# clean - start fresh #
#---------------------#

clean:
	@echo "\nMakefile: 👷 running clean ... \n"
	cd smithy-rs && $(TIMER) ./gradlew clean
	$(TIMER) cargo clean
	@echo "\nMakefile: ✅ clean done \n"
.PHONY: clean

#------------#
# help - ??? #
#------------#

help:
	@echo ""
	@echo "Makefile targets:"
	@echo ""
	@echo "  build    - (default) cargo build"
	@echo "  all      - codegen + build + test"
	@echo "  build    - cargo build"
	@echo "  test     - cargo test"
	@echo "  codegen  - builds $(CODEGEN_CRATES_DIR)"
	@echo "  clean    - clean the build"
	@echo "  help     - show this help"
	@echo ""
.PHONY: help

#---------------------------------#
# env - output shell env commands #
#---------------------------------#

env:
	@echo "export S3D_ENDPOINT='http://localhost:33333';"
	@echo "alias s3d=\"\$$PWD/target/debug/s3d\";"
	@echo "alias s3='aws --endpoint \$$S3D_ENDPOINT s3';"
	@echo "alias s3api='aws --endpoint \$$S3D_ENDPOINT s3api';"
	@echo "# usage: eval \$$(make env)"

image:
	$(TIMER) $(IMAGE_BUILDER) build . -t $(IMAGE)
.PHONY: image
