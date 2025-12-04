use super::{Command, CommandsRegistry};

pub struct EchoCommand;

impl Command for EchoCommand {
    fn run(
        &self,
        args: Vec<&str>,
        _: &CommandsRegistry,
        redirect_path: Option<String>,
    ) -> Result<(), String> {
        if let Some(path) = redirect_path {
            let output_str = args.join(" ");
            let _ = std::fs::write(path, format!("{}\n", output_str));
            Ok(())
        } else {
            Ok(println!("{}", args.join(" ")))
        }
    }

    fn get_name(&self) -> String {
        "echo".to_string()
    }

    fn get_type_message(&self) -> String {
        format!("{} is a shell builtin", self.get_name())
    }
}
