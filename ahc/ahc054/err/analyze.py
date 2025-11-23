import sys
import re

#!/usr/bin/env python3

def main():
    if len(sys.argv) < 2:
        print("Usage: python analyze.py <input_file>", file=sys.stderr)
        sys.exit(1)

    path = sys.argv[1]
    is_stdin = path == '-'
    pattern = re.compile(r'^\d+$')

    if is_stdin:
        stream = sys.stdin
    else:
        stream = open(path, 'r', encoding='utf-16')

    with stream:
        for line in stream:
            s = line.strip()
            if pattern.fullmatch(s):
                print(s)

if __name__ == "__main__":
    main()