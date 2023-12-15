use std::{env::args, process::Command};

mod language;
mod translator;

fn main() {
    let arguments: Vec<String> = args().collect();
    let message = &arguments[1];
    let output = translator::translate(message);
    println!("Translated message: {}", output);
    let mut status_code = Command::new("git")
        .arg("add")
        .arg("-A")
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code();
    if status_code != Some(0) {
        println!("Fail");
        return;
    }
    status_code = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(output)
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .code();
    println!(
        "{}",
        if status_code == Some(0) {
            "Success"
        } else {
            "Fail"
        }
    );
}
