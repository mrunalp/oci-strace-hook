extern crate serde;
extern crate serde_json;
extern crate nix;
extern crate daemonize;

#[macro_use]
extern crate serde_derive;

use std::io::{self, Read};
use nix::unistd::execvp;
use std::ffi::CString;
use daemonize::Daemonize;

#[derive(Serialize, Deserialize)]
struct ContainerState {
    id: String,
    pid: i32
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let cs: ContainerState = serde_json::from_str(&buffer).unwrap();

    println!("ID: {}, Pid: {}", cs.id, cs.pid);
    let pid_str = cs.pid.to_string();
    let prog = CString::new("strace").unwrap();
    let args = &[CString::new("strace").unwrap(), CString::new("-f").unwrap(),
                            CString::new("-o").unwrap(), CString::new("/tmp/strace.out").unwrap(),
                             CString::new("-p").unwrap(), CString::new(pid_str).unwrap()];

    let daemonize = Daemonize::new()
        .pid_file("/tmp/test.pid")
        .chown_pid_file(true)
        .working_directory("/tmp")
        .umask(0o777);
    daemonize.start().unwrap();
    execvp(&prog, args).unwrap();
}
