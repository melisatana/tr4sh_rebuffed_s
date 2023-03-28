#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;

static mut CHROM_DASHATTACK_HITBOX_ACTIVE : [bool; 8] = [false; 8];
static mut CHROM_DASHATTACK_HITBOX_FINAL : [bool; 8] = [false; 8];
static mut CHROM_DASHATTACK_HITBOX_X : [f32; 8] = [0.0; 8];
static mut CHROM_DASHATTACK_HITBOX_Y : [f32; 8] = [0.0; 8];

unsafe fn set_dashattack_hitbox(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let point_offset_x = PostureModule::lr(fighter.module_accessor) * (CHROM_DASHATTACK_HITBOX_X[entry_id] - PostureModule::pos_x(fighter.module_accessor));
    let point_offset_y = CHROM_DASHATTACK_HITBOX_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor);
    
    
    if CHROM_DASHATTACK_HITBOX_FINAL[entry_id] {
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 6.0, 80, 100, 0, 120, 6.0, 0.0, 8.0, 0.0, Some(0.0), Some(point_offset_y), Some(point_offset_x), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    else {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 367, 30, 30, 100, 6.0, 0.0, 8.0, 0.0, Some(0.0), Some(point_offset_y), Some(point_offset_x), 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 2, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::EFFECT(fighter, Hash40::new("sys_hit_sting"), Hash40::new("top"), 0, 8, -20, 0, 0, 0, 0.5, 0, 4, 10, 0, 0, 0, true);
    }
    //AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 6.0, false);
}

// A Once-Per-Fighter-Frame that only applies to Chrom
#[fighter_frame( agent = FIGHTER_KIND_CHROM )]
fn chrom_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);
        let sticky = ControlModule::get_stick_y(fighter.module_accessor);

        println!("It'sa me, Chrom, has anyone seen my daughter and my wife I miss them :(");

        if CHROM_DASHATTACK_HITBOX_ACTIVE[entry_id] {
            if status == *FIGHTER_STATUS_KIND_ATTACK_DASH {
                set_dashattack_hitbox(fighter);
            }
            else
            {
                CHROM_DASHATTACK_HITBOX_ACTIVE[entry_id] = false;
            }
        }

        if [*FIGHTER_ROY_STATUS_KIND_SPECIAL_N_LOOP, *FIGHTER_ROY_STATUS_KIND_SPECIAL_N_TURN].contains(&status) {
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_GUARD_ON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_F);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_B);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            fighter.sub_transition_group_check_air_escape();
            fighter.sub_transition_group_check_ground_escape();
            fighter.sub_transition_group_check_ground_guard();
            fighter.sub_transition_group_check_air_jump_aerial();
        }

        if [*FIGHTER_STATUS_KIND_SPECIAL_S].contains(&status) {
            if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
                fighter.sub_transition_group_check_ground_attack();
            }
                
        }

        if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status) {
            if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_AIR { 
                if sticky <= -0.5 {
                    GroundModule::set_passable_check(fighter.module_accessor, true);
                }
                else {
                    GroundModule::set_passable_check(fighter.module_accessor, false);
                }
            }
        }

    }
}

#[acmd_script( agent = "chrom", script = "game_attack11", category = ACMD_GAME )]
unsafe fn chrom_jab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 5.0, 5.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.5, 50, 109, 0, 40, 3.7, 0.0, 10.0, 7.0, Some(0.0), Some(10.0), Some(5.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 12.5, 50, 109, 0, 40, 3.8, 0.0, 0.0, 1.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 12.5, 50, 109, 0, 40, 3.5, 0.0, 0.0, 8.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 0.8);
    }
}

#[acmd_script( agent = "chrom", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn chrom_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        CHROM_DASHATTACK_HITBOX_FINAL[entry_id] = false;
    }

    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 20.0, y: 0.0, z: 0.0});
        CHROM_DASHATTACK_HITBOX_X[entry_id] = PostureModule::pos_x(fighter.module_accessor);
        CHROM_DASHATTACK_HITBOX_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor) + 8.0;
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: -20.0, y: 0.0, z: 0.0});
        MotionModule::set_rate(fighter.module_accessor, 0.2);
        JostleModule::set_status(fighter.module_accessor, true);
        CHROM_DASHATTACK_HITBOX_ACTIVE[entry_id] = true;
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        CHROM_DASHATTACK_HITBOX_FINAL[entry_id] = true;
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CHROM_DASHATTACK_HITBOX_ACTIVE[entry_id] = false;
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.8);
    }
}

