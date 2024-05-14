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

use std::{
    ffi::CStr,
    marker::{PhantomData, PhantomPinned},
};

/**
 * Constants for run-time version and validation checking.
 */
const LUA_VERSION_NUMBER: LuaNumber = (version_major!() * 100 + version_minor!()) as LuaNumber;
const LUA_NUMBER_SIZES: usize =
    (std::mem::size_of::<LuaInteger>() << 4) + std::mem::size_of::<LuaNumber>();

/**
 * Constants for code execution.
 */
const LUA_LOAD_MODE_TEXT_ONLY: &'static [u8; 2usize] = b"t\x00";
const LUA_MULTRET: std::ffi::c_int = -1;

/**
 * Constants for return status codes
 */
const LUA_OK: std::ffi::c_int = 0;
const LUA_ERRRUN: std::ffi::c_int = 2;

/**
 * Lua "native" FFI types
 */
#[repr(C)]
struct LuaStateInternal {
    _data: [u8; 0],
    _marker: PhantomData<(*mut u8, PhantomPinned)>,
}

type LuaNumber = std::ffi::c_double;
type LuaInteger = std::ffi::c_longlong;
// pub type LuaUnsigned = std::ffi::c_ulonglong;

type LuaKContext = isize;
type LuaKFunction = unsafe extern "C-unwind" fn(
    L: *mut LuaStateInternal,
    status: std::ffi::c_int,
    ctx: LuaKContext,
) -> std::ffi::c_int;

//
// Native Lua "C" functions.
//
#[link(name = "lua")]
extern "C-unwind" {
    fn luaL_checkversion_(state: *mut LuaStateInternal, ver: LuaNumber, sz: usize);

    // State management
    fn luaL_newstate() -> *mut LuaStateInternal;
    fn lua_close(state: *mut LuaStateInternal);

    // Module loader functions
    fn luaL_openlibs(state: *mut LuaStateInternal);

    // Stack access / manipulations
    fn lua_tolstring(
        state: *mut LuaStateInternal,
        idx: std::ffi::c_int,
        len: *mut usize,
    ) -> *const std::ffi::c_char;

    // Code execution
    fn lua_pcallk(
        state: *mut LuaStateInternal,
        nargs: std::ffi::c_int,
        nresults: std::ffi::c_int,
        msgh: std::ffi::c_int,
        ctx: LuaKContext,
        k: Option<LuaKFunction>,
    ) -> std::ffi::c_int;

    // Auxiliary library functions
    fn luaL_loadfilex(
        state: *mut LuaStateInternal,
        file_name: *const std::ffi::c_char,
        mode: *const std::ffi::c_char,
    ) -> std::ffi::c_int;
}

/**
 * Lua helper functions (translated from #defines in "C" API layer).
 */
#[inline]
unsafe fn lua_pcall(
    state: *mut LuaStateInternal,
    nargs: std::ffi::c_int,
    nresults: std::ffi::c_int,
    msgh: std::ffi::c_int,
) -> std::ffi::c_int {
    lua_pcallk(state, nargs, nresults, msgh, 0, None)
}

#[inline]
unsafe fn lua_tostring(
    state: *mut LuaStateInternal,
    idx: std::ffi::c_int,
) -> *const std::ffi::c_char {
    lua_tolstring(state, idx, std::ptr::null_mut())
}

/**
 * Lua state wrapper
 */
#[repr(transparent)]
pub struct LuaState(*mut LuaStateInternal);

/**
 * Lua state constructors.
 */
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

/**
 * A simple drop implementation for opaque Lua state structure that
 * will handle making sure it gets cleaned up on the 'C' side.
 */
impl Drop for LuaState {
    fn drop(&mut self) {
        unsafe {
            assert!(!self.0.is_null());
            lua_close(self.0);
        }

        self.0 = std::ptr::null_mut();
    }
}

/**
 * Pure Rust (safe) helpers for inspecting the Lua state and run-time.
 */
impl LuaState {
    pub const fn version_string(&self) -> &'static str {
        version_string!()
    }
}

/**
 * Wrappers around the Lua state for module management.
 */
impl LuaState {
    pub fn openlibs(&self) {
        unsafe {
            assert!(!self.0.is_null());
            luaL_openlibs(self.0);
        }
    }
}

/**
 * Wrappers around the Lua state for loading / managing script code.
 */
impl LuaState {
    pub fn run_script(&self, script_path: &std::path::Path) -> Result<(), crate::Error> {
        unsafe {
            assert!(!self.0.is_null());

            let raw_file_name = crate::helpers::cstr_from_path(script_path)?;

            // Attempt to load the LUA script file. This does NOT run the script, just
            // leaves the script (as a function) on the top of the stack (if successful).
            // We still need to run it if all is good.
            {
                let res = luaL_loadfilex(
                    self.0,
                    raw_file_name.as_ptr(),
                    LUA_LOAD_MODE_TEXT_ONLY.as_ptr() as *const _,
                );
                if res != LUA_OK {
                    Err(crate::Error::LuaScriptLoadError)
                } else {
                    Ok(())
                }
            }?;

            // If we get here, the script was loaded. We can now run it by just
            // invoking a pcall. We definitely want this in protected mode.
            let res = lua_pcall(self.0, 0, LUA_MULTRET, 0);
            if res == LUA_ERRRUN {
                let err = CStr::from_ptr(lua_tostring(self.0, -1));
                println!("lua_pcall(...) = {:?} ({})", err, res);
                Err(crate::Error::LuaScriptExecutionError(
                    err.to_string_lossy().into_owned(),
                ))
            } else {
                Ok(())
            }
        }
    }
}
