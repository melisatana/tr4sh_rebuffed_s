#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;

pub static mut SELF_DAMAGE_METER : [f32; 8] = [0.0; 8]; //total stored self-damage
static mut SELF_DAMAGE_FRAME : [i32; 8] = [0; 8]; //for effect frames
static mut SELF_DAMAGE_FRAME_CURRENT_STATUS : [i32; 8] = [0; 8]; //for effect frames
static mut PICHU_NEUTRALSPECIAL_FROMGROUND : [bool; 8] = [false; 8]; //if neutral special was started on the ground
static mut PICHU_SELF_DAMAGE_GRACE : [i32; 8] = [0; 8];

static PICHU_GRACE_FRAMES : i32 = 60;

static PICHU_CHARGE_LV1 : f32 = -5.0;
static PICHU_CHARGE_LV2 : f32 = -10.0;
static PICHU_CHARGE_LV3 : f32 = -20.0;
static PICHU_CHARGE_LV4 : f32 = -30.0;

// A Once-Per-Fighter-Frame that only applies to Pichu. Neat!
#[fighter_frame( agent = FIGHTER_KIND_PICHU )]
fn pichu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);

        if sv_information::is_ready_go() == false || 
        [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status) {
			SELF_DAMAGE_METER[entry_id] = 0.0;
            SELF_DAMAGE_FRAME[entry_id] = 0;
            PICHU_SELF_DAMAGE_GRACE[entry_id] = 0;
            PICHU_NEUTRALSPECIAL_FROMGROUND[entry_id] = false;
		};

        if ![*FIGHTER_STATUS_KIND_GUARD, 
        *FIGHTER_STATUS_KIND_GUARD_DAMAGE, 
        *FIGHTER_STATUS_KIND_GUARD_OFF, 
        *FIGHTER_STATUS_KIND_GUARD_ON, 
        *FIGHTER_STATUS_KIND_ATTACK_AIR, 
        *FIGHTER_STATUS_KIND_ATTACK_S4, 
        *FIGHTER_STATUS_KIND_ATTACK_LW4].contains(&status) && StopModule::is_hit(fighter.module_accessor) && PICHU_SELF_DAMAGE_GRACE[entry_id] <= 0 {
            if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV4 {
                SELF_DAMAGE_METER[entry_id] = PICHU_CHARGE_LV3;
                PICHU_SELF_DAMAGE_GRACE[entry_id] = PICHU_GRACE_FRAMES;
            }
            else if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV3 {
                SELF_DAMAGE_METER[entry_id] = PICHU_CHARGE_LV2;
                PICHU_SELF_DAMAGE_GRACE[entry_id] = PICHU_GRACE_FRAMES;
            }
            else if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV2 {
                SELF_DAMAGE_METER[entry_id] = PICHU_CHARGE_LV1;
                PICHU_SELF_DAMAGE_GRACE[entry_id] = PICHU_GRACE_FRAMES;
            }
            else {
                SELF_DAMAGE_METER[entry_id] = 0.0;
            }
        }

        if status == *FIGHTER_STATUS_KIND_SPECIAL_N && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND && MotionModule::frame(fighter.module_accessor) < 2.0 {
            
            if GroundModule::ray_check(fighter.module_accessor, &smash::phx::Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &smash::phx::Vector2f{ x: 0.0, y: 8.0}, false) == 0 {
                let pos = smash::phx::Vector3f { x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)+8.0, z: 0.0 }; // This puts Pichu 3 units above the ground
                PostureModule::set_pos(fighter.module_accessor, &pos);
                PostureModule::init_pos(fighter.module_accessor, &pos, true, true);
                macros::SET_SPEED_EX(fighter, -0.7, 1.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                PICHU_NEUTRALSPECIAL_FROMGROUND[entry_id] = true;
                
            } ;
            //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
            //macros::SA_SET(fighter, *SITUATION_KIND_AIR);
        };
        if status == *FIGHTER_STATUS_KIND_SPECIAL_N && MotionModule::frame(fighter.module_accessor) < 10.0 && PICHU_NEUTRALSPECIAL_FROMGROUND[entry_id] {
            macros::SET_SPEED_EX(fighter, -0.7, 1.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        } ;

    
        SELF_DAMAGE_FRAME[entry_id] += 1;
        if SELF_DAMAGE_FRAME[entry_id] == 9000 || SELF_DAMAGE_FRAME_CURRENT_STATUS[entry_id] != status {
            SELF_DAMAGE_FRAME[entry_id] = 0 ;
            if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV4 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 2.0, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 5.0, 0.5, 2.5);
            }
            else if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV3 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.8, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 8.0, 1.5, 0.5);
            }
            else if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV2 {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.5, true);
                macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 3.0, 0.5);
            }
            else {
                if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV1 {
                    macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.2, true);
                }
            }
            SELF_DAMAGE_FRAME_CURRENT_STATUS[entry_id] = status ;
        }


        if [*FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_HI_WARP].contains(&status) {
            if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV2 {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_LW_HIT, false);
                }
            }
        }

        if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV4 {
            DamageModule::set_damage_mul(fighter.module_accessor, 0.75);
        }
        else if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV3 {
            DamageModule::set_damage_mul(fighter.module_accessor, 0.8);
        }
        else if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV2 {
            DamageModule::set_damage_mul(fighter.module_accessor, 0.85);
        }
        else {
            DamageModule::set_damage_mul(fighter.module_accessor, 1.0);
        }

        if PICHU_SELF_DAMAGE_GRACE[entry_id] > 0 {
            PICHU_SELF_DAMAGE_GRACE[entry_id] -= 1;
        }

        let touch_right = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
        let touch_left = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
        let touch_side =  GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32);
        if status == *FIGHTER_PIKACHU_STATUS_KIND_SPECIAL_S_ATTACK {
            if touch_left || touch_right || touch_side {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, true);
            }
        }

    }
}

