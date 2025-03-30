use super::*;

unsafe extern "C" fn fly_main_pre(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if GroundModule::is_wall_touch_line(weapon.module_accessor, *GROUND_TOUCH_FLAG_ALL as u32) {
        if LinkModule::is_link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
            LinkModule::send_event_parents(
                weapon.module_accessor,
                *WEAPON_LINK_NO_CONSTRAINT,
                Hash40::new_raw(0x1f6c5febaa)
            );
            let x = WorkModule::get_float(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_LODGED_POS_X);
            let y = WorkModule::get_float(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_LODGED_POS_Y);
            let z = PostureModule::pos_z(weapon.module_accessor);
            PostureModule::set_pos(weapon.module_accessor, &Vector3f{x, y, z});
            weapon.change_status(WEAPON_MURABITO_CLAYROCKET_STATUS_KIND_BURST.into(), false.into());
            return true.into();
        }
    }
    false.into()
}

unsafe extern "C" fn set_end_frame(weapon: &mut L2CWeaponCommon, frame: L2CValue) {
    let mut frame = frame.get_f32();

    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_RIDE) {
        let mul = WorkModule::get_param_float(weapon.module_accessor, hash40("param_clayrocket"), hash40("ride_frame_mul"));
        frame *= mul;
    }

    WorkModule::set_int(weapon.module_accessor, frame as i32, *WEAPON_MURABITO_CLAYROCKET_STATUS_WORK_INT_END_FRAME);
}

unsafe extern "C" fn check_notify_panic(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let notify = WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_NOTIFY_PANIC);

    if notify {
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_PANIC);
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_NOTIFY_PANIC);
    }

    notify.into()
}

unsafe extern "C" fn fly_main(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if fly_main_pre(weapon).get_bool() {
        return 0.into();
    }

    let fall_start_frame = WorkModule::get_param_float(weapon.module_accessor, hash40("param_clayrocket"), hash40("fall_start_frame"));
    set_end_frame(weapon, fall_start_frame.into());

    check_notify_panic(weapon);

    WorkModule::set_int(weapon.module_accessor, -1, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_INT_CONTROL_SPEED);

    let motion = if WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_PANIC) {
        Hash40::new("fly_panic")
    }
    else {
        Hash40::new("fly")
    };
    MotionModule::change_motion(
        weapon.module_accessor,
        motion,
        0.0,
        1.0,
        false,
        0.0,
        false,
        false
    );

    SoundModule::play_se(
        weapon.module_accessor,
        Hash40::new("se_murabito_special_s01"),
        true,
        false,
        false,
        false,
        enSEType(0)
    );

    if weapon.global_table[globals::PREV_STATUS_KIND].get_i32() == *WEAPON_MURABITO_CLAYROCKET_STATUS_KIND_RIDE {
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_RIDE_BURST);
    }

    if !StopModule::is_stop(weapon.module_accessor) {
        if false {
            WorkModule::inc_int(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_STATUS_WORK_INT_FRAME);
        }
    }
    weapon.global_table[globals::SUB_STATUS].assign(&L2CValue::Ptr(fly_substatus as *const () as _));

    weapon.fastshift(L2CValue::Ptr(fly_fastshift as *const () as _))
}

unsafe extern "C" fn fly_substatus(weapon: &mut L2CWeaponCommon, param_1: L2CValue) -> L2CValue {
    if param_1.get_bool() {
        WorkModule::inc_int(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_STATUS_WORK_INT_FRAME);
    }

    0.into()
}

unsafe extern "C" fn check_burst(weapon: &mut L2CWeaponCommon) -> L2CValue {
    let hp = WorkModule::get_float(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_HP);
    if hp <= 0.0
    || WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_BURST) {
        if LinkModule::is_link(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT) {
            if WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_RIDE_REFLECTED) {
                LinkModule::send_event_parents(weapon.module_accessor, *WEAPON_LINK_NO_CONSTRAINT, Hash40::new_raw(0x2bd53d128c));
            }
        }
        weapon.change_status(WEAPON_MURABITO_CLAYROCKET_STATUS_KIND_BURST.into(), false.into());
        return true.into();
    }

    false.into()
}

