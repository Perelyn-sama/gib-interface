# gib-interface 

Simple library for generating solidity interfaces from solidity contracts

## Installing the cli locally
First you have to clone this repo to your machine:
```bash
git clone https://github.com/Perelyn-sama/gib-interface
```
Next, you have to enter the `gib-interface` directory then install it:
```bash
cd gib-interface 
cargo install .
```
Now you when you run:
```bash
gib-interface --help
```
You should see:
```bash
Simple program to get a solidity interface from a solidity contract

Usage: gib-interface [OPTIONS] --contract-file <CONTRACT_FILE>

Options:
  -c, --contract-file <CONTRACT_FILE>    Contract to make interface for
  -f, --function-names <FUNCTION_NAMES>  Functions you want to be in the interface
  -h, --help                             Print help
  -V, --version                          Print version
```

## Contributing

All contributions are welcome! Experimentation is highly encouraged and new issues are welcome.

## Troubleshooting & Bug Reports

Please check existing issues for similar bugs or
[open an issue](https://github.com/Perelyn-sama/gib-interface/issues/new)

if no relevant issue already exists.

## License

This project is licensed under the [MIT License](LICENSE.md).