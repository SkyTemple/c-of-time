#!/usr/bin/env python3
import ndspy.rom
import ndspy.code
import sys
import os
import re
from subprocess import Popen, PIPE
import glob
import platform

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

rom = ndspy.rom.NintendoDSRom.fromFile(rom_path)
overlays = rom.loadArm9Overlays()

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
  if not os.path.exists("build/binaries"):
    os.mkdir("build/binaries")

  # Write all symbols to a file that can be included in patches
  with open("build/binaries/symbols.asm", "w") as f:
    # Write symbols
    for symbol, offset in overlay_symbols_lookup.items():
      f.write(f".definelabel {symbol},{hex(offset)}\n")

    # Write binary and overlay RAM offsets
    f.write(f"arm9_start equ {hex(rom.arm9RamAddress)}\n")
    f.write(f"arm9_end equ {hex(rom.arm9RamAddress + len(rom.arm9))}\n")
    f.write(f"arm7_start equ {hex(rom.arm7RamAddress)}\n")
    f.write(f"arm7_end equ {hex(rom.arm7RamAddress + len(rom.arm7))}\n")

    for index, overlay in overlays.items():
      if index != OVERLAY_INDEX:
        f.write(f"overlay{index}_start equ {hex(overlay.ramAddress)}\n")
        f.write(f"overlay{index}_end equ {hex(overlay.ramAddress + overlay.ramSize)}\n")

  # Write the main binaries
  with open("build/binaries/arm9.bin", "wb") as f:
    f.write(rom.arm9)
  with open("build/binaries/arm7.bin", "wb") as f:
    f.write(rom.arm7)

  # Write overlay binaries
  for index, overlay in overlays.items():
    if index != OVERLAY_INDEX:
      with open(f"build/binaries/overlay{index}.bin", "wb") as f:
        f.write(rom.files[overlay.fileID])

  for file in glob.glob("patches/*.asm"):
    apply_binary_patch(file)

  # Apply the main binaries
  with open("build/binaries/arm9.bin", "rb") as f:
    rom.arm9 = f.read()
  with open("build/binaries/arm7.bin", "rb") as f:
    rom.arm7 = f.read()
  
  # Apply overlay binaries
  for index, overlay in overlays.items():
    if index != OVERLAY_INDEX:
      with open(f"build/binaries/overlay{index}.bin", "rb") as f:
        rom.files[overlay.fileID] = f.read()

def apply_binary_patch(file_path):
  print("Applying binary patch: " + file_path)
  
  armips_path = "armips"
  if platform.system() == 'Darwin':
    armips_path = "bin/armips/armips-mac-x64"
  elif platform.system() == 'Windows':
    armips_path = "bin/armips/armips-win-x64.exe"
  patch_file_path = os.path.join('../../', file_path) # Relative to the root `build/binaries`

  process = Popen([armips_path, patch_file_path, '-root', 'build/binaries'])
  exit_code = process.wait()

  assert exit_code == 0, f"armips failed with code {exit_code}"

load_overlay_symbols()
apply_overlay()
apply_binary_patches()

rom.saveToFile(rom_out_path)
