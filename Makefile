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
CODEBUILD_SOURCE_VERSION ?= notset

# The directory path that AWS CodeBuild uses for the build (for
# example, /tmp/src123456789/src).
CODEBUILD_SRC_DIR ?= NotSet


AWS_ACCOUNT_ID = 043206986030
AWS_REGION = us-west-2
IMAGE_NAME = rust-toolchain
IMAGE_REPO = $(AWS_ACCOUNT_ID).dkr.ecr.$(AWS_REGION).amazonaws.com/$(IMAGE_NAME)
ECR_LOGIN := $(shell aws ecr get-login --region=$(AWS_REGION) 2>/dev/null || echo 'echo "NO AWSCLI INSTALLED!" && false')


BUILD_DATETIME := $(shell date -u +%FT%TZ)


VERSION ?= $(CODEBUILD_SOURCE_VERSION)
LOG_FORMAT ?= human
CARGO=PATH=/root/.cargo/bin:$(PATH) cargo

.PHONY: all toolchain build


all:
	exit 1


toolchain:
	apt-get update && apt-get install -y \
		cmake \
		curl \
		g++ \
		gcc \
		git \
		make \
		valgrind \
		oprofile \
		linux-tools-generic ;

	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2017-05-03


build:
	$(CARGO) build --message-format=$(LOG_FORMAT) -p rsnek


release:
	$(CARGO) build --message-format=$(LOG_FORMAT) --release -p rsnek


test:
	$(CARGO) test --message-format=$(LOG_FORMAT) --all


test-release:
	$(CARGO) test --release --message-format=$(LOG_FORMAT) --all


bench:
	$(CARGO) bench --message-format=$(LOG_FORMAT) -p rsnek*


# I do not expect there to be random memory leaks because Rust handles a lot of that.
# This is more of a curiosity and an experiment to see:
#  - If the cyclical ObjectRefs cause issues
#  - Detect any hot code areas not obvious by rust benching
#
RSNEK_BINARY=target/release/rsnek
VALGRIND_PYTHON_SRCFILE=rsnek/tests/test.py
VALGRIND_MEMCHECK_XMLFILE=target/release/valgrind.memcheck.xml

valgrind:
	-$(CARGO) install cargo-profiler
	printf "%s\n%s\n\n" "#![feature(alloc_system)]" "extern crate alloc_system;" > rsnek/maingrind.rs
	cat rsnek/src/main.rs >> rsnek/maingrind.rs
	mv rsnek/src/main.rs rsnek/src/main.rs.bak
	mv rsnek/maingrind.rs rsnek/src/main.rs
	cd rsnek; \
		cargo profiler callgrind --release ; \
		cargo profiler cachegrind --release  -- $(VALGRIND_PYTHON_SRCFILE)
	mv rsnek/src/main.rs.bak rsnek/src/main.rs
	valgrind \
		--tool=memcheck \
		--leak-check=full \
		--show-leak-kinds=all \
		--verbose \
		--xml=yes \
		--xml-file=$(VALGRIND_MEMCHECK_XMLFILE) \
		--track-fds=yes -v $(RSNEK_BINARY) $(VALGRIND_PYTHON_SRCFILE)
	cat $(VALGRIND_MEMCHECK_XMLFILE)


OPROF_OUTDIR=target/oprofile_data
oprofile:
	mkdir -p $(OPROF_OUTDIR)
	operf -d $(OPROF_OUTDIR) $(RSNEK_BINARY) $(VALGRIND_PYTHON_SRCFILE)
	opreport --session-dir $(OPROF_OUTDIR) --details --verbose=stats


perf:
	perf stat -r 25 -ddd $(RSNEK_BINARY) $(VALGRIND_PYTHON_SRCFILE)
	perf stat -r 25 -ddd python -B $(VALGRIND_PYTHON_SRCFILE)


# Get the status of the stages of the AWS CodePipeline for this project and
# print the status of each stage and url to stdout.
#
pipeline-status:
	./tools/aws-cli-sugar/pipeline-status.sh

#
docs-%:
	cargo rustdoc --lib -p $* -- \
	    --no-defaults \
	    --passes strip-hidden \
	    --passes collapse-docs \
	    --passes unindent-comments \
	    --passes strip-priv-imports


# Generate the docs for all of dependencies and libraries Note that
# cargo doc --all filters out private modules in in the generated
# documentation, that is why there is a second pass using rustdoc
# directly to ensure that the private modules are present in the
# documentation.
docs:
	cargo doc --all
	$(MAKE) docs-rsnek_compile
	$(MAKE) docs-rsnek_runtime


clean:
	find . -type d -name "__pycache__" -exec rm -rf {} \;
	find . -type f -name "*.py.compiled" -exec rm -f {} \;
	find . -type f -name "*.pyc" -exec rm -f {} \;
	rm -rf target