#[acmd_script( agent = "chrom", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn chrom_ftilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 4.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.7, 77, 95, 0, 30, 4.3, 0.0, 9.0, 6.9, Some(0.0), Some(9.0), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 8.7, 77, 95, 0, 30, 4.1, 0.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 8.7, 77, 95, 0, 30, 3.7, 0.0, 0.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 2.0, false);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
}

#[acmd_script( agent = "chrom", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn chrom_utilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.8, 98, 83, 0, 40, 4.8, 0.0, 16.0, 0.0, Some(0.0), Some(16.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.8, 98, 83, 0, 40, 3.6, 0.0, 0.0, 1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.8, 98, 83, 0, 40, 2.8, 0.0, 18.0, 6.0, Some(0.0), Some(10.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 9.8, 98, 83, 0, 40, 3.4, 0.0, 16.0, 13.0, Some(0.0), Some(10.0), Some(13.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.8, 98, 83, 0, 40, 4.2, 0.0, 0.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::clear(fighter.module_accessor, 3, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.6);
    }
}

#[acmd_script( agent = "chrom", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn chrom_dtilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 76, 40, 0, 50, 4.0, 0.0, 3.5, 13.0, Some(0.0), Some(4.1), Some(9.2), 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 10.0, 76, 40, 0, 50, 4.0, 0.0, 0.0, 9.7, Some(0.0), Some(0.0), Some(4.7), 1.0, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.35, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor,);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

#[acmd_script( agent = "chrom", script = "game_attacks4", category = ACMD_GAME )]
unsafe fn chrom_fsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.8);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 22.7, 361, 90, 0, 40, 3.3, 0.0, 7.7, 9.7, Some(0.0), Some(7.7), Some(7.0), 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 1, Hash40::new("sword1"), 22.7, 361, 90, 0, 40, 4.0, 2.0, 1.0, 1.5, Some(14.0), Some(1.0), Some(1.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 1, Hash40::new("sword1"), 22.7, 361, 90, 0, 40, 3.6, 2.0, 1.0, 8.3, Some(9.5), Some(1.0), Some(8.3), 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
        macros::PLAY_SE(fighter, Hash40::new("se_chrom_smash_h02"));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_speedbooster"), Hash40::new("top"), 0, 8.0, 28.0, 0, 0, 0, 1.7, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 275, 0, 90, 15.3, 0.0, 7.7, 30.0, Some(0.0), Some(7.7), Some(7.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD); 
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_speedbooster"), false, true);
    }
}

#[acmd_script( agent = "chrom", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn chrom_usmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.8);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 1.0, 118, 100, 120, 0, 4.0, 0.0, 9.0, 4.0, Some(0.0), Some(9.0), Some(-3.0), 1.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 1.0, 127, 100, 150, 0, 4.0, 0.0, 9.0, 7.8, Some(0.0), Some(9.0), Some(-6.8), 1.5, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 1, Hash40::new("armr"), 1.5, 367, 100, 40, 40, 5.4, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 1, Hash40::new("sword1"), 1.5, 367, 100, 40, 40, 4.9, 0.0, 0.0, 6.8, None, None, None, 0.5, 0.3, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
            AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("armr"), 11.0, 90, 90, 0, 70, 5.2, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 1, Hash40::new("sword1"), 11.0, 90, 90, 0, 70, 6.9, 0.0, 0.0, 6.2, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, -1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_speedbooster"), Hash40::new("rot"), 0, 23.0, 0.0, 270, 0, 0, 1.4, true);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 90, 300, 0, 90, 11.1, 0.0, 26.7, 0.0, Some(0.0), Some(35.7), Some (0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_SWORD); 
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_speedbooster"), false, true);
    }
}

