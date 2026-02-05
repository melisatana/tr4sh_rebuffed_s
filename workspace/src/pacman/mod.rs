#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use crate::custom::global_fighter_frame;
use smash::hash40;
use smashline::Priority::*;


static mut PACMAN_CURRENT_HYDRANT_TYPE : [i32; 8] = [0; 8]; //0 = Water, 1 = Fire, 2 = Ice
static mut PACMAN_NEXT_HYDRANT_TYPE : [i32; 8] = [0; 8];

static PACMAN_HYDRANT_TYPES : i32 = 3;
static PACMAN_CAN_CYCLE_HYDRANT_TYPES : bool = false;

// A Once-Per-Fighter-Frame that only applies to Pac-Man
unsafe extern "C" fn pacman_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        global_fighter_frame(fighter);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);
        
        //println!("It'sa me, Pac-Man, waka waka!!");
        if sv_information::is_ready_go() == false || [*FIGHTER_STATUS_KIND_WIN, *FIGHTER_STATUS_KIND_LOSE, *FIGHTER_STATUS_KIND_DEAD].contains(&status) {
			PACMAN_NEXT_HYDRANT_TYPE[entry_id] = 0;
		}

        if MotionModule::frame(fighter.module_accessor) >= (8.0) && [*FIGHTER_STATUS_KIND_SPECIAL_HI, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_LOOP, *FIGHTER_PACMAN_STATUS_KIND_SPECIAL_HI_END].contains(&status) {
          crate::custom::fastfall_whenever_helper(fighter);
        }
            
        
    }
}

