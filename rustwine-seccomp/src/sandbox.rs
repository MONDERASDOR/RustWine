use std::sync::Arc;
use parking_lot::Mutex;
use anyhow::Result;
use libseccomp::*;
use nix::unistd::Pid;
use tracing::{info, warn};

#[derive(Debug, thiserror::Error)]
pub enum SandboxError {
    #[error("Failed to initialize seccomp")]
    SeccompInitError,
    #[error("Failed to add seccomp rule")]
    RuleError,
    #[error("Failed to load seccomp filter")]
    LoadError,
}

pub struct Sandbox {
    ctx: ScmpFilterContext,
}

impl Sandbox {
    pub fn new() -> Result<Self> {
        let ctx = ScmpFilterContext::new(ScmpAction::Allow)?;
        Ok(Self { ctx })
    }

    pub fn add_rule(&mut self, syscall: i32, action: ScmpAction) -> Result<()> {
        self.ctx.add_rule(action, syscall)?;
        Ok(())
    }

    pub fn load(&self) -> Result<()> {
        self.ctx.load()?;
        Ok(())
    }

    pub fn apply_to_pid(&self, pid: Pid) -> Result<()> {
        self.ctx.apply_to_pid(pid)?;
        Ok(())
    }
}

impl Drop for Sandbox {
    fn drop(&mut self) {
        let _ = self.ctx.reset(ScmpAction::Allow);
    }
} 