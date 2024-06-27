#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use crate::custom::global_fighter_frame;
use smashline::Priority::*;

static mut INKLING_UPSPECIAL_ISFALLING : [bool; 8] = [false; 8];
static mut INKLING_UPSPECIAL_FALLCHARGE : [f32; 8] = [1.0; 8];
static mut INKLING_UPSPECIAL_DIDFASTFALL : [bool; 8] = [false; 8];


static mut INKLING_AIR_INK : [bool ; 8] = [false; 8]; 

extern "C" {
    #[link_name = "\u{1}_ZN3app26FighterSpecializer_Inkling10change_inkERNS_7FighterEf"]
    pub fn FighterSpecializer_Inkling__change_ink(fighter: *mut smash::app::Fighter, change: f32);
}

// A Once-Per-Fighter-Frame that only applies to Inkling
unsafe extern "C" fn inkling_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        global_fighter_frame(fighter);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);
        let sticky = ControlModule::get_stick_y(fighter.module_accessor);


        //println!("It'sa me, Inkling, Woomy/Ngyes!");


        if status == *FIGHTER_INKLING_STATUS_KIND_SPECIAL_HI_FALL {
            if sticky <= -0.5 {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
            if INKLING_UPSPECIAL_ISFALLING[entry_id] == false {
                INKLING_UPSPECIAL_ISFALLING[entry_id] = true;
                INKLING_UPSPECIAL_DIDFASTFALL[entry_id] = false;
                INKLING_UPSPECIAL_FALLCHARGE[entry_id] = 1.0;
                //KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
            }
            else {
                //KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 0.0, y: -0.1, z: 0.0});
                INKLING_UPSPECIAL_FALLCHARGE[entry_id] += 0.1;
                if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE) {
                    INKLING_UPSPECIAL_DIDFASTFALL[entry_id] = true;
                }
            }
        }
        else
        {
            if INKLING_UPSPECIAL_ISFALLING[entry_id] {
                INKLING_UPSPECIAL_ISFALLING[entry_id] = false;
            }
        }

        if [*FIGHTER_STATUS_KIND_ATTACK_LW3].contains(&status) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false {
                macros::SET_SPEED_EX(fighter, 0, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }

        //ink consumption for Bair
        if INKLING_AIR_INK[entry_id] {
            let curr = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK);
            let fighter_ptr = fighter.global_table[0x4].get_ptr() as *mut smash::app::Fighter;
            FighterSpecializer_Inkling__change_ink(fighter_ptr, curr - 20.0);
            INKLING_AIR_INK[entry_id] = false;
        }


    }
}

unsafe fn inkling_generate_squid_helper(fighter: &mut L2CAgentBase) {
    if !WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID) {
        if macros::is_excute(fighter) {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, -1);
        }
    }
    let motion_kind = MotionModule::motion_kind(fighter.module_accessor);
    let frame = MotionModule::frame(fighter.module_accessor);
    let rate = MotionModule::rate(fighter.module_accessor);
    ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new_raw(motion_kind), false, -1.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_frame(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, frame);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, rate);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_EXIST_SQUID);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        if status_kind != *FIGHTER_STATUS_KIND_REBIRTH {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_FORCE_LOUPE);
        }
    }
}

unsafe extern "C" fn inkling_jab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.3, 361, 30, 15, 20, 3.5, 0.0, 6.7, 6.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.3, 361, 25, 15, 20, 3.5, 0.0, 6.7, 8.8, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.3, 180, 20, 15, 15, 3.8, 0.0, 6.7, 12.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.3, 361, 20, 10, 10, 3.8, 0.0, 6.7, 12.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

unsafe extern "C" fn inkling_jab2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.4, 361, 26, 14, 19, 4.5, 0.0, 6.5, 5.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.4, 180, 25, 12, 18, 4.5, 0.0, 6.5, 9.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("footr"), 2.4, 180, 20, 14, 20, 5.0, 1.0, -2.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
}

