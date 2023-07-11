#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::hash40;

// A Once-Per-Fighter-Frame that only applies to Joker
#[fighter_frame( agent = FIGHTER_KIND_JACK )]
fn jack_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);
        let sticky = ControlModule::get_stick_y(fighter.module_accessor);

        println!("Looking cool, Joker! Σ(´・ω・｀)");


        if [*FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE, *FIGHTER_JACK_STATUS_KIND_SPECIAL_N_BARRAGE_START].contains(&status) {
            if sticky <= -0.5 {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
        }


    }
}

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_jokerhat_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);
        let sticky = ControlModule::get_stick_y(fighter.module_accessor);


        if [*FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_BARRAGE, *FIGHTER_KIRBY_STATUS_KIND_JACK_SPECIAL_N_BARRAGE_START].contains(&status) {
            if sticky <= -0.5 {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
        }

    }
}

#[acmd_script( agent = "jack", script = "game_attack11", category = ACMD_GAME )]
unsafe fn jack_jab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 361, 30, 20, 27, 3.0, 0.0, 7.5, 7.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.2, 361, 30, 20, 27, 3.0, 0.0, 7.5, 10.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.2, 180, 20, 25, 32, 3.0, 0.0, 7.5, 12.5, Some(0.0), Some(7.5), Some(15.2), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.2, 361, 20, 15, 20, 3.0, 0.0, 7.5, 12.5, Some(0.0), Some(7.5), Some(15.2), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 1.2, 361, 25, 26, 27, 3.5, 0.0, 7.5, 7.5, Some(0.0), Some(7.5), Some(10.8), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            macros::ATTACK(fighter, 5, 1, Hash40::new("top"), 1.2, 180, 25, 26, 27, 3.5, 0.0, 7.5, 10.8, Some(0.0), Some(7.5), Some(14.7), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "jack", script = "game_attack12", category = ACMD_GAME )]
unsafe fn jack_jab2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.1, 361, 30, 15, 25, 3.1, 0.0, 8.0, 10.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.1, 180, 30, 15, 20, 3.2, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(14.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.1, 180, 30, 22, 30, 3.6, 0.0, 8.0, 13.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.1, 361, 30, 15, 26, 3.7, 0.0, 8.0, 18.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 1.1, 361, 30, 15, 20, 2.5, 0.0, 8.0, 10.5, Some(0.0), Some(8.0), Some(15.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
            macros::ATTACK(fighter, 5, 1, Hash40::new("top"), 1.1, 180, 30, 15, 20, 2.5, 0.0, 8.0, 15.0, Some(0.0), Some(8.0), Some(18.0), 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

#[acmd_script( agent = "jack", script = "game_attack13", category = ACMD_GAME )]
unsafe fn jack_jab3_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 80, 0, 60, 3.2, 0.0, 8.0, 10.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 361, 80, 0, 60, 3.2, 0.0, 8.0, 13.5, Some(0.0), Some(8.0), Some(14.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 4.5, 361, 80, 0, 60, 3.6, 0.0, 0.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 80, 0, 60, 3.2, 0.0, 8.0, 10.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 361, 80, 0, 60, 3.2, 0.0, 8.0, 13.5, Some(0.0), Some(8.0), Some(14.5), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 4.5, 361, 80, 0, 60, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(2.0), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 3.6, 361, 135, 0, 65, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(14.0), 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.5, 361, 80, 0, 60, 3.0, 0.0, 8.0, 15.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.5, 361, 80, 0, 60, 3.0, 0.0, 8.0, 17.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 1, Hash40::new("top"), 3.6, 361, 135, 0, 65, 3.5, 0.0, 8.0, 17.0, None, None, None, 1.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
}

#[acmd_script( agent = "jack", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn jack_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.7, 35, 95, 70, 0, 3.0, 0.0, 4.0, 4.0, Some(3.0), Some(9.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.7, 40, 95, 80, 0, 3.0, 0.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footr"), 3.7, 50, 95, 50, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.7, 25, 95, 40, 0, 3.0, 0.0, 4.0, 4.0, Some(3.0), Some(9.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 4, 0, Hash40::new("kneer"), 3.7, 10, 95, 30, 0, 3.0, 0.0, 0.0, 0.0, Some(2.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 5, 0, Hash40::new("footr"), 3.7, 15, 95, 25, 0, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 4.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 7.4, 361, 110, 0, 60, 3.2, 0.0, 0.0, 0.0, Some(5.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("footl"), 7.4, 361, 110, 0, 60, 3.2, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.8, 28, 120, 0, 70, 5.0, 0.0, 8.0, 7.0, Some(0.0), Some(8.0), Some(12.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 6.8, 28, 120, 0, 70, 5.0, 0.0, 8.0, 7.0, Some(0.0), Some(8.0), Some(12.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.1);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.1);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.8, 28, 120, 0, 70, 5.0, 0.0, 10.0, 7.0, Some(0.0), Some(10.0), Some(13.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 6.8, 28, 120, 0, 70, 5.0, 0.0, 10.0, 7.0, Some(0.0), Some(10.0), Some(13.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.1);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.1);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.8, 28, 120, 0, 70, 5.0, 0.0, 11.0, 7.0, Some(0.0), Some(11.0), Some(11.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 6.8, 28, 120, 0, 70, 5.0, 0.0, 11.0, 7.0, Some(0.0), Some(11.0), Some(11.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.1);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.1);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "jack", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn jack_ftilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 3.3, 93, 100, 20, 0, 2.7, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 3.3, 40, 100, 20, 0, 2.7, 0.0, -3.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 5.8, 49, 110, 0, 45, 2.7, 0.0, -3.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            AttackModule::set_size(fighter.module_accessor, 2, 0.0);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 5.5, 35, 114, 0, 60, 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(3.5), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 1, Hash40::new("knife"), 5.5, 35, 114, 0, 60, 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(3.5), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.3);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.3);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "jack", script = "game_attacks3hi", category = ACMD_GAME )]
unsafe fn jack_ftilt2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 3.3, 93, 100, 20, 0, 2.7, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 3.3, 40, 100, 20, 0, 2.7, 0.0, -3.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 5.8, 49, 110, 0, 45, 2.7, 0.0, -3.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            AttackModule::set_size(fighter.module_accessor, 2, 0.0);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 5.5, 35, 114, 0, 60, 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(3.5), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 1, Hash40::new("knife"), 5.5, 35, 114, 0, 60, 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(3.5), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.3);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.3);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "jack", script = "game_attacks3lw", category = ACMD_GAME )]
unsafe fn jack_ftilt3_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 3.3, 93, 100, 20, 0, 2.7, 0.0, 4.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 3.3, 40, 100, 20, 0, 2.7, 0.0, -3.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 5.8, 49, 110, 0, 45, 2.7, 0.0, -3.0, 0.0, Some(0.0), Some(4.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            AttackModule::set_size(fighter.module_accessor, 2, 0.0);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 5.5, 35, 114, 0, 60, 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(3.5), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 0, 1, Hash40::new("knife"), 5.5, 35, 114, 0, 60, 4.0, 0.0, 0.5, 0.0, Some(0.0), Some(3.5), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BOMB);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.3);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.3);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "jack", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn jack_utilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 120, 100, 150, 0, 2.7, 0.0, 4.0, 4.0, Some(0.0), Some(8.0), Some(7.5), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 3.0, 105, 100, 150, 0, 2.7, -1.0, 0.0, 0.0, Some(1.0), Some(0.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 3.0, 100, 100, 60, 0, 2.7, -1.0, 0.0, 0.0, Some(1.0), Some(0.0), Some(0.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("knife"), 3.0, 110, 100, 165, 0, 2.3, 0.0, 1.2, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("knife"), 3.0, 115, 100, 60, 0, 2.3, 0.0, 1.2, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 5, 0, Hash40::new("knife"), 3.0, 123, 100, 165, 0, 2.3, 0.0, 3.3, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 6, 0, Hash40::new("knife"), 3.0, 115, 100, 60, 0, 2.3, 0.0, 3.3, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
        AttackModule::clear(fighter.module_accessor, 2, false);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 130, 100, 20, 0, 3.2, 0.0, 23.0, 1.0, Some(0.0), Some(20.0), Some(5.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 3.0, 367, 100, 20, 0, 3.2, 0.0, 23.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 5, 0, Hash40::new("top"), 3.0, 133, 100, 0, 0, 3.2, 0.0, 13.0, 7.5, Some(0.0), Some(18.0), Some(7.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, 6, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("knife"), 1.2, 367, 20, 0, 20, 5.0, 0.0, 1.7, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, 4, false);
        AttackModule::clear(fighter.module_accessor, 5, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 0.9, 367, 20, 0, 50, 6.0, 0.0, 25.0, -1.0, Some(0.0), Some(25.0), Some(1.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 0, Hash40::new("knife"), 1.8, 90, 170, 0, 35, 5.0, 0.0, 1.7, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 3, 0, Hash40::new("knife"), 4.6, 90, 180, 0, 50, 5.0, 0.0, 1.6, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.15);
    }
}

#[acmd_script( agent = "jack", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn jack_dtilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.9, 75, 66, 0, 55, 2.8, -4.0, 0.0, 1.5, Some(5.0), Some(0.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 82, 56, 0, 55, 3.5, 0.0, 3.2, 0.0, Some(0.0), Some(3.2), Some(8.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 8.0, 82, 85, 0, 55, 3.5, 0.0, 3.2, 0.0, Some(0.0), Some(3.2), Some(8.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.5);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.9, 77, 66, 0, 55, 2.8, -4.0, 0.0, 1.5, Some(5.0), Some(0.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 82, 56, 0, 55, 3.5, 0.0, 3.2, 0.0, Some(0.0), Some(3.2), Some(8.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 8.0, 82, 85, 0, 55, 3.5, 0.0, 3.2, 0.0, Some(0.0), Some(3.2), Some(8.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.5);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.5);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        JostleModule::set_status(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "jack", script = "game_attacks4", category = ACMD_GAME )]
unsafe fn jack_fsmash_smash_script(fighter: &mut L2CAgentBase) {

    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 2.0, 5.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        let rand_num = smash::app::sv_math::rand(hash40("fighter"), 6);
        if rand_num == 0 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 2.7, 0.0, 10.0, 12.0, Some(0.0), Some(6.0), Some(12.7), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 4.0, 0.0, 11.0, 9.0, Some(0.0), Some(8.0), Some(9.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            AttackModule::set_poison_param(fighter.module_accessor, 0, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 1, 300, 50, 1.2, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 2.7, 0.0, 10.0, 12.0, Some(0.0), Some(6.0), Some(12.7), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 4.0, 0.0, 11.0, 9.0, Some(0.0), Some(8.0), Some(9.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 3.0, 0.0, 14.0, 15.5, Some(0.0), Some(8.0), Some(15.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 5.0, 0.0, 8.5, 10.0, Some(0.0), Some(12.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 9.3, 361, 155, 0, 50, 3.0, 0.0, 14.0, 15.5, Some(0.0), Some(8.0), Some(15.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 9.3, 361, 155, 0, 50, 5.0, 0.0, 8.5, 10.0, Some(0.0), Some(12.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.1);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.1);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        let rand_num = smash::app::sv_math::rand(hash40("fighter"), 6);
        if rand_num == 0 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 2.7, 0.0, 8.0, 12.0, Some(0.0), Some(6.0), Some(12.7), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 4.0, 0.0, 11.0, 9.0, Some(0.0), Some(6.0), Some(9.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            AttackModule::set_poison_param(fighter.module_accessor, 0, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 1, 300, 50, 1.2, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 2.7, 0.0, 8.0, 12.0, Some(0.0), Some(6.0), Some(12.7), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 4.0, 0.0, 11.0, 9.0, Some(0.0), Some(6.0), Some(9.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 3.0, 0.0, 14.0, 15.5, Some(0.0), Some(6.5), Some(15.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.4, 361, 102, 0, 40, 5.0, 0.0, 7.0, 10.0, Some(0.0), Some(12.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 9.3, 361, 155, 0, 50, 3.0, 0.0, 14.0, 15.5, Some(0.0), Some(6.5), Some(15.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 9.3, 361, 155, 0, 50, 5.0, 0.0, 7.0, 10.0, Some(0.0), Some(12.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_NONE);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.1);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.1);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::set_target_category(fighter.module_accessor, 0, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_target_category(fighter.module_accessor, 1, *COLLISION_CATEGORY_MASK_NO_IF as u32);
        AttackModule::set_size(fighter.module_accessor, 0, 0.0);
        AttackModule::set_size(fighter.module_accessor, 1, 0.0);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.4, 361, 102, 0, 0, 3.0, 0.0, 14.0, 15.5, Some(0.0), Some(6.5), Some(15.5), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.4, 361, 102, 0, 0, 5.0, 0.0, 7.0, 10.0, Some(0.0), Some(12.5), Some(10.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            AttackModule::set_size(fighter.module_accessor, 0, 3.0);
            AttackModule::set_size(fighter.module_accessor, 1, 5.0);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "jack", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn jack_usmash_smash_script(fighter: &mut L2CAgentBase) {
    
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        let rand_num = smash::app::sv_math::rand(hash40("fighter"), 6);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        if rand_num == 0 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 13.6, 83, 106, 0, 43, 3.4, 0.0, 2.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.6, 83, 106, 0, 43, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.6, 83, 106, 0, 43, 3.7, 0.0, 7.0, 3.0, Some(0.0), Some(12.0), Some(6.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            AttackModule::set_poison_param(fighter.module_accessor, 0, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 1, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 2, 300, 50, 1.2, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 13.6, 83, 106, 0, 43, 3.4, 0.0, 2.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.6, 83, 106, 0, 43, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.6, 83, 106, 0, 43, 3.7, 0.0, 7.0, 3.0, Some(0.0), Some(12.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 13.6, 83, 106, 0, 43, 3.8, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.6, 83, 106, 0, 43, 3.2, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.6, 83, 106, 0, 43, 3.7, 0.0, 7.0, 3.0, Some(0.0), Some(12.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 1, Hash40::new("knife"), 6.0, 83, 181, 0, 59, 3.8, 0.0, 4.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 1, Hash40::new("armr"), 6.0, 83, 181, 0, 59, 3.0, -1.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 1, Hash40::new("top"), 6.0, 83, 181, 0, 59, 3.7, 0.0, 7.0, 3.0, Some(0.0), Some(12.0), Some(6.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.25);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 5, 1.25);
            HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
            HitModule::set_status_joint(fighter.module_accessor, Hash40::new("shoulderr"), HitStatus(*HIT_STATUS_XLU), 0);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
        AttackModule::clear(fighter.module_accessor, 5, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.6, 83, 106, 0, 43, 5.0, 0.0, 28.0, 1.0, Some(0.0), Some(16.0), Some(9.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 1, Hash40::new("top"), 6.0, 83, 181, 0, 59, 5.0, 0.0, 28.0, 1.0, Some(0.0), Some(16.0), Some(9.5), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.6, 83, 106, 0, 43, 5.0, 0.0, 28.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 5, 1, Hash40::new("top"), 6.0, 83, 181, 0, 59, 5.0, 0.0, 28.0, 1.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
}

#[acmd_script( agent = "jack", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn jack_dsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        let rand_num = smash::app::sv_math::rand(hash40("fighter"), 6);
        if rand_num == 0 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 12.6, 30, 99, 0, 35, 2.4, 0.0, 1.5, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 12.6, 30, 99, 0, 35, 2.4, 1.0, 4.0, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.6, 30, 99, 0, 35, 2.9, -1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 12.6, 30, 99, 0, 35, 2.9, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            AttackModule::set_poison_param(fighter.module_accessor, 0, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 1, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 2, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 3, 300, 50, 1.2, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 12.6, 30, 99, 0, 35, 2.4, 0.0, 1.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 12.6, 30, 99, 0, 35, 2.4, 1.0, 4.0, 1.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.6, 30, 99, 0, 35, 2.9, -1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 12.6, 30, 99, 0, 35, 2.9, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 13.0, 30, 90, 0, 64, 2.4, 0.0, 1.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 13.0, 30, 90, 0, 64, 2.4, 1.0, 4.0, 1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.0, 30, 90, 0, 64, 2.9, -1.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 13.0, 30, 90, 0, 64, 2.9, 1.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, 30, 90, 0, 64, 3.5, 0.0, 6.5, 6.5, Some(0.0), Some(4.0), Some(15.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 1, Hash40::new("top"), 7.0, 30, 164, 0, 44, 3.5, 0.0, 6.5, 6.5, Some(0.0), Some(4.0), Some(15.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        let rand_num = smash::app::sv_math::rand(hash40("fighter"), 6);
        if rand_num == 0 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 12.6, 30, 99, 0, 35, 2.4, 0.0, 1.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 12.6, 30, 99, 0, 35, 2.4, 1.0, 4.5, 0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.6, 30, 99, 0, 35, 2.9, -1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 12.6, 30, 99, 0, 35, 2.9, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
            AttackModule::set_poison_param(fighter.module_accessor, 0, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 1, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 2, 300, 50, 1.2, false);
            AttackModule::set_poison_param(fighter.module_accessor, 3, 300, 50, 1.2, false);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 12.6, 30, 99, 0, 35, 2.4, 0.0, 1.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 12.6, 30, 99, 0, 35, 2.4, 1.0, 4.5, 0.5, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 12.6, 30, 99, 0, 35, 2.9, -1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 12.6, 30, 99, 0, 35, 2.9, 1.0, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 13.0, 30, 93, 0, 64, 2.4, 0.0, 1.5, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 13.0, 30, 90, 0, 64, 2.4, 1.0, 4.5, 1.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 13.0, 30, 90, 0, 64, 2.9, -1.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 13.0, 30, 90, 0, 64, 2.9, 1.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 4, 0, Hash40::new("top"), 13.0, 30, 90, 0, 64, 3.5, 0.0, 6.0, -8.0, Some(0.0), Some(3.0), Some(-15.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 5, 1, Hash40::new("top"), 7.0, 30, 164, 0, 44, 3.5, 0.0, 6.0, -8.0, Some(0.0), Some(3.0), Some(-15.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.15);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 1.15);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 2, 1.15);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 3, 1.15);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 4, 1.15);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}


#[acmd_script( agent = "jack", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn jack_nair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 8.1, 53, 85, 0, 35, 3.8, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 8.1, 53, 85, 0, 35, 3.8, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 3, 1, Hash40::new("throw"), 5.5, 60, 109, 0, 50, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "jack", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn jack_fair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 3.3, 367, 50, 0, 50, 4.0, 3.2, 0.0, 0.0, Some(3.2), Some(-0.5), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 3.3, 367, 50, 0, 50, 4.0, 4.2, -0.7, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.3, 367, 50, 0, 50, 4.0, 0.0, 8.5, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 2.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 6.4, 47, 114, 0, 46, 3.8, 4.4, -0.7, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 6.4, 47, 114, 0, 46, 3.8, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            AttackModule::clear(fighter.module_accessor, 1, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 34, 156, 0, 48, 6.0, 0.0, 13.5, 5.0, Some(0.0), Some(13.5), Some(7.5), 1.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 8.0, 34, 110, 0, 58, 6.0, 0.0, 13.5, 5.0, Some(0.0), Some(13.5), Some(7.5), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 8.0, 34, 110, 0, 58, 7.0, 0.0, 18.0, 5.0, Some(0.0), Some(17.0), Some(6.5), 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 4.0, 40, 114, 0, 46, 3.8, 4.4, -0.7, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 4.0, 40, 114, 0, 46, 3.8, 3.2, 0.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.7);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "jack", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn jack_bair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.1, 43, 110, 0, 54, 3.2, 0.0, 3.3, -6.0, Some(0.0), Some(10.7), Some(-8.4), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.1, 43, 110, 0, 54, 3.2, 0.0, 4.7, -11.1, Some(0.0), Some(11.1), Some(-14.2), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.1, 43, 110, 0, 54, 3.7, 0.0, 4.0, -3.0, Some(0.0), Some(8.5), Some(-5.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.4, 35, 116, 0, 51, 5.0, 0.0, 10.5, -12.5, Some(0.0), Some(15.0), Some(-12.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.4, 35, 116, 0, 51, 5.0, 0.0, 7.5, -10.0, Some(0.0), Some(5.5), Some(-8.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 7.8, 35, 130, 0, 58, 5.0, 0.0, 7.5, -10.0, Some(0.0), Some(5.5), Some(-8.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 7.8, 35, 130, 0, 58, 5.0, 0.0, 10.5, -12.5, Some(0.0), Some(15.0), Some(-12.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 7.8, 35, 130, 0, 58, 5.0, 0.0, 16.5, -12.0, Some(0.0), Some(20.5), Some(-9.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("knife"), 10.1, 43, 110, 0, 54, 3.2, -1.0, 2.5, 1.0, Some(-3.0), Some(-0.5), Some(8.0), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 10.1, 43, 110, 0, 54, 3.2, 0.0, -3.0, 2.5, Some(-4.5), Some(-6.0), Some(10.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.1, 43, 110, 0, 54, 4.1, 0.0, 4.5, -6.5, Some(0.0), Some(7.5), Some(-5.5), 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.4, 35, 116, 0, 51, 5.0, 0.0, 10.5, -12.5, Some(0.0), Some(15.0), Some(-12.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.4, 35, 116, 0, 51, 5.0, 0.0, 7.5, -10.0, Some(0.0), Some(5.5), Some(-8.5), 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 1, Hash40::new("top"), 7.8, 35, 130, 0, 58, 5.0, 0.0, 7.5, -10.0, Some(0.0), Some(5.5), Some(-8.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 7.8, 35, 130, 0, 58, 5.0, 0.0, 10.5, -12.5, Some(0.0), Some(15.0), Some(-12.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 7.8, 35, 130, 0, 58, 5.0, 0.0, 16.5, -12.0, Some(0.0), Some(20.5), Some(-9.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "jack", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn jack_uair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 1.0, 367, 100, 0, 35, 4.7, 4.7, 0.0, 0.0, None, None, None, 0.6, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 1.0, 367, 100, 0, 35, 4.7, -0.5, 0.0, 0.0, None, None, None, 0.6, 1.2, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5, 80, 155, 0, 50, 4.5, 4.8, 0.0, 0.0, Some(-0.5), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 3.5, 80, 160, 0, 50, 5.0, 4.3, 0.0, 0.0, Some(1.5), Some(0.0), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 7.5, 80, 131, 0, 50, 5.0, 0.0, 20.0, 1.5, Some(0.0), Some(24.0), Some(0.0), 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.15);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


#[acmd_script( agent = "jack", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn jack_dair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 8.8, 38, 100, 0, 40, 2.8, 0.0, -1.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 8.8, 38, 100, 0, 40, 3.4, 0.0, 2.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("knife"), 8.8, 275, 100, 0, 14, 2.8, 0.0, -1.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 4, 0, Hash40::new("knife"), 8.8, 275, 100, 0, 14, 3.4, 0.0, 2.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 8.0, 38, 40, 0, 40, 2.8, 0.0, -1.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 8.0, 38, 40, 0, 40, 3.4, 0.0, 2.5, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 3, 1, Hash40::new("top"), 8.0, 270, 96, 0, 35, 5.0, 0.0, 2.0, 1.0, Some(0.0), Some(-4.0), Some(1.0), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("knife"), 8.0, 38, 40, 0, 40, 2.5, 0.0, -1.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("knife"), 8.0, 38, 40, 0, 40, 3.0, 0.0, 2.5, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}


#[acmd_script( agent = "jack", script = "game_catch", category = ACMD_GAME )]
unsafe fn jack_grab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.7, 0.0, 8.1, 4.5, Some(0.0), Some(8.1), Some(9.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
}   

#[acmd_script( agent = "jack", script = "game_catchdash", category = ACMD_GAME )]
unsafe fn jack_grabd_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.4, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.9), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}   

#[acmd_script( agent = "jack", script = "game_catchturn", category = ACMD_GAME )]
unsafe fn jack_grabp_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.7, 0.0, 6.6, -7.0, Some(0.0), Some(6.6), Some(-19.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}   

#[acmd_script( agent = "jack", script = "game_throwf", category = ACMD_GAME )]
unsafe fn jack_fthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.5, 20, 38, 0, 72, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 15, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 6, -2, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "jack", script = "game_throwb", category = ACMD_GAME )]
unsafe fn jack_bthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.8, 131, 120, 0, 64, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, -7, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.2);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, -2, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "jack", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn jack_uthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.2, 80, 65, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 3, 10);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 2, 6, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "jack", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn jack_dthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.4, 73, 95, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 12.8, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.3);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 9.5, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "jack", script = "game_specialn1", category = ACMD_GAME )]
unsafe fn jack_neutralb_1_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 1, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 1, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(fighter) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}


#[acmd_script( agent = "jack", script = "game_specialn2", category = ACMD_GAME )]
unsafe fn jack_neutralb_2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 1, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 1, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(fighter) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script( agent = "jack", script = "game_specialn3", category = ACMD_GAME )]
unsafe fn jack_neutralb_3_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 1, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 1, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(44.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(84.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(fighter) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script( agent = "jack", script = "game_specialn1_ex", category = ACMD_GAME )]
unsafe fn jack_neutralb_1_ex_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 50, 0, 60, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 1, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}


#[acmd_script( agent = "jack", script = "game_specialn2_ex", category = ACMD_GAME )]
unsafe fn jack_neutralb_2_ex_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 50, 0, 60, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 1, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}


#[acmd_script( agent = "jack", script = "game_specialn3_ex", category = ACMD_GAME )]
unsafe fn jack_neutralb_3_ex_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 50, 0, 60, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 1, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_JUMP);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}


#[acmd_script( agent = "jack", script = "game_specialairn1", category = ACMD_GAME )]
unsafe fn jack_neutralb_1_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 3, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 3, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(fighter) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}


#[acmd_script( agent = "jack", script = "game_specialairn2", category = ACMD_GAME )]
unsafe fn jack_neutralb_2_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 3, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 3, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(fighter) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairn3", category = ACMD_GAME )]
unsafe fn jack_neutralb_3_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 3, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
    }
    if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND) == *FIGHTER_KIND_KIRBY {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 75, 0, 5, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 50, 0, 3, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.5, 0.0, 5.0, 9.0, Some(0.0), Some(5.0), Some(84.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        }
    }
    if macros::is_excute(fighter) {
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairn1_ex", category = ACMD_GAME )]
unsafe fn jack_neutralb_1_ex_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 50, 0, 60, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 50, 0, 3, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    
    }sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairn2_ex", category = ACMD_GAME )]
unsafe fn jack_neutralb_2_ex_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 50, 0, 60, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 50, 0, 3, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    
    }sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairn3_ex", category = ACMD_GAME )]
