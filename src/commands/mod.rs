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

pub trait Command {
    fn run(
        &self,
        args: Vec<&str>,
        reg: &CommandsRegistry,
        redirect_path: Option<String>,
        redirect_error_path: Option<String>,
    ) -> Result<(), String>;
    fn get_name(&self) -> String;
    fn get_type_message(&self) -> String;
}