unsafe extern "C" fn inkling_jab3_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 24, 90, 0, 70, 4.2, 0.0, 7.0, 6.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 24, 90, 0, 70, 4.2, 0.0, 7.0, 10.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 24, 90, 0, 70, 4.8, 0.0, 7.0, 15.8, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn inkling_jab100_smash_script(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 8, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 8, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 6.0);
                AttackModule::set_ink_value(fighter.module_accessor, 0, 12.0);
                AttackModule::set_ink_value(fighter.module_accessor, 1, 12.0);
            }
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 2.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 8, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 8, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 6.0);
                AttackModule::set_ink_value(fighter.module_accessor, 0, 12.0);
                AttackModule::set_ink_value(fighter.module_accessor, 1, 12.0);
            }
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 4.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 8, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 8, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 6.0);
                AttackModule::set_ink_value(fighter.module_accessor, 0, 12.0);
                AttackModule::set_ink_value(fighter.module_accessor, 1, 12.0);
            }
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 6.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 8, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 8, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 6.0);
                AttackModule::set_ink_value(fighter.module_accessor, 0, 12.0);
                AttackModule::set_ink_value(fighter.module_accessor, 1, 12.0);
            }
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 8.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 8, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 8, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 6.0);
                AttackModule::set_ink_value(fighter.module_accessor, 0, 12.0);
                AttackModule::set_ink_value(fighter.module_accessor, 1, 12.0);
            }
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 10.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 8, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 8, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 6.0);
                AttackModule::set_ink_value(fighter.module_accessor, 0, 12.0);
                AttackModule::set_ink_value(fighter.module_accessor, 1, 12.0);
            }
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 12.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 8, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 8, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 6.0);
                AttackModule::set_ink_value(fighter.module_accessor, 0, 12.0);
                AttackModule::set_ink_value(fighter.module_accessor, 1, 12.0);
            }
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 14.0);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.5, 361, 8, 0, 12, 3.5, 0.0, 8.0, 11.0, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 0.5, 361, 8, 0, 7, 4.5, 0.0, 8.0, 19.5, Some(0.0), Some(8.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_WATER);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
                AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
                macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 6.0);
                AttackModule::set_ink_value(fighter.module_accessor, 0, 12.0);
                AttackModule::set_ink_value(fighter.module_accessor, 1, 12.0);
            }
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 16.0);
        fighter.clear_lua_stack();
        lua_args!(fighter, 0);
        sv_animcmd::wait_loop_clear(fighter.lua_state_agent);
    }
}

unsafe extern "C" fn inkling_jab100end_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 86, 0, 60, 5.0, 0.0, 8.0, 12.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 361, 86, 0, 60, 5.0, 0.0, 8.0, 16.5, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 361, 86, 0, 60, 5.0, 0.0, 8.0, 21.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 10.0);
            AttackModule::set_ink_value(fighter.module_accessor, 1, 10.0);
            AttackModule::set_ink_value(fighter.module_accessor, 2, 10.0);
        }   
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn inkling_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    inkling_generate_squid_helper(fighter);
    if sv_animcmd::get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("attack_dash_l"), false, 0.0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("attack_dash_r"), false, 0.0);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, smash::app::ArticleOperationTarget(0));
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.8, 70, 43, 0, 69, 5.0, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 7, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_ELBOW);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 1.1);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

unsafe extern "C" fn inkling_ftilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("handr"), 8.7, 280, 89, 0, 60, 3.9, 1.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 3, 0, Hash40::new("handr"), 8.7, 280, 89, 0, 60, 3.9, 6.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 4, 0, Hash40::new("handr"), 8.7, 280, 89, 0, 20, 3.9, 1.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 5, 0, Hash40::new("handr"), 8.7, 280, 89, 0, 20, 3.9, 6.0, 0.0, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.7, 280, 89, 0, 60, 3.9, 0.0, 6.0, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.7, 280, 89, 0, 20, 3.9, 0.0, 6.0, 6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
}

unsafe extern "C" fn inkling_utilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legl"), 8.8, 101, 82, 0, 50, 4.4, 8.0, 0.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 8.8, 101, 82, 0, 50, 4.4, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("legl"), 8.8, 101, 82, 0, 50, 5.1, 6.0, 0.0, -5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 2, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.23);
    }
}

