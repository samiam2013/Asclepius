use std::thread::sleep;
use std::time::Duration;

fn main() {
    // loop continuously
    loop {
        use std::process::Command;

        let output = Command::new("sh")
                .arg("-c")
                .arg("ssh fog w")
                .output()
                .expect("failed to execute process");

        let hello = String::from_utf8(output.stdout).expect("Found invalid UTF-8");
        // get the uptime and load (before the \n newline)
        let uptime = hello.split("\n").next().unwrap();

        println!("output: {:?}", uptime);

        // if it's turned off, try to turn it back on, with a backoff function 2 ^ n tries
        sleep(Duration::from_secs(5))
    }
}
