#!/usr/bin/env python3
import ndspy.rom
import ndspy.code
import sys
import os
import re
from subprocess import Popen, PIPE
import glob
import platform
import tempfile

OVERLAY_EXTRA = 36
OVERLAY_DUNGEON = 29

region = sys.argv[1]
rom_path = sys.argv[2]
overlay_elf_path = sys.argv[3]
rom_out_path = sys.argv[4]

overlay_symbols_lookup = {} # Key = symbol_name: string, value = offset: int

rom = ndspy.rom.NintendoDSRom.fromFile(rom_path)
overlays = rom.loadArm9Overlays()

# Migration data for old CoT move/item effect hooks applied before https://github.com/SkyTemple/c-of-time/pull/268.
EMC_OLD_HOOK_ADDR = { "EU": 0x023302E4, "NA": 0x0232F8A4, "JP": 0x02330C98 }
EIC_OLD_HOOK_ADDR = { "EU": 0x0231C438, "NA": 0x0231B9D8, "JP": 0x0231CEA4 }
EMC_MIGRATION_PATCHED_BYTES = 0xE59FB0FC
EMC_MIGRATION_VANILLA_BYTES = 0xE3A01001
EIC_MIGRATION_PATCHED_BYTES = 0x0A000029
EIC_MIGRATION_VANILLA_BYTES = 0xE3500000

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
  assert OVERLAY_EXTRA in overlays, "No overlay 36 found, apply the ExtraSpace patch first."

  # TODO: Find a better process to parse section info
  process = Popen(["arm-none-eabi-objdump", "-h", overlay_elf_path], stdout=PIPE)
  (stdout, stderr) = process.communicate()
  exit_code = process.wait()
  assert exit_code == 0, f"objdump failed with code {exit_code}"
  lines = stdout.decode().split('\n')
  readline = -1
  for line in lines:
    if line.startswith("Sections"):
      readline = 2
    if readline==0:
      readline = 2
      section = line.split()
      # Line: ID, Name, Size, VMA, LMA, Offset, Align
      if len(section)>0:
        if section[1].startswith(".text"): # Retrieve only text sections
          hierarchy = section[1].split(".")
          size = int(section[2], 16)
          vma = int(section[3], 16)
          offset = int(section[5], 16)
          bank_number = int(hierarchy[3])
          if hierarchy[2] == "ov":
            overlay = overlays[bank_number]
            overlay_bytes = rom.files[overlay.fileID]
            ram_address = overlay.ramAddress
          elif hierarchy[2] == "arm":
            if bank_number == 9:
              overlay_bytes = rom.arm9
              ram_address = rom.arm9RamAddress
            elif bank_number == 7:
              overlay_bytes = rom.arm7
              ram_address = rom.arm7RamAddress
            else:
              raise ValueError("Invalid arm binary '%d'"%bank_number)
          else:
            raise ValueError("Invalid section '%s'"%hierarchy[2])

          print("Applying C patch section to",hierarchy[2],bank_number,":", section[1], hex(vma), hex(vma+size))

          with tempfile.TemporaryDirectory() as tmpdirname:
            binaryfile = os.path.join(tmpdirname, "temp.bin")
            process = Popen(["arm-none-eabi-objcopy", "-j", section[1], "-O", "binary", overlay_elf_path, binaryfile], stdout=PIPE)
            exit_code = process.wait()
            assert exit_code == 0, f"objdump failed with code {exit_code}"
            with open(binaryfile, "rb") as f:
              custom_code_bytes = f.read()
          
          assert size == len(custom_code_bytes), f"Size mismatch"
          
          # Combine the existing overlay bytes with the custom code
          padding = vma - ram_address
          new_overlay_bytes = bytearray(padding + size)
          new_overlay_bytes[0:len(overlay_bytes)] = overlay_bytes
          new_overlay_bytes[padding:padding + size] = custom_code_bytes

          if hierarchy[2] == "ov":
            overlay.data = new_overlay_bytes
            overlay.save()
            rom.files[overlay.fileID] = new_overlay_bytes
            rom.arm9OverlayTable = ndspy.code.saveOverlayTable(overlays)
          elif hierarchy[2] == "arm":
            if bank_number == 9:
              rom.arm9 = new_overlay_bytes
            elif bank_number == 7:
              rom.arm7 = new_overlay_bytes
    
    if readline>0:
      readline -= 1

