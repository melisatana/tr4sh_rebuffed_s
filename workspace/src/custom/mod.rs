use smash::hash40;
use smash::app::lua_bind::*;
use smash::lib::lua_const::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon };
use smashline::*;
use smash::app::*;
use smash::phx::Hash40;
use smash_script::*;
use smash::lib::L2CValue;
//use super::*;
//use crate::customparam::BomaExt;


pub trait BomaExt {
    unsafe fn is_fighter(&mut self) -> bool;
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool;
    unsafe fn is_weapon(&mut self) -> bool;
    unsafe fn kind(&mut self) -> i32;
}

impl BomaExt for BattleObjectModuleAccessor {
    unsafe fn is_fighter(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_FIGHTER;
    }
    unsafe fn is_status_one_of(&mut self, kinds: &[i32]) -> bool {
        let kind = StatusModule::status_kind(self);
        return kinds.contains(&kind);
    }
    unsafe fn is_weapon(&mut self) -> bool {
        return smash::app::utility::get_category(self) == *BATTLE_OBJECT_CATEGORY_WEAPON;
    }
    unsafe fn kind(&mut self) -> i32 {
        return smash::app::utility::get_kind(self);
    }
}


static DEBUG_ALLOW_MOMENTUM_JUMPS : bool = false;



//handles whether or not you can fast fall, thanks WuBoy
pub unsafe fn fastfall_helper(fighter: &mut L2CFighterCommon) {
    let speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);

    if speed_y < 0.0 {
        let dive_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dive_cont_value"));
        let dive_flick = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
        if fighter.global_table[0x1B].get_f32() < dive_cont_value // This is Stick Y in the Global Table
        && fighter.global_table[0x1D].get_i32() < dive_flick { // This is the Flick Y value in the Global Table
            let dive_speed_y = WorkModule::get_param_float(fighter.module_accessor, hash40("dive_speed_y"), 0);
            if -dive_speed_y < speed_y {
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
            }
        }
    }
}

//same as the helper above, but fastfalling is unrestricted by y speed
pub unsafe fn fastfall_whenever_helper(fighter: &mut L2CFighterCommon) {
    let dive_cont_value = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("dive_cont_value"));
    let dive_flick = WorkModule::get_param_int(fighter.module_accessor, hash40("common"), hash40("dive_flick_frame_value"));
    if fighter.global_table[0x1B].get_f32() < dive_cont_value // This is Stick Y in the Global Table
    && fighter.global_table[0x1D].get_i32() < dive_flick { // This is the Flick Y value in the Global Table
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_DIVE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REQUEST_DIVE_EFFECT);
    }
}


//determines who cannot z-nair
unsafe fn can_z_nair(fighter: &mut L2CFighterCommon) -> bool {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if [
        *FIGHTER_KIND_SAMUS,
        *FIGHTER_KIND_SAMUSD,
        *FIGHTER_KIND_LUIGI,
        *FIGHTER_KIND_YOUNGLINK,
        *FIGHTER_KIND_SZEROSUIT,
        *FIGHTER_KIND_LUCAS,
        *FIGHTER_KIND_TOONLINK
    ].contains(&fighter_kind) {
        return false;
    }
    return true;
}

//z-nair code
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_sub_transition_group_check_air_escape)]
pub unsafe fn escape_air_subtransition(fighter: &mut L2CFighterCommon) -> L2CValue {

    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
    
    if can_z_nair(fighter) && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
        ControlModule::set_attack_air_kind(fighter.module_accessor, *FIGHTER_COMMAND_ATTACK_AIR_KIND_N);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_AIR_ESCAPE); //test?
        return false.into();
        
    }
    original!()(fighter)
}

//dash dance handler, disables the code below for 0 momentum backdashes
unsafe fn dashdance_handler(fighter: &mut L2CFighterCommon) -> bool {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);

    if [
        *FIGHTER_KIND_SAMUSD
    ].contains(&fighter_kind) {
        return false;
    }
    return true;
}


