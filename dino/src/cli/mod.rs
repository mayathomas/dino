mod build;
mod init;
mod run;

pub use self::{build::BuildOpts, init::InitOpts, run::RunOpts};
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[command(name="dino", version, author, about, long_about=None)]
pub struct Opts {
    //子命令
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExecutor)]
pub enum SubCommand {
    //子命令，-- csv，中间有个空格
    #[command(name = "init", about = "Init dino project")]
    Init(InitOpts),
    #[command(name = "build", about = "Build dino project")]
    Build(BuildOpts),
    #[command(name = "run", about = "Run project")]
    Run(RunOpts),
}
