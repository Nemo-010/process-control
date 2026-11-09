#!/bin/sh
# Turns a fresh copy of the template into a new app, then deletes itself.
#
# Usage: scripts/bootstrap.sh <name> <display name> <description>
#   scripts/bootstrap.sh mdview "MD View" "A fast Markdown viewer"
#
# <name> is the crate, binary, repository and app id: lowercase letters,
# digits and dashes.
set -eu
cd "$(dirname "$0")/.."

if [ $# -ne 3 ]; then
    echo "usage: $0 <name> <display name> <description>" >&2
    exit 2
fi
name=$1
title=$2
description=$3

case $name in
'' | [!a-z]* | *[!a-z0-9-]* | *-)
    echo "name must be lowercase letters, digits and dashes: $name" >&2
    exit 2
    ;;
esac

case $title$description in
*[\&\<\>\"]*)
    echo "the display name and description cannot contain & < > or \"" >&2
    exit 2
    ;;
esac

escape() {
    printf '%s' "$1" | sed 's/[\\&|]/\\&/g'
}

script="
s|A lightweight native desktop app built with egui|$(escape "$description")|g
s|App Template|$(escape "$title")|g
s|app-template|$name|g
"

for file in Cargo.toml Cargo.lock README.md onelf.toml src/main.rs src/app.rs \
    packaging/linux/app.desktop packaging/macos/Info.plist docs/package.json \
    docs/bun.lock docs/index.md docs/.vitepress/app.ts docs/guide/*.md; do
    sed "$script" "$file" > "$file.tmp"
    mv "$file.tmp" "$file"
done

rm -- "$0"
echo "bootstrapped $name ($title)"
