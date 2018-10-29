const_time_enc
==============

[![Build Status](https://travis-ci.org/ceph3us/const-time-enc-rs.svg?branch=master)](https://travis-ci.org/ceph3us/const-time-enc-rs)
[![Latest Version](https://img.shields.io/crates/v/const_time_enc.svg)](https://crates.io/crates/const_time_enc) 
[![Coverage Status](https://coveralls.io/repos/github/ceph3us/const-time-enc-rs/badge.svg?branch=master)](https://coveralls.io/github/ceph3us/const-time-enc-rs?branch=master)

A crate containing a Rust port of
[ParagonIE's constant time encodings for PHP][cte-php]. No extra dependencies.

WARNING
-------

As per the terms of the [license](LICENSE), no guarantee of correctness,
fitness for use, or acceptance of liability is made. I am not a professional
cryptographer and I nor anyone else have audited this code. All use is strictly
at your own risk.

The performance will already be significantly worse than any lookup-based
hex/base64 encoding, but be aware that since this library does not have or
intend to have unsafe code, it may not be suitably performant for your usecase.

FEATURES
--------

 - [x] Base64 encoding
 - [x] Base64 decoding
 - [ ] Hex (base16) encoding
 - [ ] Hex (base16) decoding
 - [ ] Base32 encoding
 - [ ] Base32 decoding
 - [ ] Base32 (hex variant) encoding
 - [ ] Base32 (hex variant) decoding
 - [ ] `#![no_std]` support
 
[cte-php]: https://github.com/paragonie/constant_time_encoding
