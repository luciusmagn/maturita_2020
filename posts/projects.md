tags = ["magnusi"]
title = "Projects"
hidden = true


These are some projects of mine. Of course,
all of them are opensource. Unless specified otherwise,
every project (apart from forks, of course) is licensed
under the [ISC license](https://choosealicense.com/licenses/isc/).
There is no particular order.

## Rustgrade
A client/server duo for managing students' grades. It is just
a small example of how to use Udp for networking and TOML
Serialization/Deserialization with Serde.

Repository: <https://github.com/rust-gjk/rustgrade>

## Gjkbot
A simple bot for Github, which moves repositories
and checks for repositories, which have been
'handed in'. Ran as a cronjob on my Raspberry Pi in
conjuction with the update script from the Snapshot
repository.

Repository: <https://github.com/rust-gjk/gjkbot>

## Rhai & Nary
A small, embeddable and fast interpreted programming language
written in Rust. It is pretty early into development,
but already shows some promising qualities. Originally
written by Jonathan D. Turner. I am a maintainer and Developer
since fall 2017.
Nary is my experimental fork with a lot of dangerous
code and multithreading.

Rhai repository: <https://github.com/jonathandturner/rhai>  
Nary repository: <https://github.com/luciusmagn/nary_lang>

## light_pencil
My fork of Sharp Pencil, which is a fork of Pencil. Pencil
is a Rust web microframework inspired by Flask. For my
intents, Pencil was too big and taking too long to compile.
My fork contains many simplifications, unfinished and broken
stuff removed, clippy lints fixed, updated dependencies,
unneeded stuff removed and stripped from everything else
I didn't need, including documentation. The codebase has
been roughly halved, dep count lowered by five and compile
times pleasantly decreased.

Repository: <https://github.com/luciusmagn/light_pencil>

## Stackbomb
A collection of programs testing how deep can you go in
nested calls till you blow up the stack. Please contribute
more languages if you can. This is a fun experiment.
Haskell is surprisingly good.

Repository: <https://github.com/luciusmagn/stackbomb>

## MiniLinuxFS
Small generator of an absurdly small statically linked
Linux filesystem. The default configuration contains
most common UNIX tools and Bash but only has about 4mb.
Thanks to being statically linked and not containing
any complicated programs, it is very secure.
Used for testing students' script in Learnshell at
the ČVUT college.

Repository: <https://github.com/luciusmagn/minilinuxfs>

## Rustkr
Some of the excersizes and programs from K&R implemented
in C, but embedded in a Rust catalogueing program. The
purpose was to learn about FFI in Rust in a way that
isn't as painful as translating C2TC is/was.

Repository: <https://github.com/luciusmagn/rustkr>

## Rusty
A tiny build-system for C/C++/ASM. Written in C and
uses the mpc library for parsing its configuration
files called rustyfiles. It has nothing to do with Rust
language. Rusty was written long before I knew Rust
existed. The name meant 'A program to refresh my Rusty
C programming skills'.

Repository: <https://github.com/luciusmagn/rusty>

## C2TC
C2 Tiny Compiler. Originally intended to be a small
compiler for C2, hopefully free from LLVM, the behemoth.
Very WIP, but can parse 100% of C2. It doesn't compile

Repository: <https://github.com/luciusmagn/c2tc>

## Markov
Implementation of Markov chains. Based on orangeduck's
program from his artistic coding blog. Daniel's
implementation is in Python, my is in Rust lang.

Repository: <https://github.com/luciusmagn/Markov>

## net_47sb_59vm
A small http server written in C#. It contains a
support for modules written in any .NET language
thanks to dynamic assembly loading.

Repository: <https://github.com/luciusmagn/net_47sb_59vm>

## micro-config
One file C# library for making tiny config files. Not
much else to be said. Doesn't support many types.

Repository: <https://github.com/luciusmagn/micro-config>

## Wren.NET
Small .NET wrapper for Wren language. The Wren language
included is slightly modified to allow calling from .NET
(stupid Windows calling conventions).

Repository: <https://github.com/luciusmagn/Wren.NET>

## microtest
A few lines in a C header to test your C code. Can print a
pretty verdict if needed. Really handy if you don't have
resources for a real testing framework. Based on an old
theoretical tutorial I read a few years ago somewhere.

Repository: <https://github.com/luciusmagn/microtest>

## TSGui
Graphical interface for TShock Terraria servers. Very pretty,
but people from elevators keep breaking it. I don't really
play Terraria anymore.

Repository: <https://github.com/luciusmagn/TSGui>

## c2net
A tiny networking library for the C2 programming language.
Made as a proof-of-concept. It is a heavily modified
version of zed_net library ported to C2.

Repository: <https://github.com/c2hub/c2net>

## c2vector
A tiny vector library for the C2 programming language. Nothing
more to be said.

Repository: <https://github.com/c2hub/c2vector>

## recipe-reader
A Rust library for reading (and writing) C2's recipe files, which
are needed for compilation. Pretty efficient, but has unconventional
API which needs a refreshment.

Repository: <https://github.com/c2hub/recipe-reader>

## Pebble & Pebble-server
Package manager for C2 language libraries. Packages for Pebble are,
suprisingly, called pebbles. Mimicks Cargo in many ways. Has
really nice coloring. Uses Clap, which is a great command-line library. 
