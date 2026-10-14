use std::path::{Path, PathBuf};

use crate::ssh::SshConnection;

pub(crate) fn host() -> String {
    std::env::var("SSH_TEST_HOST").unwrap_or_else(|_| "127.0.0.1".into())
}

pub(crate) fn port() -> u16 {
    std::env::var("SSH_TEST_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(2222)
}

pub(crate) fn user() -> String {
    std::env::var("SSH_TEST_USER").unwrap_or_else(|_| "tester".into())
}

pub(crate) fn key_path() -> PathBuf {
    if let Ok(p) = std::env::var("SSH_TEST_KEY") {
        return PathBuf::from(p);
    }
    let manifest = Path::new(env!("CARGO_MANIFEST_DIR"));
    [
        manifest.join(".sandbox/id_ed25519"),
        manifest.join("../.sandbox/id_ed25519"),
    ]
    .into_iter()
    .find(|p| p.exists())
    .expect("could not find .sandbox/id_ed25519; set SSH_TEST_KEY")
}

pub(crate) fn connect() -> SshConnection<(String, u16)> {
    SshConnection::from_private_key(user(), key_path(), (host(), port()))
}
