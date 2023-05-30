#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;

// A Once-Per-Fighter-Frame that only applies to Min Min
#[fighter_frame( agent = FIGHTER_KIND_TANTAN )]
fn tantan_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        println!("It'sa me, Min Min, 我愛拉麵");
    }
}

#[acmd_script( agent = "tantan", script = "game_attack11", category = ACMD_GAME )]
unsafe fn tantan_jab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0, 4.5);
        MotionModule::set_rate(fighter.module_accessor, 1.66666667);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.8, 180, 20, 15, 20, 3.5, 0.0, 7.0, 15.0, Some(0.0), Some(7.0), Some(15.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.8, 361, 25, 20, 25, 3.5, 0.0, 7.0, 5.0, Some(0.0), Some(7.0), Some(15.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 8.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

#[acmd_script( agent = "tantan", script = "game_attack12", category = ACMD_GAME )]
unsafe fn tantan_jab2_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 4.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.3, 60, 20, 25, 30, 3.5, 0.0, 4.0, 4.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.3, 80, 20, 25, 30, 4.0, 0.0, 4.0, 9.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.3, 93, 20, 25, 30, 4.0, 0.0, 4.0, 14.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 8.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 8.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 8.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_NO_HIT_COMBO);
    }
}

#[acmd_script( agent = "tantan", script = "game_attack13", category = ACMD_GAME )]
unsafe fn tantan_jab3_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.5, 4.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 65, 80, 0, 60, 4.0, 0.0, 4.0, 5.0, Some(0.0), Some(13.5), Some(10.5), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 65, 80, 0, 60, 5.5, 0.0, 9.0, 6.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.6);
    }
}

#[acmd_script( agent = "tantan", script = "game_attack100", category = ACMD_GAME )]
unsafe fn tantan_jab100_smash_script(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 9.0);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 9.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        if macros::is_excute(fighter) {
            FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0, 4.0);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 9.0);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 9.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 9.0);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 9.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 9.0);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 9.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 9.0);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 9.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 9.0);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 9.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 12.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 16, 0, 12, 3.8, 0.0, 5.4, 7.0, Some(0.0), Some(11.6), Some(7.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 0.5, 361, 16, 0, 8, 5.6, 0.0, 5.4, 14.0, Some(0.0), Some(11.6), Some(14.0), 1.0, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 9.0);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 9.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 14.0);
        fighter.clear_lua_stack();
        lua_args!(fighter, 0);
        sv_animcmd::wait_loop_clear(fighter.lua_state_agent);
    }
}

#[acmd_script( agent = "tantan", script = "game_attack100end", category = ACMD_GAME )]
unsafe fn tantan_jab100end_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 45, 160, 0, 50, 7.5, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(15.0), 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn tantan_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 5.5, 6.5, 9.0, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 11.1, 40, 90, 0, 45, 4.0, 0.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 11.1, 40, 90, 0, 45, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 11.1, 40, 90, 0, 45, 3.5, 0.0, 0.0, -1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area_xy(fighter.module_accessor, 4.0, 2.9, 7.2, 7.2);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn tantan_utilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 3.2, 119, 80, 40, 40, 3.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 3.2, 124, 80, 40, 40, 3.9, 1.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 3.2, 135, 80, 40, 40, 3.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 3, 1, Hash40::new("legr"), 6.1, 100, 75, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 4, 1, Hash40::new("kneer"), 6.1, 100, 75, 0, 50, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 5, 1, Hash40::new("footr"), 6.1, 100, 75, 0, 50, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 4, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 5, 2.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
        AttackModule::clear(fighter.module_accessor, 1, false);
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.85);
    }
}

#[acmd_script( agent = "tantan", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn tantan_dtilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 4.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.3, 68, 70, 0, 66, 3.6, 0.0, 1.7, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.3, 73, 70, 0, 66, 3.6, 0.0, 1.7, 2.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 8.3, 78, 70, 0, 66, 4.1, 0.0, 1.7, 8.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.6);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5.0, 4.5);
    }
}

