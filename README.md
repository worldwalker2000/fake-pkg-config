# Fake pkg-config

I want my make files to work on windows as well as unix-like with out modification.
But on windows I install packages with vcpkg, so I need the calls to pkg-config to just always return vcpkg's lib and include dirs.
That is what this program does.

The code is very simple so you can modify the paths that are output for --cflags and --libs easily just by changing the source code.
Their is no config file because I am too lazy.

Then you can install with ```cargo install```
