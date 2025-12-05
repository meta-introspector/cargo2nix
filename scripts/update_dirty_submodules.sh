#!/bin/bash

# Define colors for output
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

#echo "${YELLOW}Starting recursive update of dirty submodules...${NC}"

# Iterate over all submodules
git submodule foreach --recursive 
    SUBMODULE_PATH=$(pwd)
    SUBMODULE_NAME=$(basename "$SUBMODULE_PATH")
    
    printf "\n${GREEN}--- Processing submodule: %s ---${NC}\n" "$SUBMODULE_NAME"

    # Discard any local changes and reset to the HEAD of the current branch
    #printf "${YELLOW}Resetting %s...${NC}\n" "$SUBMODULE_NAME"
    #if ! git reset --hard HEAD; then
    #    printf "${RED}Error resetting %s. Skipping.\n${NC}" "$SUBMODULE_NAME"
    #    exit 1 # Exit the foreach sub-shell
    #fi

    # Fetch latest changes from all remotes
    printf "${YELLOW}Fetching latest changes for %s...${NC}\n" "$SUBMODULE_NAME"
    if ! git fetch origin; then
        printf "${RED}Error fetching origin for %s. Skipping.\n${NC}" "$SUBMODULE_NAME"
        exit 1 # Exit the foreach sub-shell
    fi

    git status
    
    # Try to find the HEAD branch name (e.g., master or main)
    # This command can be unreliable. Fallback to 'master' or 'main'.
    REMOTE_HEAD_BRANCH=$(git remote show origin | grep "HEAD branch" | awk "{print \$NF}")
    if [ -z "$REMOTE_HEAD_BRANCH" ]; then
        printf "${YELLOW}Could not determine remote HEAD branch for %s. Trying 'master' then 'main'.\n${NC}" "$SUBMODULE_NAME"
        # Check if 'master' exists on remote
        if git show-ref --verify --quiet "refs/remotes/origin/master"; then
            REMOTE_HEAD_BRANCH="master"
        elif git show-ref --verify --quiet "refs/remotes/origin/main"; then
            REMOTE_HEAD_BRANCH="main"
        else
            printf "${RED}Could not find 'master' or 'main' branch on remote for %s. Skipping.\n${NC}" "$SUBMODULE_NAME"
            exit 1 # Exit the foreach sub-shell
        fi
    fi

    # Checkout the remote tracking branch and pull latest changes
    printf "${YELLOW}Checking out and pulling latest for %s on branch %s...${NC}\n" "$SUBMODULE_NAME" "$REMOTE_HEAD_BRANCH"
    printf "DEBUG checking out %s for %s. Skipping.\n${NC}" "$REMOTE_HEAD_BRANCH" "$SUBMODULE_NAME"
    #if ! echo git checkout "$REMOTE_HEAD_BRANCH"; then
    #    printf "${RED}Error checking out %s for %s. Skipping.\n${NC}" "$REMOTE_HEAD_BRANCH" "$SUBMODULE_NAME"
     ##   exit 1 # Exit the foreach sub-shell
   # f#i


#    if ! git pull origin "$REMOTE_HEAD_BRANCH"; then
#        printf "${RED}Error pulling latest for %s on branch %s. Skipping.\n${NC}" "$REMOTE_HEAD_BRANCH" "$SUBMODULE_NAME"
#        exit 1 # Exit the foreach sub-shell
#    fi

    printf "${GREEN}Successfully updated %s to latest on branch %s.\n${NC}" "$SUBMODULE_NAME" "$REMOTE_HEAD_BRANCH";

echo "${YELLOW}Finished processing submodules. Now updating superproject's gitlinks...${NC}"

# Update the superproject's gitlink entries for all submodules
#if ! git submodule update --remote --recursive; then
#    echo "${RED}Error updating superproject gitlinks. Please inspect 'git status'.${NC}"
#else
#    echo "${GREEN}Superproject gitlinks updated. Please run 'git status' and commit changes.${NC}"
#fi

echo "${YELLOW}Script finished.${NC}"