/*unsafe extern "C" fn pacman_neutralb_hold_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //lib::L2CValue::L2CValue(aLStack96,0);
    //lib::L2CValue::L2CValue(aLStack112,_FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_MAX_HOLD_COUNT);
    //iVar2 = lib::L2CValue::as_integer(aLStack96);
    //iVar3 = lib::L2CValue::as_integer(aLStack112);
    WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_MAX_HOLD_COUNT);
    //lib::L2CValue::_L2CValue(aLStack112);
    //lib::L2CValue::_L2CValue(aLStack96);
    //pLVar5 = (L2CValue *)lib::L2CValue::operator__((L2CValue *)&this->globalTable,0x16);
    //lib::L2CValue::L2CValue(aLStack96,_SITUATION_KIND_GROUND);
    //uVar6 = lib::L2CValue::operator__(pLVar5,aLStack96);
    //lib::L2CValue::_L2CValue(aLStack96);
    if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
        special_n_hold_set_physics_1(fighter);
        //lib::L2CValue::L2CValue(aLStack96,0x1244d4ffef);
        //lib::L2CValue::L2CValue(aLStack112,0.0);
        //lib::L2CValue::L2CValue(aLStack128,1.0);
        //lib::L2CValue::L2CValue(aLStack144,false);
        //HVar7 = lib::L2CValue::as_hash(aLStack96);
        //fVar8 = (float)lib::L2CValue::as_number(aLStack112);
        //fVar9 = (float)lib::L2CValue::as_number(aLStack128);
        //bVar1 = lib::L2CValue::as_bool(aLStack144);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        special_n_hold_set_physics_2(fighter);
        //lib::L2CValue::L2CValue(aLStack96,0xe06860a20);
        //lib::L2CValue::L2CValue(aLStack112,0.0);
        //lib::L2CValue::L2CValue(aLStack128,1.0);
        //lib::L2CValue::L2CValue(aLStack144,false);
        //HVar7 = lib::L2CValue::as_hash(aLStack96);
        //fVar8 = (float)lib::L2CValue::as_number(aLStack112);
        //fVar9 = (float)lib::L2CValue::as_number(aLStack128);
        //bVar1 = lib::L2CValue::as_bool(aLStack144);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_hold"), 0.0, 1.0, false, 0.0, false, false);
    }
    //lib::L2CValue::_L2CValue(aLStack144);
    //lib::L2CValue::_L2CValue(aLStack128);
    //lib::L2CValue::_L2CValue(aLStack112);
    //lib::L2CValue::_L2CValue(aLStack96);
    //lib::L2CValue::L2CValue(aLStack96, 0x240e24407a);
    //lib::L2CAgent::clear_lua_stack((L2CAgent *)this);
    fighter.clear_lua_stack();
    fighter.push_lua_stack(&mut L2CValue::new_int(0x240e24407a));
    notify_event_msc_cmd!(fighter.lua_state_agent);
    fighter.pop_lua_stack(1);
    //lib::L2CAgent::pop_lua_stack((L2CAgent *)this,1);
    //lib::L2CValue::_L2CValue(aLStack112);
    //lib::L2CValue::_L2CValue(aLStack96);
    //lib::L2CValue::L2CValue(aLStack96,_FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    //iVar2 = lib::L2CValue::as_integer(aLStack96);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_GUARD);
    //lib::L2CValue::_L2CValue(aLStack96);
    //lib::L2CValue::L2CValue(aLStack96,FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_ESCAPE);
    //iVar2 = lib::L2CValue::as_integer(aLStack96);
    //WorkModule::enable_transition_term_group(fighter.module_accessor, iVar2);
    //lib::L2CValue::_L2CValue(aLStack96);
    //lib::L2CValue::L2CValue(aLStack96,FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    //iVar2 = lib::L2CValue::as_integer(aLStack96);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_GROUND_JUMP);
    //lib::L2CValue::_L2CValue(aLStack96);
    //lib::L2CValue::L2CValue(aLStack96,FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    //iVar2 = lib::L2CValue::as_integer(aLStack96);
    WorkModule::enable_transition_term_group(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_ESCAPE);
    /*lib::L2CValue::_L2CValue(aLStack96);
    bVar1 = app::lua_bind::StopModule::is_stop_impl(fighter.module_accessor);
    lib::L2CValue::L2CValue(aLStack128,(bool)(bVar1 & 1));
    lib::L2CValue::L2CValue(aLStack96,false);
    uVar6 = lib::L2CValue::operator__(aLStack128,aLStack96);
    lib::L2CValue::_L2CValue(aLStack96);
    lib::L2CValue::_L2CValue(aLStack128);
    if ((uVar6 & 1) != 0) {
        lib::L2CValue::L2CValue(aLStack144,false);
        FUN_710002ef60(aLStack128,this,aLStack144);
        lib::L2CValue::_L2CValue(aLStack128);
        lib::L2CValue::_L2CValue(aLStack144);
    }
    pLVar5 = (L2CValue *)lib::L2CValue::operator__((L2CValue *)&this->globalTable,0x15);
    lib::L2CValue::L2CValue(aLStack96,&DAT_710002f0c0);
    lib::L2CValue::operator_(pLVar5,aLStack96);
    lib::L2CValue::_L2CValue(aLStack96);
    lib::L2CValue::L2CValue(aLStack96,8);
    sVar4 = lib::L2CValue::as_integer(aLStack96);
    app::lua_bind::ControlModule__set_add_jump_mini_button_life_impl(fighter.module_accessor,sVar4);
    lib::L2CValue::_L2CValue(aLStack96);
    lib::L2CValue::L2CValue(aLStack96,SpecialNHold_main_loop);
    lua2cpp::L2CFighterCommon::sub_shift_status_main(this,(L2CValue)0xa0);
    lib::L2CValue::_L2CValue(aLStack96);
    return;*/
    if !StopModule::is_stop(fighter.module_accessor) { 
        special_n_hold_substatus(fighter, false.into());
    }
    fighter.global_table[0x15].assign(&L2CValue::Ptr(special_n_hold_substatus as *const () as _));
    //ControlModule::set_add_jump_mini_button_life(fighter.module_accessor, 8);
    //fighter.main_shift(special_n_hold_main_loop)
    fighter.sub_shift_status_main(L2CValue::Ptr(special_n_hold_main_loop as *const () as _))
}


