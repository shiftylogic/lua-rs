/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Build script for _vendor libraries.
 *
 */

const LUA_HEADERS: &'static [&'static str] = &[
    "_vendor/lua/src/lapi.h",
    "_vendor/lua/src/lgc.h",
    "_vendor/lua/src/lopnames.h",
    "_vendor/lua/src/lua.h",
    "_vendor/lua/src/lauxlib.h",
    "_vendor/lua/src/ljumptab.h",
    "_vendor/lua/src/lparser.h",
    "_vendor/lua/src/luaconf.h",
    "_vendor/lua/src/lcode.h",
    "_vendor/lua/src/llex.h",
    "_vendor/lua/src/lprefix.h",
    "_vendor/lua/src/lualib.h",
    "_vendor/lua/src/lctype.h",
    "_vendor/lua/src/llimits.h",
    "_vendor/lua/src/lstate.h",
    "_vendor/lua/src/lundump.h",
    "_vendor/lua/src/ldebug.h",
    "_vendor/lua/src/lmem.h",
    "_vendor/lua/src/lstring.h",
    "_vendor/lua/src/lvm.h",
    "_vendor/lua/src/ldo.h",
    "_vendor/lua/src/lobject.h",
    "_vendor/lua/src/ltable.h",
    "_vendor/lua/src/lzio.h",
    "_vendor/lua/src/lfunc.h",
    "_vendor/lua/src/lopcodes.h",
    "_vendor/lua/src/ltm.h",
];

const LUA_SOURCES: &'static [&'static str] = &[
    "_vendor/lua/src/lapi.c",
    "_vendor/lua/src/lauxlib.c",
    "_vendor/lua/src/lbaselib.c",
    "_vendor/lua/src/lcode.c",
    "_vendor/lua/src/lcorolib.c",
    "_vendor/lua/src/lctype.c",
    "_vendor/lua/src/ldblib.c",
    "_vendor/lua/src/ldebug.c",
    "_vendor/lua/src/ldo.c",
    "_vendor/lua/src/ldump.c",
    "_vendor/lua/src/lfunc.c",
    "_vendor/lua/src/lgc.c",
    "_vendor/lua/src/linit.c",
    "_vendor/lua/src/llex.c",
    "_vendor/lua/src/lmathlib.c",
    "_vendor/lua/src/lmem.c",
    "_vendor/lua/src/loadlib.c",
    "_vendor/lua/src/lobject.c",
    "_vendor/lua/src/lopcodes.c",
    "_vendor/lua/src/loslib.c",
    "_vendor/lua/src/lparser.c",
    "_vendor/lua/src/lstate.c",
    "_vendor/lua/src/lstring.c",
    "_vendor/lua/src/lstrlib.c",
    "_vendor/lua/src/ltable.c",
    "_vendor/lua/src/ltablib.c",
    "_vendor/lua/src/ltm.c",
    "_vendor/lua/src/lundump.c",
    "_vendor/lua/src/lutf8lib.c",
    "_vendor/lua/src/lvm.c",
    "_vendor/lua/src/lzio.c",
];

fn main() {
    let mut cfg = cc::Build::new();

    for src in LUA_HEADERS {
        println!("cargo::rerun-if-changed={}", src);
    }

    for src in LUA_SOURCES {
        println!("cargo::rerun-if-changed={}", src);
        cfg.file(src);
    }

    cfg.compile("liblua.a");
}
