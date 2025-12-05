
set -x
set -e

# Change to the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

for x in cargo  config-rs dlv-list-rs  faster-hex  gitoxide heapless  ordered-multimap-rs  ron-rs  rust-base64  rust-ini time-rs ;
do echo $x;
   pushd  $x;
   mkdir -p .cargo
   cp ../../.cargo/config.toml .cargo # Corrected path for config.toml
   grep -qxF "vendor" .gitignore || echo "vendor" >> .gitignore
   # Add exception for Makefile
   grep -qxF "!Makefile" .gitignore || echo "!Makefile" >> .gitignore
   git add .gitignore
   git add .cargo
   
   # Check if Cargo.toml has a [workspace] section, if not, add one
   if ! grep -q "^\\[workspace\\]" Cargo.toml; then
       echo -e "\n[workspace]" >> Cargo.toml
       git add Cargo.toml
   fi
   
   cargo vendor # This will populate the 'vendor' directory
   ../../target/debug/cargo2nix --overwrite # Corrected path for cargo2nix

   # Copy templates
   cp ../../flake.nix.template flake.nix
   cp ../../Makefile.template Makefile

   # Run cargo build within a nix develop shell using the Makefile
   make nix-build

   # Run nix build for the submodule using the Makefile
   make nix-flake-build

   git add .cargo Cargo.nix flake.nix Makefile # Ensure all relevant files are staged (vendor is explicitly NOT staged)
   if ! git diff --cached --exit-code; then
       git commit -m "feat: Add/update Nix build files for $(basename $(pwd))"
   else
       echo "No changes to commit in $(basename $(pwd))"
   fi
   popd;
done