unsafe fn jack_neutralb_3_ex_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 361, 50, 0, 60, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -3, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 361, 50, 0, 3, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_TRIGGER);
    
    }sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BARRAGE_BUTTON_ON);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(24.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(44.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.4, 361, 0, 0, 0, 2.8, 0.0, 11.0, 9.0, Some(0.0), Some(11.0), Some(89.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 4, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_ENABLE_CONTROL_ENERGY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_BUTTON_RAPID);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_CHECK_NEXT_STATUS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_N_FLAG_UNABLE_SPECIAL_N);
    }
}

#[acmd_script( agent = "jack", script = "game_specialnjump", category = ACMD_GAME )]
unsafe fn jack_neutralb_jump_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 0.75, y: 0.4, z: 0.0});
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 270, 70, 0, 10, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 270, 70, 0, 10, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 270, 70, 0, 10, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 270, 70, 0, 10, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 270, 70, 0, 10, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.8, 50, 120, 0, 20, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 270, 70, 0, 10, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.8, 50, 120, 0, 20, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 120, 0, 20, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 50, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 0.5, 365, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 0.0, y: -0.8, z: 0.0});
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 270, 70, 0, 25, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 270, 120, 0, 25, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 270, 120, 0, 25, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 120, 0, 25, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 2, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 270, 70, 0, 40, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.3, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, -1, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 270, 120, 0, 35, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 270, 120, 0, 35, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 120, 0, 35, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(9.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 2, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairndown", category = ACMD_GAME )]
unsafe fn jack_neutralb_air_down_loop_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    for _ in 0..20 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 50, 80, 0, 25, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 45, 0, 0, 29, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 55, 0, 0, 29, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 45, 0, 0, 29, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 50, 80, 0, 25, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 45, 0, 0, 29, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 55, 0, 0, 29, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 45, 0, 0, 29, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
            }
        }
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 6.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.3, 50, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 55, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 45, 0, 0, 0, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
                macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
            }
        }
        if macros::is_excute(fighter) {
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairndownend", category = ACMD_GAME )]
unsafe fn jack_neutralb_air_down_end_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 125, 80, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 90, 0, 0, 35, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 45, 0, 0, 35, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 45, 0, 0, 35, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 125, 80, 0, 45, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-22.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -2, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 90, 0, 0, 35, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(-7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 45, 0, 0, 35, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.0, 45, 0, 0, 35, 3.0, 0.0, 0.0, 0.0, Some(0.0), Some(-50.0), Some(7.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 0, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 1, 5, 4);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 2, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW(fighter, 3, 5, 4);
        }
    }
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_63_bullet"), 2, false, 0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairnshoot", category = ACMD_GAME )]
unsafe fn jack_neutralb_air_speen_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 8.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
           macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
           macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
       AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 6.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 6.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(40.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -1, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
        macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 4, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_JACK_INSTANCE_WORK_ID_FLAG_DOYLE) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("gunl"), 8.0, 361, 75, 0, 5, 9.5, 0.0, 0.0, 6.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("gunl"), 8.0, 361, 0, 0, 0, 3.9, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(50.0), 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, -2, 0.0, 12, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_jack_bullet"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_JACK_SHOT, *ATTACK_REGION_OBJECT);
           macros::FT_SHOOTING_ATTACK_GROUND_CHECK_NEW_arg5(fighter, 1, 5, 4, Hash40::new("jack_gun_hit2"), Hash40::new("se_jack_special_n02"));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}