unsafe extern "C" fn set_se_pitch(weapon: &mut L2CWeaponCommon, param_1: L2CValue) {
    if weapon.global_table[globals::STATUS_KIND].get_i32() == *WEAPON_MURABITO_CLAYROCKET_STATUS_KIND_FLY {
        weapon.clear_lua_stack();
        lua_args!(weapon, WEAPON_KINETIC_ENERGY_RESERVE_ID_NORMAL);
        // let speed = sv_kinetic_energy::get_speed_x(weapon.lua_state_agent).abs().min(2.3);
        // This entire status rewrite exists because of this change and one other below.
        let speed = sv_kinetic_energy::get_speed_y(weapon.lua_state_agent).abs().min(2.3);
        let min = speed - 1.65;
        let max = 2.3 - speed;
        let ratio = min / max;
        let pitch = (800.0 * ratio).max(0.0);
        if 0 < param_1.get_u32() {
            SoundModule::stop_se(weapon.module_accessor, Hash40::new("se_murabito_special_s01"), param_1.get_u32());
        }
        else {
            SoundModule::set_se_pitch_cent(weapon.module_accessor, Hash40::new("se_murabito_special_s01"), pitch);
        }
    }
}

unsafe extern "C" fn set_direction(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if weapon.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
        if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32) {
            let lr = PostureModule::lr(weapon.module_accessor);
            let normal_x = GroundModule::get_touch_normal_x(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let normal_y = GroundModule::get_touch_normal_y(weapon.module_accessor, *GROUND_TOUCH_FLAG_DOWN as u32);
            let x = normal_x * lr;
            let y = normal_y * -1.0 * lr;
            WorkModule::set_float(weapon.module_accessor, x, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_DIRECTION_X);
            WorkModule::set_float(weapon.module_accessor, y, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_DIRECTION_Y);
            return 1.into();
        }
    }
    else {
        if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_LOCK_DIRECTION) {
            let speed_x = KineticModule::get_sum_speed_x(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            let speed_y = KineticModule::get_sum_speed_y(weapon.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

            let length = sv_math::vec3_length(speed_x, speed_y, 0.0);
            if 0.01 < length {
                WorkModule::set_float(weapon.module_accessor, speed_x / length, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_DIRECTION_X);
                WorkModule::set_float(weapon.module_accessor, speed_y / length, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_DIRECTION_Y);
                return 1.into();
            }
        }
    }
    false.into()
}

unsafe extern "C" fn check_situation_changed(weapon: &mut L2CWeaponCommon, is_changing: L2CValue) -> L2CValue {
    if is_changing.get_bool()
    || StatusModule::is_situation_changed(weapon.module_accessor) {
        if weapon.global_table[globals::SITUATION_KIND].get_i32() == *SITUATION_KIND_GROUND {
            weapon.set_situation(SITUATION_KIND_GROUND.into());
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            WorkModule::on_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_GROUND);
        }
        else {
            weapon.set_situation(SITUATION_KIND_AIR.into());
            GroundModule::correct(weapon.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
            WorkModule::off_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_GROUND);
        }
    }
    false.into()
}

