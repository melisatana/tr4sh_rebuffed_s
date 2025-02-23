#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash_script::*;
use smashline::*;
use crate::custom::global_fighter_frame;
use smashline::Priority::*;


static mut DARKSAMUS_ELECPOINTS_ACTIVE : [[bool; 5]; 8] = [[false; 5]; 8]; //whether the point exists
static mut DARKSAMUS_ELECPOINTS_X : [[f32; 5]; 8] = [[0.0; 5]; 8]; //the x co-ordinate of the point
static mut DARKSAMUS_ELECPOINTS_Y : [[f32; 5]; 8] = [[0.0; 5]; 8]; //the y co-ordinate of the point
static mut DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET : [[bool; 5]; 8] = [[false; 5]; 8]; //whether the point is currently being used as a target for side b
static mut DARKSAMUS_ELECPOINTS_NEXT_ID : [i32; 8] = [0; 8]; //the next id to put an elecpoint in (this is a bit unneccesary, could probably get rid of it later)
static mut DARKSAMUS_ELECPOINTS_UPSPECIAL_EXPLOSION_FRAME : [[i32; 5]; 8] = [[0; 5]; 8]; //the current frame of the up special point explosion

static mut DARKSAMUS_ELECPOINT_FRAME : [i32; 8] = [0; 8]; //the current frame for elecpoints (graphics for the point are only drawn once every 4 frames)
static mut DARKSAMUS_UPSPECIAL_TOPOINT : [bool; 8] = [false; 8]; //set to true when up special is locked on to a point
static mut DARKSAMUS_UPSPECIAL_XVEL : [f32; 8] = [0.0; 8]; //the current x velocity of up special if locked to a point
static mut DARKSAMUS_UPSPECIAL_YVEL : [f32; 8] = [0.0; 8]; //the current y velocity of up special if locked to a point
static mut DARKSAMUS_UPSPECIAL_HAS_CANCELLED : [bool; 8] = [false; 8]; //if an up special has already been cancelled before touching the ground
static mut DARKSAMUS_UPSPECIAL_WILL_CANCEL : [bool; 8] = [false; 8]; //if up special has detonated a point

static DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET : f32 = 14.0; //where the hitbox of sideb starts (x)
static DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET : f32 = 10.0; //where the hitbox of sideb starts (y)
static DARKSAMUS_SIDESPECIAL_MINIMUM_ANGLE : f32 = 0.66; //minimum value for the dot product of the stick vector and the point offset
static DARKSAMUS_SIDESPECIAL_EFFECT_DISTANCES : [f32; 5] = [8.0, 10.0, 12.0, 14.0, 16.0]; //separation between particles for side b lines
//static DARKSAMUS_SIDESPECIAL_EFFECT_DISTANCE : f32 = 8.0; //separation between particles for side b lines
static DARKSAMUS_UPSPECIAL_EFFECT_DISTANCE : f32 = 8.0; //separation between particles for up b lines

static mut DARKSAMUS_FSMASH_FULLCHARGE: [bool; 8] = [false; 8]; //full charge Fsmash shenanigans

unsafe fn is_any_elecpoint_active(fighter: &mut L2CAgentBase) -> bool {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    for i in 0..5 {
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][i as usize] {
            return true;
        }
    }
    return false;
}

unsafe fn is_any_sidespecial_target(fighter: &mut L2CAgentBase) -> bool {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    for i in 0..5 {
        if DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET[entry_id][i as usize] {
            return true;
        }
    }
    return false;
}

unsafe fn find_closest_elecpoint(fighter: &mut L2CAgentBase) -> i32 {
    let current_x = PostureModule::pos_x(fighter.module_accessor);
    let current_y = PostureModule::pos_y(fighter.module_accessor);
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut closest_distance = 12100.0; 
    let mut closest_id = -1;
    let mut this_distance;
    for i in 0..5 {
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][i as usize] {
            this_distance = (current_x - DARKSAMUS_ELECPOINTS_X[entry_id][i as usize]).powi(2) + (current_y - DARKSAMUS_ELECPOINTS_Y[entry_id][i as usize]).powi(2);
            if this_distance < closest_distance {
                closest_distance = this_distance;
                closest_id = i;
            }
        }
    }
    
    return closest_id;
}

fn get_scaled_dot_product(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    let magnitude_1 = (x1.powi(2) + y1.powi(2)).powf(0.5);
    let magnitude_2 = (x2.powi(2) + y2.powi(2)).powf(0.5);
    let dot = (x1 * x2) + (y1 * y2);
    return dot / (magnitude_1 * magnitude_2);
}

unsafe fn set_all_sideb_targets(fighter: &mut L2CAgentBase) {
    let current_x = PostureModule::pos_x(fighter.module_accessor);
    let current_y = PostureModule::pos_y(fighter.module_accessor);
    let stick_vector_x;
    let stick_vector_y;
    if ControlModule::get_stick_x(fighter.module_accessor).abs() < 0.2 && ControlModule::get_stick_y(fighter.module_accessor).abs() < 0.2 {
        stick_vector_x = PostureModule::lr(fighter.module_accessor);
        stick_vector_y = 0.0;
    }
    else {
        stick_vector_x = ControlModule::get_stick_x(fighter.module_accessor);
        stick_vector_y = ControlModule::get_stick_y(fighter.module_accessor);
    }
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut current_dot;
    let mut closest_dot = -1.0;
    let mut closest_id = -1;
    let mut any_point_in_angle = false;

    for i in 0..5 {
        DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET[entry_id][i as usize] = false;
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][i as usize] {
            current_dot = get_scaled_dot_product(stick_vector_x, stick_vector_y, DARKSAMUS_ELECPOINTS_X[entry_id][i as usize] - current_x, DARKSAMUS_ELECPOINTS_Y[entry_id][i as usize] - current_y);
            if current_dot > DARKSAMUS_SIDESPECIAL_MINIMUM_ANGLE {
                any_point_in_angle = true;
                DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET[entry_id][i as usize] = true;
            }
            else if any_point_in_angle == false && current_dot > closest_dot {
                closest_dot = current_dot;
                closest_id = i;
            }
        }
    }

    if any_point_in_angle == false {
        DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET[entry_id][closest_id as usize] = true;
    }
}