#[acmd_script( agent = "chrom", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn chrom_dsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.5, 77, 100, 0, 51, 3.3, 0.0, 4.5, 13.0, Some(0.0), Some(6.0), Some(7.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 14.5, 77, 100, 0, 51, 3.7, 0.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 14.5, 77, 100, 0, 51, 3.4, 0.0, 0.0, 8.2, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 17.7, 77, 100, 0, 51, 3.3, 0.0, 4.1, -11.0, Some(0.0), Some(4.5), Some(-8.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 17.7, 77, 100, 0, 51, 3.7, 0.0, 0.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 17.7, 77, 100, 0, 51, 3.4, -1.6, 0.0, 8.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
}

#[acmd_script( agent = "chrom", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn chrom_nair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 367, 30, 0, 28, 3.5, 0.0, 9.6, 8.0, Some(0.0), Some(11.7), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 4.0, 367, 30, 0, 28, 3.5, 0.0, 0.0, 7.3, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 367, 30, 0, 28, 4.8, 0.0, 9.6, 6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 5.8, 60, 79, 0, 41, 5.6, 0.0, 0.0, -1.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 5.8, 60, 79, 0, 41, 4.4, 2.5, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 5.8, 60, 79, 0, 41, 4.4, 2.5, 0.0, 7.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 2.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "chrom", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn chrom_fair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.6, 49, 90, 0, 50, 4.8, 0.0, 13.0, 4.0, Some(0.0), Some(10.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.6, 49, 90, 0, 50, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 12.6, 49, 90, 0, 50, 3.8, 0.0, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 12.6, 49, 90, 0, 50, 3.8, 0.0, 0.0, 8.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 12.6, 290, 100, 0, 20, 4.8, 0.0, 13.0, 4.0, Some(0.0), Some(10.0), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.6, 290, 100, 0, 20, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 12.6, 290, 100, 0, 20, 3.8, 0.0, 0.0, 0.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 12.6, 290, 100, 0, 20, 3.8, 0.0, 0.0, 8.1, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "chrom", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn chrom_bair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        PostureModule::reverse_lr(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.8, 70, 90, 0, 40, 4.2, 0.0, 10.5, -7.0, Some(0.0), Some(10.5), Some(-4.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("sword1"), 9.8, 70, 90, 0, 40, 4.2, 2.5, 0.0, 0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 9.8, 70, 90, 0, 40, 4.2, 2.5, 0.0, 7.9, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "chrom", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn chrom_uair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("colonells"), 13.8, 80, 103, 0, 33, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.8, 80, 103, 0, 33, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("sword1"), 13.8, 80, 103, 0, 33, 3.7, 0.0, 0.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("sword1"), 13.8, 80, 103, 0, 33, 4.2, 0.0, 0.0, 8.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.15);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "chrom", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn chrom_dair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_chrom_smash_h01"));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 367, 30, 30, 50, 6.1, 0.0, -7.5, 0.4, Some(0.0), Some(-1.0), Some(0.4), 0.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("chrom_attack_hi4_slash"), Hash40::new("sword1"), 0, 0, -2.8, 90, 0, 0, 0.75, true);
        MotionModule::set_rate(fighter.module_accessor, 0.05);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 8.0, 70, 70, 0, 50, 7.0, 0.0, -7.5, 0.4, Some(0.0), Some(-1.0), Some(0.4), 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        macros::EFFECT_DETACH_KIND(fighter, Hash40::new("chrom_attack_hi4_slash"), -1);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "chrom", script = "game_catch", category = ACMD_GAME )]