unsafe extern "C" fn fly_fastshift(weapon: &mut L2CWeaponCommon) -> L2CValue {
    if !StopModule::is_stop(weapon.module_accessor) {
        if check_burst(weapon).get_bool() {
            return 1.into();
        }
    }

    let is_changing = StatusModule::is_changing(weapon.module_accessor);

    let mut set_kinetic = is_changing;

    if is_changing
    || weapon.global_table[globals::IS_STOPPING].get_bool() {
        return 0.into();
    }

    if WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_DETACH) {
        WorkModule::on_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_GET_OFF);
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_RIDE);
        WorkModule::off_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_DETACH);
        let fall_start_frame = WorkModule::get_param_float(weapon.module_accessor, hash40("param_clayrocket"), hash40("fall_start_frame"));
        set_end_frame(weapon, fall_start_frame.into());
        set_kinetic = true;
    }

    let frame = WorkModule::get_int(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_STATUS_WORK_INT_FRAME);
    let end_frame = WorkModule::get_int(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_STATUS_WORK_INT_END_FRAME);
    if end_frame < frame {
        set_se_pitch(weapon, 0x3c_u32.into());
        weapon.change_status(WEAPON_MURABITO_CLAYROCKET_STATUS_KIND_FALL.into(), false.into());
        return 1.into();
    }

    if 2 < frame {
        let fall = WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_STATUS_WORK_FLAG_FALL);
        set_direction(weapon);
        let x = WorkModule::get_float(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_DIRECTION_X);
        let y = WorkModule::get_float(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLOAT_DIRECTION_Y);
        let touch_flag = GroundModule::get_touch_flag(weapon.module_accessor);
        if touch_flag != 0 {
            // you're fucking kidding me
            // do not look inside this function
            if let Some(target) = smashline::api::get_target_function("lua2cpp_murabito.nrs", 0x2f7d0) {
                let hell_incarnate: fn(&L2CValue, &mut L2CWeaponCommon, L2CValue, L2CValue, L2CValue) = std::mem::transmute(target);
                let ret = &L2CValue::Bool(false);
                hell_incarnate(ret, weapon, touch_flag.into(), x.into(), y.into());
                if ret.get_bool() {
                    return 1.into();
                }
            }

            if fall || touch_flag & *GROUND_TOUCH_FLAG_UP as u64 != 0 {
                notify_event_msc_cmd!(weapon, Hash40::new_raw(0x3c13da0448), -0.5);
                WorkModule::off_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_STATUS_WORK_FLAG_FALL);
            }
        }

        if let Some(target) = smashline::api::get_target_function("lua2cpp_murabito.nrs", 0x30620) {
            let func: fn(&mut L2CWeaponCommon, L2CValue, L2CValue) = std::mem::transmute(target);
            func(weapon, x.into(), y.into());
        }
    }

    if check_situation_changed(weapon, is_changing.into()).get_bool() {
        set_kinetic = true;
    }

    set_se_pitch(weapon, 0.into());

    let control_speed = if !WorkModule::is_flag(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_FLAG_RIDE) {
        *WEAPON_MURABITO_CLAYROCKET_CONTROL_SPEED_NORMAL
    }
    else {
        // this was the other change
        let stick_y = ControlModule::get_stick_y(weapon.module_accessor);
        if stick_y > 0.2 {
            *WEAPON_MURABITO_CLAYROCKET_CONTROL_SPEED_POSITIVE
        }
        else if stick_y < -0.2 {
            *WEAPON_MURABITO_CLAYROCKET_CONTROL_SPEED_NEGATIVE
        }
        else {
            *WEAPON_MURABITO_CLAYROCKET_CONTROL_SPEED_NORMAL
        }
    };

    let control_speed_prev = WorkModule::get_int(weapon.module_accessor, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_INT_CONTROL_SPEED);
    if control_speed != control_speed_prev {
        WorkModule::set_int(weapon.module_accessor, control_speed, *WEAPON_MURABITO_CLAYROCKET_INSTANCE_WORK_ID_INT_CONTROL_SPEED);
        set_kinetic = true;
    }

    if set_kinetic {
        KineticModule::change_kinetic(weapon.module_accessor, *WEAPON_KINETIC_TYPE_MURABITO_CLAYROCKET_FLY);
    }

    if check_notify_panic(weapon).get_bool() {
        MotionModule::change_motion(
            weapon.module_accessor,
            Hash40::new("fly_panic"),
            0.0,
            1.0,
            false,
            0.0,
            false,
            false
        );
    }

    0.into()
}

pub fn install(agent: &mut Agent) {
    agent.status(Main, *WEAPON_MURABITO_CLAYROCKET_STATUS_KIND_FLY, fly_main);
}
