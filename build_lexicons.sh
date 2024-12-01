#!/bin/bash -e

# Temporary script to build standard Lexicon JSON files from the YAML files we use
# TODO: Should be replaced by a build step of some sort

REPO_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]})" )" &> /dev/null && pwd)

YAML_FILES="$(find "${REPO_DIR}/lexicons" -name "*.yaml" -print)"

while read -r yaml_file; do
    echo "${yaml_file}"
    yq . "${yaml_file}" > "${yaml_file%.yaml}.json"
done <<< "${YAML_FILES}"