unsafe extern "C" fn inkling_dtilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 3.2, 50, 100, 20, 0, 3.0, 0.0, -1.0, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 3.2, 160, 100, 25, 0, 4.0, 5.5, -1.0, 0.0, None, None, None, 0.7, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.3, 64, 84, 0, 42, 4.4, 0.0, 1.0, 1.0, None, None, None, 1.0, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.3, 64, 84, 0, 42, 4.8, 4.0, 0.0, 0.0, None, None, None, 1.0, 0.6, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn inkling_fsmash_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, Hash40::new("attack_s4_s"), false, -1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    sv_animcmd::execute(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if macros::is_excute(fighter) {
            //WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, Hash40::new("attack_s4_s"), true, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME);
            //methodlib::L2CValue::as_hash()const(-718230741);
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_BRUSH, Hash40::new("attack_s4_s"), true, WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME));
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.8, 361, 97, 0, 47, 3.3, 0.0, 6.0, 9.0, Some(0.0), Some(6.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 17.4, 361, 97, 0, 47, 4.8, 0.0, 6.0, 17.2, Some(0.0), Some(6.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 90.0);
            AttackModule::set_ink_value(fighter.module_accessor, 1, 135.0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.1, 361, 93, 0, 42, 3.3, 0.0, 6.0, 12.0, Some(0.0), Some(6.0), Some(5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_OBJECT);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.3, 361, 93, 0, 42, 4.8, 0.0, 6.0, 17.2, Some(0.0), Some(6.0), Some(16.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_OBJECT);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 30.0);
            AttackModule::set_ink_value(fighter.module_accessor, 1, 30.0);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//something wrong with usmash, doesn't use ink at first and loads no-ink hitboxes at the start of the match when no ink is used
unsafe extern "C" fn inkling_usmash_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, Hash40::new("attack_hi4"), false, -1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    sv_animcmd::execute(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_BLASTER, Hash40::new("attack_hi4"), true, WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME));
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 125, 100, 100, 0, 5.0, 0.0, 4.0, 9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 125, 100, 100, 0, 5.0, 0.0, 4.0, -9.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.4, 88, 82, 0, 55, 10.0, 0.0, 40.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.7, 88, 82, 0, 55, 14.5, 0.0, 40.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 18.4, 88, 82, 0, 55, 6.5, 0.0, 30.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 18.4, 88, 82, 0, 55, 6.5, 0.0, 20.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 120.0);
            AttackModule::set_ink_value(fighter.module_accessor, 1, 100.0);
            AttackModule::set_ink_value(fighter.module_accessor, 2, 100.0);
            AttackModule::set_ink_value(fighter.module_accessor, 3, 100.0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 88, 66, 0, 55, 8.5, 0.0, 20.0, 0.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 88, 66, 0, 55, 12.5, 0.0, 20.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 30.0);
            AttackModule::set_ink_value(fighter.module_accessor, 1, 30.0);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        //MotionModule::set_rate(fighter.module_accessor, 1.2);
        macros::FT_MOTION_RATE(fighter, 0.7692307);
    }
}

unsafe extern "C" fn inkling_usmash_effect_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 12.5, -3.5, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        sv_animcmd::frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            //macros::EFFECT(fighter, Hash40::new("inkling_blaster_bullet"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            //macros::EFFECT(fighter, Hash40::new("inkling_blaster_bullet"), Hash40::new("top"), 0, 30, 0, 0, 0, 0, 0.7, 0, 0, 0, 0, 0, 0, true);
            macros::EFFECT(fighter, Hash40::new("inkling_blaster_bullet"), Hash40::new("top"), 0, 40, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        }
    }
    if macros::is_excute(fighter) {
        macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
    }
    else {
        sv_animcmd::frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::LANDING_EFFECT(fighter, Hash40::new("sys_v_smoke_a"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("inkling_blaster_bullet2"), Hash40::new("top"), 0, 20, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
        }
        if macros::is_excute(fighter) {
            macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
        }
    }
}

unsafe extern "C" fn inkling_dsmash_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SLOSHER, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SLOSHER, Hash40::new("attack_lw4"), false, -1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    sv_animcmd::execute(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_SMASH_SMASH_HOLD_TO_ATTACK) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SLOSHER, Hash40::new("attack_lw4"), true, WorkModule::get_float(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_WORK_FLOAT_SMASH_RESTART_FRAME));
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.4, 32, 89, 0, 46, 3.8, 0.0, 3.5, 12.0, Some(0.0), Some(3.5), Some(20.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 150.0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.6, 32, 80, 0, 35, 3.8, 0.0, 3.5, 12.0, Some(0.0), Some(3.5), Some(20.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 35.0);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.4, 32, 89, 0, 46, 4.5, 0.0, 4.0, 12.0, Some(0.0), Some(4.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 125.0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.6, 32, 80, 0, 35, 4.5, 0.0, 4.0, 12.0, Some(0.0), Some(4.0), Some(24.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 35.0);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.9, 30, 82, 0, 40, 3.8, 0.0, 3.5, -12.0, Some(0.0), Some(3.5), Some(-19.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 175.0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.5, 30, 80, 0, 35, 3.8, 0.0, 3.5, -12.0, Some(0.0), Some(3.5), Some(-19.0), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 35.0);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.9, 30, 82, 0, 40, 4.5, 0.0, 4.0, -12.0, Some(0.0), Some(4.0), Some(-23.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 150.0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.5, 30, 80, 0, 35, 4.5, 0.0, 4.0, -12.0, Some(0.0), Some(4.0), Some(-23.0), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_WATER);
            AttackModule::set_ink_value(fighter.module_accessor, 0, 35.0);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        //MotionModule::set_rate(fighter.module_accessor, 1.333);
        macros::FT_MOTION_RATE(fighter, 0.7501875); 
    }
}

