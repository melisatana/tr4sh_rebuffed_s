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

static mut MEWTWO_HAS_FUTURESIGHT : [bool; 8] = [false; 8];
static mut MEWTWO_FUTURESIGHT_X : [f32; 8] = [0.0; 8];
static mut MEWTWO_FUTURESIGHT_Y : [f32; 8] = [0.0; 8];
static mut MEWTWO_FUTURESIGHT_CURRENT_FRAME : [i32; 8] = [0; 8];
static mut MEWTWO_FUTURESIGHT_HIT_COOLDOWN_FRAME : [i32; 8] = [0; 8];
static mut MEWTWO_FUTURESIGHT_LAST_STATUS : [i32; 8] = [0; 8];
static MEWTWO_FUTURESIGHT_FUSE_TIME : i32 = 600;
static MEWTWO_FUTURESIGHT_EXPLOSION_TIME : i32 = 40;
static MEWTWO_FUTURESIGHT_HIT_COOLDOWN_TIME : i32 = 3;

pub static mut MEWTWO_NEUTRALB_UP_ANGLE : [bool; 8] = [false; 8];
pub static mut MEWTWO_NEUTRALB_DOWN_ANGLE : [bool; 8] = [false; 8];

// A Once-Per-Fighter-Frame that only applies to Mewtwo
unsafe extern "C" fn mewtwo_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        global_fighter_frame(fighter);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);
        let point_offset_x = PostureModule::lr(fighter.module_accessor) * (MEWTWO_FUTURESIGHT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor));
        let point_offset_y = MEWTWO_FUTURESIGHT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor);
        let stickx = ControlModule::get_stick_x(fighter.module_accessor);
        let sticky = ControlModule::get_stick_y(fighter.module_accessor);
        let lr = PostureModule::lr(fighter.module_accessor);
        let stickx_directional = stickx * lr;

        //println!("It'sa me, Mewtwo, bwbwbwbwbwwbqwbwbwb!!");


        if [*FIGHTER_STATUS_KIND_ATTACK_S4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_HI4_HOLD, *FIGHTER_STATUS_KIND_ATTACK_LW4_HOLD].contains(&status) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
            JostleModule::set_status(fighter.module_accessor, false);
            if stickx_directional >= 0.5 {
                KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 0.2, y: 0.0, z: 0.0});
                if KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * lr > 0.9 {
                    macros::SET_SPEED_EX(fighter, 0.9, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            }
            else if stickx_directional <= -0.5 {
                KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: -0.2, y: 0.0, z: 0.0});
                if KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN) * lr < -0.9 {
                    macros::SET_SPEED_EX(fighter, -0.9, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                }
            }
        }


        if [*FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT].contains(&status) {
            if sticky >= 0.5 {
                MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = true;
                MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = false;
            }
            if sticky <= -0.5 {
                MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = true;
                MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = false;
            }
            if stickx >= 0.5 || stickx <= -0.5 || MotionModule::frame(fighter.module_accessor) >= (7.0) {
                MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = false;
                MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = false;
            }
        }

        if status != *FIGHTER_MEWTWO_STATUS_KIND_SPECIAL_N_SHOOT {
            MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = false;
            MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = false;
        }

        if sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status) {
			MEWTWO_HAS_FUTURESIGHT[entry_id] = false;
            MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = false;
            MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = false;
        
		};

        if MEWTWO_HAS_FUTURESIGHT[entry_id] {
            MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] += 1;
            
            if MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] <= MEWTWO_FUTURESIGHT_FUSE_TIME {
                if MEWTWO_FUTURESIGHT_FUSE_TIME - MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] == 120 || MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] == 60 {
                    macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
                }
                else if MEWTWO_FUTURESIGHT_FUSE_TIME - MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] == 60 {
                    macros::PLAY_STATUS(fighter, Hash40::new("se_mewtwo_appeal_l01"));
                }
                else if MEWTWO_FUTURESIGHT_FUSE_TIME - MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] < 30 && MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] % 3 == 0 {
                    macros::EFFECT(fighter, Hash40::new("sys_flash"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.5, 5, 5, 0, 0, 0, 0, false);
                }
                MEWTWO_FUTURESIGHT_LAST_STATUS[entry_id] = status;
            }
            else if MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] <= MEWTWO_FUTURESIGHT_FUSE_TIME + MEWTWO_FUTURESIGHT_EXPLOSION_TIME {
                if MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] == MEWTWO_FUTURESIGHT_FUSE_TIME + 1 {
                    macros::PLAY_STATUS(fighter, Hash40::new("se_mewtwo_appeal_l01_02"));
                }
                if status != MEWTWO_FUTURESIGHT_LAST_STATUS[entry_id] || [*FIGHTER_STATUS_KIND_CAPTURE_CUT, *FIGHTER_STATUS_KIND_CATCH_CUT, *FIGHTER_STATUS_KIND_CATCH_TURN, *FIGHTER_STATUS_KIND_TURN, *FIGHTER_STATUS_KIND_TURN_DASH, *FIGHTER_STATUS_KIND_TURN_RUN, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status) {
                    MEWTWO_FUTURESIGHT_HIT_COOLDOWN_FRAME[entry_id] = MEWTWO_FUTURESIGHT_HIT_COOLDOWN_TIME;
                }
                if MEWTWO_FUTURESIGHT_HIT_COOLDOWN_FRAME[entry_id] > 0 {
                    MEWTWO_FUTURESIGHT_HIT_COOLDOWN_FRAME[entry_id] -= 1;
                }
                else {
                    macros::ATTACK(fighter, 7, 1, Hash40::new("top"), 32.0, 361, 60, 0, 85, 15.0, 0.0, point_offset_y, point_offset_x, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_NONE);
                }
                if (MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] - MEWTWO_FUTURESIGHT_FUSE_TIME) % 3 == 0 {
                    macros::EFFECT(fighter, Hash40::new("sys_hit_purple"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
                    macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
                }
                MEWTWO_FUTURESIGHT_LAST_STATUS[entry_id] = status;
            }
            else
            {
                MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] = 0;
                MEWTWO_HAS_FUTURESIGHT[entry_id] = false;
                AttackModule::clear(fighter.module_accessor, 7, false);
            }
        }


        
    }
}

