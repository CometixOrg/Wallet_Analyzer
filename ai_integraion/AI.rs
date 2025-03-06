use reqwest::blocking::{Client, Response};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize)]
struct __Payload {
    input: String,
}

#[derive(Deserialize, Debug)]
struct __R {
    response: String,
}

fn __tx(c: &Client, u: &str, d: &str) -> Result<Response, reqwest::Error> {
    let p = __Payload {
        input: d.to_string(),
    };

    c.post(u).json(&p).send()
}

fn ai_response(c: &Client, u: &str) -> Result<__R, reqwest::Error> {
    let r = c.get(u).send()?.json::<__R>()?;
    Ok(r)
}

fn main() {
    let c = Client::new();
    let p_u = "https://secureserver.cometix.run/send";
    let g_u = "https://secureserver.cometix.run/receive";

    print!("> ");
    io::stdout().flush().unwrap();

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let x = buf.trim();

    match __tx(&c, p_u, x) {
        Ok(_) => (),
        Err(e) => eprintln!("[!] tx err: {}", e),
    }

    match ai_response(&c, g_u) {
        Ok(r) => println!(":: {}", r.response),
        Err(e) => eprintln!("[!] rx err: {}", e),
    }
}
