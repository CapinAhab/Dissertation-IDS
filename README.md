

## Building

### From Source
The program is compiled with cargo like any other Rust project, however the version of the libuary tch is only compatable with libtorch version 2.2.0. The correct version of libtorch [https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.2.0%2Bcpu.zip](https://download.pytorch.org/libtorch/cpu/libtorch-cxx11-abi-shared-with-deps-2.2.0%2Bcpu.zip) but the compiler still needs to be pointed to it. By default tch will try to use a libuary at the location /usr/lib/libtorch.so, but the location of the libuary can be set with the environment variable LIBTORCH, bellow is an example of how to use it.

```bash
export LIBTORCH=/path_to_project/libtorch
cd path_to_project
cargo build
```

The compiler also uses the library libglog which has also been updated, so you will need to downgrade to version 0.6.0 or move the shared object files in the libglog directory to /usr/lib/.

```bash
mv libglog/ /usr/lib/
```

### Docker
Due to the complex build process, a docker file has been provided to give a consistent build environment where the correct dependencies can be installed automatically. Docker is significantly simpler than building from source and is the preferred method even with its higher resource use over bare metal. Below is an example of how to start Docker and then create and run the Docker image.

```bash
sudo systemctl start docker
cd path_to_project
sudo docker build -t ids .
sudo docker run --network host ids
```
