#pragma once

#define COT_LOG_CAT_DEFAULT "cot"
#define COT_LOG_CAT_SPECIAL_PROCESS "cot.special_process"
#define COT_LOG_CAT_EFFECTS "cot.effects"
#define COT_LOG_CAT_INSTRUCTIONS "cot.ground_instructions"
#define COT_LOG_CAT_MENUS "cot.script_menus"

// Needs two macros for some reason
#define _COT_INTERNAL_STRINGIZE_DETAIL(x) #x
#define _COT_INTERNAL_STRINGIZE(x) _COT_INTERNAL_STRINGIZE_DETAIL(x)

#ifndef NDEBUG

#define _COT_INTERNAL_LOG_MESSAGE(category, format) \
  "[" category "] " format " (" __FILE__ ":" _COT_INTERNAL_STRINGIZE(__LINE__) ")"

#define COT_LOG(category, format)           DebugPrint(0, _COT_INTERNAL_LOG_MESSAGE(category, format))
#define COT_WARN(category, format)          DebugPrint(1, _COT_INTERNAL_LOG_MESSAGE(category, format))
#define COT_ERROR(category, format)         DebugPrint(2, _COT_INTERNAL_LOG_MESSAGE(category, format))

#define COT_LOGFMT(category, format, ...)   DebugPrint(0, _COT_INTERNAL_LOG_MESSAGE(category, format), __VA_ARGS__)
#define COT_WARNFMT(category, format, ...)  DebugPrint(1, _COT_INTERNAL_LOG_MESSAGE(category, format), __VA_ARGS__)
#define COT_ERRORFMT(category, format, ...) DebugPrint(2, _COT_INTERNAL_LOG_MESSAGE(category, format), __VA_ARGS__)

#define COT_ASSERT(expr) \
  if (!(expr)) {\
    DebugPrint(2, "ASSERTION FAILED: " #expr " (" __FILE__ ":" _COT_INTERNAL_STRINGIZE(__LINE__) ")"); \
    WaitForever(); \
  }

#else

#define COT_LOG(category, format, ...)
#define COT_WARN(category, format, ...)
#define COT_ERROR(category, format, ...)

#define COT_LOGFMT(category, format, ...)
#define COT_WARNFMT(category, format, ...)
#define COT_ERRORFMT(category, format, ...)

#define COT_ASSERT(expr)

#endif
