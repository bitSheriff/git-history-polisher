use std::process::Command;
use std::fs::File;
use std::io::{Error, Write};
use rand::Rng;

        
pub fn system_call(cmd: String) -> String {

    let _command = Command::new(cmd).output().expect("failed to execute process");
    //return String::from_utf8(command.stdout);
    //return String::from_utf8_lossy(&command.stdout)
    todo!()
}

pub fn git_add(dir: &String) {
    let _command = Command::new("git")
                  .arg("-C")
                  .arg(dir)
                  .arg("add")
                  .arg(".")
                  .status();

}

pub fn git_commit(dir: &String, date: &String, msg: String) {
    let msg_cmd = format!("-m \"{}\"", msg);
     
    let _command = Command::new("git")
                  .arg("-C")
                  .arg(dir)
                  .arg("commit")
                  .arg(msg_cmd)
                  .arg("--date")
                  .arg(date)
                  .status();
}


pub fn sys_change_file(name: &String) -> Result<(), Error> {

    let mut rng = rand::thread_rng();
    let rand_num = rng.gen_range(0 .. i32::MAX);
    let mut file = File::create(name)?;
    let string = rand_num.to_string();

    let str_int: &str = &string[..];

    writeln!(file, "{}", str_int)?;
    Ok(())
}

