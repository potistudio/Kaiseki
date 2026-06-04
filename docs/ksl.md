# KSL — Kaiseki Script Language Specification

**Version 0.1**  
**Purpose:** A high-level pseudocode notation for decompiled binary analysis.  
**Scope:** This document is the authoritative reference for reading, writing, and generating KSL code.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Design Philosophy](#2-design-philosophy)
3. [Lexical Elements](#3-lexical-elements)
4. [Type System](#4-type-system)
5. [Top-Level Declarations](#5-top-level-declarations)
6. [Statements](#6-statements)
7. [Expressions](#7-expressions)
8. [Decompiled C → KSL Pattern Reference](#8-decompiled-c--ksl-pattern-reference)
9. [Formal Grammar (EBNF)](#9-formal-grammar-ebnf)
10. [Worked Example](#10-worked-example)

---

## 1. Overview

KSL (Kaiseki Script Language) is a pseudocode language designed to express decompiled binary code at a higher level of abstraction. It targets the output of decompilers such as **Ghidra** and **IDA Pro**.

KSL is **not executable** — it is an annotation and documentation language intended for human readers and LLM-assisted analysis. Its primary goals are:

- Eliminate repetitive low-level noise (HRESULT checks, null-pointer guards, vtable arithmetic)
- Make structure explicit (type layouts, virtual dispatch, nullable semantics)
- Preserve all semantics of the original machine code in readable form

### Scope of input

KSL targets C-style decompiled pseudo-code that contains:

| Pattern | Example |
|---------|---------|
| Unknown types | `undefined8`, `undefined4`, `CONCAT44(...)` |
| HRESULT error propagation | `iVar5 = f(...); if (iVar5 != 0) { _CxxThrowException(...) }` |
| Raw vtable dispatch | `(**(code **)(*(longlong *)this + 0x170))(this)` |
| Offset-based field access | `*(BEE_Item **)(param_1 + 0x290)` |
| goto labels | `goto LAB_180e2b5da` |
| Mangled names | `?NIM_GetStreamFPV@@YAXPEAVBEE_Layer@@...` |
| Null pointer casts | `(TDB_Stream *)0x0` |

---

## 2. Design Philosophy

### Principle 1 — Explicitness over brevity
Every semantic element that matters for understanding must be visible. KSL does not silently elide information. When something is unknown, say `default` or use `...`.

### Principle 2 — One translation per pattern
Each decompiled pattern maps to exactly one KSL construct. There is no ambiguity in how to render a pattern. (See Section 8.)

### Principle 3 — Structural alignment
KSL uses column-aligned formatting in declarations to make relationships scannable at a glance.

### Principle 4 — Rust/Kotlin-inspired syntax
KSL uses `fn`, `let`, `var`, `struct`, `->` (return type), `?` (nullable), block expressions. Readers familiar with Rust or Kotlin will find KSL natural.

---

## 3. Lexical Elements

### 3.1 Encoding and whitespace

KSL source is UTF-8. **Indentation is TAB** (`\t`). Spaces within a line are used for alignment only (e.g., column-aligning field names). Blank lines are significant as visual separators but carry no syntactic weight.

### 3.2 Comments

```
// single-line comment
/// doc comment (on fn, struct, interface, or type alias)
```

There are no block comments in KSL.

**Section separator comment** — a special visual decoration, written by hand or generated:

```
// ── Title ─────────────────────────────────────────────────────────────────
```

The pattern is `// ── ` + title + ` ` + enough `─` to reach column 80.

### 3.3 Identifiers

```
identifier := [A-Za-z_][A-Za-z0-9_]*
```

Identifiers preserve the names from the decompiled code unless explicitly renamed. Typical prefixes from Ghidra (`pFVar1`, `local_a8`, `iVar5`) should be renamed to meaningful names when the analyst knows their purpose.

### 3.4 Keywords

The following words are reserved:

```
fn  struct  virtual  interface  type  let  var  if  else  return
throws  null  default  true  false  as  else
```

### 3.5 Literals

| Kind | Examples |
|------|---------|
| Decimal integer | `0`, `42`, `-1` |
| Hexadecimal integer | `0x0`, `0x290`, `0xe2b340`, `0xffffffffffffff00` |
| Boolean | `true`, `false` |
| Null pointer | `null` |
| Zero-initialized | `default` |
| Variadic placeholder | `...` (only in call argument lists) |

---

## 4. Type System

KSL's type system is annotation-only; it does not perform type checking.

### 4.1 Named types

Any identifier can be a type name:

```
i32  i64  u32  u64  bool  void  Time  HResult  KfcInfo  BEE_Layer
```

**Standard numeric type aliases** (use these instead of Ghidra's unknown types):

| Ghidra type | KSL type |
|-------------|----------|
| `undefined8` / `longlong` | `i64` |
| `undefined4` / `int` | `i32` |
| `undefined2` | `i16` |
| `undefined1` / `char` | `i8` |
| `uchar` | `u8` |
| `bool` | `bool` |
| `void *` | `*void` |

### 4.2 Pointer types

```
*T         raw pointer to T
*T?        nullable raw pointer to T   (preferred over *mut Option<T>)
```

A bare `*T` (without `?`) expresses that the pointer is **assumed non-null** at the point of use. Append `?` to acknowledge nullability.

### 4.3 Reference types

```
&T         immutable reference (C++ const ref / value passed by const ref)
&mut T     mutable reference
```

Use references only when the original C++ signature uses const ref (`const T &`). Otherwise use `*T`.

### 4.4 Nullable annotation `T?`

The `?` postfix applied to any type means "this value may be null / absent".

```
*Stream?       // pointer that may be null
*KfcInfo?      // pointer that may be null
```

`T?` is a type-level annotation only. It does not imply any runtime wrapping.

### 4.5 Tuple types

```
(T1, T2)
(Time, Time)
(bool, bool)
```

Tuples are used to express multiple return values.

### 4.6 Unit type

```
()
```

Represents the absence of a return value. Equivalent to C's `void`.

---

## 5. Top-Level Declarations

All top-level declarations appear at the outermost level of a KSL file. There is no module or namespace construct.

### 5.1 Type Alias

```
type Alias = Type;
```

**Example:**
```
type Time = i64;
type HResult = i32;
```

Used to document the underlying type of opaque names.

---

### 5.2 Struct

Declares the memory layout of a C++ class or struct. Only fields that are referenced in the translated function need to be listed.

```
struct Name {
    [@offset]  field_name:  Type[,  // optional comment]
    ...
}
```

**`@offset`** is a hex integer (e.g., `@0x0290`) giving the byte offset of the field from the start of the struct. It is optional when the offset is unknown or irrelevant.

**Formatting rule:** Column-align the field names and type annotations across all fields in the struct. Use enough spaces after `@0xNNNN` to reach column 10 from the start of the field (see examples).

**Example:**
```
struct BEE_Layer {
	@0x0290  item:  *BEE_Item,
}

struct KfcInfo {
	@0x00  value:      i32,    // interpolated value
	@0x08  ease_in:    *KfcInfo?,
	@0x10  ease_out:   *KfcInfo?,
	@0x18  tangent_in: *KfcInfo?,
	@0x1c  tangent_out: *KfcInfo?,
	@0x24  key_index:  i32,
	@0x28  is_hold:    bool,
	@0x29  is_prev_hold: bool,
	@0x2a  is_next_hold: bool,
}
```

---

### 5.3 Virtual Interface

Declares the vtable layout of a C++ class.

```
virtual interface Name {
    [@vtable_offset]  fn method_name(self[, param: Type, ...]) -> ReturnType,
    ...
}
```

**`@vtable_offset`** is the byte offset of the function pointer within the vtable (e.g., `@0x0170`). Divide by 8 (on x64) to get the slot index.

**`self`** is the required first parameter for every virtual method. It has implicit type `*Self` (the interface type). Do not write a type annotation for `self`.

**Formatting rule:** Column-align all `fn` keywords. Column-align all `-> ReturnType` suffixes.

**Example:**
```
virtual interface Stream {
	@0x0170  fn is_parametric(self)                                          -> bool,
	         fn has_keys(self)                                               -> bool,
	         fn get_value(self, time: Time, raw: bool,
	                      unused: *void, out: *KfcInfo, bag: *ParamBag?)    -> HResult,
	         fn time_to_index(self, time: Time, out: *i32)                  -> HResult,
	         fn get_key(self, index: i32, unused: *void,
	                    prev_hold: *bool, next_hold: *bool)                 -> HResult,
}
```

Methods without a `@vtable_offset` annotation either have an unknown offset or are inferred (e.g., `has_keys` is known to exist but its vtable slot is unconfirmed).

---

### 5.4 Function

```
fn function_name(
	param_name:  Type[,  // optional comment]
	...
)[-> ReturnType] [throws] {
	statements...
}
```

**`throws`** — append when the function can propagate C++ exceptions (i.e., it contains at least one `expr!` call). Do not write `-> HResult` for the return type; use `throws` instead.

**Formatting rule:**
- Each parameter on its own line, indented by one TAB.
- Column-align parameter names (pad each `name:` to the width of the longest `name:` in the function).
- Column-align type expressions.
- Trailing comma after every parameter.

**Example:**
```
fn get_stream_fpv(
	layer:       *BEE_Layer,
	path:        &StreamIDPath,
	mode:        i32,
	comp_time:   *Time?,         // null = use current playhead
	layer_time:  *Time?,         // null = derived from comp_time
	out_fpv:     *StreamFPV,
	out_kfc:     *KfcInfo?,      // null = skip KFC computation
	stream:      *Stream?,       // null = auto-lookup from layer + path
) throws {
	...
}
```

---

## 6. Statements

All statements inside a function body are indented from the enclosing block.

### 6.1 `let` — immutable binding

```
let name[: Type] = expression
```

No semicolon. Declares an immutable binding. The type annotation is optional when the type is obvious from context.

```
let stream = stream ?? BEE_GetStream(layer, path)!
let ct = *comp_time
let (comp_t, layer_t): (Time, Time) = ...
```

**Destructuring:** Tuple destructuring is supported:

```
let (a, b): (T1, T2) = tuple_expression
```

---

### 6.2 `var` — mutable variable

```
var name[: Type] [= expression]
```

No semicolon. Declares a mutable variable. The initial value is optional; if omitted, the variable is conceptually uninitialized (use `= default` to make zero-initialization explicit).

```
var kfc_value: KfcInfo = default
var prev_hold: bool
var next_hold: bool
var current_time: Time = default
```

`default` means zero-initialized (all bytes are 0). This translates `= (T)0x0` in Ghidra output.

---

### 6.3 Assignment

```
target = expression
```

No semicolon. Assigns to a previously declared `var` or to a field via `->`.

```
out_kfc->key_index = stream->time_to_index(layer_t)!
out_kfc->is_hold = true
panel.smooth.current_y = panel.smooth.target_y
```

---

### 6.4 `if` / `else if` / `else`

```
if condition {
	statements
} else if condition {
	statements
} else {
	statements
}
```

Braces are **always required**. The opening `{` is on the same line as the condition. The `} else` is on its own line. No parentheses around the condition.

---

### 6.5 `return`

```
return [expression]
```

No semicolon. `return` with no expression returns from a `throws` function or a `()` function. If a function has a non-unit return type, an expression is required.

---

### 6.6 Expression statement

A bare expression followed by nothing (no semicolons). Used for function calls whose return value is discarded and for `expr!` propagation.

```
BEE_GetStreamFPVPlusWithStreamP(layer, path, stream, ...)!
FUN_180e2acf0(&current_time, layer, stream, ...)
stream->get_value(layer_t, false, null, &kfc_value, null)!
```

---

### 6.7 Section separator comment

```
// ── Title ────────────────────────────────────────────────────────────────────
```

Used to divide a long function body into labeled regions. The dashes fill to column 80. Write these by hand to document the purpose of each block.

---

## 7. Expressions

### 7.1 Operator precedence (high to low)

| Level | Operators |
|-------|-----------|
| 1 (highest) | `!` (postfix throw), `.` `->` `[...]` (member/index) |
| 2 | Unary `*` `&` `&mut` `!` (prefix not) `-` (negation) |
| 3 | `as` (cast) |
| 4 | `*` `/` `%` |
| 5 | `+` `-` |
| 6 | `<<` `>>` |
| 7 | `&` (bitwise) |
| 8 | `^` |
| 9 | `\|` (bitwise) |
| 10 | `==` `!=` `<` `<=` `>` `>=` |
| 11 | `&&` |
| 12 | `\|\|` |
| 13 (lowest) | `??` |

Use parentheses freely to clarify intent.

---

### 7.2 Member access operators

| Syntax | Meaning |
|--------|---------|
| `expr.field` | Access field of a value-type (stack-allocated) |
| `expr->field` | Access field through a pointer |
| `expr->method(args)` | Call virtual or pointer-based method |
| `expr[index]` | Index access |

**Pointer arithmetic** — when offset-based access remains unresolved, write it explicitly:

```
out_kfc + 0x08          // pointer arithmetic; document with a comment
*(out_kfc + 0x08)       // dereference at offset
```

---

### 7.3 `!` — HRESULT error propagation (postfix)

```
expression!
```

Applied to any expression that returns `HResult` (or an equivalent integer error code). Means: "if this call returned a non-zero value, throw a C++ exception and stop execution." Equivalent to:

```c
// C equivalent:
iVar = f(...);
if (iVar != 0) {
    local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_, iVar);
    _CxxThrowException(&local_a0, (ThrowInfo *)&DAT_182159f70);
}
```

**Rules for `!`:**
- Apply `!` directly to the call expression: `func(args)!`
- When the call result is assigned AND checked: `out_kfc->key_index = stream->time_to_index(layer_t)!`
- `!` is only valid in a `throws` function.
- `!` **cannot** be applied to a non-call expression.

**Examples:**
```
BEE_GetStream(layer, path)!
BEE_CompToLayerTime(layer, ct)!
stream->get_value(layer_t, false, null, &kfc_value, null)!
out_kfc->key_index = stream->time_to_index(layer_t)!
```

---

### 7.4 `??` — Null-coalesce

```
lhs ?? rhs
```

Evaluates to `lhs` if `lhs` is not null, otherwise evaluates to `rhs`.

**C equivalent:**
```c
// if (param_8 == (TDB_Stream *)0x0) {
//     param_8 = ... /* fetch from somewhere */
// }
```

**Rules for `??`:**
- Both sides must have the same base type.
- `rhs` is typically a function call (possibly with `!`).
- When combined with `!`, write `??` before `!`: `a ?? f(b)!`

**Example:**
```
let stream = stream ?? BEE_GetStream(layer, path)!
```

---

### 7.5 `let … else return` — Guard

```
let name = expression else return
```

Evaluates `expression`. If the result is `null`, immediately returns from the enclosing function. Otherwise binds the non-null value to `name`.

**C equivalent:**
```c
if (param_7 == (FEE_KfcInfo *)0x0) {
    return;
}
```

**Rules:**
- Only `return` is allowed in the `else` branch (no custom expressions).
- The bound `name` shadows any outer variable of the same name; use a different name if shadowing is unwanted.
- The expression must be of nullable type `T?`; `name` has type `*T` (non-nullable).

**Example:**
```
// param_7 == null → early return; otherwise bind as out_kfc
let out_kfc = out_kfc else return
```

---

### 7.6 Block expressions (if-as-expression)

An `if`/`else` chain can be used as an expression when all branches produce the same type. This is used to model the C pattern of initializing a variable across branching paths.

```
let binding[: Type] =
	if condition {
		stmts...
		tuple_or_value_expression
	} else if condition {
		stmts...
		tuple_or_value_expression
	} else {
		stmts...
		tuple_or_value_expression
	}
```

**Rules:**
- Every branch must end with an expression of the same type (no trailing semicolon/newline).
- If the type is a tuple, write the tuple expression as the last line of each branch.
- Indent the entire `if … else` chain one TAB from the `let` line.

**Example:**
```
let (comp_t, layer_t): (Time, Time) =
	if comp_time != null {
		let ct = *comp_time
		(ct, BEE_CompToLayerTime(layer, ct)!)
	} else if layer_time != null {
		let lt = *layer_time
		(BEE_LayerToCompTime(layer, lt)!, lt)
	} else {
		let ct = BEE_GetItemCurrentTime(layer->item, null)!
		(ct, BEE_CompToLayerTime(layer, ct)!)
	}
```

---

### 7.7 `&expr` / `&mut expr` — Address-of

```
&expression     // pass by address (const pointer)
&mut expression // pass by mutable address
```

Used when the original C passes `&local_variable` as an output parameter.

```
BEE_GetItemCurrentTime(layer->item, &current_time)!
stream->get_key(out_kfc->key_index, null, &prev_hold, &next_hold)!
```

---

### 7.8 `*expr` — Dereference

```
*expression
```

Dereferences a pointer. Used to read through a non-null pointer parameter.

```
let ct = *comp_time
let lt = *layer_time
```

---

### 7.9 `expr as Type` — Cast

```
expression as Type
```

Explicit type cast. Use sparingly — only when the cast is semantically meaningful and would otherwise be ambiguous.

```
iVar5 as HResult
```

---

### 7.10 Tuple expressions

```
(expr1, expr2)
(expr1, expr2, expr3)
```

Used to return multiple values from a block expression or as the last expression in an `if` branch.

---

### 7.11 `...` — Variadic placeholder

Used in call argument lists to represent elided arguments whose types or values are not yet determined.

```
FUN_180e2afa0(layer, stream, &comp_t, 0, ...)
```

Do not use `...` if all arguments are known.

---

## 8. Decompiled C → KSL Pattern Reference

This section maps every common Ghidra/IDA decompiled pattern to its KSL equivalent. **Apply these rules mechanically.**

---

### 8.1 HRESULT error propagation

**C pattern:**
```c
iVar5 = SomeFunction(arg1, arg2);
if (iVar5 != 0) {
    local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_, iVar5);
    /* WARNING: Subroutine does not return */
    _CxxThrowException(&local_a0, (ThrowInfo *)&DAT_182159f70);
}
```

**KSL:**
```
SomeFunction(arg1, arg2)!
```

If the call's return value is used:
```c
// C:
iVar5 = stream->time_to_index(this, (T_Time *)&local_98, (int *)(param_7 + 0x24));
if (iVar5 != 0) { /* throw */ }
```
```
// KSL:
out_kfc->key_index = stream->time_to_index(layer_t)!
```

**Rule:** Every `if (iVar != 0) { _CxxThrowException... }` following a call becomes `!` on that call.

---

### 8.2 Null pointer guard + early return

**C pattern:**
```c
if (param_7 == (FEE_KfcInfo *)0x0) {
    return;
}
```

**KSL:**
```
let out_kfc = out_kfc else return
```

Rename `param_7` to `out_kfc` as appropriate. The `let ... else return` also serves as a rename (shadows the parameter).

---

### 8.3 Conditional stream lookup (null-coalesce with error propagation)

**C pattern:**
```c
if ((param_8 == (TDB_Stream *)0x0) &&
    (iVar5 = BEE_GetStream(param_1, param_2, &param_8), iVar5 != 0)) {
    /* throw */
}
```

**KSL:**
```
let stream = stream ?? BEE_GetStream(layer, path)!
```

---

### 8.4 Virtual method dispatch (vtable call)

**C pattern:**
```c
cVar3 = (**(code **)(*(longlong *)this + 0x170))(this);
```

**KSL:**
```
stream->is_parametric()
```

Steps to translate:
1. Identify the vtable offset (`0x170`).
2. Look up or declare the method in the `virtual interface` for that class.
3. Write `recv->method_name(args)`.

If the method is not yet declared in an interface, you may write the raw form until it is named:
```
// vtable[0x2e] = 0x170 / 8
stream->vtable[0x2e]()
```

---

### 8.5 Struct field access via pointer offset

**C pattern:**
```c
pBVar2 = *(BEE_Item **)(param_1 + 0x290);
```

**KSL:**
```
layer->item
```

Steps:
1. Identify the struct type of the pointer (`param_1` → `BEE_Layer`).
2. Declare the field in the struct: `@0x0290  item: *BEE_Item`.
3. Write `layer->item`.

If the struct layout is unknown or partially known:
```
*(param_1 + 0x290) as *BEE_Item    // @BEE_Layer+0x290
```

---

### 8.6 `CONCAT44` / bit packing

**C pattern:**
```c
local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_, iVar5);
```

This pattern appears in Ghidra output when a 64-bit value is packed from two 32-bit halves. In the context of the HRESULT throw pattern, the entire expression is an artifact of Ghidra's decompilation — **discard it** and write only `!` on the preceding call.

If `CONCAT44` appears outside of a throw pattern, translate it as:
```
((high as i64) << 32) | (low as i64)
```

---

### 8.7 `undefined8` / `undefined4` variables

**C pattern:**
```c
undefined8 local_98;
undefined8 local_90;
undefined4 local_80;
```

These are Ghidra's "unknown type" placeholders. Translate based on what is stored into them:

| Usage | KSL |
|-------|-----|
| Stores `T_Time` value | `var layer_t: Time` |
| Stores pointer | `var ptr: *void` (until type is known) |
| Stores i32 | `var x: i32` |
| Output buffer for struct | `var buf: StructName = default` |

---

### 8.8 `goto` elimination

**C pattern:**
```c
if (local_88 == (TDB_Stream *)0x0) goto LAB_180e2b5da;
// ... code ...
LAB_180e2b5da:
BEE_StreamSpec::~BEE_StreamSpec(local_60);
```

**KSL:** Restructure to remove `goto`. Common patterns:

**Forward `goto` (skip a block):**
```c
if (condition) goto LABEL;
/* block A */
LABEL:
/* block B */
```
→
```
if !condition {
    /* block A */
}
/* block B */
```

**Break-out `goto` (exit a nested block):**
```c
if (condition) goto AFTER_BLOCK;
/* nested block */
AFTER_BLOCK:
/* continuation */
```
→ Use early returns or restructured if/else as appropriate.

---

### 8.9 `(Type)0x0` — null pointers

**C pattern:**
```c
(TDB_Stream *)0x0
(FEE_KfcInfo *)0x0
```

**KSL:**
```
null
```

---

### 8.10 `(Type)0x0` — zero values (non-pointer)

**C pattern:**
```c
local_a8[0] = (FEE_KfcInfo)0x0;
```

**KSL:**
```
kfc_value = default
// or at declaration:
var kfc_value: KfcInfo = default
```

---

### 8.11 Stack-allocated output parameters

**C pattern:**
```c
FEE_KfcInfo local_a8 [8];    // stack-allocated output buffer
// ...
TDB_GetValue(this, ..., (uchar *)local_a8, ...);
```

**KSL:**
```
var kfc_value: KfcInfo = default
stream->get_value(layer_t, false, null, &kfc_value, null)!
```

The stack array becomes a `var` declaration with `= default`. Pass by address with `&`.

---

### 8.12 Unknown function (`FUN_` prefix)

When a function is not yet named/reversed:

```
// FUN_180e2afa0 — binary-search key index (no-keyframes fallback)
out_kfc->key_index = FUN_180e2afa0(layer, stream, &comp_t, 0, ...)
```

Rules:
- Write a `//` comment before the call explaining what the function appears to do.
- Use the original Ghidra address-based name (`FUN_180e2afa0`).
- Use `...` for argument positions that are unclear.

---

### 8.13 C++ constructor / destructor calls

**C pattern:**
```c
BEE_StreamSpec::BEE_StreamSpec(local_60);
BEE_StreamSpec::~BEE_StreamSpec(local_60);
```

**KSL:**
```
var spec: BEE_StreamSpec = BEE_StreamSpec::new()
// ... use spec ...
// (implicit drop at end of scope)
```

Or if the constructor/destructor is a side effect:
```
BEE_StreamSpec::init(&spec)
```

---

## 9. Formal Grammar (EBNF)

```ebnf
document     = item* ;

item         = comment
             | blank_line
             | type_alias
             | struct_def
             | interface_def
             | fn_def
             ;

(* ── Declarations ── *)

type_alias   = doc_comment* "type" IDENT "=" type ";" ;

struct_def   = doc_comment* "struct" IDENT "{" field_def* "}" ;
field_def    = ("@" HEX_LIT)? IDENT ":" type "," comment? NEWLINE ;

interface_def = "virtual" "interface" IDENT "{" virtual_method* "}" ;
virtual_method = ("@" HEX_LIT)? "fn" IDENT "(" method_params ")" "->" type "," NEWLINE ;
method_params  = "self" ("," param)* | (* empty *) ;

fn_def       = doc_comment* "fn" IDENT "(" fn_params ")" ("->" type)? "throws"? block ;
fn_params    = (param ("," comment? NEWLINE)*)? ;
param        = IDENT ":" type ;

(* ── Types ── *)

type         = "*" type                  (* pointer *)
             | "&" type                  (* reference *)
             | "&" "mut" type            (* mutable reference *)
             | type "?"                  (* nullable *)
             | "(" type ("," type)* ")"  (* tuple *)
             | "()"                      (* unit *)
             | IDENT                     (* named type *)
             ;

(* ── Blocks and Statements ── *)

block        = "{" stmt* "}" ;

stmt         = let_stmt
             | var_stmt
             | assign_stmt
             | if_stmt
             | return_stmt
             | expr_stmt
             | comment
             | blank_line
             | section_comment
             ;

let_stmt     = "let" destructure (":" type)? "=" expr NEWLINE
             | "let" IDENT "=" expr "else" "return" NEWLINE  (* guard *)
             ;
destructure  = IDENT | "(" IDENT ("," IDENT)+ ")" ;
var_stmt     = "var" IDENT (":" type)? ("=" expr)? NEWLINE ;
assign_stmt  = expr "=" expr NEWLINE ;
if_stmt      = "if" expr block ("else" "if" expr block)* ("else" block)? ;
return_stmt  = "return" expr? NEWLINE ;
expr_stmt    = expr NEWLINE ;

section_comment = "//" "──" TEXT "─"+ NEWLINE ;

(* ── Expressions ── *)

expr         = expr "!"                            (* throw on error, postfix *)
             | expr "??" expr                      (* null-coalesce *)
             | expr "as" type                      (* cast *)
             | expr bin_op expr
             | unary_op expr
             | expr "->" IDENT                     (* pointer field *)
             | expr "->" IDENT "(" call_args ")"   (* pointer method call *)
             | expr "." IDENT                      (* value field *)
             | expr "[" expr "]"                   (* index *)
             | IDENT "(" call_args ")"             (* free function call *)
             | "if" expr block ("else" block)?     (* if expression *)
             | "(" expr ("," expr)+ ")"            (* tuple *)
             | "*" expr                            (* deref *)
             | "&" expr                            (* addr-of *)
             | "&" "mut" expr                      (* addr-of-mut *)
             | "null"
             | "default"
             | BOOL_LIT
             | INT_LIT
             | HEX_LIT
             | IDENT
             ;

call_args    = (call_arg ("," call_arg)*)? ;
call_arg     = expr | "..." ;

bin_op       = "+"  | "-"  | "*"  | "/"  | "%"
             | "==" | "!=" | "<"  | "<=" | ">"  | ">="
             | "&&" | "||"
             | "&"  | "|"  | "^"  | "<<" | ">>"
             ;
unary_op     = "!" | "-" ;

(* ── Terminals ── *)

IDENT        = [A-Za-z_][A-Za-z0-9_]* ;
INT_LIT      = "-"? [0-9]+ ;
HEX_LIT      = "0x" [0-9A-Fa-f]+ ;
BOOL_LIT     = "true" | "false" ;
TEXT         = (* any UTF-8 characters except newline *) ;
NEWLINE      = "\n" | "\r\n" ;
comment      = "//" TEXT NEWLINE ;
doc_comment  = "///" TEXT NEWLINE ;
blank_line   = NEWLINE ;
```

---

## 10. Worked Example

The following shows the complete translation of Ghidra's `NIM_GetStreamFPV` function.

### Original Ghidra output (excerpt)

```c
void __cdecl
NIM_GetStreamFPV(BEE_Layer *param_1, TDB_StreamIDPath *param_2, int param_3,
                 T_Time *param_4, T_Time *param_5, BEE_StreamFPV *param_6,
                 FEE_KfcInfo *param_7, TDB_Stream *param_8)
{
  // ...
  local_a8[0] = (FEE_KfcInfo)0x0;
  if ((param_8 == (TDB_Stream *)0x0) &&
     (iVar5 = BEE_GetStream(param_1,param_2,&param_8), iVar5 != 0)) {
    local_a0 = (FEE_KfcInfo *)CONCAT44(local_a0._4_4_,iVar5);
    _CxxThrowException(&local_a0,(ThrowInfo *)&DAT_182159f70);
  }
  // ...
  cVar3 = (**(code **)(*(longlong *)this + 0x170))(this);
  // ...
}
```

### KSL translation (full)

```
// KSL — Kaiseki Script Language
// Lifted from: void NIM_GetStreamFPV  addr: 0xe2b340

// ── Type layout annotations ────────────────────────────────────────────────

struct BEE_Layer {
	@0x0290  item:  *BEE_Item,
}

virtual interface Stream {
	@0x0170  fn is_parametric(self)                                          -> bool,
	         fn has_keys(self)                                               -> bool,
	         fn get_value(self, time: Time, raw: bool,
	                      unused: *void, out: *KfcInfo, bag: *ParamBag?)    -> HResult,
	         fn time_to_index(self, time: Time, out: *i32)                  -> HResult,
	         fn get_key(self, index: i32, unused: *void,
	                    prev_hold: *bool, next_hold: *bool)                 -> HResult,
}

// ── Function ───────────────────────────────────────────────────────────────

/// Get stream FPV and optionally populate key-frame interpolation info.
///
/// Time parameters:
///   - both null        →  use current playhead (comp-space → layer-space)
///   - comp_time only   →  derive layer_time via CompToLayerTime
///   - layer_time only  →  derive comp_time  via LayerToCompTime
fn get_stream_fpv(
	layer:       *BEE_Layer,
	path:        &StreamIDPath,
	mode:        i32,
	comp_time:   *Time?,         // null = use current playhead
	layer_time:  *Time?,         // null = derived from comp_time
	out_fpv:     *StreamFPV,
	out_kfc:     *KfcInfo?,      // null = skip KFC computation
	stream:      *Stream?,       // null = auto-lookup from layer + path
) throws {

	// ── acquire stream ──────────────────────────────────────────────────
	let stream = stream ?? BEE_GetStream(layer, path)!

	BEE_GetStreamFPVPlusWithStreamP(
		layer, path, stream, comp_time, layer_time, mode, out_fpv, null,
	)!

	// KFC output is optional — bail early when the caller does not need it
	let out_kfc = out_kfc else return

	// ── resolve time coordinates ────────────────────────────────────────
	let (comp_t, layer_t): (Time, Time) =
		if comp_time != null {
			let ct = *comp_time
			(ct, BEE_CompToLayerTime(layer, ct)!)
		} else if layer_time != null {
			let lt = *layer_time
			(BEE_LayerToCompTime(layer, lt)!, lt)
		} else {
			let ct = BEE_GetItemCurrentTime(layer->item, null)!
			(ct, BEE_CompToLayerTime(layer, ct)!)
		}

	// ── key-frame data (parametric streams only) ────────────────────────
	var kfc_value: KfcInfo = default

	if stream->is_parametric() {
		stream->get_value(layer_t, false, null, &kfc_value, null)!

		if stream->has_keys() {
			// Neighbor-key easing — null pointer means "skip this component"
			var ease_in:     *KfcInfo? = if *out_kfc          != null { out_kfc + 0x08 } else { null }
			var ease_out:    *KfcInfo? = if *(out_kfc + 0x10) != null { out_kfc + 0x10 } else { null }
			var tangent_in:  *KfcInfo? = if *(out_kfc + 0x18) != null { out_kfc + 0x18 } else { null }
			var tangent_out: *KfcInfo? = if *(out_kfc + 0x1c) != null { out_kfc + 0x1c } else { null }

			if layer != null {
				var current_time: Time = default
				BEE_GetItemCurrentTime(layer->item, &current_time)!
				// FUN_180e2acf0 — compute tangent / easing data from adjacent keys
				FUN_180e2acf0(
					&current_time, layer, stream,
					ease_in, ease_out, tangent_in, tangent_out, ease_in,
				)
			}
		}

		// Resolve the key index for the layer-space time
		if kfc_value == null {
			// FUN_180e2afa0 — binary-search key index (no-keyframes fallback)
			out_kfc->key_index = FUN_180e2afa0(layer, stream, &comp_t, 0, ...)
		} else {
			out_kfc->key_index = stream->time_to_index(layer_t)!
		}

		// Continuity flags: is this key on the boundary of a hold segment?
		if kfc_value == null {
			out_kfc->is_hold = true
		} else {
			var prev_hold: bool
			var next_hold: bool
			stream->get_key(out_kfc->key_index, null, &prev_hold, &next_hold)!
			out_kfc->is_prev_hold = prev_hold
			out_kfc->is_next_hold = next_hold
		}

	} else {
		out_kfc->is_hold = false
	}

	out_kfc->value = kfc_value
}
```

### Translation decisions documented

| Original | KSL | Reason |
|----------|-----|--------|
| `param_1` | `layer` | Type is `BEE_Layer *`; role is the compositor layer |
| `param_7` | `out_kfc` | Output KFC info; null = optional |
| `param_8` | `stream` | Input stream, auto-fetched when null |
| `local_90` / `local_98` | `comp_t`, `layer_t` | Two time coordinate values |
| `local_a8[0]` | `kfc_value` | Output value buffer |
| `iVar5 != 0` throw | `!` | HRESULT error propagation pattern |
| `*(longlong *)this + 0x170` dispatch | `stream->is_parametric()` | vtable slot 0x170/8 = 0x2e |
| `goto LAB_180e2b5da` | restructured `if` | Forward goto → inverted condition |
| `(FEE_KfcInfo)0x0` | `default` | Zero-initialization |
| `(TDB_Stream *)0x0` | `null` | Null pointer |

---

*End of KSL specification v0.1*
