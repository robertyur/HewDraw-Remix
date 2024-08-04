use super::*;

// shorthand for referencing steve's different MATERIAL_KIND constants
const WOOD: i32 = 0x1;
const STONE: i32 = 0x2;
const IRON: i32 = 0x3;
const GOLD: i32 = 0x4;
const DIAMOND: i32 = 0x6;

unsafe extern "C" fn game_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let mut material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if is_excute(agent) {
        agent.off_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
        agent.set_int(*FIGHTER_PICKEL_CRAFT_WEAPON_KIND_SHOVEL, *FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_REQUEST_HAVE_CRAFT_WEAPON_KIND);
        ModelModule::set_joint_translate(boma, Hash40::new("weaponr"), &Vector3f::new(0.0, 0.0, 2.0), false, false);
    }
    frame(lua_state, 5.0);
    if is_excute(agent) {
        WorkModule::on_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    frame(lua_state, 11.0);
    FT_MOTION_RATE(agent, 0.7);
    material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
        let damage = match material_kind {
            ( WOOD | GOLD ) => 11.0,
            STONE => 12.0,
            IRON => 13.0,
            /* DIAMOND */ _ => 15.0 
        };
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("weaponr"), damage, 105, 88, 0, 60, 4.5, 0.0, 4.4, 0.0, None, None, None, 1.25, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
            agent.set_float(7.0, *FIGHTER_PICKEL_INSTANCE_WORK_ID_FLOAT_ATTACK_DURABILITY);
        }
    } else {
        // fist hitboxes
        if is_excute(agent) {
            ATTACK(agent, 0, 0, Hash40::new("handr"), 8.0, 80, 88, 0, 60, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.25, 2.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        AttackModule::clear_all(boma);
        FT_MOTION_RATE(agent, if material_kind == GOLD { 0.8 } else { 1.0 });
    }
    frame(lua_state, 32.0);
    if is_excute(agent) {
        MotionModule::set_rate(boma, 1.0);
        MotionModule::set_rate_partial(boma, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        agent.on_flag(*FIGHTER_PICKEL_INSTANCE_WORK_ID_FLAG_REQUEST_REMOVE_HAVE_CRAFT_WEAPON);
    }
}
 
unsafe extern "C" fn effect_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
        let effect: [&str;2] = match material_kind {
            WOOD => ["pickel_sword_flare_wood", "pickel_atk_pick_wood"],
            STONE => ["pickel_sword_flare_stone", "pickel_atk_pick_stone"],
            IRON => ["pickel_sword_flare_iron", "pickel_atk_pick_iron"],
            GOLD => ["pickel_sword_flare_gold", "pickel_atk_pick_gold"],
            /* DIAMOND */ _ => ["pickel_sword_flare_diamond", "pickel_atk_pick_diamond"]
        };
        frame(lua_state, 6.0);
        if is_excute(agent) {
            EFFECT_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("top"), -6, 1.5, 0.8, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        }
        frame(lua_state, 9.0);
        if is_excute(agent) {
            EFFECT_FLIP(agent, Hash40::new("pickel_block_break_soil"), Hash40::new("pickel_block_break_soil"), Hash40::new("top"), 0, -4, 2.0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
            LANDING_EFFECT(agent, Hash40::new("sys_windwave"), Hash40::new("top"), 0, 0, 4.5, 0, 0, 0, 1.4, 0, 0, 0, 0, 0, 0, true);
            LAST_EFFECT_SET_ALPHA(agent, 0.2);
        }
        frame(lua_state, 10.0);
        if is_excute(agent) {
            EFFECT_FLIP(agent, Hash40::new("pickel_block_break_soil"), Hash40::new("pickel_block_break_soil"), Hash40::new("top"), 0, -0.5, 8.0, 0, 0, 0, 1.2, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        }
        frame(lua_state, 11.0);
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new(effect[0]), Hash40::new(effect[0]), Hash40::new("weaponr"), 0.2, 2.25, 0, 0, 0, 0, 1, true, *EF_FLIP_YZ);
            EffectModule::set_scale_last(boma, &Vector3f::new(1.0, 0.61, 1.6));
        }
        frame(lua_state, 14.0);
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP(agent, Hash40::new(effect[1]), Hash40::new(effect[1]), Hash40::new("top"), 0, 9.7, 1.5, 0, 35, 100, 1, true, *EF_FLIP_YZ);
            LAST_EFFECT_SET_RATE(agent, if material_kind == GOLD { 0.95 } else { 0.50 });
        }
        frame(lua_state, 19.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new(effect[0]), false, true);
        }
        frame(lua_state, 20.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new(effect[1]), true, true);
        }
    } else {
    // fist effects
        frame(lua_state, 12.0);    
        if is_excute(agent) {
            EFFECT_FOLLOW_FLIP_ALPHA(agent, Hash40::new("sys_attack_arc_b"), Hash40::new("sys_attack_arc_b"), Hash40::new("top"), -2, 8.0, -1, 0, 15, 40, 0.8, true, *EF_FLIP_YZ, 0.06);
            LAST_EFFECT_SET_COLOR(agent, 1, 1, 1);
            LAST_EFFECT_SET_RATE(agent, 0.5);
        }
        frame(lua_state, 20.0);
        if is_excute(agent) {
            EFFECT_OFF_KIND(agent, Hash40::new("sys_attack_arc_b"), true, true);
        }
    }
}