unsafe extern "C" fn kirby_mewtwohat_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);
        let stickx = ControlModule::get_stick_x(fighter.module_accessor);
        let sticky = ControlModule::get_stick_y(fighter.module_accessor);

        if [*FIGHTER_KIRBY_STATUS_KIND_MEWTWO_SPECIAL_N_SHOOT].contains(&status) {
            if sticky >= 0.5 {
                MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = true;
                MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = false;
            }
            if sticky <= -0.5 {
                MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = true;
                MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = false;
            }
            if stickx >= 0.5 || stickx <= -0.5 || MotionModule::frame(fighter.module_accessor) >= (7.0) {
                MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = false;
                MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = false;
            }
        }

        if status != *FIGHTER_KIRBY_STATUS_KIND_MEWTWO_SPECIAL_N_SHOOT {
            MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = false;
            MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = false;
        }

        if sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status) {
			MEWTWO_NEUTRALB_UP_ANGLE[entry_id] = false;
            MEWTWO_NEUTRALB_DOWN_ANGLE[entry_id] = false;
		};


    }
}


unsafe extern "C" fn mewtwo_squatf_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.7);
    }
}

unsafe extern "C" fn mewtwo_squatb_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.7);
    }
}

unsafe extern "C" fn mewtwo_jab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.8, 82, 15, 25, 25, 4.0, 0.0, 11.0, 5.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.8, 82, 15, 25, 25, 4.8, 0.0, 11.8, 9.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.8, 88, 15, 25, 20, 4.5, 0.0, 9.0, 13.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 4.8, 88, 15, 25, 20, 4.5, 0.0, 9.0, 13.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 4.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_100);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn mewtwo_jab100_smash_script(fighter: &mut L2CAgentBase) {
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 361, 10, 0, 7, 5.6, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 4.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 361, 10, 0, 7, 5.6, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 4.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 5.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 361, 10, 0, 7, 5.6, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 4.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 8.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 361, 10, 0, 7, 5.6, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 4.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 11.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 361, 10, 0, 7, 5.6, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 4.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 14.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 361, 10, 0, 7, 5.6, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 4.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 17.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.7, 361, 10, 0, 7, 5.6, 0.0, 9.0, 16.0, Some(0.0), Some(9.0), Some(11.0), 0.5, 0.4, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
            macros::ATK_SET_SHIELD_SETOFF_MUL_arg3(fighter, 0, 1, 4.0);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_100_CONTINUE_CHECK);
        }
        sv_animcmd::frame(fighter.lua_state_agent, 20.0);
        fighter.clear_lua_stack();
        lua_args!(fighter, 0);
        sv_animcmd::wait_loop_clear(fighter.lua_state_agent);
    }
}

