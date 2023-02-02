# NOTE
This project is still WIP, please help us to make this functional!!!
## The build system
- Setup the release enviroment with 
  * ```meson setup target/release --buildtype=release --optimization=3 --strip -Db_lto=true -b_lto_threads=$(nproc)```
- Setup the debug eviroment with
  *`meson setup target/debug --buildtype=debug --optimization=g`
- You can specify a compiler to use using the `CC` and `CXX` compiler flags
  * Example: You can use the llvm toolchain by setting up the build directory by using:
    * `CC=clang CXX=clang++ CC_LD=lld CXX_LD=lld`

[![Meson](https://github.com/JimniLinux/kernel-manager/actions/workflows/meson.yml/badge.svg)](https://github.com/JimniLinux/kernel-manager/actions/workflows/meson.yml)