//WuBor Code to not break shield on hold. Does not work for Jigglypuff or Shulk
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_guard_main_common)]
unsafe fn status_guard_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    // Shield Breaks no longer happen if you just hold Shield

    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);

    if shield_hp < 0.0 
    && (fighter_kind == *FIGHTER_KIND_PURIN 
        || ((fighter_kind == *FIGHTER_KIND_SHULK || fighter_kind == *FIGHTER_KIND_KIRBY) 
        && WorkModule::get_int64(fighter.module_accessor, *FIGHTER_SHULK_INSTANCE_WORK_ID_INT_SPECIAL_N_TYPE) == *FIGHTER_SHULK_MONAD_TYPE_SHIELD as u64)) {
        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into());
    return true.into();
    }
    
    if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        let min_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_STATUS_GUARD_ON_WORK_INT_MIN_FRAME);
        if min_frame <= 0 {
            if fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
                fighter.change_status(FIGHTER_STATUS_KIND_GUARD_OFF.into(), true.into());
                return true.into();
            }
        }
    }
    false.into()
}

//stop buffering dodges when landing
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Landing_MainSub)]
pub unsafe fn status_landing_main_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    
    let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);

    if StatusModule::prev_status_kind(boma, 0) == *FIGHTER_STATUS_KIND_ESCAPE_AIR || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_F);
        ControlModule::clear_command_one(boma, *FIGHTER_PAD_COMMAND_CATEGORY1, *FIGHTER_PAD_CMD_CAT1_ESCAPE_B);
    }
    original!()(fighter)
}


/*//dash out of run
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Dash_Main_common)]
unsafe extern "C" fn status_dash_main_common(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stickx = ControlModule::get_stick_x(fighter.module_accessor);
    let sticky = ControlModule::get_stick_y(fighter.module_accessor);

    if sticky <= -0.7 && fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR && stickx < 0.75 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT) && fighter.sub_check_command_squat().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT.into(), true.into());
            return 1.into();
        }
    }
    0.into()
}


#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Run_Sub)]
unsafe extern "C" fn status_run_sub(fighter: &mut L2CFighterCommon) -> L2CValue {
    let stickx = ControlModule::get_stick_x(fighter.module_accessor);
    let sticky = ControlModule::get_stick_y(fighter.module_accessor);

    if sticky <= -0.7 && fighter.global_table[0x16].get_i32() != *SITUATION_KIND_AIR && stickx < 0.75 {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT);
        if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_SQUAT) && fighter.sub_check_command_squat().get_bool() {
            fighter.change_status(FIGHTER_STATUS_KIND_SQUAT.into(), true.into());
            return 1.into();
        }
    }
    0.into()
}*/




//WuBor taunt on platforms code but it makes confetti instead
#[skyline::hook(replace = smash::lua2cpp::L2CFighterCommon_status_Rebirth_Main)]
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
    let hi = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI);
    let lw = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW);
    let s = ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L);
    
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_STATUS_REBIRTH_FLAG_MOVE_END) {
        if hi || s || lw {
            macros::EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 28, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false); //confetti 2
            //StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DEAD, false);
        }
    }
    true
}