unsafe extern "C" fn mewtwo_jab100end_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 361, 108, 0, 79, 5.5, 0.0, 10.5, 11.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 361, 108, 0, 79, 5.5, 0.0, 10.5, 15.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 361, 108, 0, 79, 5.5, 0.0, 10.5, 18.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn mewtwo_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.24);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.6, 50, 80, 0, 70, 5.5, 0.0, 10.0, 16.3, Some(0.0), Some(8.5), Some(16.3), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.6, 50, 80, 0, 70, 3.0, 0.0, 10.0, 13.0, Some(0.0), Some(10.0), Some(6.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.15);
    }
}

unsafe extern "C" fn mewtwo_ftilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("s_tail2"), 10.4, 361, 93, 0, 50, 5.6, 1.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("s_tail4"), 10.4, 361, 93, 0, 50, 5.1, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail6"), 9.3, 361, 93, 0, 50, 5.0, 2.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
}

unsafe extern "C" fn mewtwo_utilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("s_tail3"), 8.6, 106, 82, 0, 55, 5.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("s_tail5"), 8.6, 106, 82, 0, 55, 5.4, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail7"), 8.0, 106, 82, 0, 55, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 8.0, 111, 82, 0, 55, 6.5, 0.0, 10.0, 10.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 3, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

unsafe extern "C" fn mewtwo_dtilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.8, 80, 92, 0, 30, 5.3, 0.0, 3.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.8, 80, 92, 0, 30, 4.8, 0.0, 1.5, 13.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail7"), 5.6, 100, 70, 0, 50, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.24);
    }
}
   
unsafe extern "C" fn mewtwo_fsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.7, 361, 106, 0, 36, 3.5, 0.0, 9.5, 10.2, Some(0.0), Some(9.5), Some(11.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 19.4, 361, 100, 0, 36, 10.6, 0.0, 9.5, 24.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);    
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.7, 361, 106, 0, 36, 3.5, 0.0, 9.5, 10.2, Some(0.0), Some(9.5), Some(11.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 19.4, 361, 100, 0, 36, 6.6, 0.0, 9.5, 20.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);    
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 0.85);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

unsafe extern "C" fn mewtwo_fsmash_effect_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, 10, 20.5, 0, 0, 0, 1.25, true, *EF_FLIP_YZ);
        }
        else {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, 10, 16.5, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn mewtwo_fsmash2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.7, 361, 106, 0, 36, 3.5, 0.0, 11.2, 9.6, Some(0.0), Some(11.7), Some(10.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 19.4, 361, 100, 0, 36, 10.6, 0.0, 17.9, 19.8, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.7, 361, 106, 0, 36, 3.5, 0.0, 11.2, 9.6, Some(0.0), Some(11.7), Some(10.8), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 19.4, 361, 100, 0, 36, 6.6, 0.0, 15.9, 17.8, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);    
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 0.85);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

unsafe extern "C" fn mewtwo_fsmash2_effect_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, 15.5, 20.5, -30, 0, 0, 1.25, true, *EF_FLIP_YZ);
        }
        else {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, 13.5, 16.5, -30, 0, 0, 1, true, *EF_FLIP_YZ);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn mewtwo_fsmash3_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.7, 361, 106, 0, 36, 3.5, 0.0, 9.0, 10.0, Some(0.0), Some(8.2), Some(11.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 19.4, 361, 100, 0, 36, 10.6, 0.0, 3.0, 21.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);    
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.7, 361, 106, 0, 36, 3.5, 0.0, 9.0, 10.0, Some(0.0), Some(8.2), Some(11.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 19.4, 361, 100, 0, 36, 6.6, 0.0, 6.0, 19.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_PUNCH);    
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 0.85);
        JostleModule::set_status(fighter.module_accessor, true);
    }
}

