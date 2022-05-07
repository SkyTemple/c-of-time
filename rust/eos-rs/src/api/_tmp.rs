//     - name: TaskProcBoot
//       address:
//         NA: 0x2003328
//         EU: 0x2003328
//       description: |-
//         Probably related to booting the game?
//
//         This function prints the debug message "task proc boot".
//
//         No params.
//     - name: EnableAllInterrupts
//       address:
//         NA: 0x2003608
//         EU: 0x2003608
//       description: |-
//         Sets the Interrupt Master Enable (IME) register to 1, which enables all CPU interrupts (if enabled in the Interrupt Enable (IE) register).
//
//         See https://problemkaputt.de/gbatek.htm#dsiomaps.
//
//         return: old value in the IME register
//     - name: GetTime
//       address:
//         NA: 0x20037B4
//         EU: 0x20037B4
//       description: |-
//         Seems to get the current (system?) time as an IEEE 754 floating-point number.
//
//         return: current time (maybe in seconds?)
//     - name: DisableAllInterrupts
//       address:
//         NA: 0x2003824
//         EU: 0x2003824
//       description: |-
//         Sets the Interrupt Master Enable (IME) register to 0, which disables all CPU interrupts (even if enabled in the Interrupt Enable (IE) register).
//
//         See https://problemkaputt.de/gbatek.htm#dsiomaps.
//
//         return: old value in the IME register
//     - name: SoundResume
//       address:
//         NA: 0x2003CC4
//         EU: 0x2003CC4
//       description: |-
//         Probably resumes the sound player if paused?
//
//         This function prints the debug string "sound resume".
//     - name: CardPullOutWithStatus
//       address:
//         NA: 0x2003D2C
//         EU: 0x2003D2C
//       description: |-
//         Probably aborts the program with some status code? It seems to serve a similar purpose to the exit(3) function.
//
//         This function prints the debug string "card pull out %d" with the status code.
//
//         r0: status code
//     - name: CardPullOut
//       address:
//         NA: 0x2003D70
//         EU: 0x2003D70
//       description: |-
//         Sets some global flag that probably triggers system exit?
//
//         This function prints the debug string "card pull out".
//
//         No params.
//     - name: CardBackupError
//       address:
//         NA: 0x2003D94
//         EU: 0x2003D94
//       description: |-
//         Sets some global flag that maybe indicates a save error?
//
//         This function prints the debug string "card backup error".
//
//         No params.
//     - name: HaltProcessDisp
//       address:
//         NA: 0x2003DB8
//         EU: 0x2003DB8
//       description: |-
//         Maybe halts the process display?
//
//         This function prints the debug string "halt process disp %d" with the status code.
//
//         r0: status code
//     - name: EuclideanNorm
//       address:
//         NA:
//           - 0x2005050
//           - 0x20050B0
//         EU:
//           - 0x2005050
//           - 0x20050B0
//       description: |-
//         Computes the Euclidean norm of a two-component integer array, sort of like hypotf(3).
//
//         r0: integer array [x, y]
//         return: sqrt(x*x + y*y)
//     - name: ClampComponentAbs
//       address:
//         NA: 0x2005110
//         EU: 0x2005110
//       description: |-
//         Clamps the absolute values in a two-component integer array.
//
//         Given an integer array [x, y] and a maximum absolute value M, clamps each element of the array to M such that the output array is [min(max(x, -M), M), min(max(y, -M), M)].
//
//         r0: 2-element integer array, will be mutated
//         r1: max absolute value
//     - name: KeyWaitInit
//       address:
//         NA: 0x2006DA4
//         EU: 0x2006DA4
//       description: |-
//         Implements (most of?) SPECIAL_PROC_KEY_WAIT_INIT (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: DataTransferInit
//       address:
//         NA: 0x2008168
//         EU: 0x2008168
//       description: |-
//         Initializes data transfer mode to get data from the ROM cartridge.
//
//         No params.
//     - name: DataTransferStop
//       address:
//         NA: 0x2008194
//         EU: 0x2008194
//       description: |-
//         Finalizes data transfer from the ROM cartridge.
//
//         This function must always be called if DataTransferInit was called, or the game will crash.
//
//         No params.
//     - name: GetDebugFlag1
//       address:
//         NA: 0x200C110
//         EU: 0x200C198
//       description: Just returns 0 in the final binary.
//     - name: SetDebugFlag1
//       address:
//         NA: 0x200C118
//         EU: 0x200C1A0
//       description: A no-op in the final binary.
//     - name: AppendProgPos
//       address:
//         NA: 0x200C120
//         EU: 0x200C1A8
//       description: |-
//         Write a base message into a string and append the file name and line number to the end in the format "file = '%s'  line = %5d\n".
//
//         If no program position info is given, "ProgPos info NULL\n" is appended instead.
//
//         r0: [output] str
//         r1: program position info
//         r2: base message
//         return: number of characters printed, excluding the null-terminator
//     - name: GetDebugFlag2
//       address:
//         NA: 0x200C234
//         EU: 0x200C2BC
//       description: Just returns 0 in the final binary.
//     - name: SetDebugFlag2
//       address:
//         NA: 0x200C23C
//         EU: 0x200C2C4
//       description: A no-op in the final binary.
//     - name: IsAuraBow
//       address:
//         NA: 0x200CC14
//         EU: 0x200CC9C
//       description: |-
//         Checks if an item is one of the aura bows received at the start of the game.
//
//         r0: item ID
//         return: bool
//     - name: SetMoneyCarried
//       address:
//         NA: 0x200ED1C
//         EU: 0x200EDC4
//       description: |-
//         Sets the amount of money the player is carrying, clamping the value to the range [0, MAX_MONEY_CARRIED].
//
//         r0: new value
//     - name: IsBagFull
//       address:
//         NA: 0x200EDC0
//         EU: 0x200EE68
//       description: |-
//         Implements SPECIAL_PROC_IS_BAG_FULL (see ScriptSpecialProcessCall).
//
//         return: bool
//     - name: CountItemTypeInBag
//       address:
//         NA: 0x200EE88
//         EU: 0x200EF30
//       description: |-
//         Implements SPECIAL_PROC_COUNT_ITEM_TYPE_IN_BAG (see ScriptSpecialProcessCall).
//
//         r0: item ID
//         return: number of items of the specified ID in the bag
//     - name: AddItemToBag
//       address:
//         NA: 0x200F84C
//         EU: 0x200F8F4
//       description: |-
//         Implements SPECIAL_PROC_ADD_ITEM_TO_BAG (see ScriptSpecialProcessCall).
//
//         r0: pointer to an owned_item
//         return: bool
//     - name: ScriptSpecialProcess0x39
//       address:
//         NA: 0x200FD54
//         EU: 0x200FDFC
//       description: |-
//         Implements SPECIAL_PROC_0x39 (see ScriptSpecialProcessCall).
//
//         return: bool
//     - name: CountItemTypeInStorage
//       address:
//         NA: 0x200FEE4
//         EU: 0x200FF8C
//       description: |-
//         Implements SPECIAL_PROC_COUNT_ITEM_TYPE_IN_STORAGE (see ScriptSpecialProcessCall).
//
//         r0: pointer to an owned_item
//         return: number of items of the specified ID in storage
//     - name: RemoveItemsTypeInStorage
//       address:
//         NA: 0x20101E4
//         EU: 0x201028C
//       description: |-
//         Probably? Implements SPECIAL_PROC_0x2A (see ScriptSpecialProcessCall).
//
//         r0: pointer to an owned_item
//         return: bool
//     - name: AddItemToStorage
//       address:
//         NA: 0x201031C
//         EU: 0x20103C4
//       description: |-
//         Implements SPECIAL_PROC_ADD_ITEM_TO_STORAGE (see ScriptSpecialProcessCall).
//
//         r0: pointer to an owned_item
//         return: bool
//     - name: SetMoneyStored
//       address:
//         NA: 0x2010724
//         EU: 0x20107CC
//       description: |-
//         Sets the amount of money the player has stored in the Duskull Bank, clamping the value to the range [0, MAX_MONEY_STORED].
//
//         r0: new value
//     - name: GetExclusiveItemOffset
//       address:
//         NA: 0x2010E40
//         EU: 0x2010EE8
//       description: |-
//         Gets the exclusive item offset, which is the item ID relative to that of the first exclusive item, the Prism Ruff.
//
//         r0: item ID
//         return: offset
//     - name: ApplyExclusiveItemStatBoosts
//       address:
//         NA: 0x2010E64
//         EU: 0x2010F0C
//       description: |-
//         Applies stat boosts from an exclusive item.
//
//         r0: item ID
//         r1: pointer to attack stat to modify
//         r2: pointer to special attack stat to modify
//         r3: pointer to defense stat to modify
//         stack[0]: pointer to special defense stat to modify
//     - name: SetExclusiveItemEffect
//       address:
//         NA: 0x2010F80
//         EU: 0x2011028
//       description: |-
//         Sets the bit for an exclusive item effect.
//
//         r0: pointer to the effects bitvector to modify
//         r1: exclusive item effect ID
//     - name: ExclusiveItemEffectFlagTest
//       address:
//         NA: 0x2010FA4
//         EU: 0x201104C
//       description: |-
//         Tests the exclusive item bitvector for a specific exclusive item effect.
//
//         r0: the effects bitvector to test
//         r1: exclusive item effect ID
//         return: bool
//     - name: GetMoveTargetAndRange
//       address:
//         NA: 0x2013840
//         EU: 0x20138E8
//       description: |-
//         Gets the move target-and-range field. See struct move_target_and_range in the C headers.
//
//         r0: move pointer
//         r1: AI flag (every move has two target-and-range fields, one for players and one for AI)
//         return: move target and range
//     - name: GetMoveBasePower
//       address:
//         NA: 0x20139CC
//         EU: 0x2013A74
//       description: |-
//         Gets the base power of a move from the move data table.
//
//         r0: move pointer
//         return: base power
//     - name: GetMaxPp
//       address:
//         NA: 0x2013A50
//         EU: 0x2013AF8
//       description: |-
//         Gets the maximum PP for a given move.
//
//         r0: move pointer
//         return: max PP for the given move, capped at 99
//     - name: GetMoveCritChance
//       address:
//         NA: 0x2013B10
//         EU: 0x2013BB8
//       description: |-
//         Gets the critical hit chance of a move.
//
//         r0: move pointer
//         return: base power
//     - name: IsRecoilMove
//       address:
//         NA: 0x2013E14
//       description: |-
//         Checks if the given move is a recoil move (affected by Reckless).
//
//         r0: move ID
//         return: bool
//     - name: IsPunchMove
//       address:
//         NA: 0x2014D18
//       description: |-
//         Checks if the given move is a punch move (affected by Iron Fist).
//
//         r0: move ID
//         return: bool
//     - name: GetMoveCategory
//       address:
//         NA: 0x20151C8
//         EU: 0x2015270
//       description: |-
//         Gets a move's category (physical, special, status).
//
//         r0: move ID
//         return: move category enum
//     - name: LoadWteFromRom
//       address:
//         NA: 0x201DE4C
//         EU: 0x201DEE8
//       description: |-
//         Loads a SIR0-wrapped WTE file from ROM, and returns a handle to it
//
//         r0: [output] pointer to wte handle
//         r1: file path string
//         r2: load file flags
//     - name: LoadWteFromFileDirectory
//       address:
//         NA: 0x201DEC4
//         EU: 0x201DF60
//       description: |-
//         Loads a SIR0-wrapped WTE file from a file directory, and returns a handle to it
//
//         r0: [output] pointer to wte handle
//         r1: file directory id
//         r2: file index
//         r3: malloc flags
//     - name: UnloadWte
//       address:
//         NA: 0x201DF18
//         EU: 0x201DFB4
//       description: |-
//         Frees the buffer used to store the WTE data in the handle, and sets both pointers to null
//
//         r0: pointer to wte handle
//     - name: HandleSir0Translation
//       address:
//         NA: 0x201F4B4
//         EU: 0x201F550
//       description: |-
//         Translates the offsets in a SIR0 file into NDS memory addresses, changes the magic number to SirO (opened), and returns a pointer to the first pointer specified in the SIR0 header (beginning of the data).
//
//         r0: [output] double pointer to beginning of data
//         r1: pointer to source file buffer
//     - name: HandleSir0TranslationVeneer
//       address:
//         NA: 0x201F58C
//       description: |-
//         Likely a linker-generated veneer for HandleSir0Translation.
//
//         See https://developer.arm.com/documentation/dui0474/k/image-structure-and-generation/linker-generated-veneers/what-is-a-veneer-
//
//         r0: [output] double pointer to beginning of data
//         r1: pointer to source file buffer
//     - name: GetLanguageType
//       address:
//         NA: 0x20205A0
//         EU: 0x2020688
//       description: |-
//         Gets the language type.
//
//         This is the value backing the special LANGUAGE_TYPE script variable.
//
//         return: language type
//     - name: GetLanguage
//       address:
//         NA: 0x20205B0
//         EU: 0x20206B0
//       description: |-
//         Gets the single-byte language ID of the current program.
//
//         The language ID appears to be used to index some global tables.
//
//         return: language ID
//     - name: PreprocessString
//       address:
//         NA: 0x20223F0
//         EU: 0x20225EC
//       description: |-
//         An enhanced sprintf, which recognizes certain tags and replaces them with appropiate game values.
//         This function can also be used to simply insert values passed within the preprocessor args
//
//         The tags utilized for this function are lowercase, it might produce uppercase tags
//         that only are used when the text is being typewrited into a message box
//
//         r0: [output] formatted string
//         r1: maximum capacity of the output buffer
//         r2: input format string
//         r3: preprocessor flags
//         stack[0]: pointer to preprocessor args
//     - name: StringFromMessageId
//       address:
//         NA: 0x20258C4
//         EU: 0x2025B90
//       description: |-
//         Gets the string corresponding to a given message ID.
//
//         r0: message ID
//         return: string from the string files with the given message ID
//     - name: SetScreenWindowsColor
//       address:
//         NA: 0x2027A68
//         EU: 0x2027D5C
//       description: |-
//         Sets the palette of the frames of windows in the specified screen
//
//         r0: palette index
//         r1: is upper screen
//     - name: SetBothScreensWindowsColor
//       address:
//         NA: 0x2027A80
//         EU: 0x2027D74
//       description: |-
//         Sets the palette of the frames of windows in both screens
//
//         r0: palette index
//     - name: GetNotifyNote
//       address:
//         NA: 0x20484A0
//         EU: 0x20487BC
//       description: |-
//         Returns the current value of NOTIFY_NOTE.
//
//         return: bool
//     - name: SetNotifyNote
//       address:
//         NA: 0x20484B0
//         EU: 0x20487CC
//       description: |-
//         Sets NOTIFY_NOTE to the given value.
//
//         r0: bool
//     - name: InitMainTeamAfterQuiz
//       address:
//         NA: 0x20487C4
//         EU: 0x2048AE0
//       description: |-
//         Implements SPECIAL_PROC_INIT_MAIN_TEAM_AFTER_QUIZ (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: ScriptSpecialProcess0x3
//       address:
//         NA: 0x2048A0C
//         EU: 0x2048D28
//       description: |-
//         Implements SPECIAL_PROC_0x3 (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: ScriptSpecialProcess0x4
//       address:
//         NA: 0x2048A84
//         EU: 0x2048DA0
//       description: |-
//         Implements SPECIAL_PROC_0x4 (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: NoteSaveBase
//       address:
//         NA: 0x2048F84
//         EU: 0x20492A0
//       description: |-
//         Probably related to saving or quicksaving?
//
//         This function prints the debug message "NoteSave Base %d %d" with some values. It's also the only place where GetRngSeed is called.
//
//         r0: possibly a flag/code that controls the type of save file to generate?
//         others: ?
//         return: status code
//     - name: NoteLoadBase
//       address:
//         NA: 0x2049370
//         EU: 0x20496A8
//       description: |-
//         Probably related to loading a save file or quicksave?
//
//         This function prints the debug message "NoteLoad Base %d" with some value. It's also the only place where SetRngSeed is called.
//
//         return: status code
//     - name: GetGameMode
//       address:
//         NA: 0x204AFC0
//       description: |-
//         Gets the value of GAME_MODE.
//
//         return: game mode
//     - name: InitScriptVariableValues
//       address:
//         NA: 0x204B04C
//         EU: 0x204B384
//       description: |-
//         Initialize the script variable values table (SCRIPT_VARS_VALUES).
//
//         The whole table is first zero-initialized. Then, all script variable values are first initialized to their defaults, after which some of them are overwritten with other hard-coded values.
//
//         No params.
//     - name: InitEventFlagScriptVars
//       address:
//         NA: 0x204B304
//         EU: 0x204B63C
//       description: |-
//         Initializes an assortment of event flag script variables (see the code for an exhaustive list).
//
//         No params.
//     - name: ZinitScriptVariable
//       address:
//         NA: 0x204B434
//         EU: 0x204B76C
//       description: |-
//         Zero-initialize the values of the given script variable.
//
//         r0: pointer to the local variable table (only needed if id >= VAR_LOCAL0)
//         r1: script variable ID
//     - name: LoadScriptVariableRaw
//       address:
//         NA: 0x204B49C
//         EU: 0x204B7D4
//       description: |-
//         Loads a script variable descriptor for a given ID.
//
//         r0: [output] script variable descriptor pointer
//         r1: pointer to the local variable table (doesn't need to be valid; just controls the output value pointer)
//         r2: script variable ID
//     - name: LoadScriptVariableValue
//       address:
//         NA: 0x204B4EC
//         EU: 0x204B824
//       description: |-
//         Loads the value of a script variable.
//
//         r0: pointer to the local variable table (only needed if id >= VAR_LOCAL0)
//         r1: script variable ID
//         return: value
//     - name: LoadScriptVariableValueAtIndex
//       address:
//         NA: 0x204B678
//         EU: 0x204B9B0
//       description: |-
//         Loads the value of a script variable at some index (for script variables that are arrays).
//
//         r0: pointer to the local variable table (only needed if id >= VAR_LOCAL0)
//         r1: script variable ID
//         r2: value index for the given script var
//         return: value
//     - name: SaveScriptVariableValue
//       address:
//         NA: 0x204B820
//         EU: 0x204BB58
//       description: |-
//         Saves the given value to a script variable.
//
//         r0: pointer to local variable table (only needed if id >= VAR_LOCAL0)
//         r1: script variable ID
//         r2: value to save
//     - name: SaveScriptVariableValueAtIndex
//       address:
//         NA: 0x204B988
//         EU: 0x204BCC0
//       description: |-
//         Saves the given value to a script variable at some index (for script variables that are arrays).
//
//         r0: pointer to local variable table (only needed if id >= VAR_LOCAL0)
//         r1: script variable ID
//         r2: value index for the given script var
//         r3: value to save
//     - name: LoadScriptVariableValueSum
//       address:
//         NA: 0x204BB00
//         EU: 0x204BE38
//       description: |-
//         Loads the sum of all values of a given script variable (for script variables that are arrays).
//
//         r0: pointer to the local variable table (only needed if id >= VAR_LOCAL0)
//         r1: script variable ID
//         return: sum of values
//     - name: LoadScriptVariableValueBytes
//       address:
//         NA: 0x204BB64
//         EU: 0x204BE9C
//       description: |-
//         Loads some number of bytes from the value of a given script variable.
//
//         r0: script variable ID
//         r1: [output] script variable value bytes
//         r2: number of bytes to load
//     - name: SaveScriptVariableValueBytes
//       address:
//         NA: 0x204BBCC
//         EU: 0x204BF04
//       description: |-
//         Saves some number of bytes to the given script variable.
//
//         r0: script variable ID
//         r1: bytes to save
//         r2: number of bytes
//     - name: ScriptVariablesEqual
//       address:
//         NA: 0x204BC18
//         EU: 0x204BF50
//       description: |-
//         Checks if two script variables have equal values. For arrays, compares elementwise for the length of the first variable.
//
//         r0: pointer to the local variable table (only needed if id >= VAR_LOCAL0)
//         r1: script variable ID 1
//         r2: script variable ID 2
//         return: true if values are equal, false otherwise
//     - name: EventFlagBackup
//       address:
//         NA: 0x204C1E4
//         EU: 0x204C51C
//       description: |-
//         Saves event flag script variables (see the code for an exhaustive list) to their respective BACKUP script variables, but only in certain game modes.
//
//         This function prints the debug string "EventFlag BackupGameMode %d" with the game mode.
//
//         No params.
//     - name: DumpScriptVariableValues
//       address:
//         NA: 0x204C408
//         EU: 0x204C740
//       description: |-
//         Runs EventFlagBackup, then copies the script variable values table (SCRIPT_VARS_VALUES) to the given pointer.
//
//         r0: destination pointer for the data dump
//         return: always 1
//     - name: RestoreScriptVariableValues
//       address:
//         NA: 0x204C430
//         EU: 0x204C768
//       description: |-
//         Restores the script variable values table (SCRIPT_VARS_VALUES) with the given data. The source data is assumed to be exactly 1024 bytes in length.
//
//         r0: raw data to copy to the values table
//         return: whether the restored value for VAR_VERSION is equal to its default value
//     - name: InitScenarioScriptVars
//       address:
//         NA: 0x204C488
//         EU: 0x204C7C0
//       description: |-
//         Initializes most of the SCENARIO_* script variables (except SCENARIO_TALK_BIT_FLAG for some reason). Also initializes the PLAY_OLD_GAME variable.
//
//         No params.
//     - name: SetScenarioScriptVar
//       address:
//         NA: 0x204C618
//         EU: 0x204C950
//       description: |-
//         Sets the given SCENARIO_* script variable with a given pair of values [val0, val1].
//
//         In the special case when the ID is VAR_SCENARIO_MAIN, and the set value is different from the old one, the REQUEST_CLEAR_COUNT script variable will be set to 0.
//
//         r0: script variable ID
//         r1: val0
//         r2: val1
//     - name: GetSpecialEpisodeType
//       address:
//         NA: 0x204C8EC
//         EU: 0x204CC24
//       description: |-
//         Gets the special episode type from the SPECIAL_EPISODE_TYPE script variable.
//
//         return: special episode type
//     - name: ScenarioFlagBackup
//       address:
//         NA: 0x204CCB8
//         EU: 0x204CFF0
//       description: |-
//         Saves scenario flag script variables (SCENARIO_SELECT, SCENARIO_MAIN_BIT_FLAG) to their respective BACKUP script variables, but only in certain game modes.
//
//         This function prints the debug string "ScenarioFlag BackupGameMode %d" with the game mode.
//
//         No params.
//     - name: InitWorldMapScriptVars
//       address:
//         NA: 0x204CD88
//         EU: 0x204D0C0
//       description: |-
//         Initializes the WORLD_MAP_* script variable values (IDs 0x55-0x57).
//
//         No params.
//     - name: InitDungeonListScriptVars
//       address:
//         NA: 0x204CE90
//         EU: 0x204D1C8
//       description: |-
//         Initializes the DUNGEON_*_LIST script variable values (IDs 0x4f-0x54).
//
//         No params.
//     - name: ScriptSpecialProcess0x3A
//       address:
//         NA: 0x204FC28
//         EU: 0x204FF60
//       description: |-
//         Implements SPECIAL_PROC_0x3A (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: ScriptSpecialProcess0x3B
//       address:
//         NA: 0x204FEC8
//         EU: 0x2050200
//       description: |-
//         Implements SPECIAL_PROC_0x3B (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: SendSkyGiftToGuildmaster
//       address:
//         NA: 0x204FF80
//         EU: 0x20502B8
//       description: |-
//         Implements SPECIAL_PROC_SEND_SKY_GIFT_TO_GUILDMASTER (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: SubFixedPoint
//       address:
//         NA: 0x2050F10
//         EU: 0x2051248
//       description: |-
//         Compute the subtraction of two decimal fixed point floats.
//
//         Floats are in the format {16-bit integer part, 16-bit thousandths}, where the integer part is the lower word. Probably used primarily for belly.
//
//         r0: float
//         r1: decrement
//         return: max(float - decrement, 0)
//     - name: BinToDecFixedPoint
//       address:
//         NA: 0x2051020
//         EU: 0x2051358
//       description: |-
//         Convert a Q16.16 binary fixed-point float to the decimal fixed-point float used for belly calculations. Thousandths are floored.
//
//         If <data> holds the raw binary data, a Q16.16 binary fixed-point float has the value ((unsigned)data) * 2^-16), and the decimal fixed-point float used for belly has the value (data & 0xffff) + (data >> 16)/1000.
//
//         r0: pointer p, where ((const unsigned *)p)[1] is the float in Q16.16 format to convert
//         return: float in decimal fixed-point format
//     - name: CeilFixedPoint
//       address:
//         NA: 0x2051064
//         EU: 0x205139C
//       description: |-
//         Compute the ceiling of a decimal fixed point float.
//
//         Floats are in the format {16-bit integer part, 16-bit thousandths}, where the integer part is the lower word. Probably used primarily for belly.
//
//         r0: float
//         return: ceil(float)
//     - name: DungeonGoesUp
//       address:
//         NA: 0x2051288
//         EU: 0x20515C0
//       description: |-
//         Returns whether the specified dungeon is considered as going upward or not
//
//         r0: dungeon id
//         return: bool
//     - name: IsUnown
//       address:
//         NA: 0x2054A88
//         EU: 0x2054E04
//       description: |-
//         Checks if a monster ID is an Unown.
//
//         r0: monster ID
//         return: bool
//     - name: IsShaymin
//       address:
//         NA: 0x2054AA4
//         EU: 0x2054E20
//       description: |-
//         Checks if a monster ID is a Shaymin form.
//
//         r0: monster ID
//         return: bool
//     - name: IsCastform
//       address:
//         NA: 0x2054AD4
//         EU: 0x2054E50
//       description: |-
//         Checks if a monster ID is a Castform form.
//
//         r0: monster ID
//         return: bool
//     - name: IsCherrim
//       address:
//         NA: 0x2054B2C
//         EU: 0x2054EA8
//       description: |-
//         Checks if a monster ID is a Cherrim form.
//
//         r0: monster ID
//         return: bool
//     - name: IsDeoxys
//       address:
//         NA: 0x2054B74
//         EU: 0x2054EF0
//       description: |-
//         Checks if a monster ID is a Deoxys form.
//
//         r0: monster ID
//         return: bool
//     - name: IsMonsterOnTeam
//       address:
//         NA: 0x2055148
//         EU: 0x20554C4
//       description: |-
//         Checks if a given monster is on the exploration team (not necessarily the active party)?
//
//         r0: monster ID
//         r1: ?
//         return: bool
//     - name: SetTeamSetupHeroAndPartnerOnly
//       address:
//         NA: 0x20569CC
//         EU: 0x2056D48
//       description: |-
//         Implements SPECIAL_PROC_SET_TEAM_SETUP_HERO_AND_PARTNER_ONLY (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: SetTeamSetupHeroOnly
//       address:
//         NA: 0x2056AB0
//         EU: 0x2056E2C
//       description: |-
//         Implements SPECIAL_PROC_SET_TEAM_SETUP_HERO_ONLY (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: GetPartyMembers
//       address:
//         NA: 0x2056C20
//         EU: 0x2056F9C
//       description: |-
//         Appears to get the team's active party members. Implements most of SPECIAL_PROC_IS_TEAM_SETUP_SOLO (see ScriptSpecialProcessCall).
//
//         r0: [output] Array of 4 2-byte values (they seem to be indexes of some sort) describing each party member, which will be filled in by the function. The input can be a null pointer if the party members aren't needed
//         return: Number of party members
//     - name: IqSkillFlagTest
//       address:
//         NA: 0x2058F04
//         EU: 0x2059280
//       description: |-
//         Tests whether an IQ skill with a given ID is active.
//
//         r0: IQ skill bitvector to test
//         r1: IQ skill ID
//         return: bool
//     - name: GetSosMailCount
//       address:
//         NA: 0x205B97C
//         EU: 0x205BCF8
//       description: |-
//         Implements SPECIAL_PROC_GET_SOS_MAIL_COUNT (see ScriptSpecialProcessCall).
//
//         r0: ?
//         r1: some flag?
//         return: SOS mail count
//     - name: DungeonRequestsDone
//       address:
//         NA: 0x205EDA4
//         EU: 0x205F120
//       description: |-
//         Seems to return the number of missions completed.
//
//         Part of the implementation for SPECIAL_PROC_DUNGEON_HAD_REQUEST_DONE (see ScriptSpecialProcessCall).
//
//         r0: ?
//         r1: some flag?
//         return: number of missions completed
//     - name: DungeonRequestsDoneWrapper
//       address:
//         NA: 0x205EE10
//       description: |-
//         Calls DungeonRequestsDone with the second argument set to false.
//
//         r0: ?
//         return: number of mission completed
//     - name: AnyDungeonRequestsDone
//       address:
//         NA: 0x205EE20
//         EU: 0x205F19C
//       description: |-
//         Calls DungeonRequestsDone with the second argument set to true, and converts the integer output to a boolean.
//
//         r0: ?
//         return: bool: whether the number of missions completed is greater than 0
//     - name: ScriptSpecialProcess0x3D
//       address:
//         NA: 0x2065B50
//         EU: 0x2065ECC
//       description: |-
//         Implements SPECIAL_PROC_0x3D (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: ScriptSpecialProcess0x3E
//       address:
//         NA: 0x2065B60
//         EU: 0x2065EDC
//       description: |-
//         Implements SPECIAL_PROC_0x3E (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: ScriptSpecialProcess0x17
//       address:
//         NA: 0x2065C48
//         EU: 0x2065FC4
//       description: |-
//         Implements SPECIAL_PROC_0x17 (see ScriptSpecialProcessCall).
//
//         No params.
//     - name: ItemAtTableIdx
//       address:
//         NA: 0x2065CF8
//         EU: 0x2066074
//       description: |-
//         Gets info about the item at a given item table (not sure what this table is...) index.
//
//         Used by SPECIAL_PROC_COUNT_TABLE_ITEM_TYPE_IN_BAG and friends (see ScriptSpecialProcessCall).
//
//         r0: table index
//         r1: [output] pointer to an owned_item
//     - name: WaitForInterrupt
//       address:
//         NA: 0x207BC30
//         EU: 0x207BFC8
//       description: |-
//         Presumably blocks until the program receives an interrupt.
//
//         This just calls (in Ghidra terminology) coproc_moveto_Wait_for_interrupt(0). See https://en.wikipedia.org/wiki/ARM_architecture_family#Coprocessors.
//
//         No params.