unsafe extern "C" fn inkling_nair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.4, 367, 85, 30, 35, 3.5, 0.0, 9.0, 4.7, Some(0.0), Some(9.0), Some(-4.7), 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 4.4, 367, 85, 30, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("legr"), 4.4, 367, 85, 30, 35, 4.0, 6.0, 0.0, 0.0, None, None, None, 0.7, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.9, 60, 89, 0, 39, 4.8, 0.0, 9.0, 7.7, Some(0.0), Some(9.0), Some(-7.7), 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.65);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn inkling_fair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.2, 42, 90, 0, 36, 3.6, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.2, 42, 90, 0, 36, 6.0, 0.0, 6.5, 12.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.5, 15, 107, 0, 29, 3.2, 0.0, 7.0, 4.0, Some(0.0), Some(7.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.5, 15, 107, 0, 29, 5.5, 0.0, 6.5, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn inkling_bair_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        INKLING_AIR_INK[entry_id] = true;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.3, 361, 110, 0, 24, 5.0, 0.0, 3.0, -16.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.5, 361, 110, 0, 24, 3.5, 0.0, 4.7, -9.0, Some(0.0), Some(4.0), Some(-11.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        if WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK) > 0.0 {
            macros::PLAY_SE(fighter, Hash40::new("se_inkling_attack100_02"));
            macros::EFFECT_FOLLOW(fighter, Hash40::new("inkling_muzzle_shot"), Hash40::new("muzzle"), -1, 0, 0, 0, 180, 0, 1, true);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 12.0, 361, 110, 0, 24, 6.5, 0.0, -0.9, -26.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_OBJECT);
            macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
            AttackModule::set_ink_value(fighter.module_accessor, 0, 45.0);
            AttackModule::set_ink_value(fighter.module_accessor, 1, 45.0);
            AttackModule::set_ink_value(fighter.module_accessor, 2, 90.0);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn inkling_uair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 4.5, 367, 20, 40, 40, 4.6, -1.0, -0.5, -0.8, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 4.5, 367, 20, 40, 40, 4.4, 6.2, -0.5, -0.8, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 4.5, 367, 20, 40, 40, 3.5, 2.7, -6.0, 1.9, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 8.5, 80, 140, 0, 30, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 8.5, 80, 140, 0, 30, 4.5, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn inkling_dair_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        INKLING_AIR_INK[entry_id] = true;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.8, 361, 81, 0, 60, 3.7, 0.0, 3.5, 0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.8, 270, 81, 0, 30, 5.2, 0.0, -2.0, 1.0, Some(0.0), Some(-4.0), Some(1.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        if WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK) > 0.0 {
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 14.8, 270, 81, 0, 30, 6.0, 0.0, -10.0, 1.0, Some(0.0), Some(-13.0), Some(1.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_INKLING_HIT, *ATTACK_REGION_OBJECT);
            macros::PLAY_SE(fighter, Hash40::new("se_inkling_attack100_02"));
            macros::EFFECT_FOLLOW(fighter, Hash40::new("inkling_muzzle_shot"), Hash40::new("muzzle"), -1, 0, 0, 0, 180, 0, 1.2, true);
            macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
            AttackModule::set_ink_value(fighter.module_accessor, 0, 60.0);
            AttackModule::set_ink_value(fighter.module_accessor, 1, 60.0);
            AttackModule::set_ink_value(fighter.module_accessor, 2, 120.0);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.65);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn inkling_grab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.2, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(9.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn inkling_grabd_smash_script(fighter: &mut L2CAgentBase) {
    inkling_generate_squid_helper(fighter);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }
    if sv_animcmd::get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("catch_dash_l"), false, 0.0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("catch_dash"), false, 0.0);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, smash::app::ArticleOperationTarget(0));
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(12.6), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn inkling_grabp_smash_script(fighter: &mut L2CAgentBase) {
    inkling_generate_squid_helper(fighter);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }
    if sv_animcmd::get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("catch_turn_l"), false, 0.0);
        }
    }
    else {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("catch_turn"), false, 0.0);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, smash::app::ArticleOperationTarget(0));
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.2, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-17.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn inkling_fthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) { 
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 50, 90, 0, 100, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK) >= 50.0 {
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_INK_SUCCESS) {
            if macros::is_excute(fighter) { 
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.8, 361, 100, 0, 60, 5.5, 0.0, 13.0, 8.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_WATER);
                AttackModule::set_ink_value(fighter.module_accessor, 0, 200.0);
                AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
            }
        }
    }
    if macros::is_excute(fighter) { 
        macros::CHECK_FINISH_CAMERA(fighter, 10, 1);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 3, 3, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) { 
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn inkling_bthrow_smash_script(fighter: &mut L2CAgentBase) {
    inkling_generate_squid_helper(fighter);
    if macros::is_excute(fighter) { 
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }
    if sv_animcmd::get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) { 
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("throw_b_l"), false, 0.0);
        }
    }
    else {
        if macros::is_excute(fighter) { 
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("throw_b_r"), false, 0.0);
        }
    }
    if macros::is_excute(fighter) { 
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.4, 45, 112, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) { 
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, smash::app::ArticleOperationTarget(0));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) { 
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) { 
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(fighter, 17, 5);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 5, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) { 
        PostureModule::reverse_lr(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) { 
        VisibilityModule::set_whole(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) { 
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn inkling_uthrow_smash_script(fighter: &mut L2CAgentBase) {
    inkling_generate_squid_helper(fighter);
    if macros::is_excute(fighter) { 
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLAG_NO_FLIP_SQUID);
    }   
    if sv_animcmd::get_value_float(fighter.lua_state_agent, *SO_VAR_FLOAT_LR) < 0.0 {
        if macros::is_excute(fighter) { 
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("throw_hi_l"), false, 0.0);
        }
    }
    else {
        if macros::is_excute(fighter) { 
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, Hash40::new("throw_hi_r"), false, 0.0);
        }
    }
    if macros::is_excute(fighter) { 
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 90, 75, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) { 
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, true, smash::app::ArticleOperationTarget(0));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) { 
        VisibilityModule::set_whole(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) { 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 30, 0, 80, 5.5, 0.0, 18.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 1, 25);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.7);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 0, 10, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) { 
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) { 
        VisibilityModule::set_whole(fighter.module_accessor, true);
        CancelModule::enable_cancel(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) { 
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn inkling_dthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) { 
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.2, 60, 90, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) { 
        fighter.clear_lua_stack();
        lua_args!(fighter, 7, 1);
        sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
        fighter.clear_lua_stack();
        macros::CHECK_FINISH_CAMERA(fighter, 7, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
        //FighterCutInManager::set_throw_finish_offset(fighter.module_accessor, 3, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) { 
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) { 
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn inkling_sideb_start_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) { 
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 3.0, 8.0);
        macros::FT_MOTION_RATE(fighter, 0.66666666667);
    }
}

