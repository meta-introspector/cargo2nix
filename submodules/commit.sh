
set -x
set -e

# Change to the directory where the script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )"
cd "$SCRIPT_DIR"

for x in cargo  config-rs dlv-list-rs  faster-hex  gitoxide heapless  ordered-multimap-rs  ron-rs  rust-base64  rust-ini time-rs ;
do echo $x;
   pushd  $x;
   git add .
   if ! git diff --cached --exit-code; then
       git commit -m "feat: Add/update Nix build files for $(basename $(pwd))"
   else
       echo "No changes to commit in $(basename $(pwd))"
   fi
   git checkout -b  feature/CRQ-016-nixify || git checkout   feature/CRQ-016-nixify
   git push origin 
   popd;
done
