use std::env;

use zed_extension_api::{self as zed, Command, LanguageServerId, Result, Worktree};

struct ZedCdsLspExtension {
    server_found: bool,
}

impl ZedCdsLspExtension {
    const SERVER_PATH: &str = "node_modules/@sap/cds-lsp/dist/main.js";
    const PACKAGE_NAME: &str = "@sap/cds-lsp";

    fn server_exists(&self) -> bool {
        let path = std::path::Path::new(Self::SERVER_PATH);
        path.exists() && path.is_file()
    }

    fn ensure_server(&mut self, language_server_id: &LanguageServerId) -> Result<()> {
        let server_exists = self.server_exists();
        if self.server_found && server_exists {
            return Ok(());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let version = zed::npm_package_latest_version(Self::PACKAGE_NAME)?;
        let installed_version = zed::npm_package_installed_version(Self::PACKAGE_NAME)?;

        if !server_exists || installed_version.as_deref() != Some(&version) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::npm_install_package(Self::PACKAGE_NAME, &version)?;
            if !self.server_exists() {
                return Err(format!(
                    "Failed to install language server `{}`",
                    Self::PACKAGE_NAME
                ));
            }
        }

        self.server_found = true;
        Ok(())
    }
}

impl zed::Extension for ZedCdsLspExtension {
    fn new() -> Self {
        Self {
            server_found: false,
        }
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

        self.ensure_server(language_server_id)?;

        let path = env::current_dir()
            .unwrap()
            .join(Self::SERVER_PATH)
            .to_string_lossy()
            .to_string();
        Ok(Command {
            command: zed::node_binary_path()?,
            args: vec![path, "--stdio".to_string()],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(ZedCdsLspExtension);
