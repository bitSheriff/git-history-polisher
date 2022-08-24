use std::process::Command;

pub fn system_call(cmd: String, args: String) -> String {

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(cmd)
            .args(args)
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg(cmd)
            .arg(args)
            .output()
            .expect("failed to execute process")
    };
    output.stdout
}
