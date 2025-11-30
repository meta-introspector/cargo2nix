#!/usr/bin/env bash
#git submodule foreach ~/nix/vendor/rust/cargo2nix/submodules/update.sh
#pwd
#set -x
if [ -f Cargo.lock ]; then
    git add Cargo.lock
fi
if [ -f flake.nix ]; then
    git add flake.nix
fi
if [ -f flake.lock ]; then
    git add flake.lock
fi
if [ -f Cargo.toml ]; then
    git add Cargo.toml
else
    git status
fi
#grep -qxF "process-repo.log" .gitignore || echo "process-repo.log" >> .gitignore
#git checkout -b feature/CRQ-016-nixify-workflow || echo branch there
#git add .gitignore
git pull --rebase origin main
git commit -m 'feat: Add/update Nix flake for submodule (CRQ-016)' -a -n
#git remote -v >> ~/nix/vendor/rust/cargo2nix/submodules/log.txt
#pwd >> ~/nix/vendor/rust/cargo2nix/submodules/log.txt
REMOTE_URL=$(git remote get-url origin)
echo "The remote URL for this submodule is: $REMOTE_URL"
echo "Remote URL: $REMOTE_URL"

if [[ "$REMOTE_URL" == *"github.com/meta-introspector/"* ]]; then
    echo "Remote is from github.com/meta-introspector/. Attempting push."
    git push origin HEAD >> /mnt/data1/nix/vendor/rust/cargo2nix/submodules/log.txt || echo "Failed push for $REMOTE_URL"
else
    echo "Remote is NOT from github.com/meta-introspector/. Skipping push."
fi
