# BigDecimalMath-RS [![bigdecimalmath-rs](https://docs.rs/bond/badge.svg?style=svg)](https://docs.rs/bigdecimalmath) [![CircleCI](https://circleci.com/gh/ekump/bigdecimalmath-rs.svg?style=svg)](https://circleci.com/gh/ekump/bigdecimalmath-rs)

[Documentation](https://docs.rs/bigdecimalmath/0.1.0/bigdecimalmath/)

A collection of mathematical functions for the [BigDecimal](https://github.com/akubera/bigdecimal-rs) type. These functions are Rust implementations of Richard J. Mathar's [A Java Math.BigDecimal Implementation of Core Mathematical Functions](https://arxiv.org/abs/0908.3030v3). Where necessary, some functions from [OpenJDK's implementation of BigDecimal](https://github.com/openjdk-mirror/jdk7u-jdk/blob/master/src/share/classes/java/math/BigDecimal.java) have also been reimplemented here.

## Note
I write Rust code regularly, but I am not an expert in either Java or numerical analysis. I created this repo because certain mathematical functions are not present in [bigdecimal-rs](https://github.com/akubera/bigdecimal-rs) that I need for another side project I'm working on. Contributions, feature requests, and bug reports are welcome, but I have a day job and cannot promise quick turnarounds. 
