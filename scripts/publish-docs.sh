#!/usr/bin/bash

cargo rustdoc -- --no-defaults --passes "collapse-docs" --passes "unindent-comments"
mkdocs gh-deploy

git checkout gh-pages
cp -r target/doc .
git add doc
git commit --amend --no-edit
git push -f origin gh-pages
git checkout master
