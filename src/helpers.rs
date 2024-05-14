/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:   lua
 * Module:  helpers
 *
 * Purpose:
 *   Collection of simple helper functions used throughout crate.
 */

#[cfg(not(unix))]
pub(crate) fn cstr_from_path(path: &std::path::Path) -> Result<std::ffi::CString, crate::Error> {
    let path = path.to_str().ok_or(super::Error::BadFilePathError)?;
    std::ffi::CString::new(path).map_err(|_| super::Error::BadFilePathError)
}

#[cfg(unix)]
pub(crate) fn cstr_from_path(path: &std::path::Path) -> Result<std::ffi::CString, crate::Error> {
    use std::os::unix::ffi::OsStrExt;

    std::ffi::CString::new(path.as_os_str().as_bytes()).map_err(|_| super::Error::BadFilePathError)
}
