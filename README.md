# NES Emulator in Rust-WASM 

## Installation

Requires Rust with cargo, nodejs, and wasm-pack.

## Build

```bash
cargo build
cd wasmnes/www
wasm-pack build
```

## Run

To run on localhost:
```bash
npm start
```

## Resources

### CPU

http://archive.6502.org/datasheets/rockwell_r65c00_microprocessors.pdf
https://www.masswerk.at/6502/6502_instruction_set.html

Please make sure to update tests as appropriate.

## License
[MIT](https://choosealicense.com/licenses/mit/)