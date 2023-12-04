use crate::imports::imports_agent::*;


#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_STATUS_MAIN)]
unsafe fn throw_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_Throw_Sub();
    fighter.sub_shift_status_main(L2CValue::Ptr(throw_main_loop as *const () as _))
}

unsafe extern "C" fn throw_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    if motion == Hash40::new("throw_b").hash {
        if WorkModule::is_flag(fighter.module_accessor, fighter::instance::flag::THROW_SMASH_HOLD) 
        && !WorkModule::is_flag(fighter.module_accessor, fighter::instance::flag::THROW_SMASH_HOLD_FINISH) {
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
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_b_hold"), 0.0, 1.0, false, 0.0, false, false);
                //Enable smash fx
                EffectModule::req_common(fighter.module_accessor, Hash40::new("smash_hold"),0.0);

                //Freeze opponent's anim,
                let opponent = get_grabbed_opponent_boma(fighter.module_accessor);
                MotionModule::set_rate(opponent, 0.0);
                
                //Set clatter time based on Opponent's damage and clatter params
                let damage_frame_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("capture_cut_frame_damage"));
                let recovery_frame = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("capture_recovery"));
                let clatter_frame_base = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("capture_cut_frame")) as f32; 
                let opponent_damage = DamageModule::damage(opponent, 0);
                let clatter_frame_add = opponent_damage * damage_frame_mul; 
                let clatter_frame =  (clatter_frame_base + clatter_frame_add);

                println!("Base: {clatter_frame_base} Add: {clatter_frame_add} Clatter: {clatter_frame}");
                ControlModule::set_clatter_time(opponent, clatter_frame, 0)
            }
            WorkModule::on_flag(fighter.module_accessor, fighter::instance::flag::THROW_SMASH_HOLD_FINISH);
        }
    }
    else if motion == Hash40::new("throw_b_hold").hash {
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
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("throw_b"), resume_frame+1.0, 1.0, false, 0.0, false, false);

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
        }
    }
    fighter.status_Throw_Main()
}

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn throw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let speed_max = 0.75;
    let accel = 0.0075;
    let frame_max = 41.0-12.0;
    if motion == Hash40::new("throw_b_charge").hash
    || (motion == Hash40::new("throw_b").hash && MotionModule::frame(fighter.module_accessor) < frame_max)
    {
        if !KineticModule::is_enable_energy(fighter. module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL) {
            sv_kinetic_energy!(
                reset_energy,
                fighter,
                FIGHTER_KINETIC_ENERGY_ID_CONTROL,
                ENERGY_CONTROLLER_RESET_TYPE_FREE,
                0.0,
                0.0,
                0.0,
                0.0,
                0.0
            );
            KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        }
        sv_kinetic_energy!(
            set_stable_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_max,
            0.0
        );
        sv_kinetic_energy!(
            set_limit_speed,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            speed_max,
            0.0
        );
        sv_kinetic_energy!(
            controller_set_accel_x_add,
            fighter,
            accel
        );
    }
    else if (motion == Hash40::new("throw_b").hash && MotionModule::frame(fighter.module_accessor) >= frame_max) {
        sv_kinetic_energy!(
            reset_energy,
            fighter,
            FIGHTER_KINETIC_ENERGY_ID_CONTROL,
            ENERGY_CONTROLLER_RESET_TYPE_FREE,
            0.0,
            0.0,
            0.0,
            0.0,
            0.0
        );
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    0.into()
}

pub fn install() {
    install_status_scripts!(
        throw_main,
        throw_exec,
    );
}