unsafe fn draw_sidespecial_lines(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut target_count = -1;

    for i in 0..5 {
        if DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET[entry_id][i as usize] {
            target_count = target_count + 1;
        }
    }

    for j in 0..5 {
        if DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET[entry_id][j as usize] {
            let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINTS_X[entry_id][j as usize] - PostureModule::pos_x(fighter.module_accessor));
            let point_offset_y = DARKSAMUS_ELECPOINTS_Y[entry_id][j as usize] - PostureModule::pos_y(fighter.module_accessor);
            let dist_sq = (point_offset_y - DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET).powi(2) + (point_offset_x - DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET).powi(2);
            let dist_true = dist_sq.powf(0.5);
            let dist_iterator = (dist_true / DARKSAMUS_SIDESPECIAL_EFFECT_DISTANCES[target_count as usize]).trunc() as i32;
            let normal_x = (point_offset_x - DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET) * dist_true.recip();
            let normal_y = (point_offset_y - DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET) * dist_true.recip();
            
            for i in 1..(dist_iterator+1) {
                macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + (i as f32) * normal_x * DARKSAMUS_SIDESPECIAL_EFFECT_DISTANCES[target_count as usize], DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET + (i as f32) * normal_y * DARKSAMUS_SIDESPECIAL_EFFECT_DISTANCES[target_count as usize], 0, 0, 0, 0, 0.25, 0, 0, 0, 0, 0, 0, false);
            }
        }
    }
}

unsafe fn set_sidespecial_hitboxes(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    for j in 0..5 {
        if DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET[entry_id][j as usize] {
            let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINTS_X[entry_id][j as usize] - PostureModule::pos_x(fighter.module_accessor));
            let point_offset_y = DARKSAMUS_ELECPOINTS_Y[entry_id][j as usize] - PostureModule::pos_y(fighter.module_accessor);
            macros::ATTACK(fighter, j, 0, Hash40::new("top"), 16.0, 60, 88, 0, 80, 5.0, 0.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, Some(0.0), Some(point_offset_y), Some(point_offset_x), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        }
    }
}

unsafe fn clear_sidespecial_targets(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    for j in 0..5 {
        if DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET[entry_id][j as usize] {
            DARKSAMUS_ELECPOINTS_SIDESPECIAL_TARGET[entry_id][j as usize] = false;
            DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] = false;
        }
    }

    reorder_points(entry_id);
}

unsafe fn draw_upspecial_line(fighter: &mut L2CAgentBase, point_id: i32) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINTS_X[entry_id][point_id as usize] - PostureModule::pos_x(fighter.module_accessor));
    let point_offset_y = DARKSAMUS_ELECPOINTS_Y[entry_id][point_id as usize] - PostureModule::pos_y(fighter.module_accessor);
    let dist_sq = (point_offset_y - 5.0).powi(2) + (point_offset_x - 0.0).powi(2);
    let dist_true = dist_sq.powf(0.5);
    let dist_iterator = (dist_true / DARKSAMUS_UPSPECIAL_EFFECT_DISTANCE).trunc() as i32;
    let normal_x = (point_offset_x - 0.0) * dist_true.recip();
    let normal_y = (point_offset_y - 5.0) * dist_true.recip();
            
    for i in 1..(dist_iterator+1) {
        macros::EFFECT(fighter, Hash40::new("samusd_gbeam_lightning"), Hash40::new("top"), 0.0 + (i as f32) * normal_x * DARKSAMUS_UPSPECIAL_EFFECT_DISTANCE, 5.0 + (i as f32) * normal_y * DARKSAMUS_UPSPECIAL_EFFECT_DISTANCE, 0, 0, 90, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
    } 
}

unsafe fn upspecial_check_to_explode_points(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let current_x = PostureModule::pos_x(fighter.module_accessor);
    let current_y = PostureModule::pos_y(fighter.module_accessor) + 5.0;
    let mut this_distance;

    for j in 0..5 {
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] && DARKSAMUS_ELECPOINTS_UPSPECIAL_EXPLOSION_FRAME[entry_id][j as usize] == 0 {
            this_distance = (current_x - DARKSAMUS_ELECPOINTS_X[entry_id][j as usize]).powi(2) + (current_y - DARKSAMUS_ELECPOINTS_Y[entry_id][j as usize]).powi(2);
            if this_distance < 6.0 * 6.0 {
                DARKSAMUS_ELECPOINTS_UPSPECIAL_EXPLOSION_FRAME[entry_id][j as usize] = 1;
                DARKSAMUS_UPSPECIAL_WILL_CANCEL[entry_id] = true;

                let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINTS_X[entry_id][j as usize] - PostureModule::pos_x(fighter.module_accessor));
                let point_offset_y = DARKSAMUS_ELECPOINTS_Y[entry_id][j as usize] - PostureModule::pos_y(fighter.module_accessor);
            
                macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 2.0, 0, 0, 0, 0, 0, 0, false);
                macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 1.0, 0, 0, 0, 0, 0, 0, false);
            }
        }
    }
}

