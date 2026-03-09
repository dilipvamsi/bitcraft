import sys
import re

def filter_expansion(content):
    # Split into a list of top-level items (blocks)
    # This is a bit hard with regex, maybe we can just use a simple state machine for { }

    lines = content.split('\n')
    output = []

    # Skip the compiler fluff at the top (std prelude)
    start_idx = 0
    for i, line in enumerate(lines):
        if 'bitstruct' in line and (line.startswith('use') or line.startswith('extern')):
            start_idx = i
            break

    current_block = []
    brace_count = 0
    in_interesting_block = False

    for line in lines[start_idx:]:
        clean_line = line.strip()

        # Start of a block or a single-line item
        if brace_count == 0:
            current_block = []
            # We are interested in:
            # - struct definitions
            # - enum definitions
            # - impl StructName blocks
            # - impl ::bitstruct::... blocks

            # Simple check for interesting start
            if (clean_line.startswith('pub struct') or
                clean_line.startswith('pub enum') or
                (clean_line.startswith('impl') and not '::core::' in line and not '::bytemuck::' in line)):
                in_interesting_block = True
            else:
                in_interesting_block = False

        if in_interesting_block:
            current_block.append(line)

        brace_count += line.count('{')
        brace_count -= line.count('}')

        if brace_count == 0 and in_interesting_block:
            output.extend(current_block)
            output.append("") # Add spacing between blocks
            in_interesting_block = False
            current_block = []

    return "\n".join(output)

if __name__ == "__main__":
    if len(sys.argv) > 1:
        with open(sys.argv[1], 'r') as f:
            print(filter_expansion(f.read()))
    else:
        print(filter_expansion(sys.stdin.read()))
