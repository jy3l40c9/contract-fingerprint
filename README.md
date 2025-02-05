# Contract fingerprint
This program prints, if possible, the compiler version from the contract. In addition, it can print detected functions and memory using the `evmole` crate. This is mainly just a ready-to-use program based on the work from Jon-Becker. See Acknowledgements.


## Usage
Compile the program with `make release` and run:

```shell
contract-fingerprint 0x5c69bee701ef814a2b6a3edd4b1652cb9cc5aa6f --evmole
```

## Acknowledgements

Many thanks to Jon-Becker for publishing: [Compiler Fingerprinting in EVM Bytecode](https://github.com/Jon-Becker/research/blob/45e0a53/papers/evm-compiler-fingerprinting/evm-compiler-fingerprinting.md). The code for `detect_compiler_new_with_proxies.rs` is based/copied from the article. Also, many thanks for [heimdall-rs](https://github.com/Jon-Becker/heimdall-rs). And many thanks to the contributors of [evmole](https://github.com/cdump/evmole).

## License
This project is licensed under the [MIT License](./LICENSE).