use crate::imports::imports_agent::*;



unsafe fn common_update(fighter: &mut L2CFighterCommon) {
    if let Some(info) = FighterInfo::get_common(fighter) {
        let boma = &mut *info.boma;

        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_THROWN {
            let opponent_clat = ControlModule::get_clatter_time(boma, 0);
            println!("Thrown Clatter: {opponent_clat}");
        }
        else if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_CAPTURE_WAIT {
            let opponent_clat = ControlModule::get_clatter_time(boma, 0);
            println!("Capture Clatter: {opponent_clat}");
        }
    }
}

#[smashline::fighter_frame_callback]
fn global_fighter_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let category = smash::app::utility::get_category(boma);
        let kind = smash::app::utility::get_kind(boma);
        if [*BATTLE_OBJECT_CATEGORY_WEAPON,*BATTLE_OBJECT_CATEGORY_FIGHTER].contains(&category) {
            //common_update(fighter);
        }
    }
}

pub fn install() {
    smashline::install_agent_frame_callbacks!(
      global_fighter_frame
    );
}