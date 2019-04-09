#!/usr/bin/env bash

set -o errexit
set -o pipefail

if [ ! -d dataset ]; then
    git clone git@github.com:hawkingrei/image_test_dataset.git dataset
fi