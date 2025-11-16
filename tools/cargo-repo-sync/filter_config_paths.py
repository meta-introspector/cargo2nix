#!/usr/bin/env python3
import sys
import os
import argparse

def main():
    parser = argparse.ArgumentParser(description="Filter config.toml paths.")
    parser.add_argument("-v", "--verbose", action="store_true", help="Enable verbose output.")
    parser.add_argument("input_file", help="Input file containing paths.")
    parser.add_argument("output_file", help="Output file to write filtered paths.")
    args = parser.parse_args()

    filtered_paths = []
    exclude_keywords = ["target", "tests", "examples"]

    if args.verbose:
        print("--- Verbose Mode Enabled ---", file=sys.stderr)
        print(f"Input file: {args.input_file}", file=sys.stderr)
        print(f"Output file: {args.output_file}", file=sys.stderr)

    try:
        with open(args.input_file, 'r') as infile:
            input_lines = infile.readlines()
    except FileNotFoundError:
        print(f"Error: Input file '{args.input_file}' not found.", file=sys.stderr)
        sys.exit(1)

    for line in input_lines:
        path = line.strip()
        abs_path = os.path.abspath(path)
        
        should_exclude = False
        path_components = abs_path.split(os.sep)
        
        if args.verbose:
            print(f"Processing: '{path}'", file=sys.stderr)
            print(f"  Absolute Path: '{abs_path}'", file=sys.stderr)
            print(f"  Path Components: {path_components}", file=sys.stderr)

        for keyword in exclude_keywords:
            if keyword in path_components:
                should_exclude = True
                if args.verbose:
                    print(f"  Excluding because '{keyword}' found in components.", file=sys.stderr)
                break
        
        if not should_exclude:
            filtered_paths.append(abs_path)
            if args.verbose:
                print(f"  Including path.", file=sys.stderr)
        
        if args.verbose:
            print("-" * 20, file=sys.stderr)

    try:
        with open(args.output_file, 'w') as outfile:
            for p in filtered_paths:
                outfile.write(p + "\n")
        if args.verbose:
            print(f"Successfully wrote {len(filtered_paths)} paths to '{args.output_file}'.", file=sys.stderr)
    except IOError as e:
        print(f"Error writing to output file '{args.output_file}': {e}", file=sys.stderr)
        sys.exit(1)

    if args.verbose:
        print("--- Script Finished ---", file=sys.stderr)

if __name__ == "__main__":
    main()
