use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
 
pub fn system_call(cmd: String, args: String) {
}

pub fn git_add(dir: String) {
    let command = Command::new("git")
                  .arg("-- git-dir")
                  .arg(dir)
                  .arg("add")
                  .arg(".")
                  .status();

}

pub fn git_commit(dir: String, msg: String) {
    let msg_cmd = format!("-m \"{}\"", msg);
     
    let command = Command::new("git")
                  .arg("-- git-dir")
                  .arg(dir)
                  .arg("commit")
                  .arg(msg_cmd)
                  .status();
}

pub fn sys_change_file(pathString: String, name: String, text: String) {

    let path = Path::new(&pathString);

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", path.display(), why),
        Ok(_) => println!("successfully wrote to {}", path.display()),
    }

}

pub fn echo_test() {
    let command = Command::new("echo").
        arg("hello")
        .status();
}
