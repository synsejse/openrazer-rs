#!/usr/bin/env python3
"""Extract protocol facts without importing or executing GPL daemon code.

Usage: python tools/audit.py /path/to/openrazer

Writes a reproducible docs/protocol.json; no daemon implementation is copied.
"""
import ast
import json
import pathlib
import subprocess
import sys

REPOSITORY = pathlib.Path(__file__).resolve().parent.parent


def endpoint_decorators(function):
    """Yield the `@endpoint(...)` calls on a function."""
    for decorator in function.decorator_list:
        if (
            isinstance(decorator, ast.Call)
            and isinstance(decorator.func, ast.Name)
            and decorator.func.id == 'endpoint'
        ):
            yield decorator


def is_registration_tuple(node):
    """Whether a node is `(interface, name, function, in_sig, out_sig)`."""
    if not isinstance(node, ast.Tuple) or len(node.elts) != 5:
        return False
    interface = node.elts[0]
    return (
        isinstance(interface, ast.Constant)
        and isinstance(interface.value, str)
        and interface.value.startswith('razer.')
    )


def extract(root):
    """Map each (interface, name, input, output) to the places declaring it."""
    records = {}

    def add(interface, name, input_signature, output_signature, path, line):
        key = (interface, name, input_signature or '', output_signature or '')
        records.setdefault(key, []).append(f'{path}:{line}')

    for path in sorted((root / 'daemon').rglob('*.py')):
        relative = str(path.relative_to(root))
        for node in ast.walk(ast.parse(path.read_text())):
            if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
                for decorator in endpoint_decorators(node):
                    args = [ast.literal_eval(arg) for arg in decorator.args]
                    keywords = {
                        keyword.arg: ast.literal_eval(keyword.value)
                        for keyword in decorator.keywords
                    }
                    add(
                        args[0],
                        args[1],
                        keywords.get('in_sig'),
                        keywords.get('out_sig'),
                        relative,
                        node.lineno,
                    )
            if is_registration_tuple(node):
                interface, name, _, input_signature, output_signature = node.elts
                add(
                    ast.literal_eval(interface),
                    ast.literal_eval(name),
                    ast.literal_eval(input_signature),
                    ast.literal_eval(output_signature),
                    relative,
                    node.lineno,
                )
    return records


def main():
    root = pathlib.Path(sys.argv[1])
    records = extract(root)
    revision = subprocess.check_output(
        ['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True
    ).strip()
    inventory = {
        'revision': revision,
        'methods': [
            {
                'interface': interface,
                'name': name,
                'input': input_signature,
                'output': output_signature,
                'sources': sources,
            }
            for (interface, name, input_signature, output_signature), sources
            in sorted(records.items())
        ],
    }
    (REPOSITORY / 'docs/protocol.json').write_text(json.dumps(inventory, indent=2) + '\n')
    print(len(records), 'distinct method signatures')


if __name__ == '__main__':
    main()