unsafe fn set_upspecial_hitboxes(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    for j in 0..5 {
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] && DARKSAMUS_ELECPOINTS_UPSPECIAL_EXPLOSION_FRAME[entry_id][j as usize] > 0 {
            DARKSAMUS_ELECPOINTS_UPSPECIAL_EXPLOSION_FRAME[entry_id][j as usize] += 1;
            if DARKSAMUS_ELECPOINTS_UPSPECIAL_EXPLOSION_FRAME[entry_id][j as usize] >= 5 {
                AttackModule::clear(fighter.module_accessor, j + 1, false);
                //DARKSAMUS_ELECPOINTS_UPSPECIAL_EXPLOSION_FRAME[entry_id] = 0;
                DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] = false;
            }
            else {
                let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINTS_X[entry_id][j as usize] - PostureModule::pos_x(fighter.module_accessor));
                let point_offset_y = DARKSAMUS_ELECPOINTS_Y[entry_id][j as usize] - PostureModule::pos_y(fighter.module_accessor);
                
                macros::ATTACK(fighter, (j + 1) as u64, 1, Hash40::new("top"), 13.0, 90, 98, 0, 80, 18.0, 0.0, point_offset_y, point_offset_x, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_paralyze"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
                AttackModule::set_no_finish_camera(fighter.module_accessor, (j + 1), true, false);
            }
        }
    }
}

unsafe fn clear_upspecial_targets(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    for j in 0..5 {
        if DARKSAMUS_ELECPOINTS_UPSPECIAL_EXPLOSION_FRAME[entry_id][j as usize] > 0 {
            DARKSAMUS_ELECPOINTS_UPSPECIAL_EXPLOSION_FRAME[entry_id][j as usize] = 0;
            DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] = false;
        }
    }

    reorder_points(entry_id);
}

unsafe fn draw_downtaunt_explosions(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    for j in 0..5 {
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] {
            let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINTS_X[entry_id][j as usize] - PostureModule::pos_x(fighter.module_accessor));
            let point_offset_y = DARKSAMUS_ELECPOINTS_Y[entry_id][j as usize] - PostureModule::pos_y(fighter.module_accessor);
            
            macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 1.5, 0, 0, 0, 0, 0, 0, false);
            macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
        }
    }
}

unsafe fn set_downtaunt_hitboxes(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    for j in 0..5 {
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] {
            let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINTS_X[entry_id][j as usize] - PostureModule::pos_x(fighter.module_accessor));
            let point_offset_y = DARKSAMUS_ELECPOINTS_Y[entry_id][j as usize] - PostureModule::pos_y(fighter.module_accessor);
            
            macros::ATTACK(fighter, j, 0, Hash40::new("top"), 9.5, 361, 38, 0, 80, 14.0, 0.0, point_offset_y, point_offset_x, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BOMB, *ATTACK_REGION_BOMB);
            AttackModule::set_poison_param(fighter.module_accessor, j as i32, 600, 30, 0.7, false);
        }
    }
}

unsafe fn clear_all_points(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    for j in 0..5 {
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] {
            DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] = false;
        }
    }
    DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] = 0;
}

unsafe fn reorder_points(entry_id: usize) {
    for i in 0..4 {
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][i as usize] == false {
            for j in i+1..5 {
                if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] {
                    DARKSAMUS_ELECPOINTS_X[entry_id][i as usize] = DARKSAMUS_ELECPOINTS_X[entry_id][j as usize];
                    DARKSAMUS_ELECPOINTS_Y[entry_id][i as usize] = DARKSAMUS_ELECPOINTS_Y[entry_id][j as usize];
                    DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][i as usize] = true;
                    DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][j as usize] = false;
                    break;
                }
            }
        }
    }

    DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] = -1;

    for i in 0..5 {
        if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][i as usize] == false {
            DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] = i;
            return;
        }
    }
    DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] = 4;
}

unsafe fn add_new_elecpoint(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] >= 4 && DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][4 as usize] {
        DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][0 as usize] = false;
        reorder_points(entry_id);
    }

    DARKSAMUS_ELECPOINTS_X[entry_id][DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] as usize] = PostureModule::pos_x(fighter.module_accessor);
    DARKSAMUS_ELECPOINTS_Y[entry_id][DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] as usize] = PostureModule::pos_y(fighter.module_accessor) + 3.0;
    DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] as usize] = true;
    DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] = DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] + 1;
}

