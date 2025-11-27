#!/bin/bash

# Define colors for output
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo "${YELLOW}Managing branches in all submodules for feature/CRQ-016-nixify...${NC}"

git submodule foreach --recursive '
    SUBMODULE_PATH=$(pwd)
    SUBMODULE_NAME=$(basename "$SUBMODULE_PATH")
    
    printf "\n${GREEN}--- Processing submodule: %s ---${NC}\n" "$SUBMODULE_NAME"

    # Ensure a clean state
    printf "${YELLOW}Resetting %s to HEAD...${NC}\n" "$SUBMODULE_NAME"
    if ! git reset --hard HEAD; then
        printf "${RED}Error resetting %s. This might indicate uncommitted changes or issues. Skipping this submodule.${NC}\n" "$SUBMODULE_NAME"
        # Continue to the next submodule in foreach loop
        exit 0 
    fi
    
    # Fetch all remotes to ensure up-to-date branch info
    printf "${YELLOW}Fetching latest changes for %s...${NC}\n" "$SUBMODULE_NAME"
    if ! git fetch --all; then
        printf "${RED}Error fetching all remotes for %s. Branch information might be outdated. Continuing.${NC}\n" "$SUBMODULE_NAME"
    fi

    # Check if feature/CRQ-016-nixify exists locally
    if git rev-parse --verify --quiet feature/CRQ-016-nixify >/dev/null; then
        printf "${GREEN}Local branch feature/CRQ-016-nixify exists. Checking it out...${NC}\n"
        git checkout feature/CRQ-016-nixify
    # Check if origin/feature/CRQ-016-nixify exists
    elif git rev-parse --verify --quiet origin/feature/CRQ-016-nixify >/dev/null; then
        printf "${GREEN}Remote branch origin/feature/CRQ-016-nixify exists. Creating and checking out local branch...${NC}\n"
        git checkout -b feature/CRQ-016-nixify origin/feature/CRQ-016-nixify
    else
        printf "${YELLOW}Branch feature/CRQ-016-nixify does not exist in %s. Checking for main/master to rename...${NC}\n" "$SUBMODULE_NAME"
        
        # Determine the current HEAD branch (main or master)
        CURRENT_HEAD_BRANCH=""
        if git rev-parse --verify --quiet main >/dev/null; then
            CURRENT_HEAD_BRANCH="main"
        elif git rev-parse --verify --quiet master >/dev/null; then
            CURRENT_HEAD_BRANCH="master"
        fi

        if [ -n "$CURRENT_HEAD_BRANCH" ]; then
            printf "${YELLOW}Attempting to rename local branch '%s' to 'feature/CRQ-016-nixify' and checking it out...${NC}\n" "$CURRENT_HEAD_BRANCH"
            
            # If the branch to be renamed is currently checked out, switch to detached HEAD first
            if [ "$(git symbolic-ref --short HEAD 2>/dev/null)" = "$CURRENT_HEAD_BRANCH" ]; then
                printf "${YELLOW}Currently on '%s'. Detaching HEAD before renaming...${NC}\n" "$CURRENT_HEAD_BRANCH"
                git checkout --detach
            fi
            
            if git branch -m "$CURRENT_HEAD_BRANCH" feature/CRQ-016-nixify; then
                git checkout feature/CRQ-016-nixify
                printf "${GREEN}Successfully renamed and checked out 'feature/CRQ-016-nixify' in %s.${NC}\n" "$SUBMODULE_NAME"
            else
                printf "${RED}Error renaming branch '%s' to 'feature/CRQ-016-nixify' in %s. Skipping.${NC}\n" "$CURRENT_HEAD_BRANCH" "$SUBMODULE_NAME"
            fi
        else
            printf "${RED}Neither feature/CRQ-016-nixify, main, nor master branch found in %s. Skipping submodule.${NC}\n" "$SUBMODULE_NAME"
        fi
    fi
    printf "${GREEN}Finished processing submodule: %s.${NC}\n" "$SUBMODULE_NAME"
'

echo "${YELLOW}Branch management script finished.${NC}"
