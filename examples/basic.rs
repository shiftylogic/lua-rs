/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Demonstrate basic embedding of the LUA run-time.
 *
 */

use lua_sandbox::LuaRuntime;

fn main() {
    let rt = LuaRuntime::new().expect("lua runtime failure");
    println!("Lua Version: {}", rt.version_string());

    rt.openlibs();
    // rt.run_script(
    //     "/Users/robert/src/game-ws/lua-rs/_vendor/lua-5.4.6/tests/lua-5.4.6-tests/all.lua",
    // );
}
