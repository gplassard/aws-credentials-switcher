#!/usr/bin/env bash
set -ex

if [ -n "$1" ]; then
    yarn install --frozen-lockfile
    # update the version
    yarn version --$1
    yarn projen
    # will update Cargo.lock
    cargo check

    version=$(npm pkg get version | sed -e "s/\"//g")
    git add -A && git commit -m "chore(release): release v$version"
    # generate a changelog for the tag message
    git tag v$version
else
    echo "warn: please provide a version bump (patch/minor/major)"
    exit 1
fi