#[fighter_frame( agent = FIGHTER_KIND_KIRBY )]
fn kirby_pichuhat_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);

        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_FLAG_COPY) && WorkModule::get_int(fighter.module_accessor, *FIGHTER_KIRBY_INSTANCE_WORK_ID_INT_COPY_CHARA) == *FIGHTER_KIND_PICHU {
            if status == *FIGHTER_KIRBY_STATUS_KIND_PICHU_SPECIAL_N && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND && MotionModule::frame(fighter.module_accessor) < 2.0 {
                if GroundModule::ray_check(fighter.module_accessor, &smash::phx::Vector2f{ x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)}, &smash::phx::Vector2f{ x: 0.0, y: 10.0}, false) == 0 {
                    let pos = smash::phx::Vector3f { x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)+10.0, z: 0.0 }; // This puts Pichu 3 units above the ground
                    PostureModule::set_pos(fighter.module_accessor, &pos);
                    PostureModule::init_pos(fighter.module_accessor, &pos, true, true);
                    macros::SET_SPEED_EX(fighter, -0.7, 1.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    PICHU_NEUTRALSPECIAL_FROMGROUND[entry_id] = true;
                }
            }
            if status == *FIGHTER_KIRBY_STATUS_KIND_PICHU_SPECIAL_N && MotionModule::frame(fighter.module_accessor) < 10.0 && PICHU_NEUTRALSPECIAL_FROMGROUND[entry_id] {
                macros::SET_SPEED_EX(fighter, -0.7, 1.1, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            }
        }
        else {
            PICHU_NEUTRALSPECIAL_FROMGROUND[entry_id] = false;
        }
    }
}

#[acmd_script( agent = "pichu", script = "game_guarddamage", category = ACMD_GAME )]
unsafe fn pichu_shield(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pichu_attackhard_s01"));
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.6, true);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.4, 361, 5, 5, 5, 11.0, 0.0, 5.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        macros::FT_ADD_DAMAGE(fighter, 0.3);
        SELF_DAMAGE_METER[entry_id] -= 0.3;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_elec2"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek"), false, true);
    }
}

