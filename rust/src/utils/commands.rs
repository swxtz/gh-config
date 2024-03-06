use std::process::Command;

/// Run a command via cli with arguments
///
/// # How to use
/// ```no_run
/// run_command(command: &str, args: Vec<&str>);
/// ```
/// it can return an error or an Ok
///
/// # Example
/// ```no_run
/// run_command("git", ["add", "."].to_vec());
/// ```
///
///
///

pub fn run_command(command: &str, args: Vec<&str>) -> Result<(), std::io::Error> {
    let mut cmd = Command::new(command);
    cmd.args(args);

    let status = cmd.status();

    if status.is_ok() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Erro ao executar o comando: {:?}", status),
        ))
    }
}