//1 = dragon
//2 = megawatt
//3 = ram ram

#[acmd_script( agent = "tantan", script = "game_attackshortstartl1", category = ACMD_GAME )]
unsafe fn ftilt_startup_1_left(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartlb1", category = ACMD_GAME )]
unsafe fn ftilt_startup_1_left_back(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartl2", category = ACMD_GAME )]
unsafe fn ftilt_startup_2_left(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartlb2", category = ACMD_GAME )]
unsafe fn ftilt_startup_2_left_back(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartl3", category = ACMD_GAME )]
unsafe fn ftilt_startup_3_left(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartlb3", category = ACMD_GAME )]
unsafe fn ftilt_startup_3_left_back(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartr1", category = ACMD_GAME )]
unsafe fn ftilt_startup_1_right(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_COMBO_ENABLE);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_ATTACK_AIR_F);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartrb1", category = ACMD_GAME )]
unsafe fn ftilt_startup_1_right_back(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartr2", category = ACMD_GAME )]
unsafe fn ftilt_startup_2_right(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartrb2", category = ACMD_GAME )]
unsafe fn ftilt_startup_2_right_back(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
    }
}   

#[acmd_script( agent = "tantan", script = "game_attackshortstartr3", category = ACMD_GAME )]
unsafe fn ftilt_startup_3_right(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackshortstartrb3", category = ACMD_GAME )]
unsafe fn ftilt_startup_3_right_back(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
        //MotionModule::set_rate_partial(fighter.module_accessor, *FIGHTER_MOTION_PART_SET_KIND_UPPER_BODY, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
    }
}

//dragon arm ftilt/hold/fair/bair
#[acmd_script( agent = "tantan_punch1", script = "game_attackshort", category = ACMD_GAME )]
unsafe fn dragon_arm_ftilt(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 9.4, 361, 87, 0, 40, 0.7, 3.1, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::disable_tip(fighter.module_accessor);
            AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 9.4, 361, 87, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(-4.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }  
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 9.4, 361, 87, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 9.4, 361, 90, 0, 40, 0.7, 3.1, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 9.4, 361, 90, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//dragon power ftilt
#[acmd_script( agent = "tantan_punch1", script = "game_attackdragonshootshort", category = ACMD_GAME )]
unsafe fn dragon_power_arm_ftilt(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 10.1, 361, 86, 0, 40, 0.7, 5.4, 0.5, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::disable_tip(fighter.module_accessor);
            AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 10.1, 361, 86, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(-4.0), Some(0.5), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 10.1, 361, 86, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.857);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 10.1, 361, 90, 0, 40, 0.7, 5.4, 0.5, 0.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 10.1, 361, 90, 0, 50, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 9, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//megawatt arm ftilt/hold/fair/bair
#[acmd_script( agent = "tantan_punch2", script = "game_attackshort", category = ACMD_GAME )]
unsafe fn megawatt_arm_ftilt(fighter: &mut L2CAgentBase) {   
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 12.3, 45, 92, 0, 50, 0.7, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::disable_tip(fighter.module_accessor);
            AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 12.3, 45, 92, 0, 50, 3.7, 0.0, 0.0, 0.0, Some(-4.0), Some(0.0), Some(0.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 12.3, 45, 92, 0, 50, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_megabolt"), 18, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 12.3, 45, 97, 0, 50, 0.7, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 12.3, 45, 97, 0, 50, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_megabolt"), 18, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//ram ram ftilt/hold/fair/bair
#[acmd_script( agent = "tantan_punch3", script = "game_attackshort", category = ACMD_GAME )]
unsafe fn ramram_arm_ftilt(fighter: &mut L2CAgentBase) {  
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_AIR) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 6.5, 30, 85, 0, 25, 0.9, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::disable_tip(fighter.module_accessor);
            AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
        }   
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 6.5, 30, 85, 0, 25, 3.8, 0.0, 2.0, 0.0, Some(12.0), Some(2.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 6.5, 30, 85, 0, 25, 3.4, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_hotling"), 20, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 6.5, 30, 85, 0, 25, 1.9, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 6.5, 30, 85, 0, 25, 3.4, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_hotling"), 20, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "tantan_ring", script = "game_attackfly", category = ACMD_GAME )]
unsafe fn ramram_arm_projectile(fighter: &mut L2CAgentBase) {  
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 3.2, 135, 85, 0, 45, 4.2, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_AIR) {
        if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG) {
            sv_animcmd::frame(fighter.lua_state_agent, 9.0);
            if macros::is_excute(fighter) {
                //methodlib::L2CValue::operatorbool()const(is_excute);
            }
        }
        else if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG) {
            sv_animcmd::frame(fighter.lua_state_agent, 11.0);
            if macros::is_excute(fighter) {
                //methodlib::L2CValue::operatorbool()const(is_excute);
            }
        }
    }
    else {
        sv_animcmd::frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
           AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

//dragon fsmash (regular)
#[acmd_script( agent = "tantan_punch1", script = "game_attacklong", category = ACMD_GAME )]
unsafe fn dragon_arm_fsmash(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_IS_KIRBY) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 15.5, 361, 87, 0, 40, 1.2, 3.1, 0.5, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::disable_tip(fighter.module_accessor);
            AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 15.5, 361, 87, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 18, false, 0);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 15.5, 361, 87, 0, 40, 1.2, 3.1, 0.5, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            AttackModule::enable_safe_pos(fighter.module_accessor);
            AttackModule::disable_tip(fighter.module_accessor);
            AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 15.5, 361, 87, 0, 40, 2.9, 3.1, 0.5, 0.0, Some(1.0), Some(0.5), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
            ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 18, false, 0);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 15.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
}

//dragon power fsmash
#[acmd_script( agent = "tantan_punch1", script = "game_attackdragonshootlong", category = ACMD_GAME )]
unsafe fn dragon_power_arm_fsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 17.0, 361, 87, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(-7.0), Some(0.5), Some(0.3), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_dragonarm"), 6, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 17.0, 361, 87, 0, 40, 3.2, 5.4, 0.5, 0.3, Some(1.0), Some(0.5), Some(0.3), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//megawatt fsmash
#[acmd_script( agent = "tantan_punch2", script = "game_attacklong", category = ACMD_GAME )]
unsafe fn megawatt_arm_fsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 21.0, 45, 70, 0, 48, 3.7, 0.0, 0.0, 0.0, Some(-9.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 21.0, 45, 70, 0, 48, 3.7, -1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_megabolt"), 24, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//megawatt fsmash charge
#[acmd_script( agent = "tantan_punch2", script = "game_attacklonghold", category = ACMD_GAME )]
unsafe fn megawatt_arm_fsmash_hold(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 22.5, 45, 70, 0, 48, 1.2, 0.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        AttackModule::set_paralyze_frame(fighter.module_accessor, 0, 50, false);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("center"), 22.5, 45, 70, 0, 48, 4.7, -1.0, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH02, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_megabolt"), 24, false, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
        AttackModule::set_paralyze_frame(fighter.module_accessor, 0, 50, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//ramram fsmash
#[acmd_script( agent = "tantan_punch3", script = "game_attacklong", category = ACMD_GAME )]
unsafe fn ramram_arm_fsmash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 11.0, 30, 69, 0, 25, 3.5, 0.0, 2.0, 0.0, Some(12.0), Some(2.0), Some(0.0), 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 11.0, 30, 69, 0, 25, 3.2, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_hotling"), 24, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//ramram fsmash charge
#[acmd_script( agent = "tantan_punch3", script = "game_attacklonghold", category = ACMD_GAME )]
unsafe fn ramram_arm_fsmash_hold(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 12.8, 30, 67, 0, 25, 0.9, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        AttackModule::enable_safe_pos(fighter.module_accessor);
        AttackModule::disable_tip(fighter.module_accessor);
        AttackModule::set_damage_shake_scale(fighter.module_accessor, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 12.8, 30, 67, 0, 25, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_76_hotling"), 24, false, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_PUNCH1_INSTANCE_WORK_ID_FLAG_BOUND) {
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    } 
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 14.8, 30, 57, 0, 25, 4.0, 0.0, 2.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH03, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "tantan_ring", script = "game_attacks4fly", category = ACMD_GAME )]
unsafe fn ramram_arm_projectile_fsmash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("cakram1"), 5.0, 30, 70, 0, 40, 4.6, 0.0, 2.0, 0.0, Some(0.0), Some(-2.0), Some(0.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_OBJECT);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
    if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_AIR) {
        if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG) {
            //0x39fa00(false, true);
        }
    }
    else {
        if WorkModule::is_flag(fighter.module_accessor, *WEAPON_TANTAN_RING_INSTANCE_WORK_ID_FLAG_IS_LONG) {
            sv_animcmd::frame(fighter.lua_state_agent, 11.0);
            if macros::is_excute(fighter) {
                AttackModule::clear_all(fighter.module_accessor);
            }
        }
    }
}


