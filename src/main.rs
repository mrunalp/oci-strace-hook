extern crate serde;
extern crate serde_json;
extern crate nix;
extern crate daemonize;

#[macro_use] extern crate clap;
#[macro_use] extern crate serde_derive;

use std::io::{self, Read};
use nix::unistd::execvp;
use std::ffi::CString;
use std::path::Path;
use daemonize::Daemonize;

#[derive(Serialize, Deserialize)]
struct ContainerState {
    id: String,
    pid: i32
}

fn main() {
    let matches = clap_app!(ocistracehook =>
        (version: "0.1.0")
        (about: "OCI strace hook")
        (@arg LOGDIR: -l --logdir +takes_value "Specify directory for storing logs")
    ).get_matches();

    let log_dir = matches.value_of("logdir").unwrap_or("/tmp");

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let cs: ContainerState = serde_json::from_str(&buffer).unwrap();

    println!("ID: {}, Pid: {}", cs.id, cs.pid);
    let log_file_path = Path::new(log_dir).join(cs.id).as_os_str().to_str().unwrap().to_string();
    println!("LogPath: {}", log_file_path);

    let prog = CString::new("strace").unwrap();
    let args = &[CString::new("strace").unwrap(), CString::new("-f").unwrap(),
                 CString::new("-o").unwrap(), CString::new(log_file_path).unwrap(),
                 CString::new("-p").unwrap(), CString::new(cs.pid.to_string()).unwrap()];

    let daemonize = Daemonize::new().working_directory("/tmp").umask(0o777);
    daemonize.start().unwrap();

    execvp(&prog, args).unwrap();
}
