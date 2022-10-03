#!/usr/bin/env python3
import ndspy.rom
import ndspy.code
import sys
import os
import re
import keystone
from subprocess import Popen, PIPE
import glob

OVERLAY_INDEX = 36

# Overlay load address + offset to common area
# see https://docs.google.com/document/d/1Rs4icdYtiM6KYnWxMkdlw7jpWrH7qw5v6LOfDWIiYho
START_ADDRESS = 0x23D7FF0 

region = sys.argv[1]
rom_path = sys.argv[2]
overlay_bin_path = sys.argv[3]
overlay_elf_path = sys.argv[4]
rom_out_path = sys.argv[5]

overlay_symbols_lookup = {} # Key = symbol_name: string, value = offset: int
rom_symbols_lookup = {}     # Key = symbol_name: string, value = offset_and_overlay_id: Tuple<int, int>

rom = ndspy.rom.NintendoDSRom.fromFile(rom_path)
overlays = rom.loadArm9Overlays()

linkerscript_file_regex = re.compile('\/\* !file (arm9|overlay(\d\d?))', re.IGNORECASE)
linkerscript_symbol_regex = re.compile('(.+) = 0x(.+);', re.IGNORECASE)

def load_linkerscript_symbols():
  for file in [f"symbols/generated_{region}.ld", f"symbols/custom_{region}.ld"]:    
    with open(file, 'r') as f:
      overlay_index = None
      for line in f:
        line = line.strip()
        match = linkerscript_file_regex.match(line)

        if match:
          # Line in format: /* !file overlayXX */:
          file, overlay_id_str = match.groups()
          if file == 'arm9':
            overlay_index = -1
          elif overlay_id_str:
            overlay_index = int(overlay_id_str)
            assert overlay_index != OVERLAY_INDEX, "Linker script must not contain symbols from the custom overlay."

        else:
          match = linkerscript_symbol_regex.match(line)
          if match:
            assert overlay_index != None, "No file specified in linker script. " \
              "Add a comment: '/* !file overlayXX */' or '/* !file arm9 */'"
            symbol_name, offset_str = match.groups()
            offset = int(offset_str, 16)

            if symbol_name in rom_symbols_lookup:
              print(f"Warning: Duplicate symbol: '{symbol_name}'")
            rom_symbols_lookup[symbol_name] = (offset, overlay_index)

def load_overlay_symbols():
  process = Popen(["arm-none-eabi-nm", overlay_elf_path], stdout=PIPE)
  (stdout, stderr) = process.communicate()
  exit_code = process.wait()
  assert exit_code == 0, f"nm failed with code {exit_code}"

  lines = stdout.decode().split('\n')
  for line in lines:
    parts = line.strip().split()
    
    if (len(parts) < 3):
      continue
            
    offset = int(parts[0], 16)
    type = parts[1]
    name = parts[2]
    overlay_symbols_lookup[name] = offset

def resolve_symbol(symbol):
  # Returns address and overlay ID
  if symbol in overlay_symbols_lookup:
    return (overlay_symbols_lookup[symbol], OVERLAY_INDEX)
  if symbol in rom_symbols_lookup:
    return rom_symbols_lookup[symbol]

  return None

def apply_overlay():
  assert OVERLAY_INDEX in overlays, "No overlay 36 found, apply the ExtraSpace patch first."
  overlay = overlays[OVERLAY_INDEX]
  assert overlay.ramAddress == 0x023A7080, "Unexpected RAM start address"
  assert overlay.ramSize == 0x38F80, "Unexpected overlay RAM size"
  overlay_bytes = rom.files[overlay.fileID]

  with open(overlay_bin_path, "rb") as f:
    custom_code_bytes = f.read()

  # Combine the existing overlay bytes with the custom code
  padding = START_ADDRESS - overlay.ramAddress
  new_overlay_bytes = bytearray(padding + len(custom_code_bytes))
  new_overlay_bytes[0:len(overlay_bytes)] = overlay_bytes
  new_overlay_bytes[padding:padding + len(custom_code_bytes)] = custom_code_bytes

  overlay.data = new_overlay_bytes
  overlay.staticInitStart = overlay_symbols_lookup['__init_array_start']
  overlay.staticInitEnd = overlay_symbols_lookup['__init_array_end']
  overlay.bssSize = 0 # .bss is included in the binary
  overlay.save()
  rom.files[overlay.fileID] = new_overlay_bytes
  rom.arm9OverlayTable = ndspy.code.saveOverlayTable(overlays)

def apply_binary_patches():
  overlay_bytes = { -1: (bytearray(rom.arm9), rom.arm9RamAddress) }
  for index, overlay in overlays.items():
    if index != OVERLAY_INDEX:
      overlay_bytes[index] = (bytearray(rom.files[overlay.fileID]), overlay.ramAddress)

  for file in glob.glob("patches/*.cotpatch"):
    apply_binary_patch(file, overlay_bytes)

  file, ram_address = overlay_bytes[-1]
  rom.arm9 = bytes(file)
  for index, overlay in overlays.items():
    if index != OVERLAY_INDEX:
      file, ram_address = overlay_bytes[index]
      rom.files[overlay.fileID] = bytes(file)

offset_regex = re.compile('(.+)\+(\d):', re.IGNORECASE)
branch_regex = re.compile('^bl?(eq|ne|cs|hs|cc|lo|mi|pl|vs|vc|hi|ls|ge|lt|gt|le|al)?$', re.IGNORECASE)

def apply_binary_patch(file_path, overlay_bytes):
  print("Applying binary patch: " + file_path)
  assembler = keystone.Ks(keystone.KS_ARCH_ARM, keystone.KS_MODE_ARM)

  with open(file_path, 'r') as f:
    current_offset = -1
    overlay_index = -1
    for line in f:
      line = line.split("//")[0] # Remove "//" comments
      line = line.split("#")[0] # Remove "#" comments
      line = line.split(";")[0] # Remove ";" comments
      line = line.strip()
      if line == "":
        continue

      match = offset_regex.match(line)
      if match:
        # Line in format: [symbol]+[hex offset]:
        symbol, offset_str = match.groups()
        offset = int(offset_str, 16)
        assert symbol in rom_symbols_lookup, f"No symbol '{symbol}' found"
        symbol_offset, overlay_index = rom_symbols_lookup[symbol]
        current_offset = symbol_offset + offset
      else:
        # Instruction
        assert current_offset != -1, "Symbol and offset must be specified before instructions"

        split_line = line.split()
        if branch_regex.match(split_line[0]):
          assert len(split_line) >= 2, "Branch must have an operand"

          if not split_line[1].isnumeric() and not split_line[1].startswith("0x"):
            # The branch is not numeric, so it points to a symbol
            resolved = resolve_symbol(split_line[1])
            assert resolved, f"Failed to resolve symbol: '{split_line[1]}'"

            sym_offset, _ = resolved
            if sym_offset:
              line = f"{split_line[0]} {hex(sym_offset - current_offset)}"

        bytes, instruction_count = assembler.asm(line)
        overlay_file, ram_offset = overlay_bytes[overlay_index]
        for byte in bytes:
          overlay_file[current_offset - ram_offset] = byte
          current_offset += 1

load_linkerscript_symbols()
load_overlay_symbols()
apply_overlay()
apply_binary_patches()

rom.saveToFile(rom_out_path)
