#!/bin/bash
# Needs CURL to be installed

api_files=(
  "internal-api.yml"
  "app-request.schema.json"
  "view-response.schema.json"
  "manifest.schema.json"
)

function load_api_file() { #$1: api version, $2: api file 
  url="https://github.com/lenra-io/api/releases/download/v$1/$2"
  file="api/$2"
    echo "Loading $url"
    curl -Ls -o $file $url
}

function main() {
  # get the API version from the api-version.txt file
  api_version=$(cat api-version.txt)

  # create api dir if it doesn't exist
  mkdir -p api

  # get the API version from the API dir
  current_api_version=$(cat api/api-version.txt)

  # if the versions don't match, then we need to load the new API
  if [ "$api_version" != "$current_api_version" ]; then
    echo "Loading API version $api_version"
    
    # save the new API version
    echo $api_version > api/api-version.txt

    # load the new API files
    for file in "${api_files[@]}"; do
      load_api_file $api_version $file
    done
  fi
}

main