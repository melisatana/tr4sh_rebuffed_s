#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::app::sv_animcmd::*;

static mut OLIMAR_TAUNT_CAN_SPEEDUP : [bool; 8] = [false; 8];
static mut OLIMAR_TAUNT_SPEEDUP_LEVEL : [i32; 8] = [0; 8];
static mut OLIMAR_TAUNT_SPEEDUP_KEEP_FRAMES : [i32; 8] = [0; 8];

static mut OLIMAR_TAUNT_MAX_SPEEDUP_LEVEL : i32 = 20;

// A Once-Per-Fighter-Frame that only applies to Olimar
#[fighter_frame( agent = FIGHTER_KIND_PIKMIN )]
fn pikmin_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);
        //println!("fuck you");

        if OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] > 0 {
            if [*FIGHTER_STATUS_KIND_APPEAL, *FIGHTER_STATUS_KIND_ATTACK_S3].contains(&status) {
                OLIMAR_TAUNT_SPEEDUP_KEEP_FRAMES[entry_id] = 180;
                println!("[olimar] power level {}", OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id]);
            }
            else {
                OLIMAR_TAUNT_SPEEDUP_KEEP_FRAMES[entry_id] -= 1;
                if OLIMAR_TAUNT_SPEEDUP_KEEP_FRAMES[entry_id] < 1 {
                    println!("[olimar] no funny");
                    OLIMAR_TAUNT_CAN_SPEEDUP[entry_id] = false;
                    OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] = 0;
                }
            }
        }



    }
}

#[acmd_script( agent = "pikmin", script = "game_attack11", category = ACMD_GAME )]
unsafe fn pikmin_jab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 25, 20, 20, 3.4, 0.0, 5.0, 5.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 180, 25, 20, 20, 3.8, 0.0, 5.0, 8.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 361, 25, 20, 20, 3.8, 0.0, 5.0, 8.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 180, 15, 10, 25, 3.8, 0.0, 5.0, 10.8, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 361, 15, 10, 25, 3.8, 0.0, 5.0, 10.8, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attack12", category = ACMD_GAME )]
unsafe fn pikmin_jab2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 51, 80, 0, 40, 2.8, 0.0, 4.0, 6.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 51, 80, 0, 40, 3.5, 0.0, 4.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.75);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn pikmin_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 5.0, 24, 40, 30, 30, 5.8, 0.0, 7.7, 3.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("trans"), 6.0, 75, 50, 0, 80, 5.2, 0.0, 7.0, 3.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn pikmin_ftilt_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    let attack_power = 1.0 + ((OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] as f32) / (OLIMAR_TAUNT_MAX_SPEEDUP_LEVEL as f32));

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.2 * attack_power, 361, 97, 0, 55, 3.5, 0.0, 5.0, 13.5, Some(0.0), Some(5.0), Some(9.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 17.2 * attack_power, 361, 97, 0, 55, 2.2, 0.0, 5.0, 11.0, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        OLIMAR_TAUNT_CAN_SPEEDUP[entry_id] = false;
        OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] = 0;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn pikmin_utilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    for _ in 0..5 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 0.9, 120, 100, 50, 0, 3.5, 3.0, 4.0, 5.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 0.9, 120, 100, 50, 0, 3.5, -3.0, 4.0, -5.5, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 2, 0, Hash40::new("head"), 0.9, 92, 100, 20, 0, 5.0, 3.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("trans"), 4.6, 86, 94, 0, 57, 7.0, 0.0, 11.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 1, Hash40::new("trans"), 4.6, 86, 94, 0, 57, 6.0, 0.0, 17.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn pikmin_dtilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 8.4, 100, 97, 0, 35, 4.8, 4.0, 5.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("head"), 8.4, 100, 97, 0, 35, 4.0, 1.0, 2.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attacks4", category = ACMD_GAME )]
