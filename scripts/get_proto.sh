#!/bin/bash
set -o errexit -o nounset -o pipefail

SCRIPT_PATH="$(
	cd -- "$(dirname "$0")" >/dev/null 2>&1
	pwd -P
)"

DEST_DIR="${SCRIPT_PATH}/../proto"

rm -rf "${SCRIPT_PATH}/SecretNetwork"
git clone --depth 1 --filter=blob:none --sparse --branch v1.12.1 https://github.com/scrtlabs/SecretNetwork "${SCRIPT_PATH}/SecretNetwork"

cd "${SCRIPT_PATH}/SecretNetwork"

rm -rf "$DEST_DIR"
mkdir -p "$DEST_DIR"

SECRET_DIR="proto"
git sparse-checkout set "$SECRET_DIR"
cp -R "${SECRET_DIR}"/* "${DEST_DIR}"

# SECRET_THIRD_PARTY_DIR="third_party/proto"
# git sparse-checkout set "$SECRET_THIRD_PARTY_DIR"
# cp -R "${SECRET_THIRD_PARTY_DIR}"/* "${DEST_DIR}"

cd "${SCRIPT_PATH}"
rm -rf "${SCRIPT_PATH}/SecretNetwork"
