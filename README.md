wopi
====

`wopi` is an implementation of Microsoft's [Web Application Open Platform Interface Protocol (MS-WOPI)](https://msdn.microsoft.com/en-us/library/hh622722(v=office.12).aspx), which enabled use with document storage software, such as Office 365 and Collabora Office (web version of LibreOffice). `wopi` targets version 9.0 of the specification.

The intended use case is to host your own instance of Collabora Office as an alternative to [ownCloud](https://owncloud.org/) and [Nextcloud](https://nextcloud.com/).

installation
============

`wopi` builds on Rust nightly and the minimum `rustc` is `1.21.0-nightly (2017-08-10)` (due to Rocket 0.3.1).

Clone this repository and build with `cargo`:

    $ cargo build --release

By default, `wopi` connects to `https://localhost:9980` (default for Collabora Office CODE in examples). To make it work, you'll need to fetch its TLS certificates. There is a provided tool you can use to do do this:

    $ cargo run --bin get_certs localhost:9980

The above command will put the DER-encoded certificates into a directory called `certs` in the working directory, which this project will use.

Set up your database and put the authentication details in a file at the root called `.env`, such as:

    $ echo "DATABASE_URL=postgres://wopi@localhost/wopi"

Initialize the database:

    $ diesel setup

Finally run with `cargo run`:

    $ cargo run --bin wopi

license
=======

`wopi` is licensed under the terms of both the MIT license and the Apache License (Version 2.0). See LICENSE-MIT and LICENSE-APACHE for details.
