#!/bin/bash

# Script to run all the `api-collection` assertions.

VARS="tests/api-collection/props/local"

hurl --test --variables-file "$VARS" --glob "tests/api-collection/*.hurl"