// A Once-Per-Fighter-Frame that only applies to Samus
unsafe extern "C" fn dark_samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        global_fighter_frame(fighter);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);
        

        //println!("It'sa me, dark samus, brrrrrrrrrro!");

        if sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status) {
			clear_all_points(fighter);
            DARKSAMUS_UPSPECIAL_WILL_CANCEL[entry_id] = false;
		};

        if is_any_elecpoint_active(fighter) {
            DARKSAMUS_ELECPOINT_FRAME[entry_id] += 1;
            if DARKSAMUS_ELECPOINT_FRAME[entry_id] >= 4
            {
                for i in 0..5 {
                    if DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][i as usize] {
                        let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINTS_X[entry_id][i as usize] - PostureModule::pos_x(fighter.module_accessor));
                        let point_offset_y = DARKSAMUS_ELECPOINTS_Y[entry_id][i as usize] - PostureModule::pos_y(fighter.module_accessor);
                        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), point_offset_x, point_offset_y, 0, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, false);
                    }
                }
                DARKSAMUS_ELECPOINT_FRAME[entry_id] = 0;
            }
            if DARKSAMUS_UPSPECIAL_TOPOINT[entry_id] {
                if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status) {
                    /*
                    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND && MotionModule::frame(fighter.module_accessor) < 7.0 && DARKSAMUS_UPSPECIAL_YVEL[entry_id] > 0.0 {
                        let pos = smash::phx::Vector3f { x: PostureModule::pos_x(fighter.module_accessor), y: PostureModule::pos_y(fighter.module_accessor)+8.0, z: 0.0 }; // This puts dark sammus 8 units above the ground
                        PostureModule::set_pos(fighter.module_accessor, &pos);
                        PostureModule::init_pos(fighter.module_accessor, &pos, true, true);
                    }
                    */
                    KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_MOTION_FALL);
                    macros::SET_SPEED_EX(fighter, DARKSAMUS_UPSPECIAL_XVEL[entry_id], DARKSAMUS_UPSPECIAL_YVEL[entry_id], *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
                    if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) {
                        upspecial_check_to_explode_points(fighter);
                    }
                    set_upspecial_hitboxes(fighter);
                }
                else {
                    DARKSAMUS_UPSPECIAL_TOPOINT[entry_id] = false;
                    macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_lightning"), true, true);
                }
            }
        }
        if DARKSAMUS_UPSPECIAL_WILL_CANCEL[entry_id] {
            if [*FIGHTER_STATUS_KIND_SPECIAL_HI].contains(&status) {
                
            }
            else {
                clear_upspecial_targets(fighter);
                DARKSAMUS_UPSPECIAL_WILL_CANCEL[entry_id] = false;
                macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_lightning"), true, true);
            }
        }
        if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_GROUND || sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status) {
			DARKSAMUS_UPSPECIAL_HAS_CANCELLED[entry_id] = false;
		};
    }
}

unsafe extern "C" fn dark_samus_jab(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.6, 361, 10, 25, 25, 3.5, 0.0, 10.0, 4.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.6, 361, 10, 25, 25, 3.8, 0.0, 10.0, 8.8, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 4.6, 110, 10, 25, 25, 4.0, 0.0, 10.0, 13.0, None, None, None, 0.8, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 7.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 7.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 7.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

unsafe extern "C" fn dark_samus_jab2(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_INVINCIBLE), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 7.0, 361, 114, 0, 45, 5.0, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 7.0, 361, 114, 0, 45, 4.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
}

unsafe extern "C" fn dark_samus_dashattack(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.6);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 9.9, 67, 70, 0, 60, 6.1, 0.0, 10.0, 6.0, Some(0.0), Some(7.5), Some(3.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_BODY);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

unsafe extern "C" fn dark_samus_ftilt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.7, 120, 60, 0, 64, 4.2, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.7, 120, 60, 0, 64, 3.9, 0.6, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 8.7, 120, 60, 0, 64, 3.9, 6.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 1.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 1.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 2, 1.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
}

unsafe extern "C" fn dark_samus_utilt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 12.5, 0, 76, 0, 30, 6.3, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 12.5, 0, 76, 0, 30, 6.3, 6.4, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 12.5, 0, 76, 0, 30, 6.3, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("kneer"), 12.5, 0, 76, 0, 30, 6.3, 6.4, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.05);
    }
}

unsafe extern "C" fn dark_samus_dtilt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 9.8, 79, 60, 0, 60, 3.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 9.8, 79, 60, 0, 60, 7.2, 0.0, 1.6, 14.4, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.4, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 1.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 1.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn dark_samus_fsmash_charge(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        DARKSAMUS_FSMASH_FULLCHARGE[entry_id] = true ;
        macros::PLAY_SE(fighter, Hash40::new("se_common_spirits_critical_l_tail"));
        //macros::PLAY_SE(fighter, Hash40::new("se_common_assist_escapeair"));
    }
}

unsafe extern "C" fn dark_samus_fsmash(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        DARKSAMUS_FSMASH_FULLCHARGE[entry_id] = false ;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 13.3, 361, 112, 0, 30, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 13.3, 361, 112, 0, 30, 3.3, -4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("armr"), 16.6, 361, 100, 0, 40, 5.3, 7.8, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
        AttackModule::set_poison_param(fighter.module_accessor, 2, 300, 30, 0.4, false);
        if DARKSAMUS_FSMASH_FULLCHARGE[entry_id] {
            macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 22.387, -0.341, -0.169, 0.0, 0.0, 0.0, 1.3, 0.0, 0, 0, 0, 0, 0, true);
            macros::ATTACK(fighter, 3, 0, Hash40::new("armr"), 16.6, 361, 100, 0, 40, 5.3, 15.8, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            macros::EFFECT(fighter, Hash40::new("samusd_atk_bomb"), Hash40::new("armr"), 30.387, -0.341, -0.169, 0.0, 0.0, 0.0, 1.3, 0.0, 0, 0, 0, 0, 0, true);
            macros::ATTACK(fighter, 4, 0, Hash40::new("armr"), 16.6, 361, 100, 0, 40, 5.3, 23.8, 0.0, 0.0, None, None, None, 1.25, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_PUNCH);
            AttackModule::set_poison_param(fighter.module_accessor, 3, 300, 30, 0.4, false);
            AttackModule::set_poison_param(fighter.module_accessor, 4, 300, 30, 0.4, false);
        }
        
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
}

unsafe extern "C" fn dark_samus_usmash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("armr"), HitStatus(*HIT_STATUS_XLU), 0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 17.7, 80, 95, 0, 50, 4.5, -3.5, 0.0, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 17.7, 80, 95, 0, 50, 6.5, 7.8, -0.5, 0.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
}

