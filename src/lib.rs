/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Implements a Rust library to wrap the "C" LUA run-time in a sandbox.
 *
 */

mod native;

#[derive(Debug)]
pub enum Error {
    LuaError,
}

pub struct LuaRuntime(native::LuaState);

impl LuaRuntime {
    pub fn new() -> Result<Self, Error> {
        match native::LuaState::new() {
            Some(state) => Ok(Self(state)),
            None => Err(Error::LuaError),
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
}
