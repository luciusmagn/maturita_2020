# Mini Rust reference
This page contains minimalistic Rust reference,
which only contains definitions of types and functions,
sometimes stripped away from some details. Why?
For my own use when traveling, so that I don't have to
take the huge whole reference and can have everything in
one page, ripe and ready to be Ctrl+F'ked to hell.

## macros
Contains macros of the standard library. Not actually
a module.

	name			purpose
	--------------------------------------------------------------
	assert			Ensure that a boolean expression is true at runtime
	assert_eq		Asserts that two expressions are equal to each other
	assert_ne		Asserts that two expressions are not equal to each other
	cfg			Boolean evaluation of configuration flags
	column			A macro which expands to the column number on which it was invoked
	concat			Concatenates literals into a static string slice
	debug_assert		Ensure that a boolean expression is true at runtime
	debug_assert_eq		Asserts that two expressions are equal to each other
	debug_assert_ne		Asserts that two expressions are not equal to each other
	env			Inspect an environment variable at compile time
	file			A macro which expands to the file name from which it was invoked
	format			A format to create a value of type String
	format_args		The core macro for formatted string creation & output
	include			Parse a file as an expression or an item according to the context
	include_bytes		Includes a file as a reference to a byte array
	include_str		Includes a utf8-encoded file as a string
	line			A macro which expands to the line number on which it was invoked
	module_path		Expands to a string that represents the current module path
	option_env		Optionally inspect an environment variable at compile time
	panic			The entry point for panic of Rust threads
	print			Macro for printing to the standard output
	println			Macro for printing to the standard output, with a newline
	stringify		A macro which stringifies its argument
	thread_local		Declare a new thread local storage key
	try			Reduces boilerplate code for matching Result together with converting downstream errors|
	unimplemented		Placeholder for marking unfinished code. Panics with "not yet implemented"
	unreachable		A utility macro for indicating unreachable code
	vec			Creates a Vec containing the arguments
	write			Write formatted data into a buffer
	writeln			Write formatted data into a buffer, with a newline appended

## any
Implements the Any trait
	pub trait Any: 'static
	{
	  fn get_type_id(&self) -> TypeId; //nightly
	}
	pub struct TypeId { /* fields omitted */ }
	  : fn of<T>() -> TypeId

## borrow
A module for working with borrowed data.
	pub trait Borrow<Borrowed: ?Sized>
	{
		fn borrow(&self) -> &Borrowed;
	}
	pub trait BorrowMut<Borrowed: ?Sized>: Borrow<Borrowed>
	{
		fn borrow_mut(&mut self) -> &mut Borrowed;
	}

## cell
Share-able mutable containers
	//t: Send, Clone, Copy, Default, Eqs, Ords, From, CoerceUnsized
	pub struct Cell<T> { /* fields omitted */ }
		fn get(&self) -> T
		fn new(value: T) -> Cell<T>
		fn as_ptr(&self) -> *mut T
		fn get_mut(&mut self) -> &mut T
		fn get_mut(&mut self) -> &mut T
		fn replace(&self, val: T) -> T
		fn into_inner(self) -> T
		fn take(&self) -> T
	//t: Default, From, CoerceUnsized, Debug
	pub struct UnsafeCell<T: ?Sized> { /* fields omitted */ }
		fn new(value: T) -> UnsafeCell<T>
		unsafe fn into_inner(self) -> T
		fn get(&self) -> *mut T
	//t: Send, Copy, Clone, Default, Eqs, Ords, From, CoerceUnsized
	pub struct RefCell<T: ?Sized> { /* fields omitted */ }
		fn new(value: T) -> RefCell<T>
		fn into_inner(self) -> T
		fn borrow_state(&self) -> BorrowState
		fn borrow(&self) -> Ref<T>
		fn try_borrow(&self) -> Result<Ref<T>, BorrowError>
		fn borrow_mut(&self) -> RefMut<T>
		fn try_borrow_mut(&self) -> Result<RefMut<T>, BorrowMutError>
		unsafe fn as_unsafe_cell(&self) -> &UnsafeCell<T>
		fn as_ptr(&self) -> *mut T

## char
Character manipulation
	
