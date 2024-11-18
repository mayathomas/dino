use anyhow::Result;
use std::{env, fs};

use clap::Parser;

use crate::{build_project, CmdExecutor, JsWorker};

#[derive(Debug, Parser)]
pub struct RunOpts {}

impl CmdExecutor for RunOpts {
    async fn execute(self) -> Result<()> {
        let cur_dir = env::current_dir()?.display().to_string();
        let filename = build_project(&cur_dir)?;
        let content = fs::read_to_string(&filename)?;
        let worker = JsWorker::try_new(&content)?;
        worker.run("await handlers.hello()")?;
        Ok(())
    }
}