unsafe extern "C" fn pacman_neutralb_hold_status_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if StatusModule::is_situation_changed(fighter.module_accessor) {
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            special_n_hold_set_physics_2(fighter);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_hold"), -1.0, 1.0, 0.0, false, false);
        }
        else {
            special_n_hold_set_physics_1(fighter);
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_hold"), -1.0, 1.0, 0.0, false, false);
        }
    }

    let mut charge_up_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("charge_up_frame"));
    let unk = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), 0x1b9db69840);
    let charge_rank = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
    if unk <= charge_rank {
        charge_up_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("changed_charge_up_frame"));
    }
    let charge_frame = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_CHARGE_FRAME);
    if charge_up_frame <= charge_frame {
        if charge_rank < *PACMAN_SPECIAL_N_RANK_MAX {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
            if *PACMAN_SPECIAL_N_RANK_MAX <= charge_rank {
                WorkModule::set_int(fighter.module_accessor, *PACMAN_SPECIAL_N_RANK_MAX, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
                WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_MAX_HAVE_ITEM);
            }
            WorkModule::set_int(fighter.module_accessor, 0, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_CHARGE_FRAME);
            notify_event_msc_cmd!(fighter, Hash40::new_raw(0x240e24407a));
        }
    }
    //if fighter.is_button_trigger(Buttons::Special | Buttons::Attack) {
    if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
        fighter.change_status(*FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_SHOOT.into(), false.into());
        return 1.into();
    }
    if /*!fighter.is_pad_flag(PadFlag::SpecialTrigger)*/ ControlModule::get_pad_flag(fighter.module_accessor) != *FIGHTER_PAD_FLAG_SPECIAL_TRIGGER {
        if /*!fighter.is_pad_flag(PadFlag::AttackTrigger)*/ ControlModule::get_pad_flag(fighter.module_accessor) != *FIGHTER_PAD_FLAG_ATTACK_TRIGGER {
            if *PACMAN_SPECIAL_N_RANK_MAX <= charge_rank {
                let max_hold_count = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_MAX_HOLD_COUNT);
                if max_hold_count == 1 {
                    effect!(fighter, MA_MSC_EFFECT_REQUEST_FOLLOW, Hash40::new("pacman_fruit_max"), Hash40::new("havel"), 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, true, EFFECT_SUB_ATTRIBUTE_NONE, 0, 0);
                    app::FighterUtil::flash_eye_info(fighter.module_accessor);
                }
                let max_charge_frame = WorkModule::get_param_int(fighter.module_accessor, hash40("param_special_n"), hash40("max_charge_frame"));
                if max_charge_frame <= max_hold_count {
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into();
                }
            }
            if fighter.is_situation(*SITUATION_KIND_GROUND) {
                if fighter.sub_check_command_guard().get_bool() || /*fighter.is_pad_flag(PadFlag::GuardTrigger)*/ ControlModule::get_pad_flag(fighter.module_accessor) == *FIGHTER_PAD_FLAG_GUARD_TRIGGER {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into();
                }
                if fighter.sub_check_jump_in_charging().get_bool() {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_CANCEL.into(), true.into());
                    return 1.into();
                }
                // roll transitions removed
            }
            else {
                if fighter.sub_check_command_guard().get_bool() || /*fighter.is_pad_flag(PadFlag::GuardTrigger)*/ ControlModule::get_pad_flag(fighter.module_accessor) == *FIGHTER_PAD_FLAG_GUARD_TRIGGER {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_NONE, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_CANCEL.into(), false.into());
                    return 1.into();
                }
                if fighter.sub_check_jump_in_charging().get_bool() {
                    WorkModule::set_int(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_NEXT_STATUS);
                    WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_FLAG_SPECIAL_N_PULL_THROW);
                    notify_event_msc_cmd!(fighter, Hash40::new_raw(0x22baf2b632));
                    fighter.change_status(FIGHTER_PACMAN_STATUS_KIND_SPECIAL_N_JUMP_CANCEL.into(), true.into());
                    return 1.into();
                }
                // airdodge transition removed
            }
        }
    }

    return 0.into();
}

pub unsafe extern "C" fn special_n_hold_set_physics_1(fighter: &mut L2CFighterCommon) {
    let mut sum_speed_y = KineticModule::get_sum_speed_y(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    let speed_y_max = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("speed_y_max"));
    if speed_y_max < sum_speed_y {
        sum_speed_y = speed_y_max;
    }
    let air_accel_y = WorkModule::get_param_float(fighter.module_accessor, hash40("air_accel_y"), 0);
    let air_accel_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_accel_y_mul"));
    let air_speed_y_stable = WorkModule::get_param_float(fighter.module_accessor, hash40("air_speed_y_stable"), 0);
    let air_max_speed_y_mul = WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_n"), hash40("air_max_speed_y_mul"));

    sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, *ENERGY_GRAVITY_RESET_TYPE_GRAVITY, 0.0, 0.0, 0.0, 0.0, 0.0);
    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, sum_speed_y);
    sv_kinetic_energy!(set_accel, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, -(air_accel_y * air_accel_y_mul));
    sv_kinetic_energy!(set_stable_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY, air_speed_y_stable * air_max_speed_y_mul);
    KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);

    special_n_hold_set_physics_2(fighter);
}

