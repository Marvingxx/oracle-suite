//! User-owned applications registered locally, outside the release/install catalog.

use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocalApp {
    pub id: String,
    pub name: String,
    pub description: String,
    pub executable: PathBuf,
}

impl LocalApp {
    fn valid(&self) -> bool {
        !self.id.is_empty()
            && self.id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
            && !self.name.trim().is_empty()
            && oracle_suite_core::CraftApp::by_id(&self.id).is_none()
            && self.executable.is_absolute()
            && self.executable.is_file()
            && (!cfg!(windows) || self.executable.extension().and_then(|e| e.to_str()).is_some_and(|e| e.eq_ignore_ascii_case("exe")))
    }

    /// Launch directly, without passing a command through a shell.
    pub fn launch(&self) -> Result<(), String> {
        if !self.valid() {
            return Err(format!("Application unavailable: {}", self.name));
        }
        let mut command = crate::process::command(&self.executable);
        if let Some(parent) = self.executable.parent() {
            command.current_dir(parent);
        }
        crate::process::spawn_detached(command)
    }
}

/// Invalid, missing and duplicate entries are ignored. No installer owns these files.
pub fn load(path: &Path) -> Vec<LocalApp> {
    let entries: Vec<LocalApp> = std::fs::read(path).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
    let mut seen = std::collections::HashSet::new();
    entries.into_iter().filter(|app| app.valid() && seen.insert(app.id.clone())).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_registration_preserves_existing_executable_and_rejects_invalid_entries() {
        let dir = tempfile::tempdir().unwrap();
        let executable = dir.path().join("Pratique & notes.exe");
        std::fs::write(&executable, b"user-owned").unwrap();
        let app = LocalApp { id: "pratique".into(), name: "Pratique".into(), description: "Tutorials".into(), executable: executable.clone() };
        let mut reserved = app.clone();
        reserved.id = "oracle-edit".into();
        let mut relative = app.clone();
        relative.id = "relative".into();
        relative.executable = "Pratique.exe".into();
        let mut missing = app.clone();
        missing.id = "missing".into();
        missing.executable = dir.path().join("missing.exe");
        let path = dir.path().join("local-apps.json");
        std::fs::write(&path, serde_json::to_vec(&vec![app.clone(), app, reserved, relative, missing]).unwrap()).unwrap();
        let apps = load(&path);
        assert_eq!(apps.len(), 1);
        assert_eq!(apps[0].executable, executable);
        assert_eq!(std::fs::read(&executable).unwrap(), b"user-owned");
        std::fs::write(&path, b"invalid json").unwrap();
        assert!(load(&path).is_empty());
    }
}