#[acmd_script( agent = "pichu", script = "game_justshieldoff", category = ACMD_GAME )]
unsafe fn pichu_perfectshield(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pichu_attackhard_s01"));
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 1.4, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pichu_elec2"), Hash40::new("top"), 0, 4, 0, 0, 0, 0, 2.0, true);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 361, 5, 5, 5, 15.0, 0.0, 5.0, 0.0, None, None, None, 0.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, true, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        macros::FT_ADD_DAMAGE(fighter, 0.6);
        SELF_DAMAGE_METER[entry_id] -= 0.6;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_elec2"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek"), false, true);
    }
}

#[acmd_script( agent = "pichu", script = "game_attack11", category = ACMD_GAME )]
unsafe fn pichu_jab(fighter: &mut L2CAgentBase) {
    JostleModule::set_status(fighter.module_accessor, false);
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 0.3, y: 0.0, z: 0.0});
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.9, 0, 50, 0, 10, 3.7, 0.0, 3.0, 6.0, None, None, None, 0.5, 1.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.9, 0, 50, 0, 20, 3.7, 0.0, 3.0, 10.0, None, None, None, 0.5, 1.7, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackdash", category = ACMD_GAME )]
unsafe fn pichu_dashattack(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_elec"), Hash40::new("head"), 4, 0, 0, 0, 0, 0, 1.0, true);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);
        macros::PLAY_SE(fighter, Hash40::new("se_pichu_attackhard_s01"));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 1.9);
        SELF_DAMAGE_METER[entry_id] -= 1.9;
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 12.1, 35, 95, 0, 70, 4.9, 3.0, -1.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.1, 35, 95, 0, 70, 3.5, 0.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 8.0, 70, 80, 0, 60, 3.7, 3.0, -1.5, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 8.0, 70, 80, 0, 60, 3.0, 0.7, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.15);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek2"), false, true);
    }
}

#[acmd_script( agent = "pichu", script = "game_attacks3", category = ACMD_GAME )]
unsafe fn pichu_ftilt(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 0.8);
        SELF_DAMAGE_METER[entry_id] -= 0.8;
        macros::ATTACK(fighter, 0, 0, Hash40::new("footl"), 9.5, 361, 150, 0, 5, 4.5, 0.0, 0.0, 0.0, Some(0.0), Some(-4.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("footr"), 9.5, 361, 150, 0, 5, 4.0, 0.0, 0.0, 0.0, Some(0.0), Some(-3.7), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        AttackModule::clear(fighter.module_accessor, 0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackhi3", category = ACMD_GAME )]
unsafe fn pichu_utilt(fighter: &mut L2CAgentBase) {
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
        macros::ATTACK(fighter, 0, 0, Hash40::new("tail1"), 6.5, 95, 80, 0, 34, 2.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail2"), 6.5, 95, 80, 0, 34, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 2, 0, Hash40::new("tail3"), 6.5, 95, 80, 0, 34, 4.5, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
		MotionModule::set_rate(fighter.module_accessor, 1.35);
    }
}

#[acmd_script( agent = "pichu", script = "game_attacklw3", category = ACMD_GAME )]
unsafe fn pichu_dtilt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_XLU);
    }    
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("tail1"), 6.9, 106, 69, 0, 26, 3.8, 1.7, 0.4, -0.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail3"), 6.9, 120, 69, 0, 26, 4.9, -0.6, 0.8, -0.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_TAIL);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 1.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 1.0, false);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
		MotionModule::set_rate(fighter.module_accessor, 1.3);
        macros::HIT_NODE(fighter, Hash40::new("tail1"), *HIT_STATUS_NORMAL);
    }
}