//FUN_710002df20
pub unsafe extern "C" fn special_n_hold_set_physics_2(fighter: &mut L2CFighterCommon) {
    let mut sum_speed_x = KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.is_situation(*SITUATION_KIND_AIR) {
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_AIR, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x * 0.7, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    }
    else {
        sv_kinetic_energy!(reset_energy, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, *ENERGY_STOP_RESET_TYPE_GROUND, 0.0, 0.0, 0.0, 0.0, 0.0);
        sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_STOP, sum_speed_x * 0.7, 0.0);
        KineticModule::enable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_STOP);
        KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_GRAVITY);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP_ATTACK));
    }

    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.0, 0.0);
    sv_kinetic_energy!(controller_set_accel_x_add, fighter, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);

    sv_kinetic_energy!(set_speed, fighter, *FIGHTER_KINETIC_ENERGY_ID_MOTION, 0.0, 0.0);
    KineticModule::unable_energy(fighter.module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
}


//FUN_710002ef60
pub unsafe extern "C" fn special_n_hold_substatus(fighter: &mut L2CFighterCommon, param_2: L2CValue) -> L2CValue {
    if param_2.get_bool() {
        WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_CHARGE_FRAME);
        let charge_rank = WorkModule::get_int(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE_RANK);
        if *PACMAN_SPECIAL_N_RANK_MAX <= charge_rank {
            WorkModule::inc_int(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_INT_MAX_HOLD_COUNT);
        }
    }

    return 0.into()
}*/





unsafe extern "C" fn pacman_squatf_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
}

unsafe extern "C" fn pacman_squatb_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        //MotionModule::set_rate(fighter.module_accessor, 2.0);
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
}


unsafe extern "C" fn pacman_jab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 361, 25, 20, 20, 3.5, 0.0, 7.5, 5.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 361, 25, 15, 20, 3.7, 0.0, 7.5, 8.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 3.0, 180, 15, 15, 20, 4.0, 0.0, 7.5, 11.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 3.0, 361, 15, 15, 20, 4.0, 0.0, 7.5, 11.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 4.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_RESTART);
    }
}

unsafe extern "C" fn pacman_jab2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.7, 361, 35, 18, 20, 3.5, 0.0, 7.5, 1.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.7, 361, 35, 13, 15, 3.8, 0.0, 7.5, 4.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.7, 180, 30, 20, 30, 4.0, 0.0, 7.5, 8.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 4.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn pacman_jab3_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.4, 90, 100, 0, 55, 4.3, 0.0, 8.0, 7.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.4, 90, 100, 0, 55, 5.3, 0.0, 8.0, 13.0, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
}

unsafe extern "C" fn pacman_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("pizzapacman"), 0.0, 361, 100, 12, 0, 5.0, 0.0, 3.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 4, false, false, true, true, false, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 1.6, 12, 15, 0, 40, 5.5, 0.0, 5.0, 0.0, None, None, None, 0.3, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 7.0);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 4.0, 69, 65, 0, 50, 7.0, 0.0, 5.0, 0.0, None, None, None, 1.4, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 1, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_CUTUP, *ATTACK_REGION_BODY);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn pacman_ftilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.6, 361, 105, 0, 33, 4.8, 7.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneel"), 9.6, 361, 105, 0, 33, 3.6, 0.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("hip"), 9.6, 361, 105, 0, 33, 3.0, 1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn pacman_utilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 8.7, 98, 78, 0, 42, 6.5, 4.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("head"), 8.7, 98, 78, 0, 42, 5.6, 2.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, true);
    }   
}

unsafe extern "C" fn pacman_utilt_effect_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FOOT_EFFECT(fighter, Hash40::new("sys_run_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pacman_wind"), Hash40::new("head"), 0, 0, 0, 0, 0, 0, 1, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("pacman_wind"), true, true);
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_down_smoke"), Hash40::new("top"), 2, 0, 0, 0, 0, 0, 0.6, 0, 0, 0, 0, 0, 0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_attack_arc_b"), true, true);
    }
}

unsafe extern "C" fn pacman_dtilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.1, 0, 50, 0, 25, 6.0, 0.0, 3.6, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 8.1, 330, 80, 0, 25, 6.0, 0.0, 3.6, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 2, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);  
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.15);
    }
}