unsafe extern "C" fn roller_dash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 10.5, 52, 90, 0, 45, 4.1, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("neck"), 10.5, 52, 90, 0, 80, 3.5, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, 0, 120.0);
        AttackModule::set_ink_value(fighter.module_accessor, 1, 120.0);
    }
}

unsafe extern "C" fn roller_dash_air(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("roll"), 14.0, 280, 75, 0, 20, 4.1, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, 0, 120.0);
    }
}

unsafe extern "C" fn roller_walk(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 6.5, 60, 80, 0, 60, 4.1, 0.5, 3.8, 0.0, Some(0.5), Some(3.8), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("neck"), 6.5, 80, 40, 0, 60, 3.5, 0.5, 3.8, 0.0, Some(0.5), Some(3.8), Some(-2.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, 0, 60.0);
        AttackModule::set_ink_value(fighter.module_accessor, 1, 60.0);
    }
}

unsafe extern "C" fn roller_walk_air(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("roll"), 6.5, 280, 85, 0, 20, 3.6, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, 0, 60.0);
    }
}

unsafe extern "C" fn roller_run(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 12.8, 60, 105, 0, 50, 3.6, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_G_d, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        macros::ATTACK(fighter, 1, 0, Hash40::new("neck"), 12.8, 80, 40, 0, 100, 3.0, 0.5, 3.8, 5.0, Some(0.5), Some(3.8), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, 0, 100.0);
        AttackModule::set_ink_value(fighter.module_accessor, 1, 100.0);
    }
}

