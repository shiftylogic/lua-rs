/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:   lua
 * Module:  native/types
 *
 * Purpose:
 *   Define the native "C" data types for use by APIs.
 */

pub type LuaNumber = std::os::raw::c_double;
pub type LuaInteger = std::os::raw::c_longlong;
// pub type LuaUnsigned = std::os::raw::c_ulonglong;
