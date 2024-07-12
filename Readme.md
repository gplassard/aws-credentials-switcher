# aws-credentials-switcher

## Purpose

Switch AWS credentials files in order to keep different sets of credentials format for tools which have issues handling some formats (assum roles, sso, ...).

Expects to have the following files

* `~/.aws/credentials`
* `~/.aws.v1/credentials`
* `~/.aws.v2/credentials`
* `~/.aws.v3/credentials`

The commands will
* read the current credentials from the `~/.aws/credentials` file
* delete the `~/.aws` directory
* copy the `~/.aws.v1` or `~/.aws.v2` or `~/.aws.v3` to `~/.aws`
* update the access key / secret key in the new `~/.aws/credentials` file

## Running sources

`cargo run use-v1`

## Installation

`cargo install --path .`

## Commands

* `aws-credentials-switcher help`
* `aws-credentials-switcher use-v1`
* `aws-credentials-switcher use-v2`
* `aws-credentials-switcher use-v3`

## Release

* ./scripts/release.sh <fix/minor/major>
* git push --follow-tags
