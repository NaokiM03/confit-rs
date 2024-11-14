use windows::Win32::{
    Foundation::HANDLE,
    UI::Shell::{FOLDERID_RoamingAppData, SHGetKnownFolderPath, KNOWN_FOLDER_FLAG},
};

use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub(crate) fn config_dir() -> Option<PathBuf> {
    // https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/UI/Shell/fn.SHGetKnownFolderPath.html
    // https://learn.microsoft.com/en-us/windows/win32/api/shlobj_core/nf-shlobj_core-shgetknownfolderpath

    unsafe {
        let path = SHGetKnownFolderPath(
            &FOLDERID_RoamingAppData,
            KNOWN_FOLDER_FLAG::default(),
            HANDLE::default(),
        )
        .ok()?
        .to_string()
        .ok()?
        .into();

        Some(path)
    }
}