#[acmd_script( agent = "jack", script = "game_specials1", category = ACMD_GAME )]
unsafe fn jack_sideb_eiha_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_FIRE, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.83333333333);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairs1", category = ACMD_GAME )]
unsafe fn jack_sideb_eiha_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_FIRE, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.83333333333);
    }
}

#[acmd_script( agent = "jack_fire", script = "game_fly", category = ACMD_GAME )]
unsafe fn eiha_projectile(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 20, 0, 35, 1.3, 0.0, -0.7, 1.5, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 20, 0, 35, 1.3, 2.4, -0.7, 1.5, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 361, 20, 0, 35, 1.3, -2.4, -0.7, 1.5, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 361, 42, 1.2, false);
        AttackModule::set_poison_param(fighter.module_accessor, 1, 361, 42, 1.2, false);
        AttackModule::set_poison_param(fighter.module_accessor, 2, 361, 42, 1.2, false);
        //id, frames, rehit rate, damage, unknown boolean (usually false)
    }
}

#[acmd_script( agent = "jack_fire", script = "game_hit", category = ACMD_GAME )]
unsafe fn eiha_projectile_hit(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 75, 210, 0, 20, 8.5, 0.0, 1.5, 0.0, Some(0.0), Some(3.5), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 361, 42, 1.2, false);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
}

