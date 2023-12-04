use crate::imports::imports_agent::*;
use crate::chargeable_throws::vars::*;

#[skyline::hook(replace = L2CFighterCommon_status_Throw_Sub)]
unsafe extern "C" fn throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let toReturn = original!()(fighter);
    
    let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
    let opponent_clat = ControlModule::get_clatter_time(opponent, 0);
    WorkModule::set_float(fighter.module_accessor, opponent_clat, fighter::instance::float::THROW_SMASH_CLATTER_START);

    toReturn
}
#[skyline::hook(replace = L2CFighterCommon_status_Throw_Main)]
unsafe extern "C" fn throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let toReturn = original!()(fighter);
    let throw_phase = WorkModule::get_int(fighter.module_accessor,  fighter::instance::int::THROW_SMASH_PHASE);

    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if throw_phase == 0 {
        if WorkModule::is_flag(fighter.module_accessor, fighter::instance::flag::THROW_SMASH_HOLD) {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                //Store frame to resume to after charge
                let current_frame = MotionModule::frame(fighter.module_accessor);
                WorkModule::set_float(fighter.module_accessor, current_frame, fighter::instance::float::THROW_SMASH_RESTART_FRAME);
                
                //Set maximum smash frames
                let hold_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_s4_hold_frame"), 0);
                let keep_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_s4_hold_keep_frame"), 0);
                WorkModule::set_int(fighter.module_accessor, hold_frame, fighter::instance::int::THROW_SMASH_LOOP_FRAME);
                WorkModule::set_int(fighter.module_accessor, keep_frame, fighter::instance::int::THROW_SMASH_HOLD_KEEP_FRAME);

                //Change to charge motion
                let mut new_motion = Hash40::new("throw_f_hold");
                if motion == Hash40::new("throw_b").hash {
                    new_motion = Hash40::new("throw_b_hold");
                }
                else if motion == Hash40::new("throw_hi").hash {
                    new_motion = Hash40::new("throw_hi_hold");
                }
                else if motion == Hash40::new("throw_lw").hash {
                    new_motion = Hash40::new("throw_lw_hold");
                }
                MotionModule::change_motion(fighter.module_accessor, new_motion, 0.0, 1.0, false, 0.0, false, false);

                //Enable smash fx
                EffectModule::req_common(fighter.module_accessor, Hash40::new("smash_hold"),0.0);

                //Freeze opponent's anim,
                let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
                MotionModule::set_rate(opponent, 0.0);
                
                //Set clatter time based on Opponent's damage and clatter params
                let clatter_start = WorkModule::get_float(fighter.module_accessor, fighter::instance::float::THROW_SMASH_CLATTER_START);
                let damage_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("capture_cut_frame_damage"));
                let recovery_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("capture_recovery"));
                let clatter_frame_base = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("capture_cut_frame")) as f32; 
                let opponent_damage = DamageModule::damage(opponent, 0);
                let clatter_frame_add = opponent_damage * damage_frame_mul; 
                let clatter_add_mul = 0.25;
                //Cap the new clatter time at whatever the max would be for a normal grab
                let clatter_max = clatter_frame_base + clatter_frame_add;
                let clatter_frame = clatter_max.min(clatter_start + (clatter_frame_base + clatter_frame_add)*clatter_add_mul);

                println!("Init: {clatter_start} Base: {clatter_frame_base} Add: {clatter_frame_add} Clatter: {clatter_frame}");
                ControlModule::set_clatter_time(opponent, clatter_frame, 0);

                WorkModule::set_int(fighter.module_accessor, 1,fighter::instance::int::THROW_SMASH_PHASE);
            }
            else {
                WorkModule::set_int(fighter.module_accessor, 2,fighter::instance::int::THROW_SMASH_PHASE);
            }
        }
    }
    else if throw_phase == 1 {
        let hold_frame = WorkModule::get_int(fighter.module_accessor, fighter::instance::int::THROW_SMASH_LOOP_FRAME);
        let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
        let opponent_clat = ControlModule::get_clatter_time(opponent, 0);
        //Cut capture if opponent mashed out
        if opponent_clat <= 0.0 {
            StatusModule::change_status_force(opponent, FIGHTER_STATUS_KIND_CAPTURE_JUMP.into(), false.into());
            fighter.change_status(FIGHTER_STATUS_KIND_CATCH_CUT.into(), false.into());
            EffectModule::remove_common(fighter.module_accessor, Hash40::new("smash_hold"));
            return 0.into();
        }
        //Throw if letting go of grab
        if !ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
            WorkModule::off_flag(fighter.module_accessor, fighter::instance::flag::THROW_SMASH_HOLD);
        }
        else {
            //Decrement smash variables...
            if 0 < hold_frame {
                WorkModule::dec_int(fighter.module_accessor, fighter::instance::int::THROW_SMASH_LOOP_FRAME);
                if hold_frame-1 == 0 {
                    physics!(fighter, *MA_MSC_CMD_PHYSICS_STOP_CHARGE);
                }
            }
            else {
                //If Keep Frame hits 0, automatically throw
                WorkModule::dec_int(fighter.module_accessor, fighter::instance::int::THROW_SMASH_HOLD_KEEP_FRAME);
                if WorkModule::get_int(fighter.module_accessor, fighter::instance::int::THROW_SMASH_HOLD_KEEP_FRAME) <= 0
                {
                    WorkModule::off_flag(fighter.module_accessor, fighter::instance::flag::THROW_SMASH_HOLD);
                }
            }
        }
        
        //If no longer charging...
        if !WorkModule::is_flag(fighter.module_accessor, fighter::instance::flag::THROW_SMASH_HOLD) {
            //Return to throw anim at resume frame
            let resume_frame = WorkModule::get_float(fighter.module_accessor, fighter::instance::float::THROW_SMASH_RESTART_FRAME);
            let mut new_motion = Hash40::new("throw_f");
            if motion == Hash40::new("throw_b_hold").hash {
                new_motion = Hash40::new("throw_b");
            }
            else if motion == Hash40::new("throw_hi_hold").hash {
                new_motion = Hash40::new("throw_hi");
            }
            else if motion == Hash40::new("throw_lw_hold").hash {
                new_motion = Hash40::new("throw_lw");
            }
            MotionModule::change_motion(fighter.module_accessor, new_motion, resume_frame+1.0, 1.0, false, 0.0, false, false);

            //Kill smash effect
            EffectModule::remove_common(fighter.module_accessor, Hash40::new("smash_hold"));

            //Set attack power based on throw charge
            let hold_frame_max = WorkModule::get_param_int(fighter.module_accessor, hash40("attack_s4_hold_frame"), 0) as f32;
            let hold_frame = WorkModule::get_int(fighter.module_accessor, fighter::instance::int::THROW_SMASH_LOOP_FRAME) as f32;
            let hold_ratio = 1.0-(hold_frame/hold_frame_max);
            let attack_max_mul = 1.4; //Where tf is this param?
            let attack_mul = lerp(1.0,attack_max_mul,hold_ratio);
            AttackModule::set_power_mul_status(fighter.module_accessor, attack_mul);

            //Resume playback for opponent
            MotionModule::set_rate(opponent, 1.0);
            MotionModule::set_frame(opponent, resume_frame+1.0,true);
            
            WorkModule::set_int(fighter.module_accessor, 2,fighter::instance::int::THROW_SMASH_PHASE);
        }
    }
    toReturn
}
#[skyline::hook(replace = L2CFighterCommon_status_end_Throw)]
unsafe extern "C" fn throw_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    let toReturn = original!()(fighter);

    EffectModule::remove_common(fighter.module_accessor, Hash40::new("smash_hold"));

    toReturn
}

fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            throw_end,
            throw_main,
            throw_main_loop
        );
    }
}

pub fn install() {
    skyline::nro::add_hook(nro_hook);
}