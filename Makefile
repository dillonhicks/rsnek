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
		make ;

	curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly


build:
	PATH="/root/.cargo/bin:$(PATH)" cargo build --message-format=json -p rsnek


release:
	PATH="/root/.cargo/bin:$(PATH)" cargo build --message-format=json --release -p rsnek


test:
	PATH="/root/.cargo/bin:$(PATH)" cargo test --message-format=json --all


bench:
	PATH="/root/.cargo/bin:$(PATH)" cargo bench --message-format=json -p rsnek*


# Get the status of the stages of the AWS CodePipeline for this project and
# print the status of each stage and url to stdout.
#
pipeline-status:
	./tools/aws-cli-sugar/pipeline-status.sh
