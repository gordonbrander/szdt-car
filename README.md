# szdt-car

[![CI](https://img.shields.io/github/actions/workflow/status/n0-computer/szdt-car/ci.yml)](https://github.com/n0-computer/szdt-car/actions?query=workflow%3A%22Continuous+integration%22)

[.car](https://dasl.ing/car.html) support for [SZDT](https://github.com/gordonbrander/szdt). "CAR" stands
for Content Addressable aRchives. A CAR file contains a series of content-addressed data blocks.

Currently supports only [v1](https://ipld.io/specs/transport/car/carv1/).

szdt-car is a fork of [iroh-car](https://github.com/n0-computer/iroh-car) that adds support for serializing/deserializing open-ended metadata in the CBOR CAR header.

## License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br/>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>
