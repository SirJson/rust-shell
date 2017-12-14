use shell_child::ShellChild;
use result::ShellResult;
use result::ShellError;
use std::path::Path;
use std::process::Command;

pub struct ShellCommand {
    command: Command,
    has_group: bool
}

impl ShellCommand {
    pub fn new(command: Command) -> ShellCommand {
        ShellCommand {
            command: command,
            has_group: false,
        }
    }

    pub fn set_has_group(mut self) -> Self {
        self.has_group = true;
        self
    }

    pub fn current_dir(self, _path: &Path) -> Self {
        unimplemented!()
    }

    pub fn env(self, _name: &str, _value: &str) -> Self {
        unimplemented!()
    }

    pub fn run(self) -> ShellResult {
        self.spawn().and_then(|job| job.wait())
    }

    pub fn spawn(self) -> Result<ShellChild, ShellError> {
        ShellChild::new(self.command, self.has_group)
    }
}

#[test]
fn test_shell_command_2() {
    use std::process::Command;
    let mut command = Command::new("echo");
    command.arg("The command was run");
    ShellCommand::new(command).set_has_group();
}