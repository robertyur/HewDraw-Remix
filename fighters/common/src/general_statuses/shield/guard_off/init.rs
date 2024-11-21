// status imports
use super::*;
use globals::*;

#[skyline::hook(replace = L2CFighterCommon_sub_ftStatusUniqProcessGuardOff_initStatus)]
unsafe fn ftStatusUniqProcessGuardOff_initStatus(fighter: &mut L2CFighterCommon) -> L2CValue {
    if VarModule::is_flag(fighter.object(), vars::common::instance::IS_PARRY_FOR_GUARD_OFF) {
        ShieldModule::set_status(fighter.module_accessor, *FIGHTER_SHIELD_KIND_GUARD, app::ShieldStatus(*SHIELD_STATUS_NORMAL), 0);
        ShieldModule::set_shield_type(fighter.module_accessor, app::ShieldType(*SHIELD_TYPE_JUST_SHIELD), *FIGHTER_SHIELD_KIND_GUARD, 0);
        ReflectorModule::set_status(fighter.module_accessor, 0, app::ShieldStatus(*SHIELD_STATUS_NORMAL), *FIGHTER_REFLECTOR_GROUP_JUST_SHIELD);
        super::super::fighter_status_guard::set_just_shield_scale(fighter);
        let hit_stop_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), 0x20d241cd64);
        ShieldModule::set_hit_stop_mul(fighter.module_accessor, hit_stop_mul);

        let lifetime = (fighter.get_command_life(CatHdr::Parry) as i32);
        let buffer = (ControlModule::get_command_life_count_max(fighter.module_accessor) as i32);
        let shield_just_frame = (fighter.get_param_int("common", "shield_just_frame"));
        let just_frame = ((shield_just_frame + lifetime + 1 - buffer).clamp(2, shield_just_frame));
        fighter.set_int(just_frame, *FIGHTER_STATUS_GUARD_ON_WORK_INT_JUST_FRAME);
    }
    ControlModule::clear_command(fighter.module_accessor, true);
    L2CValue::I32(0)
}

pub fn install() {
    skyline::install_hook!(ftStatusUniqProcessGuardOff_initStatus);
}