unsafe extern "C" fn roller_run_air(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("roll"), 12.8, 280, 75, 0, 20, 3.6, 0.0, 0.0, 5.0, Some(0.0), Some(0.0), Some(-5.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 35, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_ink_value(fighter.module_accessor, 0, 100.0);
    }
}

unsafe extern "C" fn upb_splash(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 361, 25, 0, 70, 6.0, 0.0, 2.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
        AttackModule::set_ink_value(fighter.module_accessor, 0, 50.0);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.0, 361, 25, 0, 70, 6.0, 0.0, 2.0, -7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.0, 361, 25, 0, 70, 6.0, 0.0, 2.0, 7.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_WATER);
        AttackModule::set_ink_value(fighter.module_accessor, 0, 40.0);
        AttackModule::set_ink_value(fighter.module_accessor, 1, 40.0);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 2.0);
        macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 1, 2.0);
    }
}

unsafe extern "C" fn inkling_upspecial_landing_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        VisibilityModule::set_whole(fighter.module_accessor, true);
        if INKLING_UPSPECIAL_DIDFASTFALL[entry_id] {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASH, false, 0);
        }
        else {
            macros::EFFECT(fighter, Hash40::new("inkling_splashbomb_explosion"), Hash40::new("top"), 0, 3.0, 0, 0, 0, 0, 0.5 * INKLING_UPSPECIAL_FALLCHARGE[entry_id], 0, 0, 0, 0, 0, 0, false);
            macros::LAST_PARTICLE_SET_COLOR(fighter, WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_R), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_G), WorkModule::get_float(fighter.module_accessor, *FIGHTER_INKLING_INSTANCE_WORK_ID_FLOAT_INK_B));
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 0.1);
    if macros::is_excute(fighter) {
        if INKLING_UPSPECIAL_DIDFASTFALL[entry_id] == false {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5 * INKLING_UPSPECIAL_FALLCHARGE[entry_id], 361, 80, 0, 70, 8.0 * INKLING_UPSPECIAL_FALLCHARGE[entry_id], 0.0, 3.0, 0.0, None, None, None, 0.4 + 0.3 * INKLING_UPSPECIAL_FALLCHARGE[entry_id], 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, -9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_BOMB);  
            macros::ATK_SET_SHIELD_SETOFF_MUL(fighter, 0, 0.2);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SQUID, false, smash::app::ArticleOperationTarget(0));
    }
}

unsafe extern "C" fn inkling_downb_start_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.370370370370);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_TO_THROW_OK);
    }
}

unsafe extern "C" fn inkling_downb_start_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.370370370370);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_TO_THROW_OK);
    }
}

unsafe extern "C" fn inkling_downb_min_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.75);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASHBOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_THROW);
    }
}

unsafe extern "C" fn inkling_downb_min_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.75);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASHBOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_THROW);
    }
}

unsafe extern "C" fn inkling_downb_middle_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.75);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASHBOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_THROW);
    }
}

unsafe extern "C" fn inkling_downb_middle_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.75);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASHBOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_THROW);
    }
}

unsafe extern "C" fn inkling_downb_max_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.75);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASHBOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_THROW);
    }
}

unsafe extern "C" fn inkling_downb_max_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.75);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_INKLING_GENERATE_ARTICLE_SPLASHBOMB, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        WorkModule::on_flag(fighter.module_accessor,  *FIGHTER_INKLING_STATUS_SPECIAL_LW_FLAG_THROW);
    }
}

unsafe extern "C" fn splatbomb_thrown(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 0, 0, 0, 3.5, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 2.8, 0.0, 0.0, 1.5, None, None, None, *COLLISION_KIND_MASK_HSR, *HIT_STATUS_MASK_NI, 180, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIEB, *COLLISION_PART_MASK_ALL, false);
    }
}

