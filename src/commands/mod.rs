mod echo;
mod exit;
mod typ;
mod pwd;
mod cd;

mod registry;
pub use registry::CommandsRegistry;

mod external;
pub use external::ExternalCommand;

pub use echo::EchoCommand;
pub use exit::ExitCommand;
pub use typ::TypeCommand;
pub use pwd::PwdCommand;
pub use cd::CdCommand;

pub trait Command {
    fn run(&self, args: Vec<&str>, reg: &CommandsRegistry, redirect_path: Option<String>) -> Result<(), String>;
    fn get_name(&self) -> String;
    fn get_type_message(&self) -> String;
}