unsafe extern "C" fn mewtwo_fsmash3_effect_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 10, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, 4.5, 19.5, 30, 0, 0, 1.25, true, *EF_FLIP_YZ);
        }
        else {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_b"), Hash40::new("mewtwo_pk_attack_b"), Hash40::new("top"), 0, 6.5, 16.5, 30, 0, 0, 1, true, *EF_FLIP_YZ);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}


unsafe extern "C" fn mewtwo_usmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.5, 368, 100, 90, 0, 3.9, 0.0, 11.7, -6.0, Some(0.0), Some(11.7), Some(7.5), 0.5, 0.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40::new("top"), &smash::phx::Vector2f{x: 0.0, y:24.0}, 7, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    for _ in 0..5 {
        if macros::is_excute(fighter) {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.2, 170, 100, 18, 0, 7.1, 0.0, 27.0, -5.8, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 170, 100, 18, 0, 7.1, 0.0, 27.0, 5.8, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 95, 100, 40, 0, 5.0, 0.0, 14.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 100, 5, 0, 6.9, 0.0, 28.6, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            }
            else {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.2, 170, 100, 18, 0, 5.1, 0.0, 22.0, -5.8, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
                macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.2, 170, 100, 18, 0, 5.1, 0.0, 22.0, 5.8, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
                macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.2, 95, 100, 40, 0, 4.0, 0.0, 14.0, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
                macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 1.2, 270, 100, 5, 0, 4.5, 0.0, 23.6, 0.0, None, None, None, 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);    
            }
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    }
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.1, 80, 111, 0, 60, 10.4, 0.0, 27.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.1, 80, 111, 0, 60, 10.4, 0.0, 27.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 11.1, 90, 111, 0, 60, 8.2, 0.0, 28.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 11.1, 90, 111, 0, 60, 7.0, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 11.1, 80, 111, 0, 60, 6.4, 0.0, 22.0, -7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 11.1, 80, 111, 0, 60, 6.4, 0.0, 22.0, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 11.1, 90, 111, 0, 60, 5.2, 0.0, 23.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 11.1, 90, 111, 0, 60, 4.0, 0.0, 14.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

unsafe extern "C" fn mewtwo_usmash_effect_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 13, 10, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1, 0, 1.3, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
        if PostureModule::lr(fighter.module_accessor) < 0.0 {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mewtwo_pk_attack_a"), Hash40::new("top"), 2.5, 24, 0.5, 0, 180, 0, 1.2, true);
            }
            else {
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mewtwo_pk_attack_a"), Hash40::new("top"), 2.5, 24, 0.5, 0, 180, 0, 1, true);
            }
        }
        else {
            if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mewtwo_pk_attack_a"), Hash40::new("top"), -2.5, 24, 0.5, 0, 0, 0, 1.2, true);
            }
            else {
                macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("mewtwo_pk_attack_a"), Hash40::new("top"), -2.5, 24, 0.5, 0, 0, 0, 1, true);
            }
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn mewtwo_dsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.6);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.1, 50, 101, 0, 38, 17.0, 0.0, 4.0, 17.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
        else {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 18.1, 50, 101, 0, 38, 11.0, 0.0, 4.0, 13.7, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 0.7);
    }
}

unsafe extern "C" fn mewtwo_dsmash_effect_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("top"), 0, 15, -6, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), -1.5, 0, 2, 0, 0, 0, 0.7, true, *EF_FLIP_YZ);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), true, true);
        macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_hand"), Hash40::new("mewtwo_pk_hand"), Hash40::new("havel"), 0, 1, 3, 0, 0, 0, 1.3, true, *EF_FLIP_YZ);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        if WorkModule::get_int(fighter.module_accessor, *FIGHTER_MEWTWO_INSTANCE_WORK_ID_INT_SHADOWBALL_CHARGE_FRAME) == 105 {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 3, 19, 0, 0, 0, 1.2, true, *EF_FLIP_YZ);
        }
        else {
            macros::EFFECT_FOLLOW_FLIP(fighter, Hash40::new("mewtwo_pk_attack_c"), Hash40::new("mewtwo_pk_attack_c"), Hash40::new("top"), 0, 3, 13, 0, 0, 0, 1, true, *EF_FLIP_YZ);
        }
        macros::LANDING_EFFECT(fighter, Hash40::new("null"), Hash40::new("top"), 0, 0, 13, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_h_smoke_b"), Hash40::new("top"), -5, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("mewtwo_pk_hand"), false, false);
    }
}

