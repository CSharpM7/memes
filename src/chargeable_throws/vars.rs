//use super::*;
//use crate::imports::imports_acmd::*;
use smash::phx::Vector3f;
pub mod fighter {
    pub mod instance {
        pub mod flag {
            pub const THROW_SMASH_HOLD : i32 = 0x2100000f;
            pub const THROW_SMASH_HOLD_FINISH : i32 = 0x2100000b;
        }
        pub mod int {
            pub const THROW_SMASH_PHASE : i32 = 0x11000009;
            pub const THROW_SMASH_LOOP_FRAME : i32 = 0x1100000e;
            pub const THROW_SMASH_HOLD_KEEP_FRAME : i32 = 0x1100000f;
        }
        pub mod float {
            pub const THROW_SMASH_CLATTER_START : i32 = 0x1000008;
            pub const THROW_SMASH_RESTART_FRAME : i32 = 0x1000009;
        }
    }
}