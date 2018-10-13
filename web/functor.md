## C++-ish functor in Rust with the fn_traits
The term functor seems to have quite a few meanings in Computer Science.
In category theory, a functor is simply a map of one category to another.
In Haskell, we use the term to generalize any function that performs
mapping operations, so here, the definition is pretty clear.
But in Prolog, functor is a type of atom used in compound terms.
And finally, in C++ (and some other languages) a functor is a function
that carries state, making it a stateful function.

To create a functor in C++, you just implement the __()__ operator on a class,
like this:

```cpp
#include <iostream>
#include <string>

class Functor
{
	public:
		Functor(int x) : x(x) {}
		int operator()() { return ++x; }
	private:
		int x;
};

int main()
{
    Functor inc(1);
    int x = inc();
    std::cout << x << '\n';     // <-- 2
    std::cout << inc() << '\n'; // <-- 3
    std::cout << inc() << '\n'; // <-- 4
    std::cout << inc() << '\n'; // <-- 5
    std::cout << inc() << '\n'; // <-- 6
}
```

Let's see how to recreate the exact same functor in Rust.

In Rust, instead of implementing operators on types directly, you implement
a trait particular to a given operator.

The usage of traits for common operators is pretty straightforward. Here is
an example showing how to implement the __Add__ trait (it is a simplification
of the Rust docs example):

```rust
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Point(i32, i32);

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point(self.0 + other.0, self.1 + other.1)
    }
}

fn main() {
	assert_eq!(Point(1, 0) + Point(2, 3), Point(3, 3));
}
```

Now, to create a functor like the one above, we need to bring the trio of
function traits into play: **Fn**, **FnMut** and **FnOnce**. If you have
ever used Rust, you might have already encountered these traits when working
with closures, but chances are you had no idea that these traits could be
implemented on structs.

Sadly, at this point, implementing the Fn traits is not as ergonomic as it could
be.

First things first, we need to enable these two feaures (that requires a nightly
compiler) and import the traits:

```rust
#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::ops::{FnMut, FnOnce};
```

We need to add the __unboxed\_closures__ feature as well, because the trait functions
for Fn et al. use the _rust-call_ calling convention, which is subject to change.

So, the next step is to add a struct and a 'constructor', to make sure we have the
type as in C++:

```rust
#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::ops::{FnMut, FnOnce};

struct Functor { x: i32 }

impl Functor {
    fn new(initial: i32) -> Functor {
        Functor { x: initial }
    }
}
```

And here is the tricky part. You see, the fn traits are not standalone. There is, in fact,
a hierarchy between them. Lowest in the food chain is __Fn__ trait. It can be used in all
context where you'd expect any of the three traits. As such, both __FnMut__ and __FnOnce__
are its supertraits. Therefore, in order to use it, you would need to implement all three
of them.

Luckily, we won't need the __Fn__ trait, as we are mutating the state on every call.

The _middle_ trait is __FnMut__, it can be used in contexts where you would expect either
__FnMut__ (surprisingly) or __FnOnce__. As such, __FnOnce__ is its supertrait and we will
need to implement these two traits.

__FnOnce__ can only be used where one would expect __FnOnce__, so no other contexts and
no supertraits, easy stuff.

Let's get to the implementing the traits on our stuff. First, we need to implement
__FnOnce__:

```rust
#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::ops::{FnMut, FnOnce};

struct Functor { x: i32 }

impl Functor {
    fn new(initial: i32) -> Functor {
        Functor { x: initial }
    }
}

impl FnOnce<()> for Functor {
    type Output = i32;
    extern "rust-call" fn call_once(mut self, _args: ())
        -> Self::Output
    {
        self()
    }
}
```

This will of course not compile, because we are already calling _self_ in the implementation,
so let's quickly throw in the __FnMut__ implementation too:

```rust
#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::ops::{FnMut, FnOnce};

struct Functor { x: i32 }

impl Functor {
    fn new(initial: i32) -> Functor {
        Functor { x: initial }
    }
}

impl FnOnce<()> for Functor {
    type Output = i32;
    extern "rust-call" fn call_once(mut self, _args: ())
        -> Self::Output
    {
        self()
    }
}

impl FnMut<()> for Functor {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> i32 {
        self.x += 1;
        self.x
    }
}
```

And that's it. This is a one-for-one clone of the C++ functor from the beginning of this
pseudo-guide.

All that's left now is to test it:

```rust
#![feature(fn_traits)]
#![feature(unboxed_closures)]

use std::ops::{FnMut, FnOnce};

struct Functor { x: i32 }

impl Functor {
    fn new(initial: i32) -> Functor {
        Functor { x: initial }
    }
}

impl FnOnce<()> for Functor {
    type Output = i32;
    extern "rust-call" fn call_once(mut self, _args: ())
        -> Self::Output
    {
        self()
    }
}

impl FnMut<()> for Functor {
    extern "rust-call" fn call_mut(&mut self, _args: ()) -> i32 {
        self.x += 1;
        self.x
    }
}


fn main() {
    let mut fun = Functor::new(1);
    let x = fun();
    println!("{}", x);     // 2
    println!("{}", fun()); // 3
    println!("{}", fun()); // 4
    println!("{}", fun()); // 5
    println!("{}", fun()); // 6
}
```

If all goes well, if you try running both of the examples, you should get the exact same output.
