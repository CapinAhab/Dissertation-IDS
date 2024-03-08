

## Compiling 

The program is compiled with cargo like any other Rust project, however the version of the libuary tch is only compatable with libtorch version 2.2.0. The correct version of libtorch [https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.2.0%2Bcpu.zip](https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.2.0%2Bcpu.zip) but the compiler still needs to be pointed to it. By default tch will try to use a libuary at the location /usr/lib/libtorch.so, but the location of the libuary can be set with the environment variable LIBTORCH, bellow is an example of how to use it.

```
export LIBTORCH=/path_to_project/libtorch
cd path_to_project
cargo build
```

The compiler also uses the library libglog which has also been updated, so you will need to downgrade to version 0.6.0 or move the shared object files in the libglog directory to /usr/lib/.

```
mv libglog/ /usr/lib/
```
