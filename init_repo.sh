#!/bin/bash

# Exit when any command fails
set -e

# Get the name of the repository from git
REPO_NAME=$(basename `git rev-parse --show-toplevel`)

echo "Replacing project name in files with: \"$REPO_NAME\""

# Replace the name
git ls-files | xargs sed -i -e "s/replace_me/$REPO_NAME/g"

# Replace the README file
rm README.md
mv REPLACE_README.md README.md

# Remove this script
rm init_repo.sh

# Add everything to git
git commit -am "Setup Rust library from github-template"
