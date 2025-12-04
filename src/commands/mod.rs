mod echo;
mod exit;
mod typ;

mod registry;
pub use registry::CommandsRegistry;

mod external;
pub use external::ExternalCommand;

pub use echo::EchoCommand;
pub use exit::ExitCommand;
pub use typ::TypeCommand;

pub trait Command {
    fn run(&self, args: Vec<&str>, reg: &CommandsRegistry) -> Result<(), String>;
    fn get_name(&self) -> String;
    fn get_type_message(&self) -> String;
}