#[acmd_script( agent = "jack_fire2", script = "game_fly", category = ACMD_GAME )]
unsafe fn eigaon_projectile(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 90, 20, 0, 35, 1.7, 0.0, -0.5, 0.7, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 90, 20, 0, 35, 1.7, 3.0, -0.5, 0.7, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.0, 90, 20, 0, 35, 1.7, -3.0, -0.5, 0.7, None, None, None, 0.8, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 321, 40, 2.0, false);
        AttackModule::set_poison_param(fighter.module_accessor, 1, 321, 40, 2.0, false);
        AttackModule::set_poison_param(fighter.module_accessor, 2, 321, 40, 2.0, false);
    }
}

#[acmd_script( agent = "jack_fire2", script = "game_hit", category = ACMD_GAME )]
unsafe fn eigaon_projectile_hit(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_S);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 367, 100, 0, 5, 4.0, 0.0, 1.5, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 321, 40, 2.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_explosion"), 0, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 367, 100, 0, 5, 4.0, 0.0, 1.5, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 321, 40, 2.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.6, 367, 100, 0, 5, 4.0, 0.0, 1.5, 0.0, Some(0.0), Some(8.0), Some(0.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 5, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 321, 40, 2.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.5, 73, 105, 0, 105, 4.0, 0.0, 1.5, 0.0, Some(0.0), Some(8.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_MAGIC);
        AttackModule::set_poison_param(fighter.module_accessor, 0, 321, 40, 2.0, false);
    }
}