unsafe fn chrom_grab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 8.0, 4.0, Some(0.0), Some(8.0), Some(10.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "chrom", script = "game_catchdash", category = ACMD_GAME )]
unsafe fn chrom_grabd_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 7.25, 4.0, Some(0.0), Some(7.25), Some(12.8), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "chrom", script = "game_catchturn", category = ACMD_GAME )]
unsafe fn chrom_grabp_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 7.0, -4.0, Some(0.0), Some(7.0), Some(-18.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "chrom", script = "game_throwf", category = ACMD_GAME )]
unsafe fn chrom_fthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.7, 45, 120, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 21, 9);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "chrom", script = "game_throwb", category = ACMD_GAME )]
unsafe fn chrom_bthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.6, 117, 45, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 1, 5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn chrom_uthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.9, 97, 60, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 1, 22);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "chrom", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn chrom_dthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 59, 60, 0, 55, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 1, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialnturn", category = ACMD_GAME )]
unsafe fn chrom_neutralb_turn_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairnturn", category = ACMD_GAME )]
unsafe fn chrom_neutralb_turn_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialnend", category = ACMD_GAME )]
unsafe fn chrom_neutralb_1_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.4, 280, 110, 0, 20, 8.7, 0.0, 8.5, 15.7, Some(0.0), Some(9.0), Some(15.9), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialnend2", category = ACMD_GAME )]
unsafe fn chrom_neutralb_2_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.4, 280, 110, 0, 20, 8.7, 0.0, 8.5, 15.7, Some(0.0), Some(9.0), Some(15.9), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialnend3", category = ACMD_GAME )]
unsafe fn chrom_neutralb_3_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.4, 280, 110, 0, 20, 8.7, 0.0, 8.5, 15.7, Some(0.0), Some(9.0), Some(15.9), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialnendmax", category = ACMD_GAME )]
unsafe fn chrom_neutralb_max_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 69.0, 280, 70, 0, 50, 17.0, 0.0, 10.5, 16.0, Some(0.0), Some(10.5), Some(15.7), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairnend", category = ACMD_GAME )]
unsafe fn chrom_neutralb_1_air_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.4, 280, 110, 0, 20, 8.7, 0.0, 8.5, 15.7, Some(0.0), Some(9.0), Some(15.9), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairnend2", category = ACMD_GAME )]
unsafe fn chrom_neutralb_2_air_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.4, 280, 110, 0, 20, 8.7, 0.0, 8.5, 15.7, Some(0.0), Some(9.0), Some(15.9), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairnend3", category = ACMD_GAME )]
unsafe fn chrom_neutralb_3_air_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.4, 280, 110, 0, 20, 8.7, 0.0, 8.5, 15.7, Some(0.0), Some(9.0), Some(15.9), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairnendmax", category = ACMD_GAME )]
unsafe fn chrom_neutralb_max_air_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        KineticModule::set_consider_ground_friction(fighter.module_accessor, false, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 69.0, 280, 70, 0, 50, 17.0, 0.0, 10.5, 16.0, Some(0.0), Some(10.5), Some(15.7), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
}


#[acmd_script( agent = "chrom", script = "game_specials1", category = ACMD_GAME )]
unsafe fn chrom_sideb_1_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 30, 25, 25, 7.5, 0.0, 9.0, 13.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 120, 30, 25, 25, 6.0, 0.0, 9.0, 19.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 361, 30, 25, 25, 6.0, 0.0, 9.0, 6.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairs1", category = ACMD_GAME )]
unsafe fn chrom_sideb_1_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 367, 30, 25, 25, 7.5, 0.0, 9.0, 13.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.5, 367, 30, 25, 25, 6.0, 0.0, 9.0, 19.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.5, 367, 30, 25, 25, 6.0, 0.0, 9.0, 6.0, None, None, None, 0.5, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specials2lw", category = ACMD_GAME )]
unsafe fn chrom_sideb_2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.1, 361, 25, 30, 30, 6.5, 0.0, 9.0, 13.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.1, 98, 25, 20, 30, 6.0, 0.0, 9.0, 20.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.1, 361, 25, 30, 30, 5.0, 0.0, 9.0, 8.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairs2lw", category = ACMD_GAME )]
unsafe fn chrom_sideb_2_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.1, 367, 25, 0, 30, 6.5, 0.0, 9.0, 13.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.1, 367, 25, 0, 30, 6.0, 0.0, 9.0, 20.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.1, 367, 25, 0, 30, 5.0, 0.0, 9.0, 8.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}


