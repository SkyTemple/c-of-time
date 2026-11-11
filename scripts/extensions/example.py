# Extension function
def do_example(rom, subparams, custom_code_bytes, vma):
  # Extension prints out received data
  print(
    "Detected extension usage with params",
    subparams,
    f"at virtual memory address {hex(vma)} "
    f"with block of {hex(len(custom_code_bytes))} bytes.",
  )


# EXTENSIONS list must be defined for extension file to work
# This list contains pair of (NAME, FUNCTION) to map extension name to your custom logic
# Extension registration is currently disabled as it is commented out
EXTENSIONS = [
  #  ("myextension", do_example),
]
