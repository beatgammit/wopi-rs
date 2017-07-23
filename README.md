wopi
====

`wopi` is an implementation of Microsoft's [Web Application Open Platform Interface Protocol (MS-WOPI)](https://msdn.microsoft.com/en-us/library/hh622722(v=office.12).aspx), which enabled use with document storage software, such as Office 365 and Collabora Office (web version of LibreOffice). `wopi` targets version 9.0 of the specification.

The intended use case is to host your own instance of Collabora Office as an alternative to [ownCloud](https://owncloud.org/) and [Nextcloud](https://nextcloud.com/).

installation
============

Clone this repository and build with `cargo`:

    cargo build

license
=======

`wopi` is licensed under the terms of both the MIT license and the Apache License (Version 2.0). See LICENSE-MIT and LICENSE-APACHE for details.
