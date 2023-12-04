use crate::imports::imports_agent::*;


#[status_script(agent = "pickel", status = FIGHTER_PICKEL_STATUS_KIND_SPECIAL_N1_WAIT, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let originalReturn = original!(fighter);

    fighter.sub_change_motion_by_situation(Hash40::new("wait").into(), Hash40::new("wait").into(), false.into());
    fighter.sub_set_ground_correct_by_situation(true.into());

    fighter.sub_shift_status_main(L2CValue::Ptr(specialn_main_loop as *const () as _))
}

unsafe extern "C" fn specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let isMining = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_SPECIAL_N1_FLAG_MINING);
    let count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_SPECIAL_N1_INT_MINING_COUNT);
    let endFrame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PICKEL_STATUS_SPECIAL_N1_INT_MINING_END_FRAME);


    if count > endFrame {
        FighterSpecializer_Pickel::add_material_num(fighter.module_accessor,*FIGHTER_PICKEL_MATERIAL_KIND_DIAMOND,1);

        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_EFFECT,Hash40::new_raw(0x138c307f4f),-1);//0x1371600ae3, -1);
        LinkModule::send_event_nodes(fighter.module_accessor, *LINK_NO_ARTICLE,Hash40::new_raw(0x2280e3155a),0);
        EffectModule::req(fighter.module_accessor, Hash40::new_raw(0x138c307f4f), 
        &Vector3f{x:0.0,y:0.0,z:0.0}, 
        &Vector3f{x:0.0,y:0.0,z:0.0}, 
        1.0, 0,0,false,0);
        
        MotionAnimcmdModule::call_script_single(fighter.module_accessor, *FIGHTER_ANIMCMD_SOUND,Hash40::new_raw(0x19a6c86e10), -1);
    }

    if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
        StatusModule::change_status_request(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, false);
        /*
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        } */
    }

    0.into()
}



pub fn install() {
    install_status_scripts!(
        specialn_main,
    );
}