unsafe extern "C" fn dark_samus_dsmash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 13.0, 110, 80, 0, 90, 4.6, 8.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 13.0, 110, 80, 0, 90, 4.6, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 15.3, 110, 78, 0, 90, 4.6, 8.0, -0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 15.3, 110, 78, 0, 90, 4.6, -2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.3, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
}

unsafe extern "C" fn dark_samus_nair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 11.6, 32, 100, 0, 40, 5.5, 5.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 11.6, 32, 100, 0, 40, 5.5, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 14.6, 40, 100, 0, 40, 5.3, 5.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 14.6, 40, 100, 0, 40, 5.3, 0.0, 1.0, 1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn dark_samus_fair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        MotionModule::set_rate(fighter.module_accessor, 2.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("armr"), 12.6, 37, 92, 0, 40, 5.5, 2.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        macros::ATTACK(fighter, 1, 0, Hash40::new("armr"), 12.6, 37, 92, 0, 40, 7.2, 9.0, 0.0, 2.0, None, None, None, 0.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 41.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn dark_samus_bair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.6);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.5);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 11.5, 361, 98, 0, 42, 4.5, 2.5, 0.0, 0.0, None, None, None, 1.1, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 19.7, 35, 74, 0, 55, 3.4, 6.5, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        macros::EFFECT(fighter, Hash40::new("sys_hit_magic_s"), Hash40::new("kneer"), 0.0, 0.0, 6.5, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 6.6, 361, 90, 0, 20, 4.5, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 6.6, 361, 90, 0, 20, 4.5, 6.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 1, 2.0, false);    
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    MotionModule::set_rate(fighter.module_accessor, 1.1);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 39.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn dark_samus_uair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 10.4, 90, 80, 0, 40, 6.8, -3.0, 1.0, 2.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("toer"), 17.4, 90, 100, 0, 40, 4.0, 1.0, 0.0, 0.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
        
        macros::EFFECT(fighter, Hash40::new("sys_hit_magic_s"), Hash40::new("toer"), 0.0, 0.0, 1.0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 7.4, 80, 50, 0, 90, 6.8, -3.0, 1.0, 2.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("toer"), 7.4, 80, 50, 0, 90, 5.7, 1.0, 0.0, 0.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 0.9);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn dark_samus_dair(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 14.3, 270, 90, 0, 34, 4.7, 0.0, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 14.3, 270, 90, 0, 34, 4.5, 0.6, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 16.0, 270, 90, 0, 34, 4.5, 6.0, 0.0, -0.5, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_LL, *COLLISION_SOUND_ATTR_MAGIC, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 8.4, 76, 90, 0, 39, 4.4, 0.0, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.4, 76, 90, 0, 39, 4.1, 0.6, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneer"), 8.4, 76, 90, 0, 39, 4.1, 6.0, 0.0, -0.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    } 
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn dark_samus_dair_sound(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_swing_l"));
    }
}

unsafe extern "C" fn dark_samus_dair_effect(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("legr"), 4.289, -0.272, -0.135, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 360, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_attack_line"), Hash40::new("top"), 0, 9, 3.5, 90, 0, 0, 1.05, true);
        macros::LAST_EFFECT_SET_RATE(fighter, 0.8);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_hit_magic_s"), Hash40::new("toer"), 0.0, 0.0, 1.0, 0, 0, 0, 0.2, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("hip"), -2, 0, 0, 0, 0, 0, 2.5, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonellm"), 2, 0, 0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneer"), 0, 0, -0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("armr"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("colonells"), 2, 0, -0.5, 0, 0, 0, 2, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("kneel"), 0, 0, 0.5, 0, 0, 0, 1.7, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 2.1, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("arml"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("samusd_win3_aura"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 1.9, true);
        macros::BURN_COLOR(fighter, 0.26, 0.71, 1.5, 0.7);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 54.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_win3_aura"), false, true);
        macros::BURN_COLOR_FRAME(fighter, 20, 1, 1, 1, 0);
        macros::BURN_COLOR_NORMAL(fighter);
    }
}

unsafe extern "C" fn dark_samus_grab(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 4.6, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.4, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 51.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn dark_samus_grabd(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 4.6, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.4, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn dark_samus_grabp(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 4.6, 0.0, 0.0, 0.0, Some(0.0), Some(0.0), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.4, 0.0, 7.5, -18.0, Some(0.0), Some(7.5), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 31.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 56.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn dark_samus_zair(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_AIR_LASSO_FLAG_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("throw"), 5.5, 124, 50, 0, 30, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("throw"), 8.5, 124, 60, 0, 60, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 4, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_ENERGY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
        ArticleModule::set_rate(fighter.module_accessor, *FIGHTER_SAMUSD_GENERATE_ARTICLE_GBEAM, 1.3);
        if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            CancelModule::enable_cancel(fighter.module_accessor);
        }
    }
}

unsafe extern "C" fn dark_samus_fthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.5, 42, 30, 0, 90, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 16, 16);
        //FighterCutInManager::set_throw_finish_zoom_rate(1.2)
        //FighterCutInManager::set_throw_finish_offset(5, 5, 0)
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}
        
unsafe extern "C" fn dark_samus_bthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 40, 119, 0, 30, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(fighter, 25, 15);
        //FighterCutInManager::set_throw_finish_zoom_rate(1.2)
        //FighterCutInManager::set_throw_finish_offset(10, 3, 0)
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn dark_samus_uthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.7, 90, 85, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 80, 100, 0, 75, 6.5, 0.0, 23.0, -1.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_NONE);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 1, 32);
        //FighterCutInManager::set_throw_finish_zoom_rate(1.2)
        //FighterCutInManager::set_throw_finish_offset(0, 5, 0)
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.8);
    }
}