#[acmd_script( agent = "tantan", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn tantan_usmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_A, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        shield!(fighter,*MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_B, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 14.3, 75, 96, 0, 48, 3.0, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 14.3, 75, 96, 0, 48, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 14.3, 75, 96, 0, 48, 3.5, 0.0, 1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_A, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_TANTAN_REFLECTOR_KIND_ATTACK_HI4_B, *FIGHTER_TANTAN_REFLECTOR_GROUP_ATTACK_HI4);
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

#[acmd_script( agent = "tantan", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn tantan_dsmash_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.5, 4.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.5, 4.5);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneel"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("kneer"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("legr"), HitStatus(*HIT_STATUS_XLU), 0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 30, 98, 0, 30, 3.8, 0.0, 2.5, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.0, 30, 98, 0, 30, 3.8, 0.0, 2.5, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.0, 30, 98, 0, 30, 4.3, 0.0, 2.5, 12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 13.0, 30, 98, 0, 30, 4.3, 0.0, 2.5, -12.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
}

//dragon arm nair on (A) button
#[acmd_script( agent = "tantan", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn tantan_nair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handl"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("arml5"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_FLAG_DRAGONIZE_L) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml5"), 11.4, 50, 100, 0, 50, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handl"), 11.4, 50, 100, 0, 50, 5.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("arml5"), 9.1, 50, 95, 0, 40, 2.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handl"), 9.1, 50, 95, 0, 40, 4.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

