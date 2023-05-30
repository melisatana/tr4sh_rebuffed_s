#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;

//static mut TOONLINK_UPAIR_CANHOLD : [bool; 8] = [false; 8];
static mut TOONLINK_UPAIR_ISHOLD : [bool; 8] = [false; 8];

static mut TOONLINK_UPB_GROUNDTOAIR : [bool; 8] = [false; 8];

// A Once-Per-Fighter-Frame that only applies to Toon Link
#[fighter_frame( agent = FIGHTER_KIND_TOONLINK )]
fn toonlink_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);

        println!("It'sa me, Link, hyaaaa!");

        if TOONLINK_UPAIR_ISHOLD[entry_id] {
            if status == *FIGHTER_STATUS_KIND_ATTACK_AIR {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) == false {
                    TOONLINK_UPAIR_ISHOLD[entry_id] = false;
                    MotionModule::set_rate(fighter.module_accessor, 2.0);
                }
            }
            else {
                TOONLINK_UPAIR_ISHOLD[entry_id] = false;
            }
        }
    }
}

#[acmd_script( agent = "toonlink", script = "game_attack11", category = ACMD_GAME )]
unsafe fn toonlink_jab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.5, 361, 25, 16, 25, 3.8, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 25, 16, 25, 3.8, 0.0, 6.0, 10.0, Some(0.0), Some(6.0), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 180, 15, 12, 20, 3.8, 0.0, 6.0, 14.0, Some(0.0), Some(6.0), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.5, 361, 15, 12, 20, 3.8, 0.0, 6.0, 14.0, Some(0.0), Some(6.0), Some(6.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 2.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attack12", category = ACMD_GAME )]
unsafe fn toonlink_jab2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 361, 25, 16, 25, 3.5, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.8, 361, 25, 16, 25, 4.0, 0.0, 6.0, 11.0, Some(0.0), Some(6.0), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.8, 180, 15, 12, 20, 4.0, 0.0, 6.0, 14.5, Some(0.0), Some(6.0), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.8, 361, 15, 12, 20, 4.0, 0.0, 6.0, 14.5, Some(0.0), Some(6.0), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 2.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attack13", category = ACMD_GAME )]
unsafe fn toonlink_jab3_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.6, 121, 70, 0, 65, 3.7, 0.0, 6.0, 8.0, Some(0.0), Some(6.0), Some(8.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.6, 121, 70, 0, 65, 4.7, 0.0, 6.0, 16.5, Some(0.0), Some(6.0), Some(8.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn toonlink_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4285714);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.909090909);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 8.7, 65, 95, 0, 40, 4.2, 5.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.7, 65, 95, 0, 40, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.7, 65, 95, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 3, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.62);
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn toonlink_ftilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 9.4, 38, 75, 0, 45, 4.3, 5.0, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 9.4, 20, 75, 0, 45, 4.0, 1.0, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 9.4, 20, 75, 0, 45, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 9.4, 72, 75, 0, 45, 4.3, 5.0, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 9.4, 72, 75, 0, 45, 4.0, 1.0, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn toonlink_utilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 7.2, 95, 85, 0, 34, 3.8, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 7.2, 85, 85, 0, 34, 4.4, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 7.2, 105, 85, 0, 34, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn toonlink_dtilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 6.9, 361, 63, 0, 20, 3.6, 6.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 6.9, 361, 63, 0, 20, 4.2, 1.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 6.9, 361, 63, 0, 20, 4.2, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword2"), 6.9, 270, 98, 0, 15, 3.6, 6.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("sword2"), 6.9, 270, 98, 0, 15, 4.2, 1.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 5, 0, Hash40::new("arml"), 6.9, 270, 98, 0, 15, 4.2, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attacks4", category = ACMD_GAME )]
unsafe fn toonlink_fsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 3.2, y: 0.0, z: 0.0});
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SEQUENCE(fighter, Hash40::new("seq_toonlink_rnd_attack"));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.05, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 14.9, 40, 110, 0, 42, 3.9, 6.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 14.9, 40, 110, 0, 42, 3.4, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 14.9, 40, 110, 0, 42, 3.4, 1.2, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("colonellm"), 14.9, 40, 110, 0, 42, 3.9, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn toonlink_usmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.6, 100, 105, 0, 43, 6.0, 0.0, 6.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 13.6, 96, 105, 0, 43, 4.1, 5.5, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 13.6, 96, 105, 0, 43, 3.3, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 13.6, 96, 105, 0, 30, 4.1, 2.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn toonlink_dsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        macros::SET_SPEED_EX(fighter, 0.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.4, 368, 60, 0, 0, 4.0, 0.0, 2.2, 11.0, Some(0.0), Some(2.9), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.4, 368, 60, 0, 0, 4.0, 0.0, 2.2, 16.0, Some(0.0), Some(2.9), Some(8.0), 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        //AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &smash::phx::Vector2f{x: -24.0, y:13.0}, 14, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &smash::phx::Vector2f{x: -22.0, y:10.0}, 12, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.9);
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.6, 38, 122, 0, 55, 5.8, 0.0, 4.2, -16.0, Some(0.0), Some(4.2), Some(-5.6), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn toonlink_nair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 9.5, 100, 100, 0, 20, 4.0, 4.3, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 9.5, 100, 100, 0, 20, 4.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 9.5, 100, 100, 0, 20, 4.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("colonellm"), 9.5, 100, 100, 0, 20, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 3.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 12.4, 361, 109, 0, 40, 4.0, 4.3, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 12.4, 361, 109, 0, 40, 4.5, 0.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 12.4, 361, 109, 0, 40, 4.5, -0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("colonellm"), 12.4, 361, 109, 0, 40, 2.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn toonlink_fair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 13.5, 361, 112, 0, 35, 5.0, 4.0, -2.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 13.5, 361, 112, 0, 35, 5.0, 1.0, -2.5, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 13.5, 361, 112, 0, 35, 4.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn toonlink_bair_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 8.6, 51, 100, 0, 24, 4.0, 6.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 8.6, 51, 100, 0, 24, 5.0, 2.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 8.6, 51, 100, 0, 24, 4.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn toonlink_uair_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 15.2, 80, 101, 0, 25, 4.6, 5.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 24, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 15.2, 80, 101, 0, 25, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 24, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("arml"), 15.2, 80, 101, 0, 25, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 24, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATK_POWER(fighter, 0, 12.5);
        macros::ATK_POWER(fighter, 1, 12.5);
        macros::ATK_POWER(fighter, 2, 12.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            MotionModule::set_rate(fighter.module_accessor, 0.1);
            TOONLINK_UPAIR_ISHOLD[entry_id] = true;
        }
        else {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
        MotionModule::set_rate(fighter.module_accessor, 1.7);
        TOONLINK_UPAIR_ISHOLD[entry_id] = false;
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn toonlink_dair_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, 1.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
    //get_value_int!(*SO_VAR_INT_PREV_STATUS);
    //0x120e80(0, *FIGHTER_STATUS_KIND_DAMAGE_FLY);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
    }
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        JostleModule::set_status(fighter.module_accessor, true);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0, -3.8, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 268, 80, 0, 40, 5.5, 1.0, -1.1, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 65.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_LW_SET_ATTACK);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 71.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "toonlink", script = "game_attackairlw2bound", category = ACMD_GAME )]
unsafe fn toonlink_dair_bounce_smash_script(fighter: &mut L2CAgentBase) {
    let sticky = ControlModule::get_stick_y(fighter.module_accessor);

    if macros::is_excute(fighter) {
        if sticky <= -0.5 {
            macros::SET_SPEED_EX(fighter, 0, -3, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        else {
            macros::SET_SPEED_EX(fighter, 0, 1.6, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        KineticModule::resume_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    }
}

#[acmd_script( agent = "toonlink", script = "game_catch", category = ACMD_GAME )]
unsafe fn toonlink_grab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, smash::app::ArticleOperationTarget(0));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, 0.0);
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 55.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, smash::app::ArticleOperationTarget(0));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, smash::app::ArticleOperationTarget(0));
    }
}