def migrate_eimc_hooks():
  move_addr = EMC_OLD_HOOK_ADDR.get(region)
  item_addr = EIC_OLD_HOOK_ADDR.get(region)
  if move_addr is None or item_addr is None:
    # Unknown region, skip migration
    return

  overlay = overlays[OVERLAY_DUNGEON]
  data = bytearray(rom.files[overlay.fileID])
  ram_base = overlay.ramAddress

  emc_applied = rom.filenames.idOf("BALANCE/waza_cd.bin") is not None
  eic_applied = rom.filenames.idOf("BALANCE/item_cd.bin") is not None

  move_expected = EMC_MIGRATION_PATCHED_BYTES if emc_applied else EMC_MIGRATION_VANILLA_BYTES
  item_expected = EIC_MIGRATION_PATCHED_BYTES if eic_applied else EIC_MIGRATION_VANILLA_BYTES

  move_offset = move_addr - ram_base
  item_offset = item_addr - ram_base
  move_current = int.from_bytes(data[move_offset:move_offset + 4], "little")
  item_current = int.from_bytes(data[item_offset:item_offset + 4], "little")

  changed = False
  if move_current != move_expected:
    print(f"!!! Patching old CoT move effect hook detected at {hex(move_addr)}")
    data[move_offset:move_offset + 4] = move_expected.to_bytes(4, "little")
    changed = True
  if item_current != item_expected:
    print(f"!!! Patching old CoT item effect hook detected at {hex(item_addr)}")
    data[item_offset:item_offset + 4] = item_expected.to_bytes(4, "little")
    changed = True

  if changed:
    rom.files[overlay.fileID] = bytes(data)

def apply_binary_patches():
  migrate_eimc_hooks() # Fixup old CoT move/item effect hooks

  if not os.path.exists("build/binaries"):
    os.mkdir("build/binaries")

  # Write all symbols to a file that can be included in patches
  with open("build/binaries/symbols.asm", "w", encoding="utf-8") as f:
    # Write symbols
    for symbol, offset in overlay_symbols_lookup.items():
      f.write(f".definelabel {symbol},{hex(offset)}\n")

    # Write binary and overlay RAM offsets
    f.write(f"arm9_start equ {hex(rom.arm9RamAddress)}\n")
    f.write(f"arm9_end equ {hex(rom.arm9RamAddress + len(rom.arm9))}\n")
    f.write(f"arm7_start equ {hex(rom.arm7RamAddress)}\n")
    f.write(f"arm7_end equ {hex(rom.arm7RamAddress + len(rom.arm7))}\n")

    for index, overlay in overlays.items():
      f.write(f"overlay{index}_start equ {hex(overlay.ramAddress)}\n")
      f.write(f"overlay{index}_end equ {hex(overlay.ramAddress + overlay.ramSize)}\n")

  # Write the main binaries
  with open("build/binaries/arm9.bin", "wb") as f:
    f.write(rom.arm9)
  with open("build/binaries/arm7.bin", "wb") as f:
    f.write(rom.arm7)

  # Write overlay binaries
  for index, overlay in overlays.items():
    with open(f"build/binaries/overlay{index}.bin", "wb") as f:
      f.write(rom.files[overlay.fileID])

  for file in glob.glob("patches/*.asm") + glob.glob("modules/*/patches/*.asm"):
    apply_binary_patch(file)

  # Apply the main binaries
  with open("build/binaries/arm9.bin", "rb") as f:
    rom.arm9 = f.read()
  with open("build/binaries/arm7.bin", "rb") as f:
    rom.arm7 = f.read()
  
  # Apply overlay binaries
  for index, overlay in overlays.items():
    with open(f"build/binaries/overlay{index}.bin", "rb") as f:
      rom.files[overlay.fileID] = f.read()

def apply_binary_patch(file_path):
  print("Applying binary patch:", file_path)
  
  armips_path = "armips"
  if platform.system() == 'Darwin':
    if platform.machine() == 'arm64':
      armips_path = "bin/armips/armips-mac-arm64"
    else:
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
