use std::sync::Arc;
use parking_lot::Mutex;
use anyhow::Result;
use tracing::{info, error};

mod pe_loader;
mod dll_redirect;
mod syscall_handler;

pub use pe_loader::PELoader;
pub use dll_redirect::DllRedirector;
pub use syscall_handler::SyscallHandler;

#[derive(Debug, thiserror::Error)]
pub enum TranslationError {
    #[error("Failed to load PE file: {0}")]
    PELoadError(String),
    #[error("Syscall translation failed: {0}")]
    SyscallError(String),
    #[error("DLL redirection failed: {0}")]
    DllError(String),
}

pub struct ApiTranslator {
    pe_loader: Arc<Mutex<PELoader>>,
    dll_redirector: Arc<Mutex<DllRedirector>>,
    syscall_handler: Arc<Mutex<SyscallHandler>>,
}

impl ApiTranslator {
    pub fn new() -> Result<Self> {
        Ok(Self {
            pe_loader: Arc::new(Mutex::new(PELoader::new()?)),
            dll_redirector: Arc::new(Mutex::new(DllRedirector::new()?)),
            syscall_handler: Arc::new(Mutex::new(SyscallHandler::new()?)),
        })
    }

    pub fn load_executable(&self, path: &str) -> Result<()> {
        info!("Loading executable: {}", path);
        self.pe_loader.lock().load(path)?;
        Ok(())
    }

    pub fn handle_syscall(&self, syscall_number: u32, args: &[u64]) -> Result<u64> {
        self.syscall_handler.lock().handle(syscall_number, args)
    }

    pub fn redirect_dll(&self, dll_name: &str) -> Result<String> {
        self.dll_redirector.lock().redirect(dll_name)
    }
} 