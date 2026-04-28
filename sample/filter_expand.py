import sys

def filter_expansion(content):
    lines = content.split('\n')
    output = []

    # Skip the compiler fluff at the top (std prelude)
    start_idx = 0
    for i, line in enumerate(lines):
        if ('bitstruct' in line or 'bitcraft' in line) and (line.startswith('use') or line.startswith('extern')):
            start_idx = i
            break

    current_block = []
    brace_count = 0
    paren_count = 0
    in_interesting_block = False

    for line in lines[start_idx:]:
        clean_line = line.strip()

        # Detect start of interesting items (structs, enums, impls, or their attributes)
        if brace_count == 0:
            if not in_interesting_block:
                if (clean_line.startswith('pub struct') or
                    clean_line.startswith('pub enum') or
                    (clean_line.startswith('impl') and not '::bytemuck::' in clean_line) or
                    clean_line.startswith('#[')):
                    in_interesting_block = True
                    current_block = [line]
                else:
                    pass
            else:
                current_block.append(line)
        else:
            current_block.append(line)

        # Track braces to find block ends
        brace_count += line.count('{')
        brace_count -= line.count('}')
        paren_count += line.count('(')
        paren_count -= line.count(')')

        if brace_count == 0 and paren_count == 0 and in_interesting_block:
            # Check if what we collected so far is just attributes or a full item
            block_text = "\n".join(current_block)
            if any(keyword in block_text for keyword in ['struct', 'enum', 'impl', 'fn']):
                 output.append(block_text)
                 output.append("")
                 in_interesting_block = False
                 current_block = []
            else:
                 # Still just collecting attributes or headers
                 pass

    return "\n".join(output)

if __name__ == "__main__":
    if len(sys.argv) > 1:
        with open(sys.argv[1], 'r') as f:
            print(filter_expansion(f.read()))
    else:
        print(filter_expansion(sys.stdin.read()))