unsafe extern "C" fn pacman_fsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("ghost1"), 16.6, 361, 100, 0, 46, 7.2, 0.0, 5.4, 0.0, Some(0.0), Some(4.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 16.6, 361, 100, 0, 46, 3.5, 0.0, 6.5, 11.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("ghost1"), 11.1, 361, 107, 0, 46, 6.3, 0.0, 5.4, 0.0, Some(0.0), Some(4.2), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        AttackModule::clear(fighter.module_accessor, 1, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
}


unsafe extern "C" fn pacman_fsmash_effect_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::EFFECT(fighter, Hash40::new("sys_smash_flash"), Hash40::new("handl"), 4, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::LANDING_EFFECT(fighter, Hash40::new("sys_dash_smoke"), Hash40::new("top"), 0, 0, -2, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::EFFECT_FOLLOW(fighter, Hash40::new("pacman_akabei"), Hash40::new("ghost1"), 0, 4, 0, 0, 0, 0, 1.25, true);
    }
}

unsafe extern "C" fn pacman_usmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 1, Hash40::new("top"), 1.5, 135, 100, 125, 0, 4.4, 0.0, 8.5, 8.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 1, Hash40::new("top"), 1.5, 135, 100, 125, 0, 4.4, 0.0, 8.5, -8.0, None, None, None, 0.6, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("ghost1"), 0.9, 367, 70, 15, 15, 6.2, 0.0, 5.8, 0.0, Some(0.0), Some(5.5), Some(0.0), 0.6, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 4, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 2, Hash40::new("ghost1"), 9.6, 85, 135, 0, 50, 7.2, 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn pacman_dsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("ghost1"), 13.0, 30, 94, 0, 30, 6.7, 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("ghost2"), 13.0, 30, 94, 0, 30, 6.7, 0.0, 5.8, 0.0, Some(0.0), Some(5.3), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("ghost1"), 9.4, 40, 99, 0, 40, 5.1, 0.0, 5.8, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("ghost2"), 9.4, 40, 99, 0, 40, 5.1, 0.0, 5.8, 0.0, Some(0.0), Some(4.8), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_NONE);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

unsafe extern "C" fn pacman_nair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 12.5, 45, 100, 0, 45, 7.6, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 8.2, 0, 10, 10, 10, 7.0, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 1.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("pizzapacman"), 8.2, 65, 70, 0, 40, 7.0, 0.0, 0.5, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 2.0, false);
        
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.6);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn pacman_fair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 7.4, 50, 64, 0, 40, 5.2, 5.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 7.4, 50, 64, 0, 40, 5.2, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
}

unsafe extern "C" fn pacman_bair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 14.1, 361, 100, 0, 39, 4.5, 4.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legr"), 14.1, 361, 100, 0, 39, 5.9, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.6, 361, 99, 0, 30, 4.5, 4.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legr"), 9.6, 361, 99, 0, 30, 5.3, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.15);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn pacman_uair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 12.1, 76, 114, 0, 34, 4.4, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 12.1, 76, 114, 0, 34, 5.5, 2.8, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn pacman_dair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        FighterAreaModuleImpl::enable_fix_jostle_area(fighter.module_accessor, 4.0, 4.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    for _ in 0..3 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 367, 100, 26, 0, 5.2, 0.0, 1.8, 0.5, Some(0.0), Some(4.0), Some(0.5), 0.4, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 367, 100, 26, 0, 6.2, 0.0, -1.5, 0.5, None, None, None, 0.4, 0.7, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_KICK);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
            AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    }
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.5, 74, 83, 0, 45, 9.0, 0.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 2.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn pacman_grab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }   
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 4.2, 0.0, 6.4, 4.9, Some(0.0), Some(6.4), Some(15.2), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
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

unsafe extern "C" fn pacman_grab_effect_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
}

unsafe extern "C" fn pacman_grab_sound_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::PLAY_SE(fighter, Hash40::new("se_common_swing_05"));
    }
}

unsafe extern "C" fn pacman_grab_expression_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        slope!(fighter, *MA_MSC_CMD_SLOPE_SLOPE, *SLOPE_STATUS_LR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_elecattacks"), 0, true, *BATTLE_OBJECT_ID_INVALID as u32);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        ControlModule::set_rumble(fighter.module_accessor, Hash40::new("rbkind_erase"), 0, false, *BATTLE_OBJECT_ID_INVALID as u32);
    }
}

unsafe extern "C" fn pacman_grabd_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.6, 0.0, 6.2, 10.7, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("throw"), 3.6, 0.0, 0.5, 0.5, Some(0.0), Some(-2.5), Some(0.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.6, 0.0, 6.2, 10.7, Some(0.0), Some(6.2), Some(16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("throw"), 4.0, 0.0, 0.5, 1.5, Some(0.0), Some(-2.5), Some(1.5), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 1, Hash40::new("throw"), 4.6, 0.0, 0.5, 2.0, Some(0.0), Some(-2.5), Some(2.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn pacman_grabp_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.7, 0.0, 6.2, -10.7, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("throw"), 3.7, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    macros::game_CaptureCutCommon(fighter);
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("top"), 3.7, 0.0, 6.2, -10.7, Some(0.0), Some(6.2), Some(-16.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("throw"), 4.3, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 1, Hash40::new("throw"), 4.8, 0.0, 0.5, 0.0, Some(0.0), Some(-2.5), Some(0.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn pacman_fthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 10.0, 32, 60, 0, 75, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 23, 4);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn pacman_bthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.6, 45, 115, 0, 61, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(fighter, 14, 3);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.6);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn pacman_uthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.5, 89, 110, 0, 70, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 0, 22);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 22.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.18);
    }
}

