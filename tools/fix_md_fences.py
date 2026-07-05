#!/usr/bin/env python3
import glob
import re

MD_GLOB = './**/*.md'

def detect_language(next_lines):
    for line in next_lines:
        s = line.strip()
        if not s:
            continue
        if s.startswith('{') or s.startswith('['):
            return 'json'
        if s.startswith('use ') or s.startswith('pub ') or s.startswith('fn ') or s.startswith('let ') or s.startswith('impl ') or s.startswith('extern '):
            return 'rust'
        if s.startswith('#!') or s.startswith('$') or s.startswith('cargo ') or s.startswith('npm ') or s.startswith('npx ') or s.startswith('bash'):
            return 'bash'
        if s.startswith('```'):
            return 'text'
        if s.startswith('<') and s.endswith('>'):
            return 'xml'
        if s.startswith('<!--'):
            return 'html'
        # ASCII-art / box diagrams
        if re.match(r'^[┌┐└┘├┤─│ ]+$', s):
            return 'text'
        # Likely code with equals, arrows or types
        if re.search(r'->|=>|::|->|\{|\}|;$', s):
            return 'rust'
        # default to text
        return 'text'
    return 'text'


def process_file(path):
    with open(path, 'r', encoding='utf-8') as f:
        lines = f.readlines()

    changed = False
    out = []
    i = 0
    while i < len(lines):
        line = lines[i]
        m = re.match(r'^(\s*)```\s*$' , line)
        if m:
            # find next non-empty line
            j = i+1
            next_chunk = lines[j:j+10]
            lang = detect_language(next_chunk)
            out.append(m.group(1) + '```' + lang + '\n')
            changed = True
            i += 1
            # copy until closing fence
            while i < len(lines):
                out.append(lines[i])
                if re.match(r'^\s*```\s*$', lines[i]):
                    i += 1
                    break
                i += 1
            continue
        else:
            out.append(line)
            i += 1

    if changed:
        with open(path, 'w', encoding='utf-8') as f:
            f.writelines(out)
    return changed


def main():
    files = glob.glob(MD_GLOB, recursive=True)
    changed_files = []
    for p in files:
        if p.startswith('./.git'):
            continue
        if process_file(p):
            changed_files.append(p)
    if changed_files:
        print('Updated files:')
        for p in changed_files:
            print(' -', p)
    else:
        print('No changes')

if __name__ == '__main__':
    main()
