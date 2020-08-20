<!-- Copyright (c) 2020 Christopher Taylor                                          -->
<!--                                                                                -->
<!--   Distributed under the Boost Software License, Version 1.0. (See accompanying -->
<!--   file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)        -->

# Framedata - A Rust Dataframe Library

Project details [here](http://www.github.com/ct-clmsn/framedata/).

This project implements a dataframe functionality for rust alongside several
rolling and summary statistics. This project originally served as a learning
exercise for the language, specifically rust's algebraic enumeration support.
Takes inspiration from [Hossein Moein's C++ DataFrame libary](https://github.com/hosseinmoein/DataFrame).

### License

Boost Software License

### Features
* Load data from csv (needs improvement)
* Group Data
* Summary Statistics: sum, mean, standard deviation, population standard deviation
* Window Summary Statistics: variance, rolling mean, rolling standard deviation, rolling variance, difference, percent change
* Custom data types (float, integer, string) with hashing support
* Bloom filter implementation

### Demo
`cargo run --example example1`

### TODO
* Add parallelization support
* Improve robustness of csv support
* TimeSeries analysis support
* Index support
* More statistics

### Author
Christopher Taylor

### Dependencies
[Rust](https://www.rust-lang.org)

### Thank you
* [DATA.GOV](http://data.gov/) - for providing [the hourly precipitation sample](https://catalog.data.gov/dataset/u-s-hourly-precipitation-data) included in this project. 
