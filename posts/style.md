tags =  ["magnusi"]
title = "Recommended style guidelines"

## Allsucks coding style
Allsucks is a style which emphasizes consistency and conciseness.
It is based on the suckless coding style, it agrees with many of its points,
extends some and contradicts a few. The name is a combination of Allman
indentation style and the suckless coding style. Certain points are a
direct copy-pasta of the suckless style. Credits to their respective
authors.

The style is mainly intended for the C language, but some of the rules
should be applicable to other programming languages. I may add extensions later
on for other languages which differ from C more.

### Note
Much like the suckless coding style, these are guidelines and the most
important aspect of style is consistency. Do try to keep your style
consistent with whichever project you are correctly working on. It doesn't
hurt to keep all your projects style-consistent with each other as well.

### File layout
- Comment with license and possibly a short description of the tool/file/library.
  - If you don't expect your project's files to be taken out often and
    have a global LICENSE file, this can be omitted
- Headers
- Macros
- Types
- Function declarations
  - Can be left out in short files
  - Group/order in a logical manner
  - For functions with long signatures, include parameter names,
    otherwise, they can be omitted, but be consistent with where
- Global variables
- Function definitions in the same order as declarations
  - Follow the same grouping as declarations
- The main function
- The help function (optional)

### C Features
- C99 without extensions
  - When compiling with gcc use __-std=c99 -pedantic__
- POSIX.1-2008
  - Define either __\_POSIX\_C\_SOURCE 200809L__ or __\_XOPEN\_SOURCE 700__
- Do not mix declarations and code
- Only use initial for loop declaration for integer types and only when necessary
- Variadic macros are acceptable, but remember:
  - __\_\_VA\_ARGS\_\___ not a named parameter
  - Arg list can't be empty

### Comments
- Use __/* */__ for long comments, __//__ for short ones
- Delimit sections of your code with this:

```c
/*
** My section
*/
```

- Multi-line comments should have each non-delimiting line starting with __' \*'__
- __*/__ on its own line
- Do not mix comments into code, always use line above or end of the line for
  very short comments

### Blocks
- Place all declarations at the top of the block they belong to
- __{__ on the next line at the same indentation level as the previous line
- __}__ on the next line regardless of whether continuing a statement like if-else, or not
- Use blocks for single statements iff:
  - Inner statement needs a block
  - Another branch of the same statement needs a block

### Leading white-space
- Use tabs for indentation (recommended tab size is 4 for viewing)
- Use spaces for alignment
  - No tabs except beginning of the line
  - Everything aligns regardless of tab size
- Use spaces for multi-line-macros everywhere, indentation level 0 is where the define began

### Trailing white-space
- Delete it

### Functions
- Return type, modifiers, name and arguments on the same line
  - If the line would be more than 90 characters, break off return type and modifiers
  - For even longer, break off all arguments to their own line, starting with the second one,
    aligned to the open parenthesis
- Functions not used outside their translation unit should be declared as static

### Variables
- Global variables not used outside translation unit should be declared as static
- In declarations of pointers, asterisk (\*) is adjacent to the type, not identifier
- Strive to avoid global variables unless absolutely necessary

### Keywords
- Use a space after __if, for, while, switch__, they are not function calls
- Do not use space after opening __(__ and before closing __)__
- Always use __()__ with __sizeof__
- Never use __()__ with return
- Do not put a space after __sizeof__, it behaves like a function call

### Switch
- Do not indent cases another level
- Comment cases that FALLTHROUGH
- Very short single-statement cases can be put whole on one line
- Always put default case last, unless you need it to fall through

### Headers
- Place system/libc headers first in alphabetical order
  or descending by length :^)
- Place local headers after an empty line, using the same ordering
- Comment when headers must be included in a specific order
- When writing local headers
  - Use #ifndef guard if you want, but always ensure they are included
    when and where they are needed

### User-defined types
- Do not use __type\_t__ naming, since it is reserved by POSIX
- Typedef structs
- Do not typedef built-in types
- Either don't capitalize the name or capitalize them across the whole project
- Typedef the name, but don't name the struct you are typedefing
- Strive to avoid nested structs, in most cases you don't need them

### Line length
- Keep lines to a reasonable length, recommended ~90-100 characters,
  but defining reasonable length is up to the programmer
- If your lines are too long, your code is probably to complex

### Tests and booleans
- Do not test against NULL explicitly
- Do not test against 0 explicitly (strcmp() et al. being an exception)
- Do not use __bool__ type, instead, stick to consistently sized integer types
- Assign variables at declaration if possible
- Otherwise use compound assignment and tests unless the line grows too long

### Handling errors
- When functions return -1 on error compare against 0
- Use goto to cleanup when nested blocks would get too convoluted
- Return or exit early on failures instead of multiple nested levels
- Unreachable code should be marked with a comment
- Think long and hard on whether or not you should cleanup on fatal errors

### Enums vs #define
- Use enums for values that are grouped semantically

```c
##define MAXSZ 4096
##define MAGIC1 0xdeadbeef
enum {
  DIRECTION_X,
	DIRECTION_Y,
	DIRECTION_Z
};
```

### Misc
- Try to avoid global variables
- Strive for reasonable portability, definition of reasonable is up to the
  programmer
- Avoid memory leaks and depending on undefined behavior
- Always declare pointers which are not going to change as __const__
- Don't overuse malloc(), sometimes using stack will work just as fine
- If applicable, use sized integer types from __stdint.h__
- If you have to use the __bool__ type (for example because a library requires it),
  don't compare against __true__ and __false__ directly
- DRY
- KISS
