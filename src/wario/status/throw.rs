use crate::imports::imports_agent::*;

#[status_script(agent = "wario", status = FIGHTER_STATUS_KIND_THROW, condition = LUA_SCRIPT_STATUS_FUNC_EXEC_STATUS)]
unsafe fn throw_exec(fighter: &mut L2CFighterCommon) -> L2CValue {
    let motion = MotionModule::motion_kind(fighter.module_accessor);
    let speed_max = 1.0;
    let accel = 0.0005;
    let frame_min = 11.0;
    let frame_max = 41.0-12.0;
    let frame = MotionModule::frame(fighter.module_accessor);
    let within_frames = frame_min < frame && frame < frame_max;

    if motion == Hash40::new("throw_b_charge").hash
    || (motion == Hash40::new("throw_b").hash && within_frames)
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
        throw_exec,
    );
}