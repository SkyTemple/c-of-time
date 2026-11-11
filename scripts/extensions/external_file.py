import os

FILE_OUTPUT = "fragments"


def dofile(rom, subparams, custom_code_bytes, vma):
    os.makedirs(FILE_OUTPUT, exist_ok=True)
    with open(os.path.join(FILE_OUTPUT, ".".join(subparams)), "wb") as f:
        f.write(custom_code_bytes)