unsafe extern "C" fn pacman_dthrow_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -2.5, 2.5);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 4.0, 43, 55, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    for _ in 0..2 {
        sv_animcmd::wait(fighter.lua_state_agent, 6.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 100, 40, 0, 5.0, 0.0, 5.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
            AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 2.0);
        if macros::is_excute(fighter) {
            AttackModule::clear_all(fighter.module_accessor);
        }
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 361, 100, 40, 0, 5.0, 0.0, 5.0, 0.0, None, None, None, 0.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
        //FighterCutInManager::set_throw_finish_zoom_rate(fighter.module_accessor, 1.4);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::wait(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn pacman_neutralb_shoot_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    macros::FT_MOTION_RATE(fighter, 1.0);
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_N_WORK_FLAG_THROW);
    }
}

unsafe extern "C" fn pacman_sideb_dash_smash_script(fighter: &mut L2CAgentBase) {
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_PACMAN_STATUS_SPECIAL_S_WORK_FLAG_EAT_POWER_ESA) == true {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 1.0, 50, 108, 0, 50, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        }
    }
    else {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 1.0, 361, 66, 0, 40, 6.0, 0.0, 2.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_SPEED, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        }
    }
}

unsafe extern "C" fn pacman_upb_jump_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("pizzapacman"), 8.0, 92, 90, 0, 40, 5.9, 0.0, 2.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_cutup"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 23.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn pacman_downspecial(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) {
            PACMAN_CURRENT_HYDRANT_TYPE[entry_id] = PACMAN_NEXT_HYDRANT_TYPE[entry_id];
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) || PACMAN_CAN_CYCLE_HYDRANT_TYPES {
            PACMAN_NEXT_HYDRANT_TYPE[entry_id] += 1;
            if PACMAN_NEXT_HYDRANT_TYPE[entry_id] >= PACMAN_HYDRANT_TYPES {
                PACMAN_NEXT_HYDRANT_TYPE[entry_id] = 0;
            }
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT, false, -1);
    }
}

unsafe extern "C" fn pacman_downspecial_air(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    if macros::is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) {
            PACMAN_CURRENT_HYDRANT_TYPE[entry_id] = PACMAN_NEXT_HYDRANT_TYPE[entry_id];
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        if !ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT) || PACMAN_CAN_CYCLE_HYDRANT_TYPES {
            PACMAN_NEXT_HYDRANT_TYPE[entry_id] += 1;
            if PACMAN_NEXT_HYDRANT_TYPE[entry_id] >= PACMAN_HYDRANT_TYPES {
                PACMAN_NEXT_HYDRANT_TYPE[entry_id] = 0;
            }
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_PACMAN_GENERATE_ARTICLE_FIREHYDRANT, false, -1);
    }
}

unsafe extern "C" fn trampoline_break(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 20.0, 270, 70, 0, 60, 3.0, 0.0, 0.3, 0.5, Some(0.0), Some(0.3), Some(-0.5), 2.5, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, f32::NAN, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_NONE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *WEAPON_PACMAN_TRAMPOLINE_INSTANCE_WORK_FLAG_ON_JUMP);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn hydrant_falling(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 13.0, 270, 80, 0, 20, 5.0, 0.0, 6.5, 0.0, Some(0.0), Some(1.5), Some(0.0), 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 2, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_HIGH), false);
    }
}

unsafe extern "C" fn hydrant_hit(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("rot2"), 15.0, 45, 97, 0, 48, 5.0, 0.0, -1.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 10, 0.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn hydrantwater_shoot(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let entry_id = WorkModule::get_int(owner_owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if entry_id < 8 && PACMAN_CURRENT_HYDRANT_TYPE[entry_id] == 2 {
        //ICE
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.9, 60, 100, 0, 33, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 15, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_WATER);
            AttackModule::set_ice_frame_mul(fighter.module_accessor, 0, 1.5, false);
        }
    }
    else if entry_id < 8 && PACMAN_CURRENT_HYDRANT_TYPE[entry_id] == 1 {
        //FIRE
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.7, 361, 100, 0, 56, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WATER);
        }
    }
    else {
        //WATER
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 361, 130, 0, 55, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PACMAN_WATER, *ATTACK_REGION_WATER);
        }
    }
}