#[acmd_script( agent = "jack", script = "game_specialhi", category = ACMD_GAME )]
unsafe fn jack_upb_wire_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi"), false, 0.0);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, 0.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, smash::app::ArticleOperationTarget(0));
        AreaModule::reset_area(fighter.module_accessor, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        macros::ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
        AreaModule::reset_area(fighter.module_accessor, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
        macros::ENABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("throw"), 3.0, 1.5, 1.5, 0.7, Some(1.5), Some(-15.0), Some(-6.4), *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        macros::SEARCH(fighter, 1, 0, Hash40::new("throw"), 5.5, 1.5, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
        macros::SEARCH(fighter, 2, 0, Hash40::new("throw"), 5.5, 1.2, 1.5, 0.7, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NORMAL, 0, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_STAGE_GIMMICK, *COLLISION_PART_MASK_BODY, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::SET_SEARCH_SIZE_EXIST(fighter, 2, 8);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH_ADD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, smash::app::ArticleOperationTarget(0));
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        macros::UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
    }
}


#[acmd_script( agent = "jack", script = "game_specialhithrow", category = ACMD_GAME )]
unsafe fn jack_upb_wire_throw_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, Hash40::new("special_hi_throw"), false, 0.0);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.0, 78, 50, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, -20, 10);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, smash::app::ArticleOperationTarget(0));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairhi", category = ACMD_GAME )]
unsafe fn jack_upb_wire_air_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_JACK_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_JACK_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_EXTEND, smash::app::ArticleOperationTarget(0));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 12.6, 67, 79, 0, 65, 3.7, 1.5, 2.0, 2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.6, 67, 79, 0, 65, 3.7, 0.0, 22.0, 3.2, Some(0.0), Some(38.0), Some(10.6), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_OBJECT);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0); //frame 15 before
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        ArticleModule::change_status(fighter.module_accessor, *FIGHTER_JACK_GENERATE_ARTICLE_WIREROPE, *WEAPON_JACK_WIREROPE_STATUS_KIND_BACK, smash::app::ArticleOperationTarget(0));
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
        //UNABLE_AREA(fighter, *FIGHTER_JACK_AREA_KIND_ITEM_CATCH);
    }
}


