use std::process::Command;

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