#[acmd_script( agent = "toonlink", script = "game_catchdash", category = ACMD_GAME )]
unsafe fn toonlink_grabd_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, smash::app::ArticleOperationTarget(0));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, 0.0);
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 63.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, smash::app::ArticleOperationTarget(0));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, smash::app::ArticleOperationTarget(0));
    }
}

#[acmd_script( agent = "toonlink", script = "game_catchturn", category = ACMD_GAME )]
unsafe fn toonlink_grabp_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, smash::app::ArticleOperationTarget(0));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, 0.0);
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.5, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("handr"), 4.0, 0.0, 0.0, 0.0, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 58.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, smash::app::ArticleOperationTarget(0));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, smash::app::ArticleOperationTarget(0));
    }
}

#[acmd_script( agent = "toonlink", script = "game_aircatch", category = ACMD_GAME )]
unsafe fn toonlink_zair_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, false, 0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 6.5, 38, 50, 0, 64, 3.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_SHOOT, smash::app::ArticleOperationTarget(0));
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("shoot"), false, 0.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, *WEAPON_TOONLINK_HOOKSHOT_STATUS_KIND_REWIND);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, Hash40::new("back"), false, 0.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_INSTANCE_WORK_ID_FLAG_OFF_MAP_COLL_OFFSET);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 53.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 68.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT, smash::app::ArticleOperationTarget(0));
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_HOOKSHOT_HAND, smash::app::ArticleOperationTarget(0));
    }
}

