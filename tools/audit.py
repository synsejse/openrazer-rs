#!/usr/bin/env python3
"""Extract protocol facts without importing or executing GPL daemon code.
Usage: python tools/audit.py /path/to/openrazer
Writes a reproducible inventory; no daemon implementation is copied.
"""
import ast
import json
import pathlib
import subprocess
import sys

root = pathlib.Path(sys.argv[1])
records = {}

def add(interface, name, ins, outs, path, line):
    key = (interface, name, ins or '', outs or '')
    records.setdefault(key, []).append(f'{path}:{line}')

for path in sorted((root / 'daemon').rglob('*.py')):
    tree = ast.parse(path.read_text())
    relative = str(path.relative_to(root))
    for node in ast.walk(tree):
        if isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)):
            for deco in node.decorator_list:
                if isinstance(deco, ast.Call) and isinstance(deco.func, ast.Name) and deco.func.id == 'endpoint':
                    args = [ast.literal_eval(a) for a in deco.args]
                    kw = {k.arg: ast.literal_eval(k.value) for k in deco.keywords}
                    add(args[0], args[1], kw.get('in_sig'), kw.get('out_sig'), relative, node.lineno)
        if isinstance(node, ast.Tuple) and len(node.elts) == 5:
            a = node.elts
            if isinstance(a[0], ast.Constant) and isinstance(a[0].value, str) and a[0].value.startswith('razer.'):
                add(ast.literal_eval(a[0]), ast.literal_eval(a[1]), ast.literal_eval(a[3]), ast.literal_eval(a[4]), relative, node.lineno)

inventory = {'revision': subprocess.check_output(['git', '-C', str(root), 'rev-parse', 'HEAD'], text=True).strip(),
             'methods': [dict(interface=k[0], name=k[1], input=k[2], output=k[3], sources=v) for k,v in sorted(records.items())]}
pathlib.Path('docs/protocol.json').write_text(json.dumps(inventory, indent=2)+'\n')
print(len(records), 'distinct method signatures')
