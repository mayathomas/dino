mod cli;
mod utils;
pub use cli::*;
mod engine;
use enum_dispatch::enum_dispatch;

pub use engine::*;
pub(crate) use utils::*;
pub const BUILD_DIR: &str = ".build";

#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExecutor {
    async fn execute(self) -> anyhow::Result<()>;
}