#[acmd_script( agent = "chrom", script = "game_specials3s", category = ACMD_GAME )]
unsafe fn chrom_sideb_3_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 77, 30, 45, 55, 6.5, 0.0, 9.0, 13.5, Some(0.0), Some(10.0), Some(12.5), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 97, 30, 45, 55, 5.5, 0.0, 9.0, 17.5, Some(0.0), Some(10.5), Some(16.5), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 45, 30, 45, 55, 5.5, 0.0, 9.0, 8.0, Some(0.0), Some(9.0), Some(8.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairs3s", category = ACMD_GAME )]
unsafe fn chrom_sideb_3_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 367, 30, 30, 55, 6.5, 0.0, 9.0, 12.5, Some(0.0), Some(11.0), Some(12.5), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 367, 30, 30, 55, 5.5, 0.0, 9.0, 16.5, Some(0.0), Some(11.5), Some(16.5), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 367, 30, 30, 55, 5.5, 0.0, 9.0, 8.0, Some(0.0), Some(9.0), Some(8.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}


#[acmd_script( agent = "chrom", script = "game_specials4s", category = ACMD_GAME )]
unsafe fn chrom_sideb_4_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 110, 0, 69, 6.0, 0.0, 9.0, 10.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 110, 0, 69, 7.0, 0.0, 9.0, 16.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 361, 110, 0, 69, 6.0, 0.0, 9.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.869565217);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairs4s", category = ACMD_GAME )]
unsafe fn chrom_sideb_4_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.0, 361, 110, 0, 69, 6.0, 0.0, 9.0, 10.7, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.0, 361, 110, 0, 69, 7.0, 0.0, 9.0, 16.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 9.0, 361, 110, 0, 69, 6.0, 0.0, 9.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.869565217);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialhi2", category = ACMD_GAME )]
unsafe fn chrom_upb_2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 91, 100, 155, 0, 4.8, 0.0, 5.0, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 89, 100, 155, 0, 4.8, 0.0, 5.0, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 90, 100, 140, 0, 4.8, 0.0, 12.5, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 91, 100, 140, 0, 4.8, 0.0, 12.5, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTROL);
        WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
        WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 366, 100, 125, 0, 9.0, 0.0, 16.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairhi2", category = ACMD_GAME )]
unsafe fn chrom_upb_2_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 91, 100, 155, 0, 4.8, 0.0, 5.0, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 89, 100, 155, 0, 4.8, 0.0, 5.0, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 90, 100, 140, 0, 4.8, 0.0, 12.5, 8.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 7.0, 91, 100, 140, 0, 4.8, 0.0, 12.5, 18.0, None, None, None, 1.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_ALWAYS, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_TRANS_JUMP);
        damage!(fighter, *MA_MSC_DAMAGE_DAMAGE_NO_REACTION, *DAMAGE_NO_REACTION_MODE_NORMAL, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_HI_FLAG_CONTROL);
        WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME_INIT);
        WorkModule::set_float(fighter.module_accessor, 4.0, *FIGHTER_ROY_STATUS_SPECIAL_HI_WORK_FLOAT_SLIDEGAP_RECOVER_FRAME);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 366, 100, 125, 0, 9.0, 0.0, 16.0, 14.0, Some(0.0), Some(8.0), Some(14.0), 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 7, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
        AttackModule::set_no_damage_fly_smoke_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialhi4", category = ACMD_GAME )]
unsafe fn chrom_upb_4_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.7, 30, 130, 0, 50, 10.0, 0.0, 6.0, 11.8, Some(0.0), Some(11.0), Some(11.8), 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CHROM_HIT, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_speciallw", category = ACMD_GAME )]
unsafe fn chrom_downb_counter_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.66666666667);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairlw", category = ACMD_GAME )]
unsafe fn chrom_downb_air_counter_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.66666666667);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_ROY_STATUS_SPECIAL_LW_FLAG_SHIELD);
    }
}

