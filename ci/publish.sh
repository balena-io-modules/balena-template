#!/usr/bin/env bash

set -e

if [ -z ${TRAVIS_TAG+x} ]; then
    # Do not publish if there's no TRAVIS_TAG set
    echo "TRAVIS_TAG not set, skipping"
    exit 0
fi

# Giovanni, following must be called only when:
#
#  * `branch == master`
#  * on a commit with bumped version

# repo.yml.type == rust-* (rust-crate, rust-crate-wasm)
cargo login "$CARGO_API_TOKEN"
cargo publish
