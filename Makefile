SHELL := /bin/bash
#############################################
# AWS Code Build Environment Variables
#############################################

# The AWS region where the build is running (for example,
# us-east-1). This environment variable is used primarily by the AWS
# CLI.
AWS_DEFAULT_REGION ?= NotSet

 # The AWS region where the build is running (for example,
 # us-east-1). This environment variable is used primarily by the AWS
 # SDKs.
AWS_REGION ?= NotSet

# The Amazon Resource Name (ARN) of the build (for example,
# arn:aws:codebuild:region-ID:account-ID:build/codebuild-demo-project:b1e6661e-e4f2-4156-9ab9-82a19EXAMPLE).
CODEBUILD_BUILD_ARN ?= NotSet

 # The AWS CodeBuild ID of the build (for example,
 # codebuild-demo-project:b1e6661e-e4f2-4156-9ab9-82a19EXAMPLE).
CODEBUILD_BUILD_ID ?= NotSet

# The AWS CodeBuild build image identifier (for example,
# aws/codebuild/java:openjdk-8).
CODEBUILD_BUILD_IMAGE ?= NotSet

# The entity that started the build. If AWS CodePipeline started the
# build, this is the pipeline's name, for example
# codepipeline/my-demo-pipeline. If an IAM user started the build,
# this is the user's name, for example MyUserName. If the Jenkins
# plugin for AWS CodeBuild started the build, this is the string
# CodeBuild-Jenkins-Plugin.
CODEBUILD_INITIATOR ?= NotSet

# The identifier of the AWS KMS key that AWS CodeBuild is using to
# encrypt the build output artifact (for example,
# arn:aws:kms:region-ID:account-ID:key/key-ID or alias/key-alias).
CODEBUILD_KMS_KEY_ID ?= NotSet

# The URL to the input artifact or source code repository. For Amazon
# S3, this is s3:// followed by the bucket name and path to the input
# artifact. For AWS CodeCommit and GitHub, this is the repository's
# clone URL.
CODEBUILD_SOURCE_REPO_URL ?= NotSet

# For Amazon S3, the version ID associated with the input
# artifact. For AWS CodeCommit, the commit ID or branch name associated
# with the version of the source code to be built. For GitHub, the
# commit ID, branch name, or tag name associated with the version of the
# source code to be built.
CODEBUILD_SOURCE_VERSION ?= "$(shell git rev-parse HEAD | head -c7)"

# The directory path that AWS CodeBuild uses for the build (for
# example, /tmp/src123456789/src).
CODEBUILD_SRC_DIR ?= NotSet


AWS_ACCOUNT_ID = 043206986030
#IMAGE_NAME = rust-toolchain
#IMAGE_REPO = $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(IMAGE_NAME)
#ECR_LOGIN := $(shell aws ecr get-login --region=$(AWS_REGION) 2>/dev/null || echo 'echo "NO AWSCLI INSTALLED!" && false')

BUILD_DATETIME := $(shell date -u +%FT%TZ)
VERSION ?= $(CODEBUILD_SOURCE_VERSION)
LOG_FORMAT ?= human
ARTIFACTS_DIR=target

# When building in CODEBUILD and running on EC2 special packages
# are needed to run things like oprofile and perf.
#
ifeq ($(CODEBUILD_BUILD_ID), NotSet)
CONDITIONAL_REQUIREMENTS=
CARGO_ARGS=--color=always --message-format=human
CARGO=cargo
LOG_SUFFIX=local.$(CODEBUILD_SOURCE_VERSION).$(BUILD_DATETIME)
else
CARGO=PATH=/root/.cargo/bin:$(PATH) cargo
CARGO_ARGS=--message-format=json
CONDITIONAL_REQUIREMENTS=ec2-requirements
LOG_SUFFIX=$(CODEBUILD_BUILD_ID).$(BUILD_DATETIME)
endif



.PHONY: all toolchain build release test \
	test-release bench perf docs clean \
	pipeline-status sysinfo lshw lscpu


all:
	exit 1


# Run the steps for buildspec.yml locally with the exception of toolchain
codebuild-local: | clean $(ARTIFACTS_DIR) sysinfo build test release test-release bench perf docs


ec2-requirements:
	-apt-get update && apt-get install -y \
		linux-headers-aws \
		linux-tools-aws \
		linux-cloud-tools-4.4.0-1016-aws


$(ARTIFACTS_DIR):
	-mkdir -p $@


toolchain: $(CONDITIONAL_REQUIREMENTS) $(ARTIFACTS_DIR)
	apt-get update && apt-get install -y \
		cmake \
		curl \
		g++ \
		gcc \
		git \
		make \
		valgrind \
		oprofile \
		lshw \
		linux-tools-generic

	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly


build: $(ARTIFACTS_DIR)
	$(CARGO) build $(CARGO_ARGS) -p rsnek 2>&1 | tee -a $(ARTIFACTS_DIR)/$@.$(LOG_SUFFIX).txt


release: $(ARTIFACTS_DIR)
	$(CARGO) build $(CARGO_ARGS)  --release -p rsnek 2>&1 | tee -a $(ARTIFACTS_DIR)/$@.$(LOG_SUFFIX).txt