#[acmd_script( agent = "toonlink", script = "game_throwf", category = ACMD_GAME )]
unsafe fn toonlink_fthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 20, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 45, 180, 0, 30, 5.0, 0.0, 7.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 8, 1);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.3);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 2, 2, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "toonlink", script = "game_throwb", category = ACMD_GAME )]
unsafe fn toonlink_bthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.7, 130, 134, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, -16, 1);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, -10, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "toonlink", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn toonlink_uthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 2.0, 92, 100, 0, 35, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("sword2"), 3.0, 20, 130, 0, 50, 3.84, 6.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 3.0, 20, 130, 0, 50, 3.84, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 0, 20);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.7);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 0, 7, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "toonlink", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn toonlink_dthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.1, 120, 80, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 361, 150, 0, 60, 5.0, 0.0, 2.4, 7.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, -6, 4);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 0, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "toonlink_bowarrow", script = "game_fly", category = ACMD_GAME )]
unsafe fn arrow_projectile(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 50, 0, 8, 2.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

#[acmd_script( agent = "toonlink_bowarrow", script = "game_stick", category = ACMD_GAME )]
unsafe fn arrow_stick_projectile(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 361, 50, 0, 8, 3.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_2 as u8);
    }
}    

#[acmd_script( agent = "toonlink", script = "game_specials1", category = ACMD_GAME )]
unsafe fn toonlink_sideb_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.8333333);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

#[acmd_script( agent = "toonlink", script = "game_specialairs1", category = ACMD_GAME )]
unsafe fn toonlink_sideb_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.8333333);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
}

#[acmd_script( agent = "toonlink", script = "game_specials2", category = ACMD_GAME )]
unsafe fn toonlink_sideb_catch_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "toonlink", script = "game_specialairs2", category = ACMD_GAME )]
unsafe fn toonlink_sideb_catch_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_TOONLINK_GENERATE_ARTICLE_BOOMERANG, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
    }
}