#[acmd_script( agent = "pichu", script = "game_attacks4", category = ACMD_GAME )]
unsafe fn pichu_fsmash(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 3.5);
        SELF_DAMAGE_METER[entry_id] -= 3.5;
    }
    for _ in 0..5 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.7, 366, 50, 0, 10, 5.7, 0.0, 4.7, 12.0, None, None, None, 0.3, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.7, 20, 50, 0, 10, 4.5, 0.0, 4.7, 8.0, None, None, None, 0.3, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 10.1, 361, 118, 0, 68, 6.2, 0.0, 5.2, 12.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 10.1, 361, 118, 0, 68, 4.7, 0.0, 4.7, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackhi4", category = ACMD_GAME )]
unsafe fn pichu_usmash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 15.6, 95, 104, 0, 40, 5.8, 5.4, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 15.6, 95, 104, 0, 40, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        macros::HIT_NODE(fighter, Hash40::new("mimir1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("mimil1"), *HIT_STATUS_XLU);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
}

#[acmd_script( agent = "pichu", script = "game_attacklw4", category = ACMD_GAME )]
unsafe fn pichu_dsmash(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 2.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 173, 30, 0, 65, 4.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(-5.5), 0.7, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        macros::FT_ADD_DAMAGE(fighter, 2.7);
        SELF_DAMAGE_METER[entry_id] -= 2.7;
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.5, 173, 30, 0, 65, 4.5, 0.0, 4.0, 9.0, Some(0.0), Some(4.0), Some(-5.5), 0.7, 0.5, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.9, 32, 130, 0, 45, 6.0, 0.0, 4.5, -5.5, Some(0.0), Some(4.5), Some(4.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackairn", category = ACMD_GAME )]
unsafe fn pichu_nair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 11.2, 50, 106, 0, 25, 4.8, 5.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::HIT_NODE(fighter, Hash40::new("mimir1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("mimil1"), *HIT_STATUS_XLU);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 6.1, 361, 112, 0, 15, 4.8, 5.0, 0.0, 0.0, Some(-1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("neck"), 6.1, 0, 10, 0, 20, 4.8, 5.0, 0.0, 0.0, Some(-1.0), Some(0.0), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
		MotionModule::set_rate(fighter.module_accessor, 1.35);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackairf", category = ACMD_GAME )]
unsafe fn pichu_fair(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

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
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::FT_ADD_DAMAGE(fighter, 1.6);
        SELF_DAMAGE_METER[entry_id] -= 1.6;
    }
    for _ in 0..6 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 1.4, 367, 90, 28, 28, 6.8, 2.2, 0.5, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
            macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 1.4, 367, 90, 28, 28, 5.6, 0.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 5.8, 45, 134, 0, 46, 6.8, 2.2, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 5.8, 45, 134, 0, 46, 5.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackairb", category = ACMD_GAME )]
unsafe fn pichu_bair(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 2.0);
        SELF_DAMAGE_METER[entry_id] -= 2.0;
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.3, 367, 100, 50, 20, 4.9, 0.0, 1.8, -11.5, Some(0.0), Some(1.8), Some(0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.3, 367, 100, 50, 20, 4.9, 0.0, 2.5, -11.5, Some(0.0), Some(2.5), Some(-0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.3, 367, 100, 50, 20, 4.9, 0.0, 2.5, -11.5, Some(0.0), Some(2.5), Some(-0.5), 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.5, 42, 153, 0, 70, 6.2, 0.0, 3.0, -11.5, Some(0.0), Some(3.0), Some(-0.5), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackairhi", category = ACMD_GAME )]
unsafe fn pichu_uair(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 1.1);
        SELF_DAMAGE_METER[entry_id] -= 1.1;
        macros::PLAY_SE(fighter, Hash40::new("se_pichu_attackhard_s01"));
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_elec"), Hash40::new("tail1"), 1, 0, 0, 0, 0, 0, 0.8, true);
        macros::EFFECT_FOLLOW_NO_STOP(fighter, Hash40::new("pichu_cheek2"), Hash40::new("head"), 0, 0, 0, 0, -90, -90, 1, true);

        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("tail1"), 6.9, 70, 61, 0, 39, 3.1, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_TAIL);
        macros::ATTACK(fighter, 1, 0, Hash40::new("tail3"), 6.9, 70, 61, 0, 39, 4.1, 1.5, -0.2, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_TAIL);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
		MotionModule::set_rate(fighter.module_accessor, 1.4);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_elec"), false, true);
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pichu_cheek2"), false, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pichu", script = "game_attackairlw", category = ACMD_GAME )]
unsafe fn pichu_dair(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 1.0);
        SELF_DAMAGE_METER[entry_id] -= 1.0;
        macros::HIT_NODE(fighter, Hash40::new("mimir1"), *HIT_STATUS_XLU);
        macros::HIT_NODE(fighter, Hash40::new("mimil1"), *HIT_STATUS_XLU);
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 15.9, 270, 86, 0, 16, 5.5, 4.5, -1.3, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 12.9, 361, 107, 0, 26, 5.4, 4.5, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 12.9, 361, 107, 0, 26, 3.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

#[acmd_script( agent = "pichu", script = "game_landingairlw", category = ACMD_GAME )]
unsafe fn pichu_dair_landing(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 4.0);
        SELF_DAMAGE_METER[entry_id] -= 4.0;
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 361, 100, 0, 50, 4.0, 0.0, 4.0, -6.0, Some(0.0), Some(4.0), Some(6.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pichu", script = "game_catch", category = ACMD_GAME )]
unsafe fn pichu_grab(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.0, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(9.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

#[acmd_script( agent = "pichu", script = "game_catchdash", category = ACMD_GAME )]
unsafe fn pichu_grabd(fighter: &mut L2CAgentBase) {
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
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.5, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(11.1), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pichu", script = "game_catchturn", category = ACMD_GAME )]
unsafe fn pichu_grabp(fighter: &mut L2CAgentBase) {
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
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.8, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pichu", script = "game_catchattack", category = ACMD_GAME )]
unsafe fn pichu_pummel(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 0.7);
        SELF_DAMAGE_METER[entry_id] -= 0.7;
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.8, 80, 100, 30, 0, 4.7, 0.0, 8.2, 8.6, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
		MotionModule::set_rate(fighter.module_accessor, 1.66);
    }
}

#[acmd_script( agent = "pichu", script = "game_throwf", category = ACMD_GAME )]
unsafe fn pichu_fthrow(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 6.0, 45, 110, 0, 45, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 2.5);
        SELF_DAMAGE_METER[entry_id] -= 2.5;
    }
    for _ in 0..4 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.8, 361, 100, 0, 0, 5.5, 0.0, 8.5, 4.7, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 4.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 6, 26);
        //FighterCutInManager::set_throw_finish_zoom_rate(1.4)
        //FighterCutInManager::set_throw_finish_offset(2, 1.5, 0)
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pichu", script = "game_throwb", category = ACMD_GAME )]
unsafe fn pichu_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.3, 135, 110, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 12.8, 361, 130, 0, 50, 5.0, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, -6, 4);
        //FighterCutInManager::set_throw_finish_zoom_rate(1.3)
        //FighterCutInManager::set_throw_finish_offset(-2, 2, 0)
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pichu", script = "game_throwhi", category = ACMD_GAME )]
unsafe fn pichu_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 90, 45, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hat"), 7.9, 80, 60, 0, 70, 4.7, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 2, 17);
        //FighterCutInManager::set_throw_finish_zoom_rate(1.3)
        //FighterCutInManager::set_throw_finish_offset(0, 7, 0)
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

#[acmd_script( agent = "pichu", script = "game_throwlw", category = ACMD_GAME )]
unsafe fn pichu_dthrow(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 63, 80, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 1.0);
        SELF_DAMAGE_METER[entry_id] -= 1.0;
        macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 3.0, 361, 60, 0, 60, 5.9, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 1, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pichu", script = "game_specialn", category = ACMD_GAME )]
unsafe fn pichu_neutralspecial(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, 0);
        PICHU_NEUTRALSPECIAL_FROMGROUND[entry_id] = false;
    }
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 1.3);
        SELF_DAMAGE_METER[entry_id] -= 1.3;
    }
}

