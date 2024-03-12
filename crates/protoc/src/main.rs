use rmp_serde as rmps;
use std::process::Command;

mod api;

fn main() {
    let output = Command::new("nvim").arg("--api-info").output().unwrap();
    let v: api::Api = rmps::from_slice(&output.stdout).unwrap();
    println!("{:#?}", v);
}