#[acmd_script( agent = "toonlink_boomerang", script = "game_fly", category = ACMD_GAME )]
unsafe fn boomerang_thrown(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.8, 74, 50, 0, 50, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 74, 50, 0, 50, 2.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "toonlink_boomerang", script = "game_turn", category = ACMD_GAME )]
unsafe fn boomerang_returning(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.1, 280, 100, 0, 20, 2.7, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 3, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

#[acmd_script( agent = "toonlink", script = "game_specialhi", category = ACMD_GAME )]
unsafe fn toonlink_upb_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        TOONLINK_UPB_GROUNDTOAIR[entry_id] = true;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 2.0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 6.8, 0, 5.0, 7.5, 0, 5.0, -6.4, 1.4, 1.25, 50, false, 1.5, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        macros::ATTACK(fighter, 5, 0, Hash40::new("sword2"), 1.5, 180, 100, 45, 10, 3.0, -1.5, 0.0, -1.0, None, None, None, 0.3, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("sword2"), 1.5, 180, 100, 45, 10, 3.0, 2.0, 0.0, -1.0, None, None, None, 0.3, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 1.5, 180, 100, 45, 10, 2.7, 5.5, 0.0, -1.0, None, None, None, 0.3, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 80, 100, 10, 10, 2.5, 0.0, 3.5, 0.0, None, None, None, 0.3, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.5, 355, 100, 30, 10, 1.8, 0.0, 9.5, 1.6, Some(0.0), Some(5.0), Some(1.6), 0.3, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 5, 0, Hash40::new("sword2"), 1.5, 180, 100, 45, 10, 3.0, -1.5, 0.0, -1.0, None, None, None, 0.3, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("sword2"), 1.5, 180, 100, 45, 10, 3.0, 2.0, 0.0, -1.0, None, None, None, 0.3, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 1.5, 180, 100, 35, 10, 2.7, 5.5, 0.0, -1.0, None, None, None, 0.3, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.5, 80, 100, 10, 0, 2.5, 0.0, 3.5, 0.0, None, None, None, 0.3, 0.2, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.4, 48, 180, 0, 70, 4.0, 0.0, 7.0, -11.0, Some(0.0), Some(4.5), Some(8.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 47.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
}

#[acmd_script( agent = "toonlink", script = "game_specialairhi", category = ACMD_GAME )]
unsafe fn toonlink_upb_air_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        TOONLINK_UPB_GROUNDTOAIR[entry_id] = false;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 6.8, 0, 6.5, 10.0, 0, 4.0, -5.5, 1.4, 1.25, 50, false, 1.5, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        macros::SET_SPEED_EX(fighter, 2.3, 1.72, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.3, 73, 105, 0, 50, 6.0, 0.0, 7.5, 13.5, Some(0.0), Some(7.5), Some(7.5), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 14.3, 73, 105, 0, 50, 5.5, 3.0, 2.0, 0.0, None, None, None, 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.3, 73, 105, 0, 50, 6.0, 0.0, 8.0, 13.5, Some(0.0), Some(7.5), Some(7.5), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 14.3, 73, 105, 0, 50, 5.5, 2.5, 1.5, 0.0, None, None, None, 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.3, 73, 105, 0, 50, 6.0, 0.0, 8.5, 13.5, Some(0.0), Some(7.5), Some(7.5), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 14.3, 73, 105, 0, 50, 5.5, 3.0, 1.0, 0.0, None, None, None, 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.3, 73, 105, 0, 50, 6.5, 0.0, 9.0, 13.5, Some(0.0), Some(7.5), Some(7.5), 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword2"), 14.3, 73, 105, 0, 50, 5.5, 4.0, 1.0, 0.0, None, None, None, 1.0, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_REFLECTOR, *COLLISION_KIND_REFLECTOR, 0, Hash40::new("top"), 6.8, 0, 8.5, 11.0, 0, 4.0, -2.5, 1.5, 2.0, 50, false, 1.5, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 20.3, 73, 100, 0, 50, 8.5, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(8.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 48.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LINK_STATUS_RSLASH_FLAG_RESET_SPEED_MAX_X);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 49.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, 0, *FIGHTER_REFLECTOR_GROUP_HOMERUNBAT);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        if TOONLINK_UPB_GROUNDTOAIR[entry_id] {
            StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
        }
    }
}

#[acmd_script( agent = "toonlink", script = "game_appeallwl", category = ACMD_GAME )]
unsafe fn toonlink_downtaunt_left_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 65, 60, 0, 20, 6.5, 0.0, 6.0, 3.0, Some(0.0), Some(6.0), Some(8.3), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "toonlink", script = "game_appeallwr", category = ACMD_GAME )]
unsafe fn toonlink_downtaunt_right_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 65, 60, 0, 20, 6.5, 0.0, 6.0, 3.0, Some(0.0), Some(6.0), Some(8.3), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TOONLINK_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        toonlink_frame
    );
    smashline::install_acmd_scripts!(
        toonlink_jab_smash_script,
        toonlink_jab2_smash_script,
        toonlink_jab3_smash_script,
        toonlink_dashattack_smash_script,
        toonlink_ftilt_smash_script,
        toonlink_utilt_smash_script,
        toonlink_dtilt_smash_script,
        toonlink_fsmash_smash_script,
        toonlink_usmash_smash_script,
        toonlink_dsmash_smash_script,
        toonlink_nair_smash_script,
        toonlink_fair_smash_script,
        toonlink_bair_smash_script,
        toonlink_uair_smash_script,
        toonlink_dair_smash_script,
        toonlink_dair_bounce_smash_script,
        toonlink_grab_smash_script,
        toonlink_grabd_smash_script,
        toonlink_grabp_smash_script,
        toonlink_zair_smash_script,
        toonlink_fthrow_smash_script,
        toonlink_bthrow_smash_script,
        toonlink_uthrow_smash_script,
        toonlink_dthrow_smash_script,
        arrow_projectile,
        arrow_stick_projectile,
        toonlink_sideb_smash_script,
        toonlink_sideb_air_smash_script,
        toonlink_sideb_catch_smash_script,
        toonlink_sideb_catch_air_smash_script,
        boomerang_thrown,
        boomerang_returning,
        toonlink_upb_smash_script,
        toonlink_upb_air_smash_script,
        toonlink_downtaunt_left_smash_script,
        toonlink_downtaunt_right_smash_script

        

        
    );
}
