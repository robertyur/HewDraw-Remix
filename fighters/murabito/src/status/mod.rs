use super::*;
use globals::*;
// status script import

mod attack_air;
mod jump;
mod special_hi;
mod special_lw;
mod special_s;
 
// Prevents sideB from being used again if it has already been used once in the current airtime
unsafe extern "C" fn should_use_special_s_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_AIR) && VarModule::is_flag(fighter.battle_object, vars::murabito::instance::DISABLE_SPECIAL_S) {
        false.into()
    } else {
        true.into()
    }
}

// Re-enables the ability to use sideB when connecting to ground or cliff
unsafe extern "C" fn change_status_callback(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.is_situation(*SITUATION_KIND_GROUND) || fighter.is_situation(*SITUATION_KIND_CLIFF)
    || fighter.is_status_one_of(&[*FIGHTER_STATUS_KIND_REBIRTH, *FIGHTER_STATUS_KIND_DEAD, *FIGHTER_STATUS_KIND_LANDING]) {
        VarModule::off_flag(fighter.battle_object, vars::murabito::instance::DISABLE_SPECIAL_S);
    }
    true.into()
}

pub unsafe extern "C" fn special_s_shoot_clayrocket(fighter: &mut L2CFighterCommon, status: L2CValue) {
    WorkModule::set_int(fighter.module_accessor, status.get_i32(), *FIGHTER_MURABITO_STATUS_SPECIAL_S_INT_SHOOT_STATUS);

    ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MURABITO_GENERATE_ARTICLE_CLAYROCKET, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
}

pub unsafe extern "C" fn special_s_end_common(fighter: &mut L2CFighterCommon) {
    let status = fighter.global_table[STATUS_KIND].get_i32();
    if ![
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE,
        *FIGHTER_MURABITO_STATUS_KIND_SPECIAL_S_RIDE_LOOP
    ].contains(&status)
    && LinkModule::is_link(fighter.module_accessor, *FIGHTER_MURABITO_LINK_NO_CLAYROCKET) {
        LinkModule::unlink(fighter.module_accessor, *FIGHTER_MURABITO_LINK_NO_CLAYROCKET);
    }
}

unsafe extern "C" fn on_start(fighter: &mut L2CFighterCommon) {
    // set the callbacks on fighter init
    fighter.global_table[globals::USE_SPECIAL_S_CALLBACK].assign(&L2CValue::Ptr(should_use_special_s_callback as *const () as _));
    fighter.global_table[globals::STATUS_CHANGE_CALLBACK].assign(&L2CValue::Ptr(change_status_callback as *const () as _));   
}

pub fn install(agent: &mut Agent) {
    agent.on_start(on_start);

    attack_air::install(agent);
    jump::install(agent);
    special_hi::install(agent);
    special_lw::install(agent);
    special_s::install(agent);
}