//nair on (B) button
#[acmd_script( agent = "tantan", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn tantan_nair_neutralb_smash_script(fighter: &mut L2CAgentBase) {
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 1 as u64 {
    //if(0x39fa00(*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R, 1)){;
        sv_animcmd::frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 0.84);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_XLU), 0);
            HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr5"), HitStatus(*HIT_STATUS_XLU), 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr5"), 12.8, 70, 115, 0, 40, 2.9, 1.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 12.8, 70, 115, 0, 40, 5.0, 3.5, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 0.9);
        }
    }
    else if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R) == 2 as u64 {
        //if(0x39fa00(*FIGHTER_TANTAN_INSTANCE_WORK_ID_INT_PUNCH_KIND_R, 2)){;
        sv_animcmd::frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.5);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_XLU), 0);
            HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr5"), HitStatus(*HIT_STATUS_XLU), 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr5"), 6.8, 48, 75, 0, 35, 2.9, 1.0, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 6.8, 48, 75, 0, 35, 4.0, 3.5, 0.0, 0.0, None, None, None, 0.9, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
            AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    else {
        sv_animcmd::frame(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 2.0);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            MotionModule::set_rate(fighter.module_accessor, 1.0);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 10.0);
        if macros::is_excute(fighter) {
            HitModule::set_status_joint(fighter.module_accessor, Hash40::new("handr"), HitStatus(*HIT_STATUS_XLU), 0);
            HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr5"), HitStatus(*HIT_STATUS_XLU), 0);
            macros::ATTACK(fighter, 0, 0, Hash40::new("armr5"), 9.1, 280, 95, 0, 20, 2.9, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("handr"), 9.1, 280, 95, 0, 20, 4.5, 3.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 26.0);
        if macros::is_excute(fighter) {
            HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
            AttackModule::clear_all(fighter.module_accessor);
            MotionModule::set_rate(fighter.module_accessor, 1.2);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn tantan_uair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 9.6, 75, 134, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.6, 70, 134, 0, 30, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 9.6, 70, 134, 0, 30, 4.0, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 5.2, 103, 55, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 5.2, 103, 55, 0, 40, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footl"), 5.2, 103, 55, 0, 40, 4.0, 0.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.6);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "tantan", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn tantan_dair_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_LANDING_CLEAR_SPEED);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 0.0, 0.5, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        KineticModule::suspend_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_GRAVITY_STABLE_UNABLE);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
        macros::SET_SPEED_EX(fighter, 2.9, -3.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_NO_SPEED_OPERATION_CHK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.7, 285, 92, 0, 25, 6.0, 0.0, 2.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.3, 55, 91, 0, 48, 5.7, 0.0, 2.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "tantan", script = "game_landingairlw", category = ACMD_GAME )]
unsafe fn tantan_dair_landing_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "tantan", script = "game_catch", category = ACMD_GAME )]
unsafe fn tantan_grab_start_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

