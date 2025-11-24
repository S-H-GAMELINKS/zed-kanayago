use zed_extension_api::{self as zed, Result};

struct KanayagoExtension;

impl zed::Extension for KanayagoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match worktree.which("kanayago") {
            Some(path) => Ok(zed::Command {
                command: path,
                args: vec!["--lsp".into()],
                env: Default::default(),
            }),
            None => Err(
                "Not found kanayago. Please run 'gem install kanayago'".into(),
            ),
        }
    }
}

zed::register_extension!(KanayagoExtension);