unsafe fn pikmin_fsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_SMASH_ATTACK_FLAG_SHOOT_PIKMIN);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.9, 361, 129, 0, 45, 2.7, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4sjump", category = ACMD_GAME )]
unsafe fn red_fsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 18.6, 361, 83, 0, 33, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -10, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 14.0, 361, 83, 0, 33, 3.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -7, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4sjump_b", category = ACMD_GAME )]
unsafe fn blue_fsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 14.5, 361, 90, 0, 33, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -4, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4sjump_y", category = ACMD_GAME )]
unsafe fn yellow_fsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 11.3, 55, 94, 0, 33, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4sjump_v", category = ACMD_GAME )]
unsafe fn purple_fsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: -0.9, y: 0.4, z: 0.0});
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 24.4, 361, 75, 0, 33, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -11.6, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 13.0, 361, 82, 0, 33, 3.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -8, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacks4sjump_w", category = ACMD_GAME )]
unsafe fn white_fsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 11.6, 361, 94, 0, 33, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn pikmin_usmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_SMASH_ATTACK_FLAG_SHOOT_PIKMIN);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.8, 82, 126, 0, 45, 2.7, 0.0, 3.0, 2.0, Some(0.0), Some(7.0), Some(2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
}   

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn red_usmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.2, 83, 80, 0, 50, 6.0, 0.0, 3.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.0, 94, 72, 0, 50, 4.0, 0.0, 3.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.4, 60, 74, 0, 50, 5.0, 0.0, 3.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4_b", category = ACMD_GAME )]
unsafe fn blue_usmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.7, 83, 91, 0, 50, 6.0, 0.0, 3.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.7, 83, 91, 0, 50, 4.0, 0.0, 3.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4_y", category = ACMD_GAME )]
unsafe fn yellow_usmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.1, 83, 77, 0, 50, 7.0, 0.0, 3.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.1, 83, 77, 0, 50, 5.0, 0.0, 3.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4_v", category = ACMD_GAME )]
unsafe fn purple_usmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 21.2, 28, 64, 0, 45, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.5, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 15.3, 28, 64, 0, 34, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackhi4_w", category = ACMD_GAME )]
unsafe fn white_usmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 83, 91, 0, 50, 6.0, 0.0, 3.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.0, 83, 91, 0, 50, 4.0, 0.0, 3.5, 9.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn pikmin_dsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_SMASH_ATTACK_FLAG_SHOOT_PIKMIN);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.8, 29, 120, 0, 45, 3.1, 0.0, 5.0, 7.0, Some(0.0), Some(5.0), Some(-7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn red_dsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 15.4, 28, 80, 0, 32, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 12.6, 28, 80, 0, 25, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4_b", category = ACMD_GAME )]
unsafe fn blue_dsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 12.2, 28, 89, 0, 32, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4_y", category = ACMD_GAME )]
unsafe fn yellow_dsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 10.1, 28, 89, 0, 32, 6.75, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4_v", category = ACMD_GAME )]
unsafe fn purple_dsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 18.6, 28, 64, 0, 32, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.6, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 15.0, 28, 64, 0, 25, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attacklw4_w", category = ACMD_GAME )]
unsafe fn white_dsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 8.8, 28, 89, 0, 32, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.2, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn pikmin_nair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 1.3, 367, 40, 0, 40, 4.2, 0.0, 3.0, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 1.3, 367, 40, 0, 30, 3.5, 0.0, 6.5, -4.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 2, 0, Hash40::new("trans"), 1.3, 367, 70, 0, 30, 3.5, 0.0, 6.5, 4.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
            macros::ATTACK(fighter, 3, 0, Hash40::new("trans"), 1.3, 367, 90, 0, 30, 4.2, 0.0, 9.5, 0.0, None, None, None, 0.5, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0); 
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 4.0, 57, 95, 0, 50, 4.5, 0.0, 3.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 4.0, 57, 95, 0, 50, 3.5, 0.0, 6.5, -4.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("trans"), 4.0, 57, 95, 0, 50, 3.5, 0.0, 6.5, 4.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 3, 0, Hash40::new("trans"), 4.0, 57, 95, 0, 50, 4.5, 0.0, 9.5, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn pikmin_fair_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.9, 50, 100, 0, 25, 2.8, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn red_fair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 12.6, 40, 103, 0, 32, 3.0, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf_b", category = ACMD_GAME )]
unsafe fn blue_fair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 10.6, 361, 109, 0, 32, 3.0, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf_y", category = ACMD_GAME )]
unsafe fn yellow_fair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 9.2, 55, 82, 0, 32, 4.5, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 5.0, false);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf_v", category = ACMD_GAME )]
unsafe fn purple_fair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 14.2, 280, 72, 0, 22, 3.0, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairf_w", category = ACMD_GAME )]
unsafe fn white_fair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 7.2, 51, 90, 0, 32, 3.0, 1.0, 3.5, 1.0, Some(0.0), Some(-1.6), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn pikmin_bair_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.9, 40, 100, 0, 25, 2.7, 0.0, 5.0, -2.0, Some(0.0), Some(5.0), Some(-6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn red_bair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 15.4, 40, 93, 0, 30, 3.0, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairb_b", category = ACMD_GAME )]
unsafe fn blue_bair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 12.6, 45, 104, 0, 30, 3.0, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairb_y", category = ACMD_GAME )]
unsafe fn yellow_bair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 10.8, 60, 77, 0, 30, 4.5, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairb_v", category = ACMD_GAME )]
unsafe fn purple_bair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 18.0, 47, 30, 0, 40, 3.0, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PIKMIN);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 6.0, false);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairb_w", category = ACMD_GAME )]
unsafe fn white_bair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 9.1, 20, 98, 0, 42, 3.0, 0.0, 3.5, 0.0, Some(0.0), Some(-3.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn pikmin_uair_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 82, 120, 0, 45, 3.1, 0.0, 4.0, 0.0, Some(0.0), Some(9.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn red_uair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 13.5, 95, 85, 0, 50, 3.3, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi_b", category = ACMD_GAME )]
unsafe fn blue_uair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 10.3, 95, 92, 0, 20, 3.3, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi_y", category = ACMD_GAME )]
unsafe fn yellow_uair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 9.0, 95, 82, 0, 50, 4.95, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi_v", category = ACMD_GAME )]
unsafe fn purple_uair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 16.6, 95, 73, 0, 50, 3.3, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairhi_w", category = ACMD_GAME )]
unsafe fn white_uair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 8.2, 270, 92, 0, 30, 3.3, 0.0, 3.5, 0.0, Some(0.0), Some(-1.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn pikmin_dair_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_SYNC);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 6.0, 260, 140, 0, 10, 3.1, 0.0, 0.0, 0.0, Some(0.0), Some(-4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_ATTACK_AIR_WORK_FLAG_DETACH);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn red_dair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 13.3, 270, 84, 0, 10, 3.5, 0.0, 3.0, 0.0, Some(0.0), Some(2.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        macros::ATTACK(fighter, 1, 0, Hash40::new("head1"), 13.3, 50, 84, 0, 30, 4.0, 0.0, 3.5, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairlw_b", category = ACMD_GAME )]
unsafe fn blue_dair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("head1"), 10.5, 270, 99, 0, 20, 4.0, 0.0, 3.5, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairlw_y", category = ACMD_GAME )]
unsafe fn yellow_dair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("head1"), 9.0, 65, 79, 0, 30, 6.0, 0.0, 3.5, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairlw_v", category = ACMD_GAME )]
unsafe fn purple_dair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 17.4, 270, 79, 0, 10, 3.5, 0.0, 2.5, 0.0, Some(0.0), Some(2.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        macros::ATTACK(fighter, 1, 0, Hash40::new("head1"), 17.4, 50, 79, 0, 30, 4.0, 0.0, 3.5, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_attackairlw_w", category = ACMD_GAME )]
unsafe fn white_dair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("head1"), 8.0, 270, 99, 0, 30, 4.0, 0.0, 3.5, 0.0, Some(0.0), Some(-2.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("waist"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin", script = "game_throwf", category = ACMD_GAME )]
unsafe fn pikmin_fthrow_smash_script(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_RED as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 1.6, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_YELLOW as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.8, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_BLUE as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.9, 361, 80, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_WHITE as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.8, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_VIOLET as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.8, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.3);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 12, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin", script = "game_throwb", category = ACMD_GAME )]
unsafe fn pikmin_bthrow_smash_script(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_RED as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.2, 45, 60, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_YELLOW as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_BLUE as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 13.3, 45, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_WHITE as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 45, 60, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_VIOLET as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 45, 100, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.3);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 10, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn pikmin_uthrow_smash_script(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_RED as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.4, 85, 70, 0, 61, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_YELLOW as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.5, 85, 119, 0, 81, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_BLUE as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.5, 85, 100, 0, 77, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_WHITE as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 85, 77, 0, 82, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_flower"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_VIOLET as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 85, 76, 0, 83, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 6, 2, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

#[acmd_script( agent = "pikmin", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn pikmin_dthrow_smash_script(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_RED as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.6, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_YELLOW as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_BLUE as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.9, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_WHITE as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        }
    }
    else {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PIKMIN_INSTANCE_WORK_INT_THROW_PIKMIN_VARIATION) == *WEAPON_PIKMIN_PIKMIN_VARIATION_VIOLET as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
            }
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.3);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 6, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwf", category = ACMD_GAME )]
unsafe fn red_fthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.6, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwf_b", category = ACMD_GAME )]
unsafe fn blue_fthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.9, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwf_y", category = ACMD_GAME )]
unsafe fn yellow_fthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwf_v", category = ACMD_GAME )]
unsafe fn purple_fthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwf_w", category = ACMD_GAME )]
unsafe fn white_fthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 361, 60, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwb", category = ACMD_GAME )]
unsafe fn red_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.2, 45, 60, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwb_b", category = ACMD_GAME )]
unsafe fn blue_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 15.3, 45, 60, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwb_y", category = ACMD_GAME )]
unsafe fn yellow_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 60, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwb_v", category = ACMD_GAME )]
unsafe fn purple_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 60, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwb_w", category = ACMD_GAME )]
unsafe fn white_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 45, 60, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn red_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.4, 85, 80, 0, 81, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 1.0, 90, 200, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwhi_b", category = ACMD_GAME )]
unsafe fn blue_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.5, 85, 66, 0, 83, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 1.0, 90, 200, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwhi_y", category = ACMD_GAME )]
unsafe fn yellow_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 85, 79, 0, 81, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 1.0, 90, 200, 0, 50, 6.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwhi_v", category = ACMD_GAME )]
unsafe fn purple_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 85, 76, 0, 83, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 1.0, 90, 200, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwhi_w", category = ACMD_GAME )]
unsafe fn white_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 85, 77, 0, 82, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head1"), 1.0, 90, 200, 0, 50, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn red_dthrow(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 90, 200, 0, 50, 7.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.6, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwlw_b", category = ACMD_GAME )]
unsafe fn blue_dthrow(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 90, 200, 0, 50, 7.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.9, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwlw_y", category = ACMD_GAME )]
unsafe fn yellow_dthrow(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 90, 200, 0, 50, 9.45, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwlw_v", category = ACMD_GAME )]
unsafe fn purple_dthrow(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 90, 200, 0, 50, 7.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_throwlw_w", category = ACMD_GAME )]
unsafe fn white_dthrow(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.8, 90, 200, 0, 50, 7.0, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_PIKMIN);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::ATTACK_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.0, 60, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *WEAPON_PIKMIN_PIKMIN_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_TASK), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}


#[acmd_script( agent = "pikmin_pikmin", script = "game_spsgrabattack", category = ACMD_GAME )]
unsafe fn red_sideb_latchattack(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        sv_animcmd::frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.6, 361, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PIKMIN);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_spsgrabattack_b", category = ACMD_GAME )]
unsafe fn blue_sideb_latchattack(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        sv_animcmd::frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.3, 361, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PIKMIN);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_spsgrabattack_y", category = ACMD_GAME )]
unsafe fn yellow_sideb_latchattack(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        sv_animcmd::frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.3, 361, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_HARISEN, *ATTACK_REGION_PIKMIN);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_spsgrabattack_w", category = ACMD_GAME )]
unsafe fn white_sideb_latchattack(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        sv_animcmd::frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 0, 0, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2.3, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PIKMIN);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 19.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        fighter.clear_lua_stack();
        wait_loop_sync_mot(fighter.lua_state_agent);
    }
}

#[acmd_script( agent = "pikmin_pikmin", script = "game_spsthrown_v", category = ACMD_GAME )]
unsafe fn purple_sideb_thrown(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 7.0, 361, 74, 0, 60, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -2.5, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_INSTANCE_WORK_ID_FLAG_TAKE_FROM_POCKET) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.4, 361, 80, 0, 40, 4.5, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -5.2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PIKMIN);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_PIKMIN_PIKMIN_STATUS_SPECIAL_S_WORK_FLAG_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 100.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pikmin", script = "game_speciallw", category = ACMD_GAME )]
unsafe fn pikmin_downb_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_SPECIAL_LW_FLAG_SORT);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

#[acmd_script( agent = "pikmin", script = "game_specialairlw", category = ACMD_GAME )]
unsafe fn pikmin_downb_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKMIN_STATUS_SPECIAL_LW_FLAG_SORT);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

