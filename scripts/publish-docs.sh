#!/usr/bin/bash

cargo doc --no-deps
mkdocs gh-deploy
git checkout gh-pages
cargo rustdoc -- --no-deps --no-defaults --passes "collapse-docs" --passes "unindent-comments"
cp -r target/doc .
git add doc
git commit --amend --no-edit
git push -f origin gh-pages
git checkout master
