use std::process::Command;

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

pub fn sys_create_file(name: String) {

    let command = Command::new("touch")
                  .arg(name)
                  .status();
}

pub fn sys_change_file(name: String) {

}

pub fn echo_test() {
    let command = Command::new("echo").
        arg("hello")
        .status();
}
