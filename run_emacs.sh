#!/bin/bash

echo "Entering first nix develop shell (from dev1.sh) and then nesting another nix develop to launch Emacs..."
nix develop ~/pick-up-nix2/ --command bash -c "nix develop --command emacs"

echo "Emacs session finished."
