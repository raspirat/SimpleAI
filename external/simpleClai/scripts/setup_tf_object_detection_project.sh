#!/bin/bash
PROJECT_NAME=$1
THIS_PROFILE_PATH=$2
PROJECT_PATH="$THIS_PROFILE_PATH/projects/$PROJECT_NAME"
echo "Project path: $PROJECT_PATH"
mkdir -p $PROJECT_PATH
# still something todo here. (if not then this is obsolete)
