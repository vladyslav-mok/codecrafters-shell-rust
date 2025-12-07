mod cd;
mod echo;
mod exit;
mod pwd;
mod typ;

mod registry;
pub use registry::CommandsRegistry;

mod external;
pub use external::ExternalCommand;

pub use cd::CdCommand;
pub use echo::EchoCommand;
pub use exit::ExitCommand;
pub use pwd::PwdCommand;
pub use typ::TypeCommand;

#[derive(Debug, Default)]
pub struct OutputOfCommand {
    pub output_create: Option<String>,
    pub error_output_create: Option<String>,
    pub output_append: Option<String>,
    pub error_output_append: Option<String>,
}

pub trait Command {
    fn run(
        &self,
        args: &[&str],
        reg: &CommandsRegistry,
        output_of_command: &OutputOfCommand,
    ) -> Result<(), String>;
    fn get_name(&self) -> String;
    fn get_type_message(&self) -> String;
}
