#!/usr/bin/awk -f

{
    # Extract the first word (crate name) from the line
    crate_name = $1
    # Print in the desired format
    print "submodule/" crate_name
}