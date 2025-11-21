use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

struct ZedCdsLspExtension {}

impl ZedCdsLspExtension {
    const SERVER_BINARY_NAME: &'static str = "cds-lsp";
}

impl zed::Extension for ZedCdsLspExtension {
    fn new() -> Self {
        Self {}
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &Worktree,
    ) -> Result<Command> {
        if language_server_id.as_ref() != "cap_cds" {
            return Err(format!(
                "Unknown language server id: {}",
                language_server_id
            ));
        }

        // Look up cds-lsp in PATH
        let path = worktree
            .which(Self::SERVER_BINARY_NAME)
            .ok_or_else(|| format!("Could not find `{}` on PATH", Self::SERVER_BINARY_NAME))?;

        Ok(Command {
            command: path,
            args: vec!["--stdio".to_string()],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(ZedCdsLspExtension);
