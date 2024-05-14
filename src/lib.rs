/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Implements a Rust library to wrap the "C" LUA run-time in a sandbox.
 *
 */

mod helpers;
mod native;

#[derive(Debug)]
pub enum Error {
    BadFilePathError,
    LuaInitializationError,
    LuaScriptLoadError,
    LuaScriptExecutionError(String),
}

pub struct LuaRuntime(native::LuaState);

impl LuaRuntime {
    pub fn new() -> Result<Self, Error> {
        match native::LuaState::new() {
            Some(state) => Ok(Self(state)),
            None => Err(Error::LuaInitializationError),
        }
    }
}

impl LuaRuntime {
    pub const fn version_string(&self) -> &'static str {
        self.0.version_string()
    }
}

impl LuaRuntime {
    #[inline]
    pub fn openlibs(&self) {
        self.0.openlibs();
    }

    #[inline]
    pub fn run_script(&self, script: &std::path::Path) -> Result<(), Error> {
        self.0.run_script(script)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn change_to_repo_dir(relative_target: &'static str) {
        let target_path: std::path::PathBuf = [env!("CARGO_MANIFEST_DIR"), relative_target]
            .iter()
            .collect();
        assert!(std::env::set_current_dir(&target_path).is_ok());
    }

    fn change_to_test_asset_dir() {
        change_to_repo_dir("resources/test");
    }

    fn run_test_script(script_target: &'static str) {
        change_to_test_asset_dir();

        let rt = LuaRuntime::new().expect("lua runtime failure");
        rt.openlibs();
        rt.run_script(&std::path::Path::new(script_target))
            .expect("test script failed");
    }

    #[test]
    fn test_lua_version() {
        let rt = LuaRuntime::new().expect("lua runtime failure");
        assert_eq!(rt.version_string(), "5.4.6");
    }

    #[test]
    fn test_lua_simple() {
        run_test_script("simple.lua");
    }

    #[test]
    fn test_lua_sandbox() {
        run_test_script("sandbox.lua");
    }
}