unsafe extern "C" fn dark_samus_dthrow(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 7.1, 285, 77, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_elec_whip"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 17, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_INSTANCE_WORK_ID_FLAG_ARTICLE_MOTION_RATE_SYNC);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn dark_samus_sidespecial(fighter: &mut L2CAgentBase) {
    //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        //WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON)
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 5.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 10.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 15.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 20.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 25.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 30.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.4, 120, 58, 0, 80, 5.0, 0.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, Some(0.0), Some(DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET), Some(DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 30.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.7692307);
    }
}

unsafe extern "C" fn dark_samus_sidespecial_air(fighter: &mut L2CAgentBase) {
    //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        //WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON)
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 5.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 10.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 15.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 20.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 25.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT(fighter, Hash40::new("sys_hit_elec_s"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 30.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
            
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.4, 120, 58, 0, 80, 5.0, 0.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, Some(0.0), Some(DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET), Some(DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + 30.0), 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        AttackModule::set_add_reaction_frame(fighter.module_accessor, 0, 4.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::FT_MOTION_RATE(fighter, 0.7692307);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn dark_samus_sidespecial_super(fighter: &mut L2CAgentBase) {
    //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        //WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON)
        if is_any_elecpoint_active(fighter) {
            set_all_sideb_targets(fighter);
        }
        if is_any_sidespecial_target(fighter) {
            /* 
            let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor));
            let point_offset_y = DARKSAMUS_ELECPOINT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor);
            let dist_sq = (point_offset_y - DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET).powi(2) + (point_offset_x - DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET).powi(2);
            let dist_true = dist_sq.powf(0.5);
            let dist_iterator = (dist_true * 0.2).trunc() as i32;
            let normal_x = (point_offset_x - DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET) * dist_true.recip();
            let normal_y = (point_offset_y - DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET) * dist_true.recip();
            
            for i in 1..(dist_iterator+1) {
                macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + (i as f32) * normal_x * 5.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET + (i as f32) * normal_y * 5.0, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
            }
            */

            draw_sidespecial_lines(fighter);

            set_sidespecial_hitboxes(fighter);
        }
        else {
            macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 60, 88, 0, 80, 8.0, 0.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        if is_any_sidespecial_target(fighter) {
            //macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 60, 88, 0, 80, 5.0, 0.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, Some(0.0), Some(DARKSAMUS_ELECPOINT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor)), Some(PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor))), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
            set_sidespecial_hitboxes(fighter);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        if is_any_sidespecial_target(fighter) {
            //macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 60, 88, 0, 80, 5.0, 0.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, Some(0.0), Some(DARKSAMUS_ELECPOINT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor)), Some(PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor))), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
            set_sidespecial_hitboxes(fighter);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        //DARKSAMUS_HAS_ELECPOINT[entry_id] = false;
        clear_sidespecial_targets(fighter);
    }
}

unsafe extern "C" fn dark_samus_sidespecial_air_super(fighter: &mut L2CAgentBase) {
    //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        //WorkModule::on_flag(Flag=FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_WEAPON)
        if is_any_elecpoint_active(fighter) {
            set_all_sideb_targets(fighter);
        }
        if is_any_sidespecial_target(fighter) {
            /* 
            let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor));
            let point_offset_y = DARKSAMUS_ELECPOINT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor);
            let dist_sq = (point_offset_y - DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET).powi(2) + (point_offset_x - DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET).powi(2);
            let dist_true = dist_sq.powf(0.5);
            let dist_iterator = (dist_true * 0.2).trunc() as i32;
            let normal_x = (point_offset_x - DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET) * dist_true.recip();
            let normal_y = (point_offset_y - DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET) * dist_true.recip();
            
            for i in 1..(dist_iterator+1) {
                macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET + (i as f32) * normal_x * 5.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET + (i as f32) * normal_y * 5.0, 0, 0, 0, 0, 0.2, 0, 0, 0, 0, 0, 0, false);
            }
            */

            draw_sidespecial_lines(fighter);

            set_sidespecial_hitboxes(fighter);
        }
        else {
            macros::EFFECT(fighter, Hash40::new("sys_hit_elec"), Hash40::new("top"), DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, 0, 0, 0, 0, 0.4, 0, 0, 0, 0, 0, 0, false);
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 60, 88, 0, 80, 8.0, 0.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        if is_any_sidespecial_target(fighter) {
            //macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 60, 88, 0, 80, 5.0, 0.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, Some(0.0), Some(DARKSAMUS_ELECPOINT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor)), Some(PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor))), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
            set_sidespecial_hitboxes(fighter);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        if is_any_sidespecial_target(fighter) {
            //macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.0, 60, 88, 0, 80, 5.0, 0.0, DARKSAMUS_SIDESPECIAL_HITBOX_Y_OFFSET, DARKSAMUS_SIDESPECIAL_HITBOX_X_OFFSET, Some(0.0), Some(DARKSAMUS_ELECPOINT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor)), Some(PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor))), 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, -6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
            set_sidespecial_hitboxes(fighter);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        //DARKSAMUS_HAS_ELECPOINT[entry_id] = false;
        clear_sidespecial_targets(fighter);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 35.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_S_WORK_FLAG_AIR_CONTROL);
    }
}

unsafe extern "C" fn dark_samus_upspecial(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        //WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.7, 85, 93, 0, 56, 8.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        GroundModule::set_passable_check(fighter.module_accessor, true);
        /* 
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && DARKSAMUS_HAS_ELECPOINT[entry_id] {
            let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor));
            let point_offset_y = DARKSAMUS_ELECPOINT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor);
            let dist_sq = point_offset_y.powi(2) + point_offset_x.powi(2);
            let dist_true = dist_sq.powf(0.5);

            DARKSAMUS_UPSPECIAL_XVEL[entry_id] = point_offset_x * dist_true.recip() * 3.0;
            DARKSAMUS_UPSPECIAL_YVEL[entry_id] = point_offset_y * dist_true.recip() * 3.0;
            DARKSAMUS_UPSPECIAL_TOPOINT[entry_id] = true;
        }
        else {
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_ACC_X);
        }
        */
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        if DARKSAMUS_UPSPECIAL_TOPOINT[entry_id] {
            DARKSAMUS_UPSPECIAL_TOPOINT[entry_id] = false;
            //DARKSAMUS_HAS_ELECPOINT[entry_id] = false;
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        GroundModule::set_passable_check(fighter.module_accessor, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn dark_samus_upspecial_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        DARKSAMUS_UPSPECIAL_WILL_CANCEL[entry_id] = false;
        //WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.7, 85, 93, 0, 56, 8.0, 0.0, 6.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BODY);
        GroundModule::set_passable_check(fighter.module_accessor, true);
        if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) && is_any_elecpoint_active(fighter) {
            let target_id = find_closest_elecpoint(fighter);
            if target_id > -1 {
                let point_offset_x = PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINTS_X[entry_id][target_id as usize] - PostureModule::pos_x(fighter.module_accessor));
                let point_offset_y = DARKSAMUS_ELECPOINTS_Y[entry_id][target_id as usize] - PostureModule::pos_y(fighter.module_accessor);
                let dist_sq = point_offset_y.powi(2) + point_offset_x.powi(2);
                let dist_true = dist_sq.powf(0.5);

                DARKSAMUS_UPSPECIAL_XVEL[entry_id] = point_offset_x * dist_true.recip() * 3.0;
                DARKSAMUS_UPSPECIAL_YVEL[entry_id] = point_offset_y * dist_true.recip() * 3.0;
                DARKSAMUS_UPSPECIAL_TOPOINT[entry_id] = true;

                draw_upspecial_line(fighter, target_id);
            }
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        //WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_HI_FLAG_DISABLE_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 27.0);
    if macros::is_excute(fighter) {
        if DARKSAMUS_UPSPECIAL_TOPOINT[entry_id] {
            DARKSAMUS_UPSPECIAL_TOPOINT[entry_id] = false;
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_lightning"), true, true);
            //DARKSAMUS_HAS_ELECPOINT[entry_id] = false;
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        GroundModule::set_passable_check(fighter.module_accessor, false);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        if DARKSAMUS_UPSPECIAL_WILL_CANCEL[entry_id] {
            clear_upspecial_targets(fighter);
            DARKSAMUS_UPSPECIAL_WILL_CANCEL[entry_id] = false;
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("samusd_gbeam_lightning"), true, true);
            if DARKSAMUS_UPSPECIAL_HAS_CANCELLED[entry_id] == false {
                DARKSAMUS_UPSPECIAL_HAS_CANCELLED[entry_id] = true;
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_FALL, false);
            }
        }
    }
}

unsafe extern "C" fn dark_samus_downspecial(fighter: &mut L2CAgentBase) {
    //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
        macros::FT_MOTION_RATE(fighter, 1.08695652);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_JUMP);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        //if DARKSAMUS_HAS_ELECPOINT[entry_id] {
        //    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 90, 70, 0, 56, 10.0, 0.0, DARKSAMUS_ELECPOINT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor), PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor)), None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        //}
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("body").hash as i64, Hash40::new("body_sphere").hash as i64);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        //DARKSAMUS_ELECPOINT_X[entry_id] = PostureModule::pos_x(fighter.module_accessor);
        //DARKSAMUS_ELECPOINT_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor) + 3.0;
        //DARKSAMUS_HAS_ELECPOINT[entry_id] = true;

        //DARKSAMUS_ELECPOINTS_X[entry_id][DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] as usize] = PostureModule::pos_x(fighter.module_accessor);
        //DARKSAMUS_ELECPOINTS_Y[entry_id][DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] as usize] = PostureModule::pos_y(fighter.module_accessor) + 3.0;
        //DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] as usize] = true;
        //DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] = (DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] + 1) % 5;

        add_new_elecpoint(fighter);
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_special_l01"));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("body").hash as i64, Hash40::new("body_normal").hash as i64);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_CHK_CROUCH);
        macros::FT_MOTION_RATE(fighter, 0.6);
    }
}

unsafe extern "C" fn dark_samus_downspecial_air(fighter: &mut L2CAgentBase) {
    //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.08695652);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        //if DARKSAMUS_HAS_ELECPOINT[entry_id] {
        //    macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 90, 70, 0, 56, 10.0, 0.0, DARKSAMUS_ELECPOINT_Y[entry_id] - PostureModule::pos_y(fighter.module_accessor), PostureModule::lr(fighter.module_accessor) * (DARKSAMUS_ELECPOINT_X[entry_id] - PostureModule::pos_x(fighter.module_accessor)), None, None, None, 0.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_elec"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_ELEC, *ATTACK_REGION_BOMB);
        //}
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("body").hash as i64, Hash40::new("body_sphere").hash as i64);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
        //DARKSAMUS_ELECPOINT_X[entry_id] = PostureModule::pos_x(fighter.module_accessor);
        //DARKSAMUS_ELECPOINT_Y[entry_id] = PostureModule::pos_y(fighter.module_accessor) + 2.0;
        //DARKSAMUS_HAS_ELECPOINT[entry_id] = true;

        //DARKSAMUS_ELECPOINTS_X[entry_id][DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] as usize] = PostureModule::pos_x(fighter.module_accessor);
        //DARKSAMUS_ELECPOINTS_Y[entry_id][DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] as usize] = PostureModule::pos_y(fighter.module_accessor) + 3.0;
        //DARKSAMUS_ELECPOINTS_ACTIVE[entry_id][DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] as usize] = true;
        //DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] = (DARKSAMUS_ELECPOINTS_NEXT_ID[entry_id] + 1) % 5;

        add_new_elecpoint(fighter);
        macros::PLAY_SE(fighter, Hash40::new("se_samusd_special_l01"));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_SAMUS_STATUS_SPECIAL_LW_FLAG_MV);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("body").hash as i64, Hash40::new("body_normal").hash as i64);
    }
}