// hdr code for b reverses
//=================================================================
//== FighterStatusModuleImpl::set_fighter_status_data
//=================================================================
#[skyline::hook(replace=FighterStatusModuleImpl::set_fighter_status_data)]
unsafe fn set_fighter_status_data_hook(boma: &mut BattleObjectModuleAccessor, arg2: bool, treaded_kind: i32, arg4: bool, arg5: bool, arg6: bool, log_mask_flag: u64, status_attr: u32, power_up_attack_bit: u32, arg10: u32) {
    let id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let mut new_status_attr = status_attr;

    if boma.is_fighter() {

        // this handles turnaround special/b-reversible moves
        if boma.kind() == *FIGHTER_KIND_MARIO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT])
        || boma.kind() == *FIGHTER_KIND_DONKEY && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N])
        || boma.kind() == *FIGHTER_KIND_LINK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_LINK_STATUS_KIND_SPECIAL_LW_BLAST]) 
        || boma.kind() == *FIGHTER_KIND_KIRBY && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S])
        || boma.kind() == *FIGHTER_KIND_PURIN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_CAPTAIN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW])

        || boma.kind() == *FIGHTER_KIND_FALCO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_GANON && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_YOUNGLINK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]) 
        || boma.kind() == *FIGHTER_KIND_MARTH && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_LUCINA && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_ROY && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_CHROM && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_MEWTWO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])

        || boma.kind() == *FIGHTER_KIND_TOONLINK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]) 
        || boma.kind() == *FIGHTER_KIND_IKE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_LUCARIO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_PIKMIN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_DEDEDE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S])
        || boma.kind() == *FIGHTER_KIND_PZENIGAME && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N])
        || boma.kind() == *FIGHTER_KIND_SNAKE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI])
        || boma.kind() == *FIGHTER_KIND_WARIO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])

        || boma.kind() == *FIGHTER_KIND_MURABITO && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_LITTLEMAC && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N2, *FIGHTER_LITTLEMAC_STATUS_KIND_SPECIAL_N_START, *FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_MIIFIGHTER && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_N, *FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_MIISWORDSMAN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_GEKKOUGA && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_SHULK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_PALUTENA && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_CLOUD && boma.is_status_one_of(&[*FIGHTER_CLOUD_STATUS_KIND_SPECIAL_HI2, *FIGHTER_STATUS_KIND_SPECIAL_LW]) 
        || boma.kind() == *FIGHTER_KIND_KAMUI && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S, *FIGHTER_STATUS_KIND_SPECIAL_LW])

        || boma.kind() == *FIGHTER_KIND_SHIZUE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_SET, *FIGHTER_SHIZUE_STATUS_KIND_SPECIAL_LW_FAILURE]) 
        || boma.kind() == *FIGHTER_KIND_SIMON && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW]) 
        || boma.kind() == *FIGHTER_KIND_RICHTER && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_GAOGAEN && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_KROOL && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_JACK && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_BRAVE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_MASTER && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_S])
        || boma.kind() == *FIGHTER_KIND_TRAIL && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        || boma.kind() == *FIGHTER_KIND_EDGE && boma.is_status_one_of(&[*FIGHTER_STATUS_KIND_SPECIAL_LW])
        {
            // if b-reverse flag does not already exist in status_attr bitmask
            if status_attr & *FIGHTER_STATUS_ATTR_START_TURN as u32 == 0 {
                // add b-reverse flag to status_attr bitmask
                new_status_attr = status_attr + *FIGHTER_STATUS_ATTR_START_TURN as u32;
            }
        }

    }

    original!()(boma, arg2, treaded_kind, arg4, arg5, arg6, log_mask_flag, new_status_attr, power_up_attack_bit, arg10)
}

//determines who cannot cancel jab1 into ftilt
unsafe fn can_jab1_cancel_to_ftilt(fighter: &mut L2CFighterCommon) -> bool {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if [
        *FIGHTER_KIND_TANTAN, 
        *FIGHTER_KIND_RYU, 
        *FIGHTER_KIND_KEN, 
        *FIGHTER_KIND_DEMON, 
        *FIGHTER_KIND_PALUTENA
    ].contains(&fighter_kind) {
        return false;
    }
    return true;
}

//determines who cannot cancel jab1 into utilt
unsafe fn can_jab1_cancel_to_utilt(fighter: &mut L2CFighterCommon) -> bool {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if [
        *FIGHTER_KIND_RYU, 
        *FIGHTER_KIND_KEN
    ].contains(&fighter_kind) {
        return false;
    }
    return true;
}

//determines who cannot cancel jab1 into dtilt
unsafe fn can_jab1_cancel_to_dtilt(fighter: &mut L2CFighterCommon) -> bool {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if [
        *FIGHTER_KIND_KIRBY, 
        *FIGHTER_KIND_DOLLY, 
        *FIGHTER_KIND_PACMAN, 
        *FIGHTER_KIND_ROBOT, 
        *FIGHTER_KIND_RYU, 
        *FIGHTER_KIND_KEN
    ].contains(&fighter_kind) {
        return false;
    }
    return true;
}


