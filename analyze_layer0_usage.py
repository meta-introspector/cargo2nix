import re
import sys

def analyze_layer0_usage(merged_output_file):
    layer0_modules = {}
    in_layer0_section = False

    try:
        with open(merged_output_file, 'r') as f:
            for line in f:
                line = line.strip()

                if "Clustered by Layer (0-N) with Usage Counts:" in line:
                    in_layer0_section = True
                    continue
                
                if in_layer0_section:
                    layer_header_match = re.match(r'Layer (\d+) \(Total modules: \d+\):', line)
                    if layer_header_match:
                        layer_num = int(layer_header_match.group(1))
                        if layer_num == 0:
                            continue # Still in Layer 0 header
                        else:
                            # Moved past Layer 0
                            in_layer0_section = False
                            break # Stop processing once we're past Layer 0

                    module_match = re.match(r'^- (.+), Usage Count: (\d+)$', line)
                    if module_match:
                        crate_name = module_match.group(1)
                        usage_count = int(module_match.group(2))
                        layer0_modules[crate_name] = usage_count
    except FileNotFoundError:
        sys.stderr.write(f"Error: Merged output file not found at {merged_output_file}\n")
        return

    if not layer0_modules:
        sys.stdout.write("No Layer 0 modules found or parsed.\n")
        return

    most_used_module = None
    max_usage_count = -1

    for crate, count in layer0_modules.items():
        if count > max_usage_count:
            max_usage_count = count
            most_used_module = crate
        elif count == max_usage_count and most_used_module is not None:
            # If there's a tie, prefer lexicographically smaller name for consistent output
            if crate < most_used_module:
                most_used_module = crate

    if most_used_module:
        sys.stdout.write(f"The single most used Layer 0 module (not being overridden) is: {most_used_module} with Usage Count: {max_usage_count}\n")
    else:
        sys.stdout.write("Could not determine the most used Layer 0 module.\n")

if __name__ == "__main__":
    analyze_layer0_usage('merged_output.txt')