unsafe extern "C" fn dark_samus_downtaunt(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 28.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 0.5);
        if is_any_elecpoint_active(fighter) {
            macros::PLAY_SE(fighter, Hash40::new("se_common_bomb_l"));
        }
        draw_downtaunt_explosions(fighter);
        set_downtaunt_hitboxes(fighter);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 0.499999);
    if macros::is_excute(fighter) {
        set_downtaunt_hitboxes(fighter);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 0.499999);
    if macros::is_excute(fighter) {
        set_downtaunt_hitboxes(fighter);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 0.499999);
    if macros::is_excute(fighter) {
        set_downtaunt_hitboxes(fighter);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        clear_all_points(fighter);
        MotionModule::set_rate(fighter.module_accessor, 0.75);
    }
}

unsafe extern "C" fn dark_samus_chargeshot(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.5, 361, 42, 0, 14, 1.9, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 20.5, 40, 72, 0, 46, 8.0, 0.0, 0.0, 0.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_curse_poison"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_ENERGY);
        attack!(fighter, *MA_MSC_CMD_ATTACK_SET_LERP, 0, 1);
        AttackModule::set_poison_param(fighter.module_accessor, 1, 300, 30, 0.8, false);
        GroundModule::set_collidable(fighter.module_accessor, false);
    }
}

