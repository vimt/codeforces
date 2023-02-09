import os

import click

BASE_DIR = os.path.dirname(os.path.realpath(__file__))


@click.command()
@click.argument("filename")
def main(filename):
    with open(filename, 'r', encoding='utf-8') as f:
        content = f.read()
    lines = content.split('\n')
    uses = [i for i in lines if i.startswith('use codefoces::')]
    used_mods = set([i.removeprefix("use codefoces::").partition("::")[0] for i in uses])
    content = content.replace("use codefoces::", "use crate::")
    result = [content]
    for mod in used_mods:
        with open(os.path.join(BASE_DIR, "src", mod + ".rs")) as f:
            mod_content = f.read()
            result.append(f"mod {mod} {{\n{mod_content}\n}}")
    with open("_generated.rs", "w", encoding='utf-8') as f:
        f.write("\n".join(result))


if __name__ == '__main__':
    main()