#[acmd_script( agent = "chrom", script = "game_speciallwhit", category = ACMD_GAME )]
unsafe fn chrom_downb_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 70, 50, 0, 35, 7.7, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(3.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 70, 50, 0, 35, 7.5, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_is_critical_attack(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairlwhit", category = ACMD_GAME )]
unsafe fn chrom_downb_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 70, 50, 0, 35, 7.7, 0.0, 8.0, 13.0, Some(0.0), Some(8.0), Some(3.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.0, 70, 50, 0, 35, 7.5, 0.0, 8.0, 18.0, Some(0.0), Some(8.0), Some(2.0), 1.2, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_SWORD);
        AttackModule::set_force_reaction(fighter.module_accessor, 0, true, false);
        AttackModule::set_force_reaction(fighter.module_accessor, 1, true, false);
        AttackModule::set_is_critical_attack(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specials2hi", category = ACMD_GAME )]
unsafe fn chrom_sideb_2_hi_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 90, 30, 30, 30, 4.5, 0.0, 6.0, 12.5, Some(0.0), Some(16.0), Some(12.5), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 110, 30, 30, 30, 3.5, 0.0, 4.0, 18.0, Some(0.0), Some(21.0), Some(18.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 60, 30, 30, 30, 4.8, 0.0, 9.0, 8.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairs2hi", category = ACMD_GAME )]
unsafe fn chrom_sideb_2_airhi_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 30, 30, 30, 4.5, 0.0, 6.0, 12.5, Some(0.0), Some(16.0), Some(12.5), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.5, 367, 30, 30, 30, 3.0, 0.0, 4.0, 18.0, Some(0.0), Some(21.0), Some(18.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 367, 30, 30, 30, 4.5, 0.0, 9.0, 8.0, None, None, None, 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specials3hi", category = ACMD_GAME )]
unsafe fn chrom_sideb_3_hi_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 85, 30, 50, 55, 4.5, 0.0, 6.0, 9.0, Some(0.0), Some(15.0), Some(9.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 85, 30, 50, 55, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(16.0), Some(12.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 45, 50, 40, 55, 4.5, 0.0, 9.0, 3.5, Some(0.0), Some(11.0), Some(3.5), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairs3hi", category = ACMD_GAME )]
unsafe fn chrom_sideb_3_airhi_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 367, 30, 30, 55, 4.5, 0.0, 6.0, 9.0, Some(0.0), Some(15.0), Some(9.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 367, 30, 30, 55, 3.0, 0.0, 6.0, 12.0, Some(0.0), Some(16.0), Some(12.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.0, 367, 50, 30, 55, 4.5, 0.0, 9.0, 3.5, Some(0.0), Some(11.0), Some(3.5), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specials3lw", category = ACMD_GAME )]
unsafe fn chrom_sideb_3_lw_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 278, 30, 0, 47, 5.8, 0.0, 5.6, 14.0, Some(0.0), Some(7.0), Some(10.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 278, 30, 0, 47, 4.0, 0.0, 4.0, 19.0, Some(0.0), Some(7.0), Some(10.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairs3lw", category = ACMD_GAME )]
unsafe fn chrom_sideb_3_airlw_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::set_attack_reference_joint_id(fighter.module_accessor, Hash40::new("sword1"), AttackDirectionAxis(*ATTACK_DIRECTION_X), AttackDirectionAxis(*ATTACK_DIRECTION_Y), AttackDirectionAxis(*ATTACK_DIRECTION_Z));
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 278, 30, 0, 47, 5.8, 0.0, 5.6, 14.0, Some(0.0), Some(7.0), Some(10.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 278, 30, 0, 47, 4.0, 0.0, 4.0, 19.0, Some(0.0), Some(7.0), Some(10.0), 0.7, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, true, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_MOTION_CHANGE_ENABLE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_MARTH_STATUS_SPECIAL_S_FLAG_INPUT_CHECK);
    }
}

#[acmd_script( agent = "chrom", script = "game_specials4hi", category = ACMD_GAME )]
unsafe fn chrom_sideb_4_hi_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 79, 35, 0, 85, 4.5, 0.0, 6.0, 10.5, Some(0.0), Some(21.0), Some(10.5), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 87, 35, 0, 80, 3.0, 0.0, 6.0, 15.0, Some(0.0), Some(21.0), Some(15.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 79, 35, 0, 85, 4.0, 0.0, 21.0, 11.0, Some(0.0), Some(24.0), Some(4.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.5, 87, 35, 0, 80, 5.0, 0.0, 21.0, 14.0, Some(0.0), Some(29.0), Some(6.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 3.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairs4hi", category = ACMD_GAME )]
unsafe fn chrom_sideb_4_airhi_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.5, 79, 35, 0, 85, 4.5, 0.0, 6.0, 10.5, Some(0.0), Some(21.0), Some(10.5), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.5, 87, 35, 0, 80, 3.0, 0.0, 6.0, 15.0, Some(0.0), Some(21.0), Some(15.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.5, 79, 35, 0, 85, 4.0, 0.0, 21.0, 11.0, Some(0.0), Some(24.0), Some(4.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.5, 87, 35, 0, 80, 5.0, 0.0, 21.0, 14.0, Some(0.0), Some(29.0), Some(6.0), 1.0, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 3.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 3, 3.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specials4lw", category = ACMD_GAME )]
unsafe fn chrom_sideb_4_lw_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 80, 20, 0, 2, 5.0, 0.0, 6.0, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 80, 20, 0, 2, 4.0, 0.0, 6.0, 16.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 80, 20, 0, 2, 4.5, 0.0, 6.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 30, 90, 0, 57, 6.0, 0.0, 7.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 30, 90, 0, 60, 3.8, 0.0, 8.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 30, 90, 0, 57, 5.5, 0.0, 6.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "chrom", script = "game_specialairs4lw", category = ACMD_GAME )]
unsafe fn chrom_sideb_4_airlw_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 367, 20, 2, 2, 5.0, 0.0, 6.0, 11.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 367, 20, 2, 2, 4.0, 0.0, 6.0, 16.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.0, 367, 20, 2, 2, 4.5, 0.0, 6.0, 6.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_SWORD);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 30, 90, 0, 57, 6.0, 0.0, 7.0, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 30, 90, 0, 60, 3.8, 0.0, 8.0, 18.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 5.0, 30, 90, 0, 57, 5.5, 0.0, 6.0, 5.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_SWORD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        chrom_frame
    );
    smashline::install_acmd_scripts!(
        chrom_jab_smash_script,
        chrom_dashattack_smash_script,
        chrom_ftilt_smash_script,
        chrom_utilt_smash_script,
        chrom_dtilt_smash_script,
        chrom_fsmash_smash_script,
        chrom_usmash_smash_script,
        chrom_dsmash_smash_script,
        chrom_nair_smash_script,
        chrom_fair_smash_script,
        chrom_bair_smash_script,
        chrom_uair_smash_script,
        chrom_dair_smash_script,
        chrom_grab_smash_script,
        chrom_grabd_smash_script,
        chrom_grabp_smash_script,
        chrom_fthrow_smash_script,
        chrom_bthrow_smash_script,
        chrom_uthrow_smash_script,
        chrom_dthrow_smash_script,
        chrom_neutralb_turn_smash_script,
        chrom_neutralb_turn_air_smash_script,
        chrom_neutralb_1_smash_script,
        chrom_neutralb_2_smash_script,
        chrom_neutralb_3_smash_script,
        chrom_neutralb_max_smash_script,
        chrom_neutralb_1_air_smash_script,
        chrom_neutralb_2_air_smash_script,
        chrom_neutralb_3_air_smash_script,
        chrom_neutralb_max_air_smash_script,
        chrom_sideb_1_smash_script,
        chrom_sideb_1_air_smash_script,
        chrom_sideb_2_smash_script,
        chrom_sideb_2_air_smash_script,
        chrom_sideb_3_smash_script,
        chrom_sideb_3_air_smash_script,
        chrom_sideb_4_smash_script,
        chrom_sideb_4_air_smash_script,
        chrom_upb_2_smash_script,
        chrom_upb_2_air_smash_script,
        chrom_upb_4_smash_script,
        chrom_downb_counter_smash_script,
        chrom_downb_air_counter_smash_script,
        chrom_downb_smash_script,
        chrom_downb_air_smash_script,
        chrom_sideb_2_hi_smash_script,
        chrom_sideb_2_airhi_smash_script,
        chrom_sideb_3_hi_smash_script,
        chrom_sideb_3_airhi_smash_script,
        chrom_sideb_3_lw_smash_script,
        chrom_sideb_3_airlw_smash_script,
        chrom_sideb_4_hi_smash_script,
        chrom_sideb_4_airhi_smash_script,
        chrom_sideb_4_lw_smash_script,
        chrom_sideb_4_airlw_smash_script
        
    );
}