#[acmd_script( agent = "pikmin", script = "game_appeallwl", category = ACMD_GAME )]
unsafe fn pikmin_downtaunt_left_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        if OLIMAR_TAUNT_CAN_SPEEDUP[entry_id] {
            OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] += 1;
            if OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] > OLIMAR_TAUNT_MAX_SPEEDUP_LEVEL {
                OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] = OLIMAR_TAUNT_MAX_SPEEDUP_LEVEL;
            }
            MotionModule::set_rate(fighter.module_accessor, 1.0 + ((OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] as f32) / (OLIMAR_TAUNT_MAX_SPEEDUP_LEVEL as f32)));
        }
        OLIMAR_TAUNT_CAN_SPEEDUP[entry_id] = true;
        println!("[olimar] taunt");
    }
    sv_animcmd::frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        OLIMAR_TAUNT_CAN_SPEEDUP[entry_id] = false;
        println!("[olimar] end taunt :(");
    }
}

#[acmd_script( agent = "pikmin", script = "game_appeallwr", category = ACMD_GAME )]
unsafe fn pikmin_downtaunt_right_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        if OLIMAR_TAUNT_CAN_SPEEDUP[entry_id] {
            OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] += 1;
            if OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] > OLIMAR_TAUNT_MAX_SPEEDUP_LEVEL {
                OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] = OLIMAR_TAUNT_MAX_SPEEDUP_LEVEL;
            }
            MotionModule::set_rate(fighter.module_accessor, 1.0 + ((OLIMAR_TAUNT_SPEEDUP_LEVEL[entry_id] as f32) / (OLIMAR_TAUNT_MAX_SPEEDUP_LEVEL as f32)));
        }
        OLIMAR_TAUNT_CAN_SPEEDUP[entry_id] = true;
        println!("[olimar] taunt");
    }
    sv_animcmd::frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        OLIMAR_TAUNT_CAN_SPEEDUP[entry_id] = false;
        println!("[olimar] end taunt :(");
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        pikmin_frame
    );
    smashline::install_acmd_scripts!(
        pikmin_jab_smash_script,
        pikmin_jab2_smash_script,
        pikmin_dashattack_smash_script,
        pikmin_ftilt_smash_script,
        pikmin_utilt_smash_script,
        pikmin_dtilt_smash_script,
        pikmin_fsmash_smash_script,
        red_fsmash,
        blue_fsmash,
        yellow_fsmash,
        purple_fsmash,
        white_fsmash,
        pikmin_usmash_smash_script,
        red_usmash,
        blue_usmash,
        yellow_usmash,
        purple_usmash,
        white_usmash,
        pikmin_dsmash_smash_script,
        red_dsmash,
        blue_dsmash,
        yellow_dsmash,
        purple_dsmash,
        white_dsmash,
        pikmin_nair_smash_script,
        pikmin_fair_smash_script,
        red_fair,
        blue_fair,
        yellow_fair,
        purple_fair,
        white_fair,
        pikmin_bair_smash_script,
        red_bair,
        blue_bair,
        yellow_bair,
        purple_bair,
        white_bair,
        pikmin_uair_smash_script,
        red_uair,
        blue_uair,
        yellow_uair,
        purple_uair,
        white_uair,
        pikmin_dair_smash_script,
        red_dair,
        blue_dair,
        yellow_dair,
        purple_dair,
        white_dair,
        pikmin_fthrow_smash_script,
        pikmin_bthrow_smash_script,
        pikmin_uthrow_smash_script,
        pikmin_dthrow_smash_script,
        red_fthrow,
        blue_fthrow,
        yellow_fthrow,
        purple_fthrow,
        white_fthrow,
        red_bthrow,
        blue_bthrow,
        yellow_bthrow,
        purple_bthrow,
        white_bthrow,
        red_uthrow,
        blue_uthrow,
        yellow_uthrow,
        purple_uthrow,
        white_uthrow,
        red_dthrow,
        blue_dthrow,
        yellow_dthrow,
        purple_dthrow,
        white_dthrow,
        red_sideb_latchattack,
        blue_sideb_latchattack,
        yellow_sideb_latchattack,
        white_sideb_latchattack,
        purple_sideb_thrown,
        pikmin_downb_smash_script,
        pikmin_downb_air_smash_script,
        pikmin_downtaunt_left_smash_script,
        pikmin_downtaunt_right_smash_script

    );
}
