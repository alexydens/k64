# k64
## Why?
I was writing up a kernel in C when I read an article in which the NSA declared
that C is not a memory safe language, and should be phased out where possible,
so I decided to use a memory safe language instead. That began a struggle with
the rust compiler and cargo build system that is not yet over. This is said
struggle. If you have Rust, git, make and xorriso and are running a UNIX based
system, you should be able to test it yourself. You can build the ISO image
with `make`, and run it with your emulator of choice, or even on real hardware
if you so desire. I use `make test`, but that requires you to have my version
of bochs compiled and in the right directory.