unsafe extern "C" fn mewtwo_nair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 100, 20, 20, 12.5, 0.0, 7.0, -1.0, None, None, None, 0.3, 0.5, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 3, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.6, 70, 106, 0, 56, 14.5, 0.0, 7.0, -2.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn mewtwo_fair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.4, 47, 100, 0, 40, 6.9, 0.0, 8.2, 14.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 13.4, 47, 100, 0, 40, 5.6, 0.0, 8.2, 7.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 13.4, 47, 100, 0, 40, 6.5, 0.0, 3.8, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn mewtwo_bair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.1, 42, 108, 0, 25, 6.2, 0.0, 12.0, -7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("s_tail5"), 11.5, 42, 108, 0, 25, 5.5, 1.0, 0.0, -2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail7"), 10.3, 42, 108, 0, 25, 4.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.15);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn mewtwo_uair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("s_tail3"), 13.1, 95, 99, 0, 35, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("s_tail5"), 13.1, 95, 99, 0, 35, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail7"), 11.6, 95, 106, 0, 35, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_TAIL);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn mewtwo_dair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.6);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.9, 270, 100, 0, 15, 5.8, 0.0, -9.5, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.7, 60, 98, 0, 30, 7.8, 0.0, -5.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn mewtwo_grab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.7, 0.0, 10.0, 4.5, Some(0.0), Some(10.0), Some(12.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
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

unsafe extern "C" fn mewtwo_grabd_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.3, 0.0, 8.5, 4.5, Some(0.0), Some(8.5), Some(15.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn mewtwo_grabp_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.7, 0.0, 8.5, -4.5, Some(0.0), Some(8.5), Some(-22.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn mewtwo_fthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 63, 100, 35, 0, 0.0, 0.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 33.0);
    for _ in 0..8 {
        if macros::is_excute(fighter) {   
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, false, -1);   
        }
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    }       
    sv_animcmd::frame(fighter.lua_state_agent, 61.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, false, -1);   
    }
}

unsafe extern "C" fn shadowball_fthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.3, 38, 110, 0, 50, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 16, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn mewtwo_bthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 42, 125, 0, 35, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, -15, 11);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn mewtwo_uthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.0, 90, 122, 0, 72, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 0, 27);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 43.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn mewtwo_dthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 64, 80, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("s_tail7"), 2.0, 270, 158, 0, 40, 4.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("s_tail5"), 2.0, 270, 158, 0, 40, 4.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("s_tail3"), 2.0, 270, 158, 0, 40, 4.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_TAIL);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 13, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn mewtwo_neutralb_shoot_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.19047619);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

unsafe extern "C" fn mewtwo_neutralb_shoot_air_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.123595505);
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_SHADOWBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_LAST), false);
    }
}

unsafe extern "C" fn shadowball_charge(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 85, 45, 0, 22, 3.3, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 6, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn shadowball_charge_max(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 85, 45, 0, 22, 3.3, 0.0, 0.0, 0.0, None, None, None, 0.8, 1.3, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0.5, 0.0, 6, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
    }
}

unsafe extern "C" fn shadowball_b(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.4, 361, 30, 0, 14, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 31.3, 47, 71, 0, 30, 2.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 4, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_NONE);
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
    }
}

unsafe extern "C" fn mewtwo_sideb_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.66666667);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 1, Hash40::new("top"), 10.2, 0.0, 8.4, 17.0, None, None, None, *FIGHTER_STATUS_KIND_MEWTWO_THROWN, *COLLISION_SITUATION_MASK_GA);
        shield!(fighter, *MA_MSC_CMD_SHIELD_ON, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 280, 96, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 5.0, 361, 100, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, 0, 1.3, 280, 16, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only(fighter.module_accessor, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, true, true);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    for _ in 0..7 {
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW_MEWTWO, Hash40::new("throw"), *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_TARGET_OBJECT_ID as u64, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_GROUP as u64, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_WORK_INT_THROWN_HIT_NO as u64);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        shield!(fighter, *MA_MSC_CMD_SHIELD_OFF, *COLLISION_KIND_REFLECTOR, *FIGHTER_MEWTWO_REFLECTOR_KIND_REFLECTOR, *FIGHTER_REFLECTOR_GROUP_EXTEND);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_GRAVITY_NORMAL);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MEWTWO_STATUS_SPECIAL_S_FLAG_HIT);
    }
}

unsafe extern "C" fn mewtwo_downb_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_MEWTWO_GENERATE_ARTICLE_BINDBALL, false, 0); 
        if MEWTWO_HAS_FUTURESIGHT[entry_id] == false || MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] < MEWTWO_FUTURESIGHT_FUSE_TIME {
            MEWTWO_FUTURESIGHT_CURRENT_FRAME[entry_id] = 0;
            MEWTWO_FUTURESIGHT_X[entry_id] = PostureModule::pos_x(fighter.module_accessor) + (6.0 * PostureModule::lr(fighter.module_accessor));
            MEWTWO_FUTURESIGHT_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor) + 16.0;
            MEWTWO_HAS_FUTURESIGHT[entry_id] = true;
        }
    }
}

