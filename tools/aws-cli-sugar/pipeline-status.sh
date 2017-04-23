#!/bin/bash

export AWS_DEFAULT_PROFILE=rsnek
AWS_REGION=us-east-1

aws codepipeline --region=$AWS_REGION list-pipelines | \
    jq .pipelines[].name | \
    xargs aws --region=$AWS_REGION codepipeline get-pipeline-state --name  | \
    jq -r '.stageStates[].actionStates | to_entries[] | [.value.actionName, .value.latestExecution.status, "\nurl:", .value.latestExecution.externalExecutionUrl, "\n"] | join(" ")'