unsafe extern "C" fn hydrantwater_shoot_up(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let entry_id = WorkModule::get_int(owner_owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if entry_id < 8 && PACMAN_CURRENT_HYDRANT_TYPE[entry_id] == 2 {
        //ICE
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.9, 90, 100, 0, 33, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 2, 0.0, 15, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_WATER);
            AttackModule::set_ice_frame_mul(fighter.module_accessor, 0, 1.5, false);
        }
    }
    else if entry_id < 8 && PACMAN_CURRENT_HYDRANT_TYPE[entry_id] == 1 {
        //FIRE
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.7, 90, 100, 0, 56, 4.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 60, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_WATER);
        }
    }
    else {
        //WATER
        sv_animcmd::wait(fighter.lua_state_agent, 1.0);
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 0.0, 90, 130, 0, 43, 4.0, 0.0, 0.0, 0.0, None, None, None, 0.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 1, true, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_water"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PACMAN_WATER, *ATTACK_REGION_WATER);
        }
    }
}

unsafe extern "C" fn hydrantwater_shoot_effect(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let entry_id = WorkModule::get_int(owner_owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 6, *WEAPON_PACMAN_FIREHYDRANT_WATER_INSTANCE_WORK_ID_INT_HIT_EFFECT_FRAME);
    }
    
    if entry_id < 8 && PACMAN_CURRENT_HYDRANT_TYPE[entry_id] == 2 {
        //ICE
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
        for _ in 0..99999 {
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 3, 3, 3, 0, 0, 0, false);
            }
            sv_animcmd::wait(fighter.lua_state_agent, 4.0);
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_hit_ice_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 5, 5, 5, 90, 90, 90, false);
            }
            sv_animcmd::wait(fighter.lua_state_agent, 4.0);
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_hit_ice_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 5, 5, 5, 90, 90, 90, false);
            }
            sv_animcmd::wait(fighter.lua_state_agent, 4.0);
        }
    }
    else if entry_id < 8 && PACMAN_CURRENT_HYDRANT_TYPE[entry_id] == 1 {
        //FIRE
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
        for _ in 0..99999 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, false);
            }
            sv_animcmd::wait(fighter.lua_state_agent, 20.0);
        }
    }
    else {
        //WATER
        
    }
}