unsafe extern "C" fn sound_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    let material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
    if material_kind != *FIGHTER_PICKEL_MATERIAL_KIND_NONE {
        frame(lua_state, 7.0);
        if is_excute(agent) {
            let handle = SoundModule::play_se(boma, Hash40::new("se_pickel_special_n01_soil2"), true, false, false, false, app::enSEType(0));
            SoundModule::set_se_vol(boma, handle as i32, 2.0, 0);
        }
        frame(lua_state, 10.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_common_swing_06"));
        }
        frame(lua_state, 11.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_pickel_special_n02_soil1"));
            PLAY_SE(agent, Hash40::new("se_pickel_special_n02_soil2"));
            PLAY_SE(agent, Hash40::new("se_pickel_special_n02_soil3"));
        }
    } else {
        frame(lua_state, 10.0);
        if is_excute(agent) {
            PLAY_SE(agent, Hash40::new("se_common_swing_06"));
        }
    }  
}

unsafe extern "C" fn expression_attackhi4(agent: &mut L2CAgentBase) {
    let lua_state = agent.lua_state_agent;
    let boma = agent.boma();
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, false, 0);
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 1.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE_INTP, *SLOPE_STATUS_NONE, 3);
    }
    frame(lua_state, 6.0);
    if WorkModule::is_flag(boma, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if is_excute(agent) {
            ItemModule::set_have_item_visibility(boma, false, 0);
        }
    }
    frame(lua_state, 10.0);
    if is_excute(agent) {
        ControlModule::set_rumble(boma, Hash40::new("rbkind_77_nohitm"), 7, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    frame(lua_state, 11.0);
    if is_excute(agent) {
        let material_kind = agent.get_int(*FIGHTER_PICKEL_INSTANCE_WORK_ID_INT_HAVE_CRAFT_WEAPON_MATERIAL_KIND);
        let (rumble, frames) = match material_kind {
            WOOD => ("rbkind_77_attacks", 0),
            STONE => ("rbkind_77_attacks", 0),
            IRON => ("rbkind_slashll", 9),
            GOLD => ("rbkind_slashl", 12),
            DIAMOND => ("rbkind_slash_critical", 22),
            /* NONE */ _ => ("rbkind_77_attackm", 0) 
        };
        RUMBLE_HIT(agent, Hash40::new(rumble), frames);
    }
    frame(lua_state, 19.0);
    if is_excute(agent) {
        slope!(agent, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    frame(lua_state, 43.0);
    if is_excute(agent) {
        ItemModule::set_have_item_visibility(boma, true, 0);
    }
}

pub fn install(agent: &mut Agent) {
    agent.acmd("game_attackhi4", game_attackhi4, Priority::Low);
    agent.acmd("effect_attackhi4", effect_attackhi4, Priority::Low);
    agent.acmd("sound_attackhi4", sound_attackhi4, Priority::Low);
    agent.acmd("expression_attackhi4", expression_attackhi4, Priority::Low);
}
