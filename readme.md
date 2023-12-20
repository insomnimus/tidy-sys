# tidy-sys
Rust bindings for the [HTML Tidy](https://github.com/htacg/tidy-html5) library.

## Documentation
- [tidy-sys](https://docs.rs/tidy-sys/latest/tidy_sys/)
- [libtidy](https://www.html-tidy.org/developer/)

## Note About Vendored Dependencies
The source code for Tidy is vendored in this repository for ease of use, under the `vendor/` directory.
Otherwise this is an unassociated project.

Also note that Tidy has its own license; this crate's license is for files excluding the `vendor/` directory.

## Requirements
This crate will build the Tidy library from source and generate bindings depending on the target platform.
Some external tools are required:
- CMake (Tidy uses CMake as the build system)
- A C compiler (to compile Tidy)
- Clang (as [bindgen](https://github.com/htacg/tidy-html5) needs it)

## Setting Build Options
This crate respects the standard environment variables used by C compilers and related tools, such as `CC`, `CFLAGS`, `LD` and etc.

In addition, you can set the `TIDY_SYS_CFLAGS` environment variable; the values will be appended to `CFLAGS` while building Tidy.

## Crate Features
Currently there's only one feature, by default disabled:
- `localization`: Enable multi-language support in Tidy (sets the `SUPPORT_LOCALIZATIONS` option to `on` with CMake).
