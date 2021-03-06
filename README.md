# Rust OP-TEE TrustZone SDK

Rust OP-TEE TrustZone SDK provides abilities to build safe TrustZone
applications in Rust. The SDK is based on the [OP-TEE](https://www.op-tee.org/)
project which follows [GlobalPlatform](https://globalplatform.org/) TEE
specifications and provides ergonomic APIs. In addition, it enables capability
to write TrustZone applications with Rust's standard library and many
third-party libraries (i.e., crates).

Rust OP-TEE TrustZone SDK is under the [MesaTEE](https://mesatee.org) project.

## Getting started

To get started, you need to clone the project, initialize related submodules,
and install building dependencies.
Alternatively, you can use a docker container built with our [Dockerfile](Dockerfile).

``` sh
# clone the project and initialize related submodules
$ git clone git@github.com:mesalock-linux/rust-optee-trustzone-sdk.git
$ cd rust-optee-trustzone-sdk
$ git submodule update --init
$ (cd rust/compiler-builtins && git submodule update --init libm)
$ (cd rust/rust && git submodule update --init src/stdsimd src/llvm-project)

# install dependencies
$ sudo apt-get install android-tools-adb android-tools-fastboot autoconf \
        automake bc bison build-essential ccache cscope curl device-tree-compiler \
        expect flex ftp-upload gdisk iasl libattr1-dev libc6:i386 libcap-dev \
        libfdt-dev libftdi-dev libglib2.0-dev libhidapi-dev libncurses5-dev \
        libpixman-1-dev libssl-dev libstdc++6:i386 libtool libz1:i386 make \
        mtools netcat python-crypto python3-crypto python-pyelftools \
        python3-pycryptodome python3-pyelftools python-serial python3-serial \
        rsync unzip uuid-dev xdg-utils xterm xz-utils zlib1g-dev

# install Rust and select a proper version
$ curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain nightly-2019-07-08
$ source $HOME/.cargo/env
$ rustup component add rust-src && rustup target install aarch64-unknown-linux-gnu arm-unknown-linux-gnueabihf

# install patched Xargo
$ cargo install --git https://github.com/mssun/xargo.git --branch mssun/relative-patch-path --force
```

Then, download ARM toolchains and build OP-TEE libraries. Note that the OP-TEE
target is QEMUv8, and you can modify the Makefile to other targets accordingly.

``` sh
$ make optee
```

Before building examples, the environment should be properly setup.

``` sh
$ source environment
```

By default, the target platform is `aarch64`. If you want to build for the `arm`
target, you can setup `ARCH` before source the environment like this:

```sh
$ export ARCH=arm
$ source environment
```

At last, you can get started with our examples.

``` sh
$ make examples
```

Please read detailed
[instructions](https://github.com/mesalock-linux/rust-optee-trustzone-sdk/wiki/Getting-started-with-OPTEE-for-QEMU-ARMv8)
to run these examples on OP-TEE for QEMU. For other supported devices, please find
more documents [here](https://optee.readthedocs.io/building/devices/index.html).

## Contributing

The project is maintained by Mingshen Sun (@mssun) and Shengye Wan (@SimonWan),
under the supervision of steering committee (Tao Wei and Yulong Zhang).
Contributions are very welcome, please submit issues or send pull requests

## License

Rust OP-TEE TrustZone SDK is distributed under the Apache License (Version 2.0).
See [LICENSE](LICENSE) for details.