#[acmd_script( agent = "pichu", script = "game_specialairn", category = ACMD_GAME )]
unsafe fn pichu_neutralspecial_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PICHU_GENERATE_ARTICLE_DENGEKIDAMA, false, 0);
        PICHU_NEUTRALSPECIAL_FROMGROUND[entry_id] = false;
    }
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 1.3);
        SELF_DAMAGE_METER[entry_id] -= 1.3;
    }
}

#[acmd_script( agent = "pichu", script = "game_specials", category = ACMD_GAME )]
unsafe fn pichu_sidespecial(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_NONE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_ATTACK_TRIGGER);
        macros::FT_ADD_DAMAGE(fighter, 1.5);
        SELF_DAMAGE_METER[entry_id] -= 1.5;
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 90, 0, 15, 5.0, 0.0, 3.3, 4.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 9, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_HEAD);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_CALC_ATTACK_POWER);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PIKACHU_STATUS_WORK_ID_FLAG_SKULL_BASH_BRAKE_TRIGGER);
    }
}

#[acmd_script( agent = "pichu", script = "game_specialhistart", category = ACMD_GAME )]
unsafe fn pichu_upspecial_start(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
}

#[acmd_script( agent = "pichu", script = "sound_specialhistart", category = ACMD_SOUND )]
unsafe fn pichu_upspecialstart_sound(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_pichu_special_h01"));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pichu_special_h01"));
    }
}


