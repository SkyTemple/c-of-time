#!/usr/bin/env python3
import sys
import glob
from pathlib import Path

from yaml import load, Loader

region = sys.argv[1]
itcm_region = region + "-ITCM"

linkerscript_lines = []
all_symbols = set()

linkerscript_lines.append("/* THIS FILE IS AUTO-GENERATED. DO NOT MODIFY! */")

for yaml_file_path in Path("pmdsky-debug/symbols").rglob("*.yml"):
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
        addr = None
        # *-ITCM regions are runtime overrides for the normal addresses
        if itcm_region in addresses:
          addr = addresses[itcm_region]
        elif region in addresses:
          addr = addresses[region]

        if addr is not None:
          if isinstance(addr, list):
            if len(addr) == 0:
              continue
            addr = addr[0]

          if name in all_symbols:
            print(f"Warning: Duplicate symbol: '{name}'")
          linkerscript_lines.append(f"{name} = {hex(addr)};")
          all_symbols.add(name)

with open(f"symbols/generated_{region}.ld", "w") as f:
  for line in linkerscript_lines:
    f.write(line)
    f.write('\n')
