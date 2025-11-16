import re
import sys
import subprocess
from collections import defaultdict

# Re-use parse_dot_file from process_depgraph.py
def parse_dot_file(dot_file_path):
    graph = defaultdict(list)
    nodes = set()
    node_id_to_name = {}
    
    with open(dot_file_path, 'r') as f:
        for line in f:
            line = line.strip()
            
            node_match = re.match(r'(\d+)\s*\[\s*label\s*=\s*"([^"]+)"', line)