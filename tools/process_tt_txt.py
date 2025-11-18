import re
import os

def process_tt_txt_simple(tt_txt_path):
    output_lines = []
    with open(tt_txt_path, 'r') as f:
        for line in f:
            line = line.strip()
            if not line:
                output_lines.append("")
                continue

            # Regex to capture crate name
            match = re.match(r'(\w[\w-]*)\s*=', line)
            if match:
                crate_name = match.group(1)
                output_lines.append(f'{crate_name} = {{ path = "submodules/{crate_name}" }}')
            else:
                output_lines.append(line) # Keep original line if it doesn't match expected pattern

    return "\n".join(output_lines)

if __name__ == "__main__":
    tt_txt_path = "/data/data/com.termux.nix/files/home/pick-up-nix2/vendor/rust/cargo2nix/tt.txt"
    
    processed_content = process_tt_txt_simple(tt_txt_path)
    print(processed_content)