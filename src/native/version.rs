/*
 * Copyright (c) 2024-present Robert Anderson.
 * SPDX-License-Identifier: MIT
 *
 * Crate:   lua
 * Module:  native/version
 *
 * Purpose:
 *   Define helper macros / constants for version data of native "C" LUA apis.
 */

macro_rules! version_major {
    () => {
        5
    };
}

macro_rules! version_minor {
    () => {
        4
    };
}

macro_rules! version_patch {
    () => {
        6
    };
}

macro_rules! version_string {
    () => {
        concat!(
            version_major!(),
            ".",
            version_minor!(),
            ".",
            version_patch!()
        )
    };
}
