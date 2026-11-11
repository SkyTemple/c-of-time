def dofile(rom, subparams, custom_code_bytes, vma):
    ctype = subparams[0]
    if ctype == "move":
        fn = "BALANCE/waza_cd.bin"
    elif ctype == "item":
        fn = "BALANCE/item_cd.bin"
    elif ctype == "sp":
        fn = "BALANCE/process.bin"
    else:
        raise ValueError(f"Unknown type ctype {ctype}")
    cnum = int(subparams[1])
    data = rom.getFileByName(fn)
    limit = int.from_bytes(data[:4], "little")
    last_ptr = int.from_bytes(data[limit : limit + 4], "little")
    effects = []
    for x in range(limit, last_ptr, 8):
        start = int.from_bytes(data[x : x + 4], "little")
        length = int.from_bytes(data[x + 4 : x + 8], "little")
        effects.append(data[start : start + length])
    while len(effects) <= cnum:
        effects.append(b"")
    effects[cnum] = custom_code_bytes
    new_data = bytearray(limit + len(effects) * 8)
    new_data[:limit] = data[:limit]
    for i, c in enumerate(effects):
        new_data[limit + i * 8 : limit + i * 8 + 8] = len(new_data).to_bytes(
            4, "little"
        ) + len(c).to_bytes(4, "little")
        new_data += c
    rom.setFileByName(fn, new_data)
