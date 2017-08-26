extern crate regex;

extern crate wopi;

use std::env::args;
use std::process::{Command, Stdio};
use std::io::Write;

use regex::Regex;

use wopi::errors::*;

fn main() {
    // TODO: use an argument parser
    let args: Vec<String> = args().collect();
    if args.len() != 2 {
        println!("Usage: {} <addr>", args[0]);
        return;
    }

    if let Err(ref e) = run(args[1].clone()) {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}


fn run(addr: String) -> Result<()> {
    let out = Command::new("openssl")
        .args(&["s_client", "-showcerts", "-connect", addr.as_str()])
        .stdin(Stdio::null())
        .output()
        .chain_err(|| "error getting certs")?;

    let re = Regex::new(
        r"((?m)-----BEGIN CERTIFICATE-----[^-]+-----END CERTIFICATE-----)",
    ).chain_err(|| "error initializing regex")?;

    let output = String::from_utf8_lossy(&out.stdout);
    let mut i = 0;
    for cert in re.captures_iter(&output) {
        let cert = cert.get(1).unwrap().as_str();
        let cert_file = format!("certs/cert-{}.der", i);
        println!("creating cert: {}", cert_file);
        let mut child = Command::new("openssl")
            .args(&["x509", "-outform", "der", "-out", cert_file.as_str()])
            .stdin(Stdio::piped())
            .spawn()
            .chain_err(|| "error starting openssl")?;

        child
            .stdin
            .as_mut()
            .unwrap()
            .write_all(cert.as_bytes())
            .chain_err(|| "error writing cert to openssl")?;
        child.wait().chain_err(|| "error converting cert to der")?;

        i += 1;
    }

    Ok(())
}
