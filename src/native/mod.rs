/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:   lua
 * Module:  native
 *
 * Purpose:
 *   Define the native "C" API layer + wrappers.
 */

#[macro_use]
mod version;

mod state;
mod types;

pub(crate) use state::LuaState;
