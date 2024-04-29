/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:   lua
 * Module:  native/state
 *
 * Purpose:
 *   Wrapper around native "C" LUA state.
 */

use super::types::*;
use std::marker::{PhantomData, PhantomPinned};

const LUA_VERSION_NUMBER: LuaNumber = (version_major!() * 100 + version_minor!()) as LuaNumber;
const LUA_NUMBER_SIZES: usize =
    (std::mem::size_of::<LuaInteger>() << 4) + std::mem::size_of::<LuaNumber>();

#[link(name = "lua")]
extern "C-unwind" {
    fn luaL_checkversion_(state: *mut LuaStateInternal, ver: LuaNumber, sz: usize);

    fn luaL_newstate() -> *mut LuaStateInternal;
    fn lua_close(state: *mut LuaStateInternal);

    fn luaL_openlibs(state: *mut LuaStateInternal);
}

#[repr(C)]
struct LuaStateInternal {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

#[repr(transparent)]
pub struct LuaState(*mut LuaStateInternal);

impl LuaState {
    pub fn new() -> Option<Self> {
        unsafe {
            let ctx = luaL_newstate();
            if ctx.is_null() {
                return None;
            }

            luaL_checkversion_(ctx, LUA_VERSION_NUMBER, LUA_NUMBER_SIZES);

            Some(Self(ctx))
        }
    }

    //TODO: fn new_with_allocator(...) -> Option<Self> {}
}

impl Drop for LuaState {
    fn drop(&mut self) {
        unsafe {
            assert!(!self.0.is_null());
            lua_close(self.0);
        }

        self.0 = std::ptr::null_mut();
    }
}

impl LuaState {
    pub const fn version_string(&self) -> &'static str {
        version_string!()
    }
}

impl LuaState {
    pub fn openlibs(&self) {
        unsafe {
            assert!(!self.0.is_null());
            luaL_openlibs(self.0);
        }
    }
}