//#[skyline::main(name = "tr4sh_rebuffed")]
pub fn install() {
    Agent::new("samusd")
      .on_line(Main, dark_samus_frame) //opff
      .game_acmd("game_attack11", dark_samus_jab, Low)
      .game_acmd("game_attack12", dark_samus_jab2, Low)
      .game_acmd("game_attackdash", dark_samus_dashattack, Low)
      .game_acmd("game_attacks3", dark_samus_ftilt, Low)
      .game_acmd("game_attacks3hi", dark_samus_ftilt, Low)
      .game_acmd("game_attacks3lw", dark_samus_ftilt, Low)
      .game_acmd("game_attackhi3", dark_samus_utilt, Low)
      .game_acmd("game_attacklw3", dark_samus_dtilt, Low)
      .game_acmd("game_attacks4charge", dark_samus_fsmash_charge, Low)
      .game_acmd("game_attacks4", dark_samus_fsmash, Low)
      .game_acmd("game_attacks4hi", dark_samus_fsmash, Low)
      .game_acmd("game_attacks4lw", dark_samus_fsmash, Low)
      .game_acmd("game_attackhi4", dark_samus_usmash, Low)
      .game_acmd("game_attacklw4", dark_samus_dsmash, Low)
      .game_acmd("game_attackairn", dark_samus_nair, Low)
      .game_acmd("game_attackairf", dark_samus_fair, Low)
      .game_acmd("game_attackairb", dark_samus_bair, Low)
      .game_acmd("game_attackairhi", dark_samus_uair, Low)
      .game_acmd("game_attackairlw", dark_samus_dair, Low)
      .effect_acmd("effect_attackairlw", dark_samus_dair_effect, Low)
      .sound_acmd("sound_attackairlw", dark_samus_dair_sound, Low)
      .game_acmd("game_catch", dark_samus_grab, Low)
      .game_acmd("game_catchdash", dark_samus_grabd, Low)
      .game_acmd("game_catchturn", dark_samus_grabp, Low)
      .game_acmd("game_aircatch", dark_samus_zair, Low)
      .game_acmd("game_throwf", dark_samus_fthrow, Low)
      .game_acmd("game_throwb", dark_samus_bthrow, Low)
      .game_acmd("game_throwhi", dark_samus_uthrow, Low)
      .game_acmd("game_throwlw", dark_samus_dthrow, Low)
      .game_acmd("game_special", dark_samus_sidespecial, Low)
      .game_acmd("game_specialair", dark_samus_sidespecial_air, Low)
      .game_acmd("game_specials", dark_samus_sidespecial_super, Low)
      .game_acmd("game_specialairs", dark_samus_sidespecial_air_super, Low)
      .game_acmd("game_specialhi", dark_samus_upspecial, Low)
      .game_acmd("game_specialairhi", dark_samus_upspecial_air, Low)
      .game_acmd("game_speciallw", dark_samus_downspecial, Low)
      .game_acmd("game_specialairlw", dark_samus_downspecial_air, Low)
      .game_acmd("game_appeallwl", dark_samus_downtaunt, Low)
      .game_acmd("game_appeallwr", dark_samus_downtaunt, Low)
      .install();
  
      Agent::new("samusd_cshot")
      .game_acmd("game_shoot", dark_samus_chargeshot, Low)
      .install();
      
      
}