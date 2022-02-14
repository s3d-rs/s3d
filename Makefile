LOG = @echo "\nMakefile: 🥷  $(1)\n"
LOG_START = @echo "\nMakefile: 🥷  $@ start ...\n"
LOG_DONE = @echo "\nMakefile: ✅ $@ done\n"
TIMER := time
IMAGE := s3d:dev
IMAGE_BUILDER := docker
CODEGEN_CRATES_DIR := smithy-rs/s3d/build/crates
CODEGEN_SERVER_S3 := $(CODEGEN_CRATES_DIR)/s3d-smithy-codegen-server-s3
CODEGEN_SDK_DIR := smithy-rs/aws/sdk/build/aws-sdk/sdk

MODE := debug
ifeq ($(RELEASE),1)
	MODE = release
	CARGO_BUILD_FLAGS += --release
endif
ifdef VERBOSE
	CARGO_BUILD_FLAGS += -v
endif

CARGO_BUILD_CMD := $(TIMER) cargo build $(CARGO_BUILD_FLAGS)
CARGO_TEST_CMD := $(TIMER) cargo test $(CARGO_BUILD_FLAGS) # using same flags as build for now


#------------------------#
# build - default target #
#------------------------#

build: codegen_init_once
	$(LOG_START)
	$(CARGO_BUILD_CMD)
	$(LOG_DONE)
.PHONY: build

#--------------------------#
# all - might take a while #
#--------------------------#

all: codegen build test
	$(LOG_DONE)
.PHONY: all

#-----------------------------------#
# codegen - generate with smithy-rs #
#-----------------------------------#

codegen: submodules_init_once
	$(LOG_START)
	cd smithy-rs && $(TIMER) ./gradlew \
		:rust-runtime:assemble \
		:aws:sdk:assemble \
		:s3d:assemble \
		-Paws.services=+sts,+sso,+s3 \
		-Ps3d.release=false
	@#####################################
	@## moving all crates to crates dir ##
	@#####################################
	rm -rf $(CODEGEN_CRATES_DIR)
	mkdir -p $(CODEGEN_CRATES_DIR)
	mv smithy-rs/rust-runtime/build/smithy-rs/rust-runtime/* $(CODEGEN_CRATES_DIR)/
	mv $(CODEGEN_SDK_DIR)/aws-config $(CODEGEN_CRATES_DIR)/
	mv $(CODEGEN_SDK_DIR)/aws-endpoint $(CODEGEN_CRATES_DIR)/
	mv $(CODEGEN_SDK_DIR)/aws-http $(CODEGEN_CRATES_DIR)/
	mv $(CODEGEN_SDK_DIR)/aws-sig-auth $(CODEGEN_CRATES_DIR)/
	mv $(CODEGEN_SDK_DIR)/aws-sigv4 $(CODEGEN_CRATES_DIR)/
	mv $(CODEGEN_SDK_DIR)/aws-types $(CODEGEN_CRATES_DIR)/
	mv $(CODEGEN_SDK_DIR)/s3 $(CODEGEN_CRATES_DIR)/
	mv $(CODEGEN_SDK_DIR)/sso $(CODEGEN_CRATES_DIR)/
	mv $(CODEGEN_SDK_DIR)/sts $(CODEGEN_CRATES_DIR)/
	mv smithy-rs/s3d/build/smithyprojections/s3d/s3/rust-server-codegen $(CODEGEN_SERVER_S3)
	$(LOG_DONE)
.PHONY: codegen

# CAUTION:
# submodules target should NOT be used if you are making changes directly
# on the smithy-rs submodule (which is useful for dual development),
# because it will effectively `git reset --hard` on the submodule HEAD
# and discard local commits and worktree changes. however, for most users
# this is desired as they would not change the submodule directly.

submodules:
	$(LOG_START)
	cd smithy-rs && git remote -v
	cd smithy-rs && git branch -avv
	git submodule status
	git submodule update --init
	git submodule status
	$(LOG_DONE)
.PHONY: submodules

# the "init_once" targets avoid rebuilding more than once when used as dep, 
# but we can still run the main targets unconditionally as needed.

codegen_init_once: | $(CODEGEN_SERVER_S3)
.PHONY: codegen_init_once

submodules_init_once: | smithy-rs/README.md
.PHONY: submodules_init_once

$(CODEGEN_SERVER_S3):
	$(call LOG,call make codegen)
	$(TIMER) $(MAKE) codegen

smithy-rs/README.md:
	$(call LOG,call make submodules)
	$(TIMER) $(MAKE) submodules

#-------------------#
# test - with cargo #
#-------------------#

test:
	$(LOG_START)
	@#### no tests yet... ####
	@# $(CARGO_TEST_CMD)
	@# cd $(CODEGEN_SERVER_S3) && $(CARGO_TEST_CMD)
	$(LOG_DONE)
.PHONY: test

#-------------------------------------#
# image - containerization buildation #
#-------------------------------------#

image:
	$(LOG_START)
	$(TIMER) $(IMAGE_BUILDER) build . -t $(IMAGE)
	$(LOG_DONE)
.PHONY: image

#---------------------#
# clean - start fresh #
#---------------------#

clean:
	$(LOG_START)
	cd smithy-rs && $(TIMER) ./gradlew clean
	$(TIMER) cargo clean
	$(LOG_DONE)
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
	@echo "  codegen  - builds $(CODEGEN_CRATES_DIR)"
	@echo "  test     - cargo test"
	@echo "  clean    - clean the build"
	@echo "  env      - echos dev envs and aliases"
	@echo "  help     - show this help"
	@echo ""
.PHONY: help

#---------------------------------#
# env - output shell env commands #
#---------------------------------#

env:
	@echo "export S3D_ENDPOINT='http://localhost:33333';"
	@echo "alias s3d=\"\$$PWD/target/$(MODE)/s3d\";"
	@echo "alias s3='aws --endpoint \$$S3D_ENDPOINT s3';"
	@echo "alias s3api='aws --endpoint \$$S3D_ENDPOINT s3api';"
	@echo "# usage: eval \$$(make env)"