test: $(ARTIFACTS_DIR)
	$(CARGO) test $(CARGO_ARGS) --all 2>&1 2>&1 | tee -a $(ARTIFACTS_DIR)/$@.$(LOG_SUFFIX).txt


test-release: $(ARTIFACTS_DIR)
	$(CARGO) test $(CARGO_ARGS) --release --all 2>&1 | tee -a $(ARTIFACTS_DIR)/$@.$(LOG_SUFFIX).txt



bench: bench-python_ast bench-rsnek


bench-%: $(ARTIFACTS_DIR)
	-$(CARGO) bench $(CARGO_ARGS) -p $* 2>&1 | tee -a $(ARTIFACTS_DIR)/$@.$(LOG_SUFFIX).txt
	#-$(CARGO) bench --all-features $(CARGO_ARGS) -p $* 2>&1 | tee -a $(ARTIFACTS_DIR)/$@.all-features.$(LOG_SUFFIX).txt


sysinfo: lshw lscpu


lshw: $(ARTIFACTS_DIR)
	-lshw -sanitize -xml 2>&1 | tee -a $(ARTIFACTS_DIR)/$@.$(LOG_SUFFIX).xml


lscpu: $(ARTIFACTS_DIR)
	-lscpu 2>&1 | tee -a $(ARTIFACTS_DIR)/$@.$(LOG_SUFFIX).txt

# I do not expect there to be random memory leaks because Rust handles a lot of that.
# This is more of a curiosity and an experiment to see:
#  - If the cyclical ObjectRefs cause issues
#  - Detect any hot code areas not obvious by rust benching
#
RSNEK_BINARY=$(ARTIFACTS_DIR)/release/rsnek
VALGRIND_PYTHON_SRCFILE=rsnek/tests/test.py
VALGRIND_MEMCHECK_XMLFILE=$(ARTIFACTS_DIR)/valgrind.memcheck.$(LOG_SUFFIX).xml
VALGRIND_CACHEGRIND_FILE=$(ARTIFACTS_DIR)/valgrind.cachegrind.$(LOG_SUFFIX).txt
OPROF_OUTDIR=$(ARTIFACTS_DIR)/oprofile_data.$(LOG_SUFFIX)
PERF_STATS_FILE=$(ARTIFACTS_DIR)/perf.stats.$(LOG_SUFFIX).txt

perf: $(ARTIFACTS_DIR)
	printf "%s\n%s\n\n" "#![feature(alloc_system)]" "extern crate alloc_system;" > rsnek/maingrind.rs
	cat rsnek/src/main.rs >> rsnek/maingrind.rs
	mv rsnek/src/main.rs rsnek/src/main.rs.bak
	mv rsnek/maingrind.rs rsnek/src/main.rs
	$(CARGO) rustc -p rsnek --release -- -g
	mv rsnek/src/main.rs.bak rsnek/src/main.rs
	-valgrind \
		--tool=memcheck \
		--leak-check=full \
		--show-leak-kinds=all \
		--verbose \
		--xml=yes \
		--xml-file=$(VALGRIND_MEMCHECK_XMLFILE) \
		--track-fds=yes -v $(RSNEK_BINARY) $(VALGRIND_PYTHON_SRCFILE)
	-cat $(VALGRIND_MEMCHECK_XMLFILE)
	-valgrind \
		--tool=cachegrind \
		--branch-sim=yes \
		--cachegrind-out-file=$(VALGRIND_CACHEGRIND_FILE) \
		-v $(RSNEK_BINARY) \
		$(VALGRIND_PYTHON_SRCFILE)
	-cat $(VALGRIND_CACHEGRIND_FILE)
	-mkdir -p $(OPROF_OUTDIR)
	-operf -d $(OPROF_OUTDIR) $(RSNEK_BINARY) $(VALGRIND_PYTHON_SRCFILE)
	-opreport --session-dir $(OPROF_OUTDIR) --details --verbose=stats

	-perf stat -r 25 -ddd $(RSNEK_BINARY) $(VALGRIND_PYTHON_SRCFILE) 2>&1 | tee -a $(PERF_STATS_FILE)
	-perf stat -r 25 -ddd python -B $(VALGRIND_PYTHON_SRCFILE) 2>&1 | tee -a $(PERF_STATS_FILE)



# Get the status of the stages of the AWS CodePipeline for this project and
# print the status of each stage and url to stdout.
#
pipeline-status:
	./tools/aws-cli-sugar/pipeline-status.sh


# Generate the docs for all of dependencies and libraries Note that
# cargo doc --all filters out private modules in in the generated
# documentation, that is why there is a second pass using rustdoc
# directly to ensure that the private modules are present in the
# documentation.
docs:
	$(CARGO) doc --all
	$(MAKE) docs-python_ast
	$(MAKE) docs-rsnek


docs-%: $(ARTIFACTS_DIR)
	$(CARGO) rustdoc --lib -p $* -- \
            --no-defaults \
	    --passes strip-hidden \
	    --passes collapse-docs \
	    --passes unindent-comments \
	    --passes strip-priv-imports


clean:
	find . -type d -name "__pycache__" -exec rm -rf {} \;
	find . -type f -name "*.py.compiled" -exec rm -f {} \;
	find . -type f -name "*.pyc" -exec rm -f {} \;
	rm -rf target