#[acmd_script( agent = "tantan", script = "game_catchstart", category = ACMD_GAME )]
unsafe fn tantan_grab_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.8, 0.0, -1.0, -12.0, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.8, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    macros::game_CaptureCutCommon(fighter);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

#[acmd_script( agent = "tantan", script = "game_catchdash", category = ACMD_GAME )]
unsafe fn tantan_grabd_start_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

#[acmd_script( agent = "tantan", script = "game_catchdashstart", category = ACMD_GAME )]
unsafe fn tantan_grabd_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.8, 0.0, -1.0, -12.0, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.8, 0.0, -1.0, -1.5, Some(0.0), Some(-1.0), Some(-0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    macros::game_CaptureCutCommon(fighter);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

#[acmd_script( agent = "tantan", script = "game_catchturn", category = ACMD_GAME )]
unsafe fn tantan_grabp_start_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

#[acmd_script( agent = "tantan", script = "game_catchturnstart", category = ACMD_GAME )]
unsafe fn tantan_grabp_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.5, 3.5);
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::HIT_NO(fighter, 2, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 3, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 4, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 5, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 6, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 7, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 8, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 9, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 10, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 11, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 12, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 13, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 18, *HIT_STATUS_OFF);
        macros::HIT_NO(fighter, 19, *HIT_STATUS_OFF);
    }
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.8, 0.0, -1.0, -1.0, Some(0.0), Some(-1.0), Some(7.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.8, 0.0, -1.0, 1.4, Some(0.0), Some(-1.0), Some(-2.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    macros::game_CaptureCutCommon(fighter);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 60.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
    }
}

#[acmd_script( agent = "tantan", script = "game_throwf", category = ACMD_GAME )]
unsafe fn tantan_fthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 30, 75, 0, 58, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 3.0, 45, 130, 0, 30, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 3.0, 45, 130, 0, 30, 3.5, 4.0, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 13, 8);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.3);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "tantan", script = "game_throwb", category = ACMD_GAME )]
unsafe fn tantan_bthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.1, 47, 180, 0, 45, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 280, 100, 0, 70, 7.0, 0.0, 2.7, -7.0, Some(0.0), Some(2.7), Some(-13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, -4, 5);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
    }
}

