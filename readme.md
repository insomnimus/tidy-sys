# tidy-sys
Rust bindings for the [HTML Tidy](https://github.com/htacg/tidy-html5) library.

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

## Troubleshooting
If you get errors during the install step with cmake and you're on Windows:
- Use a different CMake generator by setting the `CMAKE_GENERATOR` env variable; I've not had good luck with MSBuild on my machine. Ninja works fine.
- Clear CC/CXX environment variables.

##Crate Features
Currently there's only one feature, by default disabled:
- `localization`: Enable multi-language support in Tidy (sets the `SUPPORT_LOCALIZATIONS` option to `on` with CMake).
