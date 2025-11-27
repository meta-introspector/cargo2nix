#!/bin/bash

# Define colors for output
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "${YELLOW}Generating report on recent branches for all submodules...${NC}"

# Iterate over all submodules
git submodule foreach --recursive '
    SUBMODULE_PATH=$(pwd)
    SUBMODULE_NAME=$(basename "$SUBMODULE_PATH")
    
    printf "\n${GREEN}--- Submodule: %s ---${NC}\n" "$SUBMODULE_NAME"
    printf "${YELLOW}Recent Branches (sorted by commit date):${NC}\n"
    
    # List all local and remote branches sorted by committer date
    # Format: YYYY-MM-DD <tab> branch_name
    git for-each-ref --sort=-committerdate --format="%(committerdate:short)%09%(refname:short)" refs/heads/ refs/remotes/
    echo ""
'

echo "${YELLOW}Submodule branch report finished.${NC}"

