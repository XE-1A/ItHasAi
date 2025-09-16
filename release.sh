#!/usr/bin/env bash
set -euxo pipefail

cd $(dirname $0)
output=$(git status --porcelain) && [ -z "$output" ] # Check for clean workdir

VERSION=$(yq '.package.version' Cargo.toml)
git tag "$VERSION"
#git push --tags

./build.sh

butler login
butler push 'itHasAi.zip' 'XE-1A/it-has-ai:html' --userversion "$VERSION"

butler status XE-1A/it-has-ai