unsafe extern "C" fn splatbomb_explode(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) { 
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 81, 85, 0, 60, 13.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ink_hit"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_WATER, *ATTACK_REGION_BOMB);
        AttackModule::set_ink_value(fighter.module_accessor, 0, 0.0);
        macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) { 
        AttackModule::set_size(fighter.module_accessor, 0, 15.0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) { 
        AttackModule::set_size(fighter.module_accessor, 0, 22.0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) { 
        AttackModule::clear_all(fighter.module_accessor);
    }
}


//#[skyline::main(name = "tr4sh_rebuffed")]
pub fn install() {
    Agent::new("inkling")
      .on_line(Main, inkling_frame) //opff
      .game_acmd("game_attack11", inkling_jab_smash_script, Low)
      .game_acmd("game_attack12", inkling_jab2_smash_script, Low)
      .game_acmd("game_attack13", inkling_jab3_smash_script, Low)
      .game_acmd("game_attack100", inkling_jab100_smash_script, Low)
      .game_acmd("game_attack100end", inkling_jab100end_smash_script, Low)
      .game_acmd("game_attackdash", inkling_dashattack_smash_script, Low)
      .game_acmd("game_attacks3", inkling_ftilt_smash_script, Low)
      .game_acmd("game_attackhi3", inkling_utilt_smash_script, Low)
      .game_acmd("game_attacklw3", inkling_dtilt_smash_script, Low)
      .game_acmd("game_attacks4", inkling_fsmash_smash_script, Low)
      .game_acmd("game_attackhi4", inkling_usmash_smash_script, Low)
      .effect_acmd("effect_attackhi4", inkling_usmash_effect_script, Low)
      .game_acmd("game_attacklw4", inkling_dsmash_smash_script, Low)
      .game_acmd("game_attackairn", inkling_nair_smash_script, Low)
      .game_acmd("game_attackairf", inkling_fair_smash_script, Low)
      .game_acmd("game_attackairb", inkling_bair_smash_script, Low)
      .game_acmd("game_attackairhi", inkling_uair_smash_script, Low)
      .game_acmd("game_attackairlw", inkling_dair_smash_script, Low)
      .game_acmd("game_catch", inkling_grab_smash_script, Low)
      .game_acmd("game_catchdash", inkling_grabd_smash_script, Low)
      .game_acmd("game_catchturn", inkling_grabp_smash_script, Low)
      .game_acmd("game_throwf", inkling_fthrow_smash_script, Low)
      .game_acmd("game_throwb", inkling_bthrow_smash_script, Low)
      .game_acmd("game_throwhi", inkling_uthrow_smash_script, Low)
      .game_acmd("game_throwlw", inkling_dthrow_smash_script, Low)
      .game_acmd("game_specialsstart", inkling_sideb_start_smash_script, Low)
      .game_acmd("game_specialairsstart", inkling_sideb_start_smash_script, Low)
      .game_acmd("game_specialhilanding", inkling_upspecial_landing_script, Low)
      .game_acmd("game_speciallwstart", inkling_downb_start_smash_script, Low)
      .game_acmd("game_specialairlwstart", inkling_downb_start_air_smash_script, Low)
      .game_acmd("game_speciallwmin", inkling_downb_min_smash_script, Low)
      .game_acmd("game_specialairlwmin", inkling_downb_min_air_smash_script, Low)
      .game_acmd("game_speciallwmiddle", inkling_downb_middle_smash_script, Low)
      .game_acmd("game_specialairlwmiddle", inkling_downb_middle_air_smash_script, Low)
      .game_acmd("game_speciallwmax", inkling_downb_max_smash_script, Low)
      .game_acmd("game_specialairlwmax", inkling_downb_max_air_smash_script, Low)
      .install();
  

      Agent::new("inkling_roller")
      .game_acmd("game_specialswalk", roller_walk, Low)
      .game_acmd("game_specialairswalk", roller_walk_air, Low)
      .game_acmd("game_specialsrun", roller_run, Low)
      .game_acmd("game_specialairsrun", roller_run_air, Low)
      .game_acmd("game_specialsdash", roller_dash, Low)
      .game_acmd("game_specialairsdash", roller_dash_air, Low)
      .install();
  
      Agent::new("inkling_splash")
      .game_acmd("game_normal", upb_splash, Low)
      .install();

      Agent::new("inkling_splashbomb")
      .game_acmd("game_loop", splatbomb_thrown, Low)
      .game_acmd("game_explode", splatbomb_explode, Low)
      .install();
}