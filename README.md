![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# drone-fec

Blazing fast implmentation of various forward error correction algorithms optimized for embedded systems.
The crate contains:

* A BCJR decoder, parallelized using the `SIMD` instructions for `Cortex-M4`.
* A Turbo decoder.
* An iterative QPP interleaver, with parameters from `3GPP`.

## Usage

Add the crate to your `Cargo.toml` dependencies:

```toml
[dependencies]
drone-fec = { git = "https://github.com/rmja/drone-fec" }
```

## References
There are a series of references that are needed to understand the code in this repository:

* [_Implementation of 3GPP LTE QPP Interleaver for SiLago_](ref/qpp.pdf) by Spandan Dey
* [_Turbo Codes in UMTS/WiMAX/LTE Systems: Solutions for an Efficient FPGA Implementation_](ref/bcjr.pdf) by Christian ANGHEL
* [_From BCJR to Turbo decoding: MAP algorithms made easier_](ref/turbo.pdf) by Silvio A. Abrantes

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.