#[acmd_script( agent = "pichu", script = "game_specialairhistart", category = ACMD_GAME )]
unsafe fn pichu_upspecial_start_air(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
}

#[acmd_script( agent = "pichu", script = "sound_specialairhistart", category = ACMD_SOUND )]
unsafe fn pichu_upspecialstart_air_sound(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("vc_pichu_special_h01"));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_pichu_special_h01"));
    }
}

#[acmd_script( agent = "pichu", script = "game_specialhi1", category = ACMD_GAME )]
unsafe fn pichu_upspecial_1(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::SEARCH(fighter, 0, 0, Hash40::new("top"), 5.5, 0.0, 4.0, 0.0, None, None, None, *COLLISION_KIND_MASK_ALL, *HIT_STATUS_MASK_NORMAL, 60, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false);
        macros::FT_ADD_DAMAGE(fighter, 0.7);
        SELF_DAMAGE_METER[entry_id] -= 0.7;
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pichu", script = "game_specialhi2", category = ACMD_GAME )]
unsafe fn pichu_upspecial_2(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 1.4);
        SELF_DAMAGE_METER[entry_id] -= 1.4;
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pichu", script = "game_specialairhi1", category = ACMD_GAME )]
unsafe fn pichu_upspecial_air_1(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 0.7);
        SELF_DAMAGE_METER[entry_id] -= 0.7;
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pichu", script = "game_specialairhi2", category = ACMD_GAME )]
unsafe fn pichu_upspecial_air_2(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 1.4);
        SELF_DAMAGE_METER[entry_id] -= 1.4;
        JostleModule::set_status(fighter.module_accessor, false);
    }
}

