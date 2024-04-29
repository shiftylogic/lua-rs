/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Build script for _vendor libraries.
 *
 */

const LUA_HEADERS: &'static [&'static str] = &[
    "_vendor/lua-5.4.6/src/lapi.h",
    "_vendor/lua-5.4.6/src/lgc.h",
    "_vendor/lua-5.4.6/src/lopnames.h",
    "_vendor/lua-5.4.6/src/lua.h",
    "_vendor/lua-5.4.6/src/lauxlib.h",
    "_vendor/lua-5.4.6/src/ljumptab.h",
    "_vendor/lua-5.4.6/src/lparser.h",
    "_vendor/lua-5.4.6/src/luaconf.h",
    "_vendor/lua-5.4.6/src/lcode.h",
    "_vendor/lua-5.4.6/src/llex.h",
    "_vendor/lua-5.4.6/src/lprefix.h",
    "_vendor/lua-5.4.6/src/lualib.h",
    "_vendor/lua-5.4.6/src/lctype.h",
    "_vendor/lua-5.4.6/src/llimits.h",
    "_vendor/lua-5.4.6/src/lstate.h",
    "_vendor/lua-5.4.6/src/lundump.h",
    "_vendor/lua-5.4.6/src/ldebug.h",
    "_vendor/lua-5.4.6/src/lmem.h",
    "_vendor/lua-5.4.6/src/lstring.h",
    "_vendor/lua-5.4.6/src/lvm.h",
    "_vendor/lua-5.4.6/src/ldo.h",
    "_vendor/lua-5.4.6/src/lobject.h",
    "_vendor/lua-5.4.6/src/ltable.h",
    "_vendor/lua-5.4.6/src/lzio.h",
    "_vendor/lua-5.4.6/src/lfunc.h",
    "_vendor/lua-5.4.6/src/lopcodes.h",
    "_vendor/lua-5.4.6/src/ltm.h",
];

const LUA_SOURCES: &'static [&'static str] = &[
    "_vendor/lua-5.4.6/src/lapi.c",
    "_vendor/lua-5.4.6/src/lauxlib.c",
    "_vendor/lua-5.4.6/src/lbaselib.c",
    "_vendor/lua-5.4.6/src/lcode.c",
    "_vendor/lua-5.4.6/src/lcorolib.c",
    "_vendor/lua-5.4.6/src/lctype.c",
    "_vendor/lua-5.4.6/src/ldblib.c",
    "_vendor/lua-5.4.6/src/ldebug.c",
    "_vendor/lua-5.4.6/src/ldo.c",
    "_vendor/lua-5.4.6/src/ldump.c",
    "_vendor/lua-5.4.6/src/lfunc.c",
    "_vendor/lua-5.4.6/src/lgc.c",
    "_vendor/lua-5.4.6/src/linit.c",
    "_vendor/lua-5.4.6/src/liolib.c",
    "_vendor/lua-5.4.6/src/llex.c",
    "_vendor/lua-5.4.6/src/lmathlib.c",
    "_vendor/lua-5.4.6/src/lmem.c",
    "_vendor/lua-5.4.6/src/loadlib.c",
    "_vendor/lua-5.4.6/src/lobject.c",
    "_vendor/lua-5.4.6/src/lopcodes.c",
    "_vendor/lua-5.4.6/src/loslib.c",
    "_vendor/lua-5.4.6/src/lparser.c",
    "_vendor/lua-5.4.6/src/lstate.c",
    "_vendor/lua-5.4.6/src/lstring.c",
    "_vendor/lua-5.4.6/src/lstrlib.c",
    "_vendor/lua-5.4.6/src/ltable.c",
    "_vendor/lua-5.4.6/src/ltablib.c",
    "_vendor/lua-5.4.6/src/ltm.c",
    "_vendor/lua-5.4.6/src/lundump.c",
    "_vendor/lua-5.4.6/src/lutf8lib.c",
    "_vendor/lua-5.4.6/src/lvm.c",
    "_vendor/lua-5.4.6/src/lzio.c",
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
