// KSL (Kaiseki Script Language) — Abstract Syntax Tree
//
// KSL abstracts the following decompiled-C patterns:
//   expr!                   if (res != 0) { _CxxThrowException(...) }
//   a ?? b                  null-coalesce
//   let x = e else return   guard / early-exit unwrap
//   T?                      nullable pointer annotation
//   @0xNN field: *T         struct field at fixed memory offset
//   @0xNN fn m(self) -> T   virtual method at vtable offset
//   recv->method()          pointer member access / virtual dispatch
//   var x: T = default      zero-initialized mutable variable

#![allow(dead_code)]

// ── Top-level ─────────────────────────────────────────────────────────────────

pub struct Document {
	pub items: Vec<Item>,
}

pub enum Item {
	Comment(String),
	BlankLine,
	TypeAlias(TypeAlias),
	Struct(StructDef),
	Interface(InterfaceDef),
	Function(FnDef),
}

// ── Type declarations ─────────────────────────────────────────────────────────

pub struct TypeAlias {
	pub docs: Vec<String>,
	pub name: String,
	pub ty: Type,
}

pub struct StructDef {
	pub docs: Vec<String>,
	pub name: String,
	pub fields: Vec<FieldDef>,
}

pub struct FieldDef {
	/// Byte offset annotation, e.g. 0x290.
	pub offset: Option<u64>,
	pub name: String,
	pub ty: Type,
	pub comment: Option<String>,
}

/// A vtable-backed virtual interface declaration.
pub struct InterfaceDef {
	pub name: String,
	pub methods: Vec<VirtualMethod>,
}

pub struct VirtualMethod {
	/// Byte offset into the vtable, e.g. 0x170.
	pub vtable_offset: Option<u64>,
	pub name: String,
	pub params: Vec<Param>,
	pub return_ty: Type,
}

// ── Function ──────────────────────────────────────────────────────────────────

pub struct FnDef {
	pub docs: Vec<String>,
	pub name: String,
	pub params: Vec<Param>,
	pub return_ty: Option<Type>,
	/// Propagates errors via C++ exception on non-zero HRESULT.
	pub throws: bool,
	pub body: Block,
}

pub struct Param {
	pub name: String,
	pub ty: Type,
	/// Inline comment shown after the trailing comma.
	pub comment: Option<String>,
}

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum Type {
	Named(String),
	/// Raw pointer: *T
	Pointer(Box<Type>),
	/// Immutable reference: &T
	Ref(Box<Type>),
	/// Mutable reference: &mut T
	RefMut(Box<Type>),
	/// Nullable pointer / optional: T?
	Nullable(Box<Type>),
	/// Tuple: (T1, T2, …)
	Tuple(Vec<Type>),
	Unit,
}

// ── Statements ────────────────────────────────────────────────────────────────

pub struct Block {
	pub stmts: Vec<Stmt>,
}

pub enum Stmt {
	/// `let name[: ty] = value`
	Let { name: String, ty: Option<Type>, value: Expr },
	/// `var name[: ty] [= value]`  — mutable, optionally initialized
	Var { name: String, ty: Option<Type>, value: Option<Expr> },
	/// `target = value`
	Assign { target: Expr, value: Expr },
	If { cond: Expr, then_block: Block, else_branch: Option<ElseBranch> },
	Return(Option<Expr>),
	/// Bare expression statement (function call, `expr!`, etc.).
	Expr(Expr),
	/// `// text`
	Comment(String),
	BlankLine,
	/// `// ── Title ─────────────────────────────` section separator line.
	Section(String),
}

pub enum ElseBranch {
	Block(Block),
	/// `else if …`
	If(Box<Stmt>),
}

// ── Expressions ───────────────────────────────────────────────────────────────

pub enum Expr {
	Null,
	Default,
	Bool(bool),
	Int(i64),
	Hex(u64),
	Ident(String),
	Binary(BinOp, Box<Expr>, Box<Expr>),
	Unary(UnOp, Box<Expr>),
	Call { func: Box<Expr>, args: Vec<CallArg> },
	/// `expr.field` — value/stack access.
	Field(Box<Expr>, String),
	/// `expr->field` — pointer dereference + field access.
	Arrow(Box<Expr>, String),
	/// `*expr`
	Deref(Box<Expr>),
	/// `&expr`
	AddrOf(Box<Expr>),
	/// `&mut expr`
	AddrOfMut(Box<Expr>),
	/// `expr!` — throw C++ exception on non-zero HRESULT.
	Throw(Box<Expr>),
	/// `lhs ?? rhs` — null-coalesce operator.
	NullCoalesce(Box<Expr>, Box<Expr>),
	/// `let name = value else return` — guard / unwrap-or-return.
	LetElse { name: String, value: Box<Expr> },
	Tuple(Vec<Expr>),
	/// Block-valued if expression.
	IfExpr { cond: Box<Expr>, then_block: Block, else_block: Option<Block> },
	/// Type cast: `expr as Type`.
	As(Box<Expr>, Type),
}

/// An argument in a function call.
pub enum CallArg {
	Expr(Expr),
	/// `...` — placeholder for elided variadic arguments.
	Varargs,
}

#[derive(Clone, Copy, Debug)]
pub enum BinOp {
	Add, Sub, Mul, Div, Rem,
	Eq, Ne, Lt, Le, Gt, Ge,
	And, Or,
	BitAnd, BitOr, BitXor,
	Shl, Shr,
}

#[derive(Clone, Copy, Debug)]
pub enum UnOp { Not, Neg }
