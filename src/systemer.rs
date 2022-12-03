use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use rand::Rng;

        
pub fn system_call(cmd: String, args: String) {
}

pub fn git_add(dir: &String) {
    let command = Command::new("git")
                  .arg("-C")
                  .arg(dir)
                  .arg(" add")
                  .arg(".")
                  .status();

}

pub fn git_commit(dir: &String, msg: String) {
    let msg_cmd = format!("-m \"{}\"", msg);
     
    let command = Command::new("git")
                  .arg("-C")
                  .arg(dir)
                  .arg(" commit")
                  .arg(msg_cmd)
                  .status();
}

pub fn sys_create_file(name: &String) {

    let command = Command::new("touch")
                  .arg(name)
                  .status();
}

pub fn sys_change_file(name: String) {

    let mut rng = rand::thread_rng();
    let randNum = rng.gen_range(0 .. i32::MAX);

    let echCmd = format!("echo '{}' > ", randNum);

    let command = Command::new("touch")
                  .arg(name)
                  .status();
}

pub fn echo_test() {
    let command = Command::new("echo").
        arg("hello")
        .status();
}

