#!/bin/bash

echo "Entering first nix develop shell (from dev1.sh) and then nesting another nix develop to run doit.sh..."
nix develop ~/pick-up-nix2/ --command bash -c "nix develop "

echo "Script finished."
