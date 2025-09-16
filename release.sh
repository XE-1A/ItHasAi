#!/usr/bin/env bash
set -euxo pipefail

cd $(dirname $0)
git status --porcelain

VERSION=$(yq '.package.version' Cargo.toml)
git tag "$VERSION"
#git push --tags

butler login
butler push 'itHasAi.zip' 'XE-1A/it-has-ai:html' --userversion "$VERSION"

butler status XE-1A/it-has-ai