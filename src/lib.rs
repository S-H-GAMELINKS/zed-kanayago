use zed_extension_api::{self as zed, Result};

struct KanayagoExtension;

impl zed::Extension for KanayagoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: "kanayago".to_string(),
            args: vec!["--lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(KanayagoExtension);