unsafe extern "C" fn confusion_projectile(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 140, 0, 0, 2.3, 0.0, -1.7, 2.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 361, 180, 0, 20, 3.0, 0.0, -1.7, 2.5, None, None, None, 6.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FEB, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bind_extra"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_lr_check_front_lr(fighter.module_accessor, 0);
        AttackModule::set_no_finish_camera(fighter.module_accessor, 0, true, false);
    }
}


pub fn install() {
    Agent::new("mewtwo")
    .on_line(Main, mewtwo_frame) //opff
    .game_acmd("game_squatf", mewtwo_squatf_script, Low)
    .game_acmd("game_squatb", mewtwo_squatb_script, Low)
    .game_acmd("game_attack11", mewtwo_jab_smash_script, Low)
    .game_acmd("game_attack100", mewtwo_jab100_smash_script, Low)
	.game_acmd("game_attack100end", mewtwo_jab100end_smash_script, Low)
    .game_acmd("game_attackdash", mewtwo_dashattack_smash_script, Low)
    .game_acmd("game_attacks3", mewtwo_ftilt_smash_script, Low)
    .game_acmd("game_attacks3hi", mewtwo_ftilt_smash_script, Low)
    .game_acmd("game_attacks3lw", mewtwo_ftilt_smash_script, Low)
    .game_acmd("game_attackhi3", mewtwo_utilt_smash_script, Low)
    .game_acmd("game_attacklw3", mewtwo_dtilt_smash_script, Low)
    .game_acmd("game_attacks4", mewtwo_fsmash_smash_script, Low)
    .game_acmd("game_attacks4hi", mewtwo_fsmash2_smash_script, Low)
    .game_acmd("game_attacks4lw", mewtwo_fsmash3_smash_script, Low)
    .effect_acmd("effect_attacks4", mewtwo_fsmash_effect_script, Low)
    .effect_acmd("effect_attacks4hi", mewtwo_fsmash2_effect_script, Low)
    .effect_acmd("effect_attacks4lw", mewtwo_fsmash3_effect_script, Low)
    .game_acmd("game_attackhi4", mewtwo_usmash_smash_script, Low)
    .effect_acmd("effect_attackhi4", mewtwo_usmash_effect_script, Low)
    .game_acmd("game_attacklw4", mewtwo_dsmash_smash_script, Low)
    .effect_acmd("effect_attacklw4", mewtwo_dsmash_effect_script, Low)
    .game_acmd("game_attackairn", mewtwo_nair_smash_script, Low)
    .game_acmd("game_attackairf", mewtwo_fair_smash_script, Low)
    .game_acmd("game_attackairb", mewtwo_bair_smash_script, Low)
    .game_acmd("game_attackairhi", mewtwo_uair_smash_script, Low)
    .game_acmd("game_attackairlw", mewtwo_dair_smash_script, Low)
    .game_acmd("game_catch", mewtwo_grab_smash_script, Low)
    .game_acmd("game_catchdash", mewtwo_grabd_smash_script, Low)
    .game_acmd("game_catchturn", mewtwo_grabp_smash_script, Low)
    .game_acmd("game_throwf", mewtwo_fthrow_smash_script, Low)
    .game_acmd("game_throwb", mewtwo_bthrow_smash_script, Low)
    .game_acmd("game_throwhi", mewtwo_uthrow_smash_script, Low)
    .game_acmd("game_throwlw", mewtwo_dthrow_smash_script, Low)
    .game_acmd("game_specialnshoot", mewtwo_neutralb_shoot_smash_script, Low)
    .game_acmd("game_specialairnshoot", mewtwo_neutralb_shoot_air_smash_script, Low)
    .game_acmd("game_specials", mewtwo_sideb_smash_script, Low)
    .game_acmd("game_speciallw", mewtwo_downb_smash_script, Low)
    .game_acmd("game_specialairlw", mewtwo_downb_smash_script, Low)
    .install();

    Agent::new("kirby")
    .on_line(Main, kirby_mewtwohat_frame)
    .install();

    Agent::new("mewtwo_shadowball")
    .game_acmd("game_shootthrowf", shadowball_fthrow, Low)
    .game_acmd("game_charge", shadowball_charge, Low)
    .game_acmd("game_chargemax", shadowball_charge_max, Low)
    .game_acmd("game_shoot", shadowball_b, Low)
    .install();

    Agent::new("mewtwo_bindball")
    .game_acmd("game_shoot", confusion_projectile, Low)
    .install();

}