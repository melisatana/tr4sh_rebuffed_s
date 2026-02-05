use smash::hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon};
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



//WuBor taunt on platforms code
/*#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Rebirth_Main)]
unsafe extern "C" fn status_rebirth_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.sub_rebirth_common().get_bool() {
        return 1.into();
    }
    rebirth_motion_handler(fighter);
    if fighter.sub_air_check_fall_common().get_bool() {
        return 1.into();
    }
    if !fighter.global_table[0x8].get_bool() {
        fighter.sub_rebirth_uniq_process_exec_fix_pos();
    }
    0.into()
}

unsafe extern "C" fn rebirth_motion_handler(fighter: &mut L2CFighterCommon) {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    if appeal_motion_uniq_handler(fighter) {
        let kind = fighter.global_table[0x2].get_i32();
        if MotionModule::is_end(fighter.module_accessor) {
            if [*FIGHTER_KIND_CHROM].contains(&kind) && ![hash40("wait"), hash40("wait_2"), hash40("wait_3"), hash40("wait_4")].contains(&mot) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait_4"), 0.0, 1.0, false, 0.0, false, false);
            }
            else if [*FIGHTER_KIND_SAMUSD, *FIGHTER_KIND_ELIGHT].contains(&kind) && [hash40("down_stand_d")].contains(&mot) {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait_4"), 0.0, 1.0, false, 0.0, false, false);
            }
            else {
                fighter.sub_wait_motion(false.into());
            }
        }
    }
}

unsafe extern "C" fn appeal_motion_uniq_handler(fighter: &mut L2CFighterCommon) -> bool {
    let mot = MotionModule::motion_kind(fighter.module_accessor);
    let check_basic_taunts = [
        hash40("appeal_hi_l"),
        hash40("appeal_hi_r"),
        hash40("appeal_s_l"),
        hash40("appeal_s_r"),
        hash40("appeal_lw_l"),
        hash40("appeal_lw_r")
    ].contains(&mot);
    let kind = fighter.global_table[0x2].get_i32();
    let hi = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI);
    let lw = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW);
    let s = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L);
    if kind == *FIGHTER_KIND_SNAKE {
        if MotionModule::is_end(fighter.module_accessor) {
            if check_basic_taunts {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_wait"), 0.0, 1.0, false, 0.0, false, false);
                return false;
            }
        }
        if mot == hash40("appeal_wait") {
            if hi || lw || s {
                MotionModule::change_motion(fighter.module_accessor, Hash40::new("appeal_end"), 0.0, 1.0, false, 0.0, false, false);
                return false;
            }
        }
        if mot == hash40("appeal_end") {
            return true;
        }
    }
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_REBIRTH_FLAG_MOVE_END)
    && !check_basic_taunts
    || (
        FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(mot), true) != 0.0
        && FighterMotionModuleImpl::get_cancel_frame(fighter.module_accessor, Hash40::new_raw(mot), true) <= MotionModule::frame(fighter.module_accessor)
    ) {
        let lr = PostureModule::lr(fighter.module_accessor);
        let mot = if hi {
            if lr >= 0.0 {
                hash40("appeal_hi_r")
            }
            else {
                hash40("appeal_hi_l")
            }
        }
        else if lw {
            if lr >= 0.0 {
                hash40("appeal_lw_r")
            }
            else {
                hash40("appeal_lw_l")
            }
        }
        else if s {
            if lr >= 0.0 {
                hash40("appeal_s_r")
            }
            else {
                hash40("appeal_s_l")
            }
        }
        else {
            hash40("invalid")
        };
        if mot != hash40("invalid") {
            MotionModule::change_motion(
                fighter.module_accessor,
                Hash40::new_raw(mot),
                0.0,
                1.0,
                false,
                0.0,
                false,
                false
            );
            return false;
        }
    }
    true
}*/