#!/bin/sh
set -e

rm -rf target/doc
cargo doc -p device_tree
gittar -b gh-pages file:target/doc/*
git push origin gh-pages