#[acmd_script( agent = "jack", script = "game_speciallwend", category = ACMD_GAME )]
unsafe fn jack_downb_guard_end_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.980392156);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairlwend", category = ACMD_GAME )]
unsafe fn jack_downb_guard_air_end_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.980392156);
    }
}

#[acmd_script( agent = "jack", script = "game_speciallwcounter", category = ACMD_GAME )]
unsafe fn jack_downb_counter_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 51, 0, 80, 17.0, 0.0, 10.5, -5.0, Some(0.0), Some(10.5), Some(10.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "jack", script = "game_specialairlwcounter", category = ACMD_GAME )]
unsafe fn jack_downb_counter_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.0, 361, 51, 0, 80, 17.0, 0.0, 10.5, -5.0, Some(0.0), Some(10.5), Some(10.0), 0.75, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}



pub fn install() {
    smashline::install_agent_frames!(
        jack_frame,
        kirby_jokerhat_frame
    );
    smashline::install_acmd_scripts!(
        jack_jab_smash_script,
        jack_jab2_smash_script,
        jack_jab3_smash_script,
        jack_dashattack_smash_script,
        jack_ftilt_smash_script,
        jack_ftilt2_smash_script,
        jack_ftilt3_smash_script,
        jack_utilt_smash_script,
        jack_dtilt_smash_script,
        jack_fsmash_smash_script,
        jack_usmash_smash_script,
        jack_dsmash_smash_script,
        jack_nair_smash_script,
        jack_fair_smash_script,
        jack_bair_smash_script,
        jack_uair_smash_script,
        jack_dair_smash_script,
        jack_grab_smash_script,
        jack_grabd_smash_script,
        jack_grabd_smash_script,
        jack_grabp_smash_script,
        jack_fthrow_smash_script,
        jack_bthrow_smash_script,
        jack_uthrow_smash_script,
        jack_dthrow_smash_script,
        jack_neutralb_1_smash_script,
        jack_neutralb_2_smash_script,
        jack_neutralb_3_smash_script,
        jack_neutralb_1_ex_smash_script,
        jack_neutralb_2_ex_smash_script,
        jack_neutralb_3_ex_smash_script,
        jack_neutralb_1_air_smash_script,
        jack_neutralb_2_air_smash_script,
        jack_neutralb_3_air_smash_script,
        jack_neutralb_1_ex_air_smash_script,
        jack_neutralb_2_ex_air_smash_script,
        jack_neutralb_3_ex_air_smash_script,
        jack_neutralb_jump_script,
        jack_neutralb_air_down_loop_smash_script,
        jack_neutralb_air_down_end_smash_script,
        jack_neutralb_air_speen_smash_script,
        jack_sideb_eiha_smash_script,
        jack_sideb_eiha_air_smash_script,
        eiha_projectile,
        eiha_projectile_hit,
        eigaon_projectile,
        eigaon_projectile_hit,
        jack_upb_wire_smash_script,
        jack_upb_wire_throw_smash_script,
        jack_upb_wire_air_smash_script,
        jack_downb_guard_end_script,
        jack_downb_guard_air_end_script,
        jack_downb_counter_smash_script,
        jack_downb_counter_air_smash_script
        
    );
}
