# aws-credentials-switcher

## Purpose

Switch AWS credentials files in order to keep a valid format for both V1 and V2 credentials as supported by the AWS Java SDK.

Expects to have the following files

* `~/.aws/credentials`
* `~/.aws.v1/credentials`
* `~/.aws.v2/credentials`

The commands will
* read the current credentials from the `~/.aws/credentials` file
* delete the `~/.aws` directory
* copy the `~/.aws.v1` or `~/.aws.v2` to `~/.aws`
* update the access key / secret key in the new `~/.aws/credentials` file

## Running sources

`cargo run use-v1`

## Installation

`cargo install --path .`

## Commands

* `aws-credentials-switcher help`
* `aws-credentials-switcher use-v1`
* `aws-credentials-switcher use-v2`

## Release

* Update `Cargo.toml` / `Cargo.lock`
* `git commit -m 'Release(v1.2.0)'`
* `git tag v1.2.0`
* `git push && git push --tags`
