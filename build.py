import os

import click

BASE_DIR = os.path.dirname(os.path.realpath(__file__))


def analyse(filename):
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    lines = content.split('\n')
    other = []
    use = []
    functions = {}
    mods = {}
    head = []
    i = 0

    def read_block(i):
        blocks = []
        while other and (other[-1].startswith('///') or other[-1].startswith('#[')):
            blocks.append(other.pop())
        blocks.reverse()
        while lines[i] != '}':
            blocks.append(lines[i])
            i += 1
        blocks.append(lines[i])
        return i, blocks

    while i < len(lines):
        line = lines[i]
        if line.startswith('use '):
            use.append(line)
        elif line.startswith('fn '):
            fn_name = line.removeprefix('fn ').strip().partition('(')[0].partition('<')[0]
            i, blocks = read_block(i)
            functions[fn_name] = '\n'.join(blocks)
        elif line.startswith('mod '):
            mod_name = line.removeprefix('mod ').strip().partition(' ')[0]
            i, blocks = read_block(i)
            mods[mod_name] = '\n'.join(blocks)
        elif line.startswith('//!'):
            head.append(line)
        elif line:
            other.append(line)
        i += 1
    return use, functions, mods, other, head


@click.command()
@click.argument("filename")
def main(filename):
    use, functions, mods, other, head = analyse(filename)

    main_func = functions.pop('main')
    solve_mod_name = main_func.split('\n')[-2].partition('::solve')[0].strip()
    solve_mod = mods.pop(solve_mod_name)

    solve_lines = [i[4:] for i in solve_mod.partition('mod ')[2].split('\n')[1:-1]]
    for line in solve_lines:
        if line.startswith('use ') and line not in use and not line.startswith('use super::'):
            use.append(line)
    solve_lines = [i for i in solve_lines if not i.startswith('use ')]

    used_mods = set([i.removeprefix("use codeforces::").partition("::")[0] for i in use if i.startswith('use codeforces::')])
    used_mods.add('raw')
    use_content = '\n'.join(use)
    use_content = use_content.replace("use codeforces::", "use crate::")
    main_func = main_func.replace("    use codeforces::raw;\n", '')

    main_func = main_func.replace(f'{solve_mod_name}::', '')

    result = [*head, use_content, *functions.values(), *other, *solve_lines, main_func]
    split_line = '// ---------- tools ------------'
    result.append('\n' + split_line)
    for mod in used_mods:
        with open(os.path.join(BASE_DIR, "src", mod + ".rs")) as f:
            mod_content = f.read()
            mod_content = '\n'.join('    ' + i for i in mod_content.split('\n'))
            result.append(f"mod {mod} {{\n{mod_content}\n}}")
    with open("_generated.rs", "w", encoding='utf-8') as f:
        f.write("\n".join(result))


if __name__ == '__main__':
    main()
