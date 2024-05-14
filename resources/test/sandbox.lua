--[[
    Copyright (c) 2024-present Robert Anderson.
    SPDX-License-Identifier: MIT

    Tests changes to the LUA runtime to create a more secure sandbox.

    See notes in '_vendor/lua/INFO' to view all changes.
]]

function whitelist_module(mod, wl)
    for k, v in pairs(_G[mod]) do
        if not wl[k] then
            print("["..mod.."] Contains unexpected key '"..k.."'")
            assert(false)
        end
    end
end

-- Functionality for arbitrary script loading (except through 'require') should
-- have been removed (from the core Lua APIs [lbaselib.c]).
assert(type(dofile) == "nil")
assert(type(load) == "nil")
assert(type(loadfile) == "nil")
assert(type(require) == "function")

-- Functionality in the 'package' module [loadlib.c] considered unsafe should be gone.
assert(type(package) == "table")
assert(type(package.loadlib) == "nil")
assert(type(package.cpath) == "nil")
assert(type(package.path) == "nil")
assert(type(package.searchers) == "nil")
whitelist_module("package", {
    searchpath = true,
    loaded = true,
    preload = true,
})

-- Loading of binary LUA chunks via 'require' [loadlib.c] should fail.
local status, err = pcall(require, "hello-bin")
assert(not status)
assert(err == [[error loading module 'hello-bin' from file './hello-bin.lua':
	attempt to load a binary chunk (mode is 't')]])

-- Removal of 'unsafe' OS module [loslib.c] functionality.
assert(type(os.execute) == "nil")
assert(type(os.exit) == "nil")
assert(type(os.getenv) == "nil")
assert(type(os.remove) == "nil")
assert(type(os.rename) == "nil")
assert(type(os.setlocale) == "nil")
assert(type(os.tmpname) == "nil")
whitelist_module("os", {
    clock = true,
    date = true,
    difftime = true,
    time = true,
})

-- The 'string' module rep function should be gone ('unsafe' for memory).
assert(type(string) == "table")
assert(type(string.rep) == "nil")

-- Entire 'io' module should be removed. Will have a better "asset" module
-- for loading assets from a secure location.
assert(type(io) == "nil")

-- Entire 'debug' module should be removed from the Lua side.
assert(type(debug) == "nil")