#[acmd_script( agent = "tantan", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn tantan_uthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 73, 68, 0, 65, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("toer"), 4.5, 45, 130, 0, 30, 5.0, 1.0, 1.0, 1.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 15, 10);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
}

#[acmd_script( agent = "tantan", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn tantan_dthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 56, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 0, 130, 0, 30, 4.0, 0.0, 4.0, 0.0, Some(0.0), Some(4.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 3, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "tantan", script = "game_specialairhistart", category = ACMD_GAME )]
unsafe fn tantan_upb_start_air_hitbox_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_TANTAN_STATUS_SPECIAL_HI_FLAG_REVERSE_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        GroundModule::select_cliff_hangdata(fighter.module_accessor, *FIGHTER_TANTAN_CLIFF_HANG_DATA_AIR_LASSO as u32);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_HANG_IMMIDIATE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
}

#[acmd_script( agent = "tantan_punch1", script = "game_specialairhiattack", category = ACMD_GAME )]
unsafe fn tantan_upb_air_hitbox_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 10.5, 45, 106, 0, 56, 4.5, 3.9, 0.0, 0.3, Some(-6.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 10.5, 45, 106, 0, 56, 4.5, 3.9, 0.0, 0.3, Some(0.0), Some(0.0), Some(0.3), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "tantan_punch1", script = "game_specialairhiattackdragon", category = ACMD_GAME )]
unsafe fn tantan_upb_air_hitbox_dragon_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 13.9, 72, 94, 0, 66, 5.5, 4.8, 0.0, 0.4, Some(-6.0), Some(0.0), Some(0.4), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("have"), 13.9, 72, 94, 0, 66, 5.5, 4.8, 0.0, 0.4, Some(0.0), Some(0.0), Some(0.4), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_TANTAN_PUNCH01, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        tantan_frame
    );
    smashline::install_acmd_scripts!(
        tantan_jab_smash_script,
        tantan_jab2_smash_script,
        tantan_jab3_smash_script,
        tantan_jab100_smash_script,
        tantan_jab100end_smash_script,
        tantan_dashattack_smash_script,
        tantan_utilt_smash_script,
        tantan_dtilt_smash_script,
        ftilt_startup_1_left,
        ftilt_startup_1_left_back,
        ftilt_startup_2_left,
        ftilt_startup_2_left_back,
        ftilt_startup_3_left,
        ftilt_startup_3_left_back,
        ftilt_startup_1_right,
        ftilt_startup_1_right_back,
        ftilt_startup_2_right,
        ftilt_startup_2_right_back,
        ftilt_startup_3_right,
        ftilt_startup_3_right_back,
        dragon_arm_ftilt,
        dragon_power_arm_ftilt,
        megawatt_arm_ftilt,
        ramram_arm_ftilt,
        ramram_arm_projectile,
        dragon_arm_fsmash,
        dragon_power_arm_fsmash,
        megawatt_arm_fsmash,
        megawatt_arm_fsmash_hold,
        ramram_arm_fsmash,
        ramram_arm_fsmash_hold,
        ramram_arm_projectile_fsmash,
        tantan_usmash_smash_script,
        tantan_dsmash_smash_script,
        tantan_nair_smash_script,
        tantan_nair_neutralb_smash_script,
        tantan_uair_smash_script,
        tantan_dair_smash_script,
        tantan_dair_landing_smash_script,
        tantan_grab_start_smash_script,
        tantan_grab_smash_script,
        tantan_grabd_start_smash_script,
        tantan_grabd_smash_script,
        tantan_grabp_start_smash_script,
        tantan_grabp_smash_script,
        tantan_fthrow_smash_script,
        tantan_bthrow_smash_script,
        tantan_uthrow_smash_script,
        tantan_dthrow_smash_script,
        tantan_upb_start_air_hitbox_script,
        tantan_upb_air_hitbox_script,
        tantan_upb_air_hitbox_dragon_script
        
    );
}
