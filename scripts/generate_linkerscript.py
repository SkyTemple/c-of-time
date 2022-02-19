#!/usr/bin/env python3
import sys
import glob
from yaml import load, Loader

region = sys.argv[1]

linkerscript_lines = []
all_symbols = set()

linkerscript_lines.append("/* THIS FILE IS AUTO-GENERATED. DO NOT MODIFY! */")

for yaml_file_path in glob.glob("pmdsky-debug/symbols/*.yml"):
  with open(yaml_file_path, 'r') as f:
    linkerscript_lines.append("")
    linkerscript_lines.append(f"/* --- {yaml_file_path} --- */")
    yaml_string = f.read()
    symbol_def = load(yaml_string, Loader)
    for file_name, contents in symbol_def.items():
      linkerscript_lines.append("")
      linkerscript_lines.append(f"/* !file {file_name} */")
      if 'address' in contents:
        addresses = contents['address']
        if region in addresses:
          symbol = f"{file_name.upper()}_LOAD_ADDR"

          # Overlay load addresses are duplicated, ignore the duplicates
          if not symbol in all_symbols:
            addr = addresses[region]
            linkerscript_lines.append(f"{symbol} = {hex(addr)};")
            all_symbols.add(symbol)

      symbols = []
      if 'functions' in contents:
        symbols.extend(contents['functions'])
      if 'data' in contents:
        symbols.extend(contents['data'])
      
      for function in symbols:
        name = function['name']
        addresses = function['address']
        if region in addresses:
          addr = addresses[region]

          if isinstance(addr, list):
            if len(addr) == 0:
              continue
            addr = addr[0]

          assert not name in all_symbols, f"Duplicate symbol: '{name}'"
          linkerscript_lines.append(f"{name} = {hex(addr)};")
          all_symbols.add(name)

with open('symbols.ld', 'w') as f:
  for line in linkerscript_lines:
    f.write(line)
    f.write('\n')