#[acmd_script( agent = "pichu", scripts = ["game_speciallwhit", "game_specialairlwhit"], category = ACMD_GAME, low_priority )]
unsafe fn pichu_downspecial_hit(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let thunder_damage;

    if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV4 {
        thunder_damage = 50.0;
    }
    else if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV3 {
        thunder_damage = 33.0;
    }
    else if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV2 {
        thunder_damage = 24.0;
    }
    else if SELF_DAMAGE_METER[entry_id] <= PICHU_CHARGE_LV1 {
        thunder_damage = 16.0;
    }
    else {
        thunder_damage = 11.0;
    }

    if macros::is_excute(fighter) {
        //FT_ADD_DAMAGE(SelfDamage=3.5)
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), thunder_damage, 361, 91, 0, 58, 14.0, 0.0, 10.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        if SELF_DAMAGE_METER[entry_id] < 0.0 && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            DamageModule::add_damage(fighter.module_accessor, SELF_DAMAGE_METER[entry_id], 0);
            //KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 2.0, y: 2.0, z: 0.0});
        }
        SELF_DAMAGE_METER[entry_id] = 0.0;
        PICHU_SELF_DAMAGE_GRACE[entry_id] = 0;

        macros::FT_MOTION_RATE(fighter, 1.25);
    }
}


#[acmd_script( agent = "pichu_dengekidama", script = "game_regular", category = ACMD_GAME )]
unsafe fn pichu_thunderjolt(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 130, 30, 0, 70, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.5, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -4.5, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
}

#[acmd_script( agent = "pichu_dengeki", script = "game_regular", category = ACMD_GAME )]
unsafe fn pichu_thunderjolt_ground(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 130, 30, 0, 70, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 130, 30, 0, 70, 10.0, 0.0, 0.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, -3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

#[acmd_script( agent = "pichu_kaminari", script = "game_regular", category = ACMD_GAME )]
unsafe fn pichu_thunder(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        //beans
    }
}

#[acmd_script( agent = "pichu_cloud", script = "game_regular", category = ACMD_GAME )]
unsafe fn pichu_thundercloud(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_PIKACHU_CLOUD_INSTANCE_WORK_ID_FLAG_ACTIVATE_KAMINARI);
    }
}

#[acmd_script( agent = "pichu_cloud", script = "effect_regular", category = ACMD_EFFECT )]
unsafe fn pichu_thundercloud_effect(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        //beans?
    }
}

#[acmd_script( agent = "pichu", script = "game_appealhil", category = ACMD_GAME )]
unsafe fn pichu_uptaunt_left_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 3.0);
        SELF_DAMAGE_METER[entry_id] -= 3.0;
    }
}

#[acmd_script( agent = "pichu", script = "game_appealhir", category = ACMD_GAME )]
unsafe fn pichu_uptaunt_right_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::FT_ADD_DAMAGE(fighter, 3.0);
        SELF_DAMAGE_METER[entry_id] -= 3.0;
    }
}

pub fn install() {
    smashline::install_agent_frames!(
        pichu_frame,
        kirby_pichuhat_frame
    );
    smashline::install_acmd_scripts!(
        pichu_shield,
        pichu_perfectshield,
        pichu_jab,
        pichu_dashattack,
        pichu_ftilt,
        pichu_utilt,
        pichu_dtilt,
        pichu_fsmash,
        pichu_dsmash,
        pichu_usmash,
        pichu_nair,
        pichu_fair,
        pichu_bair,
        pichu_uair,
        pichu_dair,
        pichu_dair_landing,
        pichu_grab,
        pichu_grabd,
        pichu_grabp,
        pichu_pummel,
        pichu_fthrow,
        pichu_bthrow,
        pichu_uthrow,
        pichu_dthrow,
        pichu_neutralspecial,
        pichu_neutralspecial_air,
        pichu_sidespecial,
        pichu_upspecial_start,
        pichu_upspecialstart_sound,
        pichu_upspecial_start_air,
        pichu_upspecialstart_air_sound,
        pichu_upspecial_1,
        pichu_upspecial_2,
        pichu_upspecial_air_1,
        pichu_upspecial_air_2,
        pichu_downspecial_hit,
        pichu_thunderjolt,
        pichu_thunderjolt_ground,
        pichu_thunder,
        pichu_thundercloud,
        pichu_thundercloud_effect,
        pichu_uptaunt_left_smash_script,
        pichu_uptaunt_right_smash_script
    );
}
