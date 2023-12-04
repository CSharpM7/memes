use crate::imports::imports_acmd::*;

#[acmd_script( agent = "pickel", script = "game_attacks4charge", category = ACMD_GAME)]
unsafe fn game_attacks4charge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        WorkModule::off_flag(agent.module_accessor, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        WorkModule::set_int(agent.module_accessor, *FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SWORD, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);

        FighterSpecializer_Pickel::add_material_num(agent.module_accessor,*FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND,1);
    }
}

#[acmd_script( agent = "pickel", script = "effect_attacks4charge", category = ACMD_EFFECT)]
unsafe fn effect_attacks4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        EFFECT(agent, Hash40::new_raw(0x138c307f4f),Hash40::new("top"),  //pickel_craft_icon_diamond?
        0.0,0.0,0.0, 
        0.0,0.0,0.0, 
        1.0, 0,0,0,0,0,0,false);
    }

    frame(agent.lua_state_agent, 5.0);
    for i in 1..i32::MAX{
        if macros::is_excute(agent) {
            macros::FOOT_EFFECT(agent, Hash40::new("sys_run_smoke"), Hash40::new("top"), -2, 0, 0, 0, 0, 0, 1, 10, 0, 4, 0, 0, 0, false);
        }
        wait(agent.lua_state_agent, 5.0);
        macros::EFFECT_FLIP(agent, Hash40::new("sys_smash_flash_s"), Hash40::new("sys_smash_flash_s"), Hash40::new("top"), -3, 15, -7.4, 0, 0, 0, 1, 3, 3, 3, 0, 0, 0, true, *EF_FLIP_YZ);
    }
}

#[acmd_script( agent = "pickel", script = "sound_attacks4charge", category = ACMD_SOUND)]
unsafe fn sound_attacks4charge(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 1.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_pickel_special_n05"));
    }
    frame(agent.lua_state_agent, 2.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_common_smash_start"));
    }
}

#[acmd_script( agent = "pickel", script = "expression_attacks4charge", category = ACMD_EXPRESSION)]
unsafe fn expression_attacks4charge(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
        //physics!(*MA_MSC_CMD_PHYSICS_START_CHARGE, -1, -1, -1, -1, 0.1, -1, Hash40::new("invalid"));
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold1"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(agent.lua_state_agent, 61.0);
    if macros::is_excute(agent) {
        ControlModule::set_rumble(agent.module_accessor, Hash40::new("rbkind_smashhold2"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

pub fn install() {
    install_acmd_scripts!(
        game_attacks4charge,
        effect_attacks4charge,
        sound_attacks4charge,
        //expression_attacks4charge,
    );
}