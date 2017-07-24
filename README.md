wopi
====

`wopi` is an implementation of Microsoft's [Web Application Open Platform Interface Protocol (MS-WOPI)](https://msdn.microsoft.com/en-us/library/hh622722(v=office.12).aspx), which enabled use with document storage software, such as Office 365 and Collabora Office (web version of LibreOffice). `wopi` targets version 9.0 of the specification.

The intended use case is to host your own instance of Collabora Office as an alternative to [ownCloud](https://owncloud.org/) and [Nextcloud](https://nextcloud.com/).

installation
============

Clone this repository and build with `cargo`:

    $ cargo build --release

By default, `wopi` connects to `https://localhost:9980` (default for Collabora Office CODE in examples). To make it work, you'll need to fetch its TLS certificates:

Fetch certificates:

    $ openssl s_client -showcerts -connect 127.0.0.1:9980 < /dev/null

Extract the certificate(s) from the file (starts with `-----BEGIN CERTIFICATE-----` and ends with `-----END CERTIFICATE-----`). For each certificate, turn it into a DER-encoded certificate:

    $ openssl x509 -in my-cert.cert -outform der -out my-cert.der

Put the DER-encoded certificates into a directory called `certs` in the working directory.

Run with `cargo run`.

license
=======

`wopi` is licensed under the terms of both the MIT license and the Apache License (Version 2.0). See LICENSE-MIT and LICENSE-APACHE for details.
