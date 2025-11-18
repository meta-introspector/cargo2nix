#!/usr/bin/env bash
#git submodule foreach ~/nix/vendor/rust/cargo2nix/submodules/add.sh
pwd
set -x
if [ -f Cargo.lock ]; then
    git add Cargo.lock
fi

if [ -f Cargo.toml ]; then
    git add Cargo.toml
else
    git status
fi

grep -qxF "process-repo.log" .gitignore || echo "process-repo.log" >> .gitignore
git checkout -b feature/CRQ-016-nixify || echo branch there
git add .gitignore
git commit -m 'wip packaging for cargo2nix' -a -n
git remote -v >> ~/nix/vendor/rust/cargo2nix/submodules/log.txt
pwd >> ~/nix/vendor/rust/cargo2nix/submodules/log.txt
git push origin >> ~/nix/vendor/rust/cargo2nix/submodules/log.txt || echo failed push
