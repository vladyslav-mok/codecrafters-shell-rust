mod app;
mod commands;

fn main() {
    let mut repl = app::Repl::new();
    repl.run();
}