unsafe extern "C" fn hydrantwater_shoot_up_effect(fighter: &mut L2CAgentBase) {
    let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(fighter.module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let owner_owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(owner_module_accessor, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
    let entry_id = WorkModule::get_int(owner_owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    if macros::is_excute(fighter) {
        WorkModule::set_int(fighter.module_accessor, 6, *WEAPON_PACMAN_FIREHYDRANT_WATER_INSTANCE_WORK_ID_INT_HIT_EFFECT_FRAME);
    }
    
    if entry_id < 8 && PACMAN_CURRENT_HYDRANT_TYPE[entry_id] == 2 {
        //ICE
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
        for _ in 0..99999 {
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_hit_ice"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.75, 3, 3, 3, 0, 0, 0, false);
            }
            sv_animcmd::wait(fighter.lua_state_agent, 4.0);
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_hit_ice_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 5, 5, 5, 90, 90, 90, false);
            }
            sv_animcmd::wait(fighter.lua_state_agent, 4.0);
            if macros::is_excute(fighter) {
                macros::EFFECT(fighter, Hash40::new("sys_hit_ice_s"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.0, 5, 5, 5, 90, 90, 90, false);
            }
            sv_animcmd::wait(fighter.lua_state_agent, 4.0);
        }
    }
    else if entry_id < 8 && PACMAN_CURRENT_HYDRANT_TYPE[entry_id] == 1 {
        //FIRE
        sv_animcmd::wait(fighter.lua_state_agent, 3.0);
        for _ in 0..99999 {
            if macros::is_excute(fighter) {
                macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_fire_fly"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1.5, false);
            }
            sv_animcmd::wait(fighter.lua_state_agent, 20.0);
        }
    }
    else {
        //WATER
        
    }
}

unsafe extern "C" fn pacman_uptaunt_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 30.0);
    if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == *FIGHTER_PACMAN_APPEAL_HI_I as u64 {
        if macros::is_excute(fighter) {
            macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 15.0, 80, 85, 0, 43, 7.8, 0.0, 13.0, 12.5, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_saving"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_BODY);
            AttackModule::set_attack_level(fighter.module_accessor, 0, *FIGHTER_RYU_SAVING_LV_3 as u8);
        }
    }
    else {
        if WorkModule::get_int64(fighter.module_accessor, *FIGHTER_PACMAN_INSTANCE_WORK_ID_INT_APPEAL_HI_KIND) == *FIGHTER_PACMAN_APPEAL_HI_S as u64 {
            if macros::is_excute(fighter) {
                macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 25.0, 80, 85, 0, 73, 7.8, 0.0, 13.0, 12.5, None, None, None, 3.0, 0.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, f32::NAN, 1.0, 0, true, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_BODY);
            }
        }
    }
    sv_animcmd::frame(fighter.lua_state_agent, 38.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

pub fn install() {
    Agent::new("pacman")
    .on_line(Main, pacman_frame) //opff
    .game_acmd("game_squatf", pacman_squatf_smash_script, Low)
    .game_acmd("game_squatb", pacman_squatb_smash_script, Low)
    .game_acmd("game_attack11", pacman_jab_smash_script, Low)
    .game_acmd("game_attack12", pacman_jab2_smash_script, Low)
    .game_acmd("game_attack13", pacman_jab3_smash_script, Low)
    .game_acmd("game_attackdash", pacman_dashattack_smash_script, Low)
    .game_acmd("game_attacks3", pacman_ftilt_smash_script, Low)
    .game_acmd("game_attacks3hi", pacman_ftilt_smash_script, Low)
    .game_acmd("game_attacks3lw", pacman_ftilt_smash_script, Low)
    .game_acmd("game_attackhi3", pacman_utilt_smash_script, Low)
    .effect_acmd("effect_attackhi3", pacman_utilt_effect_script, Low)
    .game_acmd("game_attacklw3", pacman_dtilt_smash_script, Low)
    .game_acmd("game_attacks4", pacman_fsmash_smash_script, Low)
    .effect_acmd("effect_attacks4", pacman_fsmash_effect_script, Low)
    .game_acmd("game_attackhi4", pacman_usmash_smash_script, Low)
    .game_acmd("game_attacklw4", pacman_dsmash_smash_script, Low)
    .game_acmd("game_attackairn", pacman_nair_smash_script, Low)
    .game_acmd("game_attackairf", pacman_fair_smash_script, Low)
    .game_acmd("game_attackairb", pacman_bair_smash_script, Low)
    .game_acmd("game_attackairhi", pacman_uair_smash_script, Low)
    .game_acmd("game_attackairlw", pacman_dair_smash_script, Low)
    .game_acmd("game_catch", pacman_grab_smash_script, Low)
    .effect_acmd("effect_catch", pacman_grab_effect_script, Low)
    .sound_acmd("sound_catch", pacman_grab_sound_script, Low)
    .expression_acmd("expression_catch", pacman_grab_expression_script, Low)
    .game_acmd("game_catchdash", pacman_grabd_smash_script, Low)
    .game_acmd("game_catchturn", pacman_grabp_smash_script, Low)
    .game_acmd("game_throwf", pacman_fthrow_smash_script, Low)
    .game_acmd("game_throwb", pacman_bthrow_smash_script, Low)
    .game_acmd("game_throwhi", pacman_uthrow_smash_script, Low)
    .game_acmd("game_throwlw", pacman_dthrow_smash_script, Low)
    .game_acmd("game_specialnshoot", pacman_neutralb_shoot_smash_script, Low)
    .game_acmd("game_specialairnshoot", pacman_neutralb_shoot_smash_script, Low)
    .game_acmd("game_specialsdash", pacman_sideb_dash_smash_script, Low)
    .game_acmd("game_specialairhiloop", pacman_upb_jump_smash_script, Low)
    .game_acmd("game_speciallw", pacman_downspecial, Low)
    .game_acmd("game_specialairlw", pacman_downspecial_air, Low)
    .game_acmd("game_appealhil", pacman_uptaunt_script, Low)
    .game_acmd("game_appealhir", pacman_uptaunt_script, Low)
    .install();

    Agent::new("pacman_trampoline")
    .game_acmd("game_break", trampoline_break, Low)
    .install();

    Agent::new("pacman_firehydrant")
    .game_acmd("game_fall", hydrant_falling, Low)
    .game_acmd("game_fly", hydrant_hit, Low)
    .install();

    Agent::new("pacman_firehydrantwater")
    .game_acmd("game_shoot", hydrantwater_shoot, Low)
    .effect_acmd("effect_shoot", hydrantwater_shoot_effect, Low)
    .game_acmd("game_shootup", hydrantwater_shoot_up, Low)
    .effect_acmd("effect_shootup", hydrantwater_shoot_up_effect, Low)
    .install();

}