//determines who cannot cancel jab2 into ftilt
unsafe fn can_jab2_cancel_to_ftilt(fighter: &mut L2CFighterCommon) -> bool {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if [
        *FIGHTER_KIND_DONKEY, 
        *FIGHTER_KIND_TANTAN, 
        *FIGHTER_KIND_SAMUS, 
        *FIGHTER_KIND_SAMUSD, 
        *FIGHTER_KIND_YOSHI, 
        *FIGHTER_KIND_POPO, 
        *FIGHTER_KIND_NANA, 
        *FIGHTER_KIND_PURIN, 
        *FIGHTER_KIND_ROBOT, 
        *FIGHTER_KIND_PEACH, 
        *FIGHTER_KIND_DAISY, 
        *FIGHTER_KIND_PIKMIN, 
        *FIGHTER_KIND_WARIO, 
        *FIGHTER_KIND_DEMON, 
        *FIGHTER_KIND_KOOPA, 
    ].contains(&fighter_kind) {
        return false;
    }
    return true;
}

//determines who cannot cancel jab2 into utilt
unsafe fn can_jab2_cancel_to_utilt(fighter: &mut L2CFighterCommon) -> bool {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if [
        *FIGHTER_KIND_SAMUS, 
        *FIGHTER_KIND_SAMUSD, 
        *FIGHTER_KIND_DONKEY, 
        *FIGHTER_KIND_MARTH, 
        *FIGHTER_KIND_YOSHI, 
        *FIGHTER_KIND_POPO, 
        *FIGHTER_KIND_NANA, 
        *FIGHTER_KIND_PURIN, 
        *FIGHTER_KIND_ROBOT, 
        *FIGHTER_KIND_PEACH, 
        *FIGHTER_KIND_DAISY, 
        *FIGHTER_KIND_PIKMIN, 
        *FIGHTER_KIND_WARIO, 
        *FIGHTER_KIND_RYU, 
        *FIGHTER_KIND_KEN, 
        *FIGHTER_KIND_KOOPA
    ].contains(&fighter_kind) {
        return false;
    }
    return true;
}

//determines who cannot cancel jab2 into dtilt
unsafe fn can_jab2_cancel_to_dtilt(fighter: &mut L2CFighterCommon) -> bool {
    let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
    if [
        *FIGHTER_KIND_KIRBY, 
        *FIGHTER_KIND_DONKEY, 
        *FIGHTER_KIND_DOLLY, 
        *FIGHTER_KIND_PACMAN, 
        *FIGHTER_KIND_ROBOT, 
        *FIGHTER_KIND_SAMUS, 
        *FIGHTER_KIND_SAMUSD, 
        *FIGHTER_KIND_MARTH, 
        *FIGHTER_KIND_POPO, 
        *FIGHTER_KIND_NANA, 
        *FIGHTER_KIND_PURIN, 
        *FIGHTER_KIND_PEACH, 
        *FIGHTER_KIND_DAISY, 
        *FIGHTER_KIND_PIKMIN, 
        *FIGHTER_KIND_WARIO, 
        *FIGHTER_KIND_RYU, 
        *FIGHTER_KIND_KEN, 
        *FIGHTER_KIND_KOOPA 
    ].contains(&fighter_kind) {
        return false;
    }
    return true;
}


// Use this for general per-frame fighter-level hooks
pub unsafe extern "C" fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status = StatusModule::status_kind(fighter.module_accessor);
        let stickx = ControlModule::get_stick_x(module_accessor);
        let sticky = ControlModule::get_stick_y(module_accessor);
        let lr = PostureModule::lr(module_accessor);
        let stickx_directional = stickx * lr;
        let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
        //let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
            
        if StatusModule::is_situation_changed(module_accessor) {
            let situation_kind = &format!("{}", StatusModule::situation_kind(module_accessor));
            println!(
                "[Fighter Hook]\nFighter Kind: {}\nStatus Kind: {}\nSituation Kind: {}",
                get_kind(module_accessor),
                StatusModule::status_kind(module_accessor),
                match StatusModule::situation_kind(module_accessor) {
                    0 => "SITUATION_KIND_GROUND",
                    1 => "SITUATION_KIND_CLIFF",
                    2 => "SITUATION_KIND_AIR",
                    _ => situation_kind
                }
            );
        }


        //momentum jumps (currently disabled)
        if [*FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_STATUS_KIND_JUMP].contains(&status) && DEBUG_ALLOW_MOMENTUM_JUMPS 
        && WorkModule::is_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT) == false {
            WorkModule::on_flag(module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_JUMP_NO_LIMIT);
        };


        //flag check for hitfalling
        if WorkModule::is_flag(fighter.module_accessor, crate::consts::FIGHTER_STATUS_ATTACK_WORK_FLAG_HITFALL) {
            crate::consts::common_attack_hitfall_flag(fighter);
        }            

        //Shield dropping through platforms using the taunt button or holding the stick diagonal down 
        if [*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_ON].contains(&status) {
            if GroundModule::is_passable_ground(fighter.module_accessor) {
                if (ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) 
                || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
                || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) 
                || ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R)) 
                    
                || (sticky <= -0.65 && (stickx >= 0.4 || stickx <= -0.4)) {
                    GroundModule::pass_floor(fighter.module_accessor);
                }
            }
            else {
                if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) 
                || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
                || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) 
                || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                    macros::EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 28, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false); //confetti!
                }
            }
        }

        //Shield drop while your shield is being hit by holding down
        if status == *FIGHTER_STATUS_KIND_GUARD_DAMAGE {
            if sticky <= -0.7 {
                GroundModule::pass_floor(fighter.module_accessor);
            }
        }


        //salty confetti 
        if [*FIGHTER_STATUS_KIND_DAMAGE, 
        *FIGHTER_STATUS_KIND_DAMAGE_AIR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_ROLL, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_METEOR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_D, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_U, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_LR, 
        *FIGHTER_STATUS_KIND_DAMAGE_FLY_REFLECT_JUMP_BOARD,
        *FIGHTER_STATUS_KIND_FURAFURA,
        *FIGHTER_STATUS_KIND_ICE].contains(&status) {
            if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) 
            || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) 
            || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) 
            || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) {
                macros::EFFECT(fighter, Hash40::new("sys_kusudama"), Hash40::new("top"), 0, 28, 0, 0, 0, 0, 0.75, 0, 0, 0, 0, 0, 0, false); //confetti!
            }
        }


        //fall through platforms by holding down during an aerial/zair 
        if [*FIGHTER_STATUS_KIND_ATTACK_AIR, 
            *FIGHTER_STATUS_KIND_AIR_LASSO,
            *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, 
            *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N, 
            *FIGHTER_BAYONETTA_STATUS_KIND_ATTACK_AIR_F
            ].contains(&status) {
            if sticky <= -0.5 {
                GroundModule::set_passable_check(fighter.module_accessor, true);
            }
            else {
                GroundModule::set_passable_check(fighter.module_accessor, false);
            }
        }

        //DACUS (allows dash attacks to cancel into Up/Down Smash) 
        if MotionModule::motion_kind(module_accessor) == hash40("attack_dash") && MotionModule::frame(module_accessor) <= (9.0) {
            if sticky > 0.5 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_HI4_START, false);
            }
            if sticky < -0.5 && ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_ATTACK_LW4_START, false);
            }
        }

        
        //perfect pivot on dash (does not work properly)
        if MotionModule::motion_kind(module_accessor) == hash40("dash") && MotionModule::frame(module_accessor) <= (15.0) {
            if stickx_directional < -0.5 && sticky > 0.65 { //-0.5
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_TURN, false);
                println!("Perfect pivot lets go!");
            };
        };

        
        //perfect pivot on turndash (does not work properly)
        if MotionModule::motion_kind(module_accessor) == hash40("turn_dash") && MotionModule::frame(module_accessor) <= (15.0) {
            if stickx_directional < -0.5 && sticky > 0.65 { //-0.5
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_TURN, false);
                println!("Perfect pivot lets go!");
            };
        };


        //backdashes on non-FGCs have set speed, allowing them to dashdance normally 
        if dashdance_handler(fighter) && [*FIGHTER_STATUS_KIND_TURN_DASH].contains(&status) && MotionModule::frame(fighter.module_accessor) <= (1.0) {
            macros::SET_SPEED_EX(fighter, 0.3, 0.0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        }

    

        //Shield after starting a dash (works but had to be modified)
        if [*FIGHTER_STATUS_KIND_DASH, *FIGHTER_STATUS_KIND_TURN_DASH].contains(&status) {
            if MotionModule::frame(fighter.module_accessor) >= 2.0 {
                if sticky <= -0.6 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT, false);
                }
            }
            if MotionModule::frame(fighter.module_accessor) >= 4.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) && ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, false);
                }
            }
        }

        

        //Shield or crouch after starting a brake (works I think)
        if [*FIGHTER_STATUS_KIND_RUN_BRAKE, *FIGHTER_STATUS_KIND_TURN_RUN_BRAKE].contains(&status) {
            if MotionModule::frame(fighter.module_accessor) >= 2.0 {
                if sticky <= -0.6 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT, false);
                }
            }
            if MotionModule::frame(fighter.module_accessor) >= 6.0 {
                if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_GUARD) {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_GUARD_ON, false);
                }        
            }
        }

        
        //cancel Jab1 with Ftilt 
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_11") {
            let hitlag_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
            if can_jab1_cancel_to_ftilt(fighter) 
            && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
            && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false 
            && hitlag_frame <= 0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                fighter.sub_transition_group_check_ground_attack();
            }
        }

        //cancel Jab1 with Utilt 
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_11") {
            let hitlag_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
            if can_jab1_cancel_to_utilt(fighter) 
            && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
            && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false 
            && hitlag_frame <= 0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
                fighter.sub_transition_group_check_ground_attack();
            }
        }

        //cancel Jab1 with Dtilt 
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_11") {
            let hitlag_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
            if can_jab1_cancel_to_dtilt(fighter) 
            && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
            && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false 
            && hitlag_frame <= 0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
                fighter.sub_transition_group_check_ground_attack();
            }
        }

        //cancel Jab2 with Ftilt 
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") {
            let hitlag_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
            if can_jab2_cancel_to_ftilt(fighter) 
            && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
            && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false 
            && hitlag_frame <= 0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S3);
                fighter.sub_transition_group_check_ground_attack();
            }
        }

        //cancel Jab2 with Utilt 
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") {
            let hitlag_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
            if can_jab2_cancel_to_utilt(fighter) 
            && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
            && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false 
            && hitlag_frame <= 0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3);
                fighter.sub_transition_group_check_ground_attack();
            }
        }

        //cancel Jab2 with Dtilt 
        if MotionModule::motion_kind(fighter.module_accessor) == hash40("attack_12") {
            let hitlag_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_HIT_STOP_ATTACK_SUSPEND_FRAME);
            if can_jab2_cancel_to_dtilt(fighter) 
            && AttackModule::is_infliction_status(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) 
            && AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) == false 
            && hitlag_frame <= 0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW3);
                fighter.sub_transition_group_check_ground_attack();
            }
        }

        
        //global ledge cancelling on aerial landings, light/heavy landing, special fall landings and taunts
        if [*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR, 
        *FIGHTER_STATUS_KIND_LANDING, 
        *FIGHTER_STATUS_KIND_LANDING_LIGHT, 
        *FIGHTER_STATUS_KIND_LANDING_FALL_SPECIAL, 
        *FIGHTER_STATUS_KIND_APPEAL
        ].contains(&status) {
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND));
        }


        //fastfall in tumble and zair 
        if [*FIGHTER_STATUS_KIND_DAMAGE_FALL, *FIGHTER_STATUS_KIND_AIR_LASSO].contains(&status) {
            fastfall_helper(fighter);
        }


        //disables jostle on pivot grabs 
        if [*FIGHTER_STATUS_KIND_CATCH_TURN].contains(&status) {
            JostleModule::set_status(fighter.module_accessor, false);
        }

        //act out of ledge faster 
        let fighter_kind = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_KIND);
        if [*FIGHTER_STATUS_KIND_CLIFF_CATCH_MOVE,
        *FIGHTER_STATUS_KIND_CLIFF_CATCH,
        *FIGHTER_STATUS_KIND_CLIFF_WAIT].contains(&status) {
            if fighter_kind != *FIGHTER_KIND_NANA {
                if MotionModule::frame(module_accessor) > 5.0 {
                    WorkModule::on_flag(module_accessor, *FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_CHANGE_STATUS_DLAY_MOTION);
                }
            }
        }

        //cancel techs with Smash Attacks 
        if status == *FIGHTER_STATUS_KIND_PASSIVE_FB {
            if MotionModule::frame(module_accessor) > 14.0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
                fighter.sub_transition_group_check_ground_attack();
            }
        }

        if status == *FIGHTER_STATUS_KIND_PASSIVE {
            if MotionModule::frame(module_accessor) > 10.0 {
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_S4_START);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI4_START);
                WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_LW4_START);
                fighter.sub_transition_group_check_ground_attack();
            }
        }

        //being thrown forces an item drop 
        if status == *FIGHTER_STATUS_KIND_THROWN {
            ItemModule::drop_item(fighter.module_accessor, 0.0, 0.0, 0);
        }


        //airdodges can be canceled with a wall jump
        if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status)  {
            let touch_right = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
            let touch_left = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
            //let touch_side = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32);
            if touch_left {
                if stickx >= 0.5 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, false);
                }
            }
            if touch_right {
                if stickx <= -0.5 {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, false);
                }
            }
        }

        /*if [*FIGHTER_STATUS_KIND_ESCAPE_AIR, *FIGHTER_STATUS_KIND_ESCAPE_AIR_SLIDE].contains(&status)  {
            let touch_right = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_RIGHT_SIDE as u32);
            let touch_left = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_LEFT_SIDE as u32);
            let touch_side = GroundModule::is_wall_touch_line(fighter.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32);
            if touch_left || touch_right || touch_side {
                if stickx_directional <= -0.5 { 
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WALL_JUMP, false);
                }
            }
        }*/

        

        //Hold buffer clears after 12 frames
        /*let control_pad = [
        *CONTROL_PAD_BUTTON_APPEAL_HI, 
        *CONTROL_PAD_BUTTON_APPEAL_LW, 
        *CONTROL_PAD_BUTTON_APPEAL_S_L, 
        *CONTROL_PAD_BUTTON_APPEAL_S_R, 
        *CONTROL_PAD_BUTTON_ATTACK, 
        *CONTROL_PAD_BUTTON_ATTACK_RAW, 
        *CONTROL_PAD_BUTTON_CATCH, 
        *CONTROL_PAD_BUTTON_CSTICK_ON,
        *CONTROL_PAD_BUTTON_FLICK_JUMP, 
        *CONTROL_PAD_BUTTON_GUARD, 
        *CONTROL_PAD_BUTTON_GUARD_HOLD, 
        *CONTROL_PAD_BUTTON_INVALID, 
        *CONTROL_PAD_BUTTON_JUMP, 
        *CONTROL_PAD_BUTTON_JUMP_MINI, 
        *CONTROL_PAD_BUTTON_SMASH, 
        *CONTROL_PAD_BUTTON_SPECIAL, 
        *CONTROL_PAD_BUTTON_SPECIAL_RAW, 
        *CONTROL_PAD_BUTTON_SPECIAL_RAW2, 
        *CONTROL_PAD_BUTTON_STOCK_SHARE, 
        *CONTROL_PAD_BUTTON_TERM, 
        *CONTROL_PAD_CLATTER_CAUSE_NONE, 
        *CONTROL_PAD_CLATTER_FLOWER, 
        *CONTROL_PAD_CLATTER_MAIN, 
        *CONTROL_PAD_CLATTER_NONE,
        *CONTROL_PAD_CLATTER_TERM, 
        *CONTROL_PAD_STICK_REVERSE_ALL, 
        *CONTROL_PAD_STICK_REVERSE_NONE, 
        *CONTROL_PAD_STICK_REVERSE_X, 
        *CONTROL_PAD_STICK_REVERSE_Y];
        for i in control_pad {
            if ControlModule::get_trigger_count(module_accessor, i as u8) > 12 && ControlModule::check_button_on(module_accessor, i)
            && ![*FIGHTER_STATUS_KIND_GUARD, *FIGHTER_STATUS_KIND_GUARD_ON, *FIGHTER_STATUS_KIND_GUARD_DAMAGE, *FIGHTER_STATUS_KIND_GUARD_OFF].contains(&status) {
                ControlModule::reset_trigger(module_accessor);
                ControlModule::clear_command(module_accessor, true);
            }
        }*/




    }
}


//=================================================================
//== KineticModule::change_kinetic
//== Note: Double jump cancels for Ness, Sora and Mewtwo
//== Note: This changes the kinetic energy, not the animation
//=================================================================
#[skyline::hook(replace=KineticModule::change_kinetic)]
unsafe fn change_kinetic_hook(boma: &mut BattleObjectModuleAccessor, kinetic_type: i32) -> i32 {
    let mut kinetic_type_new = kinetic_type;

    let fighter_category = utility::get_category(boma);
    let fighter_kind = utility::get_kind(boma);
    let status = StatusModule::status_kind(boma);

    if fighter_category == *BATTLE_OBJECT_CATEGORY_FIGHTER
        && ( ( [*FIGHTER_KIND_NESS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_TRAIL].contains(&fighter_kind)
        && (status == *FIGHTER_STATUS_KIND_ATTACK_AIR || ([*FIGHTER_KIND_TRAIL].contains(&fighter_kind) && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N].contains(&status)))
        && kinetic_type == FIGHTER_KINETIC_TYPE_JUMP_AERIAL_MOTION_2ND
        && !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) )
           ) {
            kinetic_type_new = *FIGHTER_KINETIC_TYPE_MOTION_FALL;
    }
    original!()(boma, kinetic_type_new)
}

