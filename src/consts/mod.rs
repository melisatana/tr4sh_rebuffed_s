use smash::hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smashline::*;
use smash::app::*;
use smash::phx::Hash40;
use smash_script::*;
use smash::lib::L2CValue;
use smash::phx::Vector3f;
use skyline::nn::ro::LookupSymbol;
//use super::*;
//use crate::customparam::BomaExt;




//start of custom consts
pub const FIGHTER_STATUS_ATTACK_WORK_FLAG_HITFALL : i32 = 495;
//pub const FIGHTER_STATUS_ATTACK_WORK_FLAG_CRITICAL: i32 = 495; //0x100000b;

//end of custom consts lmfao


//start of custom functions


pub static mut IS_HITFALL : [bool; 8] = [false; 8];


//hitfall function, allows fastfalling when an attack lands
pub unsafe fn common_attack_hitfall_flag(fighter: &mut L2CFighterCommon) {
    let status = StatusModule::status_kind(fighter.module_accessor);
    if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_GROUND || [*FIGHTER_STATUS_KIND_JUMP_AERIAL].contains(&status){ 
        WorkModule::off_flag(fighter.module_accessor, FIGHTER_STATUS_ATTACK_WORK_FLAG_HITFALL);
    }
    if AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false {
        let dive_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dive_cont_value"));
        let dive_flick = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
        if fighter.global_table[0x1B].get_f32() < dive_cont_value // This is Stick Y in the Global Table
        && fighter.global_table[0x1D].get_i32() < dive_flick { // This is the Flick Y value in the Global Table
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
        }

    }
}


/*pub unsafe fn common_attack_critical_flag(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
        COUNTER[entry_id] += 1;
        IS_CRIT[entry_id] = true;
        if COUNTER[entry_id] < 2 {
            EffectModule::req_follow(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), Hash40::new("haver"), &Vector3f{x: 0.0, y: 8.0, z: 0.0} as *const Vector3f, &Vector3f{x: 0.0, y: 0.0, z: 0.0} as *const Vector3f, 1.0, false, 0, 0, 0, 0, 0, false, false);
            CURRENT_ON_FRAME[entry_id] = MotionModule::frame(fighter.module_accessor);
            SlowModule::set_whole(fighter.module_accessor, 2, 0);
            macros::PLAY_SE(fighter, Hash40::new("se_common_criticalhit"));
            macros::QUAKE(fighter, *CAMERA_QUAKE_KIND_L);
            let lr = PostureModule::lr(fighter.module_accessor);
            macros::CAM_ZOOM_IN_arg5(fighter, /*frames*/ 4.0,/*no*/ 0.0,/*zoom*/ 3.0,/*yrot*/ 0.0,/*xrot*/ 0.0 * lr);
        }
    }
    if MotionModule::frame(fighter.module_accessor) >= (CURRENT_ON_FRAME[entry_id] + 1.0) && IS_CRIT[entry_id] {
        COUNTER[entry_id] = 0;
        SlowModule::clear_whole(fighter.module_accessor);
        CameraModule::reset_all(fighter.module_accessor);
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        if StatusModule::status_kind(fighter.module_accessor) != 510 {
            macros::CAM_ZOOM_OUT(fighter);
        }
    }
    if IS_CRIT[entry_id] && MotionModule::frame(fighter.module_accessor) < 2.0 {
        macros::CAM_ZOOM_OUT(fighter);
        IS_CRIT[entry_id] = false;
        EffectModule::kill_kind(fighter.module_accessor, Hash40::new("sys_bg_criticalhit"), false, false);
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        SlowModule::clear_whole(fighter.module_accessor);
    };
}*/
