#!/bin/bash

# Script to run all the `api-collection` assertions.

VARS="tests/api-collection/props/local"

#
# Test `Meta`

hurl --test --variables-file "$VARS" --glob "tests/api-collection/meta/*.hurl"

#
# Test `User`

# Only run `query` assertions.
files=(
	"users"
	"user"
	"create_user"
	"update_user"
	"delete_user"
)
for file in "${files[@]}"; do
	hurl --test --variables-file "$VARS" "tests/api-collection/user/${file}.hurl"
done