//=================================================================
//== MotionModule::add_motion_2nd
//== Note: Double jump cancels for Ness, Sora and Mewtwo
//== Note: This changes the animation, not the kinetic energy
//=================================================================
#[skyline::hook(replace=MotionModule::add_motion_2nd)]
unsafe fn add_motion_2nd_hook(boma: &mut BattleObjectModuleAccessor, motion_kind: smash::phx::Hash40, arg3: f32, arg4: f32, arg5: bool, arg6: f32) -> u64 {
    let fighter_category = utility::get_category(boma);
    let fighter_kind = utility::get_kind(boma);
    let status = StatusModule::status_kind(boma);
    if fighter_category == *BATTLE_OBJECT_CATEGORY_FIGHTER && [*FIGHTER_KIND_NESS, *FIGHTER_KIND_MEWTWO, *FIGHTER_KIND_TRAIL].contains(&fighter_kind) {
        if [hash40("jump_aerial_f"), hash40("jump_aerial_b")].contains(&motion_kind.hash) {
            if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                return 0;
            }
        }
        if status == *FIGHTER_STATUS_KIND_ATTACK_AIR || ([*FIGHTER_KIND_TRAIL].contains(&fighter_kind) && [*FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_F, *FIGHTER_TRAIL_STATUS_KIND_ATTACK_AIR_N].contains(&status)) {
            if MotionModule::frame(boma) < 6.0 {
                if !ControlModule::check_button_on(boma, *CONTROL_PAD_BUTTON_JUMP) {
                    return 0;
                }
            }
        }
    }
    original!()(boma, motion_kind, arg3, arg4, arg5, arg6)
}


// Use this for general per-frame weapon-level hooks
/*#[weapon_frame_callback]
pub fn global_weapon_frame(fighter_base : &mut L2CFighterBase) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter_base.lua_state_agent);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;

        if frame % 10 == 0 {
            println!("[Weapon Hook] Frame : {}", frame);
        }
    }
}*/



fn nro_hook(info: &skyline::nro::NroInfo) {
    if info.name == "common" {
        skyline::install_hooks!(
            status_landing_main_sub,
            escape_air_subtransition,
            status_guard_main_common,
            status_rebirth_main
            //status_dash_main_common,
            //status_run_sub
        );
    }
}


pub fn install() {
    
    Agent::new("common")
    .on_line(Main, global_fighter_frame)
    .install();

    skyline::install_hooks!(
        set_fighter_status_data_hook,
        change_kinetic_hook,
        add_motion_2nd_hook
    );
    skyline::nro::add_hook(nro_hook);
    
}