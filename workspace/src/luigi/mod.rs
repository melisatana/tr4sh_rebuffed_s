#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smash_script::*;
use smashline::*;
use smash::hash40;
use crate::custom::global_fighter_frame;
use smashline::Priority::*;

pub static mut LUIGI_UPB_SUPERFAST_FALL : [bool; 8] = [false; 8];

static mut LUIGI_FSMASH_ELECTRIC : [bool; 8] = [false; 8];


//pub const FIGHTER_LUIGI_GENERATE_ARTICLE_MISFIRE_AXE : i32 = 0x4;


// A Once-Per-Fighter-Frame that only applies to Luigi
unsafe extern "C" fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        global_fighter_frame(fighter);
        let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let status = StatusModule::status_kind(fighter.module_accessor);


        //println!("It'sa me, Luigi, ye-hey!!");


        if [*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_CHARGE].contains(&status) {
            //if StatusModule::situation_kind(fighter.module_accessor) == SITUATION_KIND_AIR { 
            
            if (ControlModule::get_stick_x(fighter.module_accessor) >= 0.5 && PostureModule::lr(fighter.module_accessor) <= -0.5) || (ControlModule::get_stick_x(fighter.module_accessor) <= -0.5 && PostureModule::lr(fighter.module_accessor) >= 0.5) {
                PostureModule::reverse_lr(fighter.module_accessor);
                PostureModule::update_rot_y_lr(fighter.module_accessor);
            }    
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT_BUTTON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_AERIAL_BUTTON);
            WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ESCAPE_AIR);
            fighter.sub_transition_group_check_air_escape();
            fighter.sub_transition_group_check_ground_jump();
            fighter.sub_transition_group_check_ground_jump_mini_attack();
            fighter.sub_transition_group_check_air_jump_aerial();   
        }

        if [*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL].contains(&status) {
            LUIGI_UPB_SUPERFAST_FALL[entry_id] = true;
        }
        if ![*FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_DROP, *FIGHTER_LUIGI_STATUS_KIND_SPECIAL_HI_FALL].contains(&status) {
            LUIGI_UPB_SUPERFAST_FALL[entry_id] = false;
        }



    }
}



/*unsafe extern "C" fn luigi_sideb_ram_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    //lib::L2CValue::L2CValue(aLStack80,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_HIT);
    //iVar3 = lib::L2CValue::as_integer(aLStack80);
    WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_HIT);
    //lib::L2CValue::_L2CValue(aLStack80);
    //lib::L2CValue::L2CValue(aLStack80,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    //iVar3 = lib::L2CValue::as_integer(aLStack80);
    WorkModule::unable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    //lib::L2CValue::_L2CValue(aLStack80);
    //lib::L2CValue::L2CValue(aLStack96,_FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
    //iVar3 = lib::L2CValue::as_integer(aLStack96);
    //bVar1 = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
    //lib::L2CValue::L2CValue(aLStack80,(bool)(bVar1 & 1));
    //bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack80);
    //lib::L2CValue::_L2CValue(aLStack80);
    //lib::L2CValue::_L2CValue(aLStack96);

    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) == false {
    //if ((bVar2 & 1U) == 0) {
        //lib::L2CValue::L2CValue(aLStack80,0x976c3b29b);
        //lib::L2CValue::L2CValue(aLStack96,0.0);
        //lib::L2CValue::L2CValue(aLStack112,1.0);
        //lib::L2CValue::L2CValue(aLStack128,false);
        //HVar4 = lib::L2CValue::as_hash(aLStack80);
        //fVar5 = (float)lib::L2CValue::as_number(aLStack96);
        //fVar6 = (float)lib::L2CValue::as_number(aLStack112);
        //bVar1 = lib::L2CValue::as_bool(aLStack128);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s"), 0.0, 1.0, false, 0.0, false, false);
    }
    else {
        //lib::L2CValue::L2CValue(aLStack80,0x1378df2f6a);
        //lib::L2CValue::L2CValue(aLStack96,0.0);
        //lib::L2CValue::L2CValue(aLStack112,1.0);
        //lib::L2CValue::L2CValue(aLStack128,false);
        //HVar4 = lib::L2CValue::as_hash(aLStack80);
        //fVar5 = (float)lib::L2CValue::as_number(aLStack96);
        //fVar6 = (float)lib::L2CValue::as_number(aLStack112);
        //bVar1 = lib::L2CValue::as_bool(aLStack128);
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_s_discharge"), 0.0, 1.0, false, 0.0, false, false);
    }
    //lib::L2CValue::_L2CValue(aLStack128);
    //lib::L2CValue::_L2CValue(aLStack112);
    //lib::L2CValue::_L2CValue(aLStack96);
    //lib::L2CValue::_L2CValue(aLStack80);
    //lib::L2CValue::L2CValue(aLStack80,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS);
    //iVar3 = lib::L2CValue::as_integer(aLStack80);
    WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS);
    //lib::L2CValue::_L2CValue(aLStack80);
    //lib::L2CValue::L2CValue(aLStack80,SpecialSRam_main_loop);
    //lua2cpp::L2CFighterCommon::sub_shift_status_main(this,(L2CValue)0xb0);
    //lib::L2CValue::_L2CValue(aLStack80);
    //return;
    fighter.sub_shift_status_main(L2CValue::Ptr(special_s_ram_main_loop as *const () as _))
}


unsafe extern "C" fn sideb_ram_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    //ppBVar9 = &this->moduleAccessor;
    //bVar1 = MotionModule::is_end(fighter.module_accessor);
    //lib::L2CValue::L2CValue(aLStack80,(bool)(bVar1 & 1));
    //bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack80);
    //lib::L2CValue::_L2CValue(aLStack80);
    if MotionModule::is_end(fighter.module_accessor) {
        //lib::L2CValue::L2CValue(aLStack416,_FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END);
        //lib::L2CValue::L2CValue(aLStack432,false);
        //lua2cpp::L2CFighterBase::change_status(this,(L2CValue)0x60,(L2CValue)0x50);
        //lib::L2CValue::_L2CValue(aLStack432);
        fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
        //pLVar5 = aLStack416;
        return 0.into();
        
    }
    //lib::L2CValue::L2CValue(aLStack96,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_HIT);
    //iVar3 = lib::L2CValue::as_integer(aLStack96);
    //bVar1 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_HIT);
    //lib::L2CValue::L2CValue(aLStack80,(bool)(bVar1 & 1));
    //bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack80);
    //lib::L2CValue::_L2CValue(aLStack80);
    //lib::L2CValue::_L2CValue(aLStack96);
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_HIT) {
        //bVar1 = app::lua_bind::AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT);
        //lib::L2CValue::L2CValue(aLStack80,(bool)(bVar1 & 1));
        //bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack80);
        //lib::L2CValue::_L2CValue(aLStack80);
        if AttackModule::is_infliction(fighter.module_accessor, *COLLISION_KIND_MASK_HIT) {
            //lib::L2CValue::L2CValue(aLStack80,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS);
            //iVar3 = lib::L2CValue::as_integer(aLStack80);
            app::lua_bind::WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_LAST_STRANS);
            //lib::L2CValue::_L2CValue(aLStack80);
            //lib::L2CValue::L2CValue(aLStack448,_FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END);
            //lib::L2CValue::L2CValue(aLStack464,false);
            //lua2cpp::L2CFighterBase::change_status(this,(L2CValue)0x40,(L2CValue)0x30);
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            //lib::L2CValue::_L2CValue(aLStack464);
            //pLVar5 = aLStack448;
            return 0.into();
        }
    }
    //lib::L2CValue::L2CValue(aLStack96,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    //iVar3 = lib::L2CValue::as_integer(aLStack96);
    //bVar1 = WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    //lib::L2CValue::L2CValue(aLStack80,(bool)(bVar1 & 1));
    //bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack80);
    //lib::L2CValue::_L2CValue(aLStack80);
    //lib::L2CValue::_L2CValue(aLStack96);
    if WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND) {
        //pLVar5 = (L2CValue *)lib::L2CValue::operator__((L2CValue *)&this->globalTable,0x16);
        //lib::L2CValue::L2CValue(aLStack80,_SITUATION_KIND_GROUND);
        //uVar6 = lib::L2CValue::operator__(pLVar5,aLStack80);
        //lib::L2CValue::_L2CValue(aLStack80);
        if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
            //lib::L2CValue::L2CValue(aLStack480,_FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END);
            //lib::L2CValue::L2CValue(aLStack496,false);
            //lua2cpp::L2CFighterBase::change_status(this,(L2CValue)0x20,(L2CValue)0x10);
            //lib::L2CValue::_L2CValue(aLStack496);
            //pLVar5 = aLStack480;
            fighter.change_status(FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END.into(), false.into());
            return 0.into();
        }
    }
    //lib::L2CValue::L2CValue(aLStack96,0);
    //lib::L2CValue::L2CValue(aLStack112,0);
    //lib::L2CValue::L2CValue(aLStack128,0);
    //lib::L2CValue::L2CValue(aLStack144,0);
    //lib::L2CValue::L2CValue(aLStack160,0);
    //lib::L2CValue::L2CValue(aLStack176,0);
    //lib::L2CValue::L2CValue(aLStack192,0);
    //lib::L2CValue::L2CValue(aLStack208,0);
    //lib::L2CValue::L2CValue(aLStack224,0);
    //lib::L2CValue::L2CValue(aLStack240,0);
    //lib::L2CValue::L2CValue(aLStack256,0);
    //lib::L2CValue::L2CValue(aLStack272,0);
    //lib::L2CValue::L2CValue(aLStack288,0);
    //iVar3 = GroundModule::get_touch_flag(fighter.module_accessor);
    GroundModule::get_touch_flag(fighter.module_accessor);
    //lib::L2CValue::L2CValue(aLStack80,iVar3);
    //lib::L2CValue::operator_(aLStack112,aLStack80);
    //lib::L2CValue::_L2CValue(aLStack80);
    //fVar10 = (float)PostureModule::lr(fighter.module_accessor);
    //lib::L2CValue::L2CValue(aLStack80,fVar10);
    //lib::L2CValue::operator_(aLStack192,aLStack80);
    PostureModule::lr(fighter.module_accessor) = 0;
    //lib::L2CValue::_L2CValue(aLStack80);
    //lib::L2CValue::L2CValue(aLStack304,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    //iVar3 = lib::L2CValue::as_integer(aLStack304);
   //bVar1 = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    //lib::L2CValue::L2CValue(aLStack80,(bool)(bVar1 & 1));
    //bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack80);
    //lib::L2CValue::_L2CValue(aLStack80);
    //lib::L2CValue::_L2CValue(aLStack304);
    if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK) {
        //LAB_710000e820:
        //lib::L2CValue::L2CValue(aLStack320,_FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
        //iVar3 = lib::L2CValue::as_integer(aLStack320);
        //bVar1 = app::lua_bind::WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE);
        //lib::L2CValue::L2CValue(aLStack304,(bool)(bVar1 & 1));
        //lib::L2CValue::operator_(aLStack304);
        //bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack80);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_CHARGE_FLAG_DISCHARGE) {
            lib::L2CValue::_L2CValue(aLStack80);
            lib::L2CValue::_L2CValue(aLStack304);
            //pLVar5 = aLStack320;
            goto LAB_710000ed64;
        }
        //lib::L2CValue::L2CValue(aLStack368,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER);
        //iVar3 = lib::L2CValue::as_integer(aLStack368);
       //bVar1 = WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER);
        //lib::L2CValue::L2CValue(aLStack352,(bool)(bVar1 & 1));
        //lib::L2CValue::operator_(aLStack352);
        //bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack336);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER) == false {
            //bVar1 = 0;
            WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER) = false; 
        }
        else {
            //lib::L2CValue::L2CValue(aLStack400, 0);
            //iVar3 = lib::L2CValue::as_integer(aLStack400);
            //bVar1 = AttackModule::is_attack(fighter.module_accessor, 0, false);
            //lib::L2CValue::L2CValue(aLStack384,(bool)(bVar1 & 1));
            //bVar1 = lib::L2CValue::operator_cast_to_bool(aLStack384);
            //lib::L2CValue::_L2CValue(aLStack384);
            //lib::L2CValue::_L2CValue(aLStack400);
            AttackModule::is_attack(fighter.module_accessor, 0, false);
        }
        //lib::L2CValue::_L2CValue(aLStack336);
        //lib::L2CValue::_L2CValue(aLStack352);
        //lib::L2CValue::_L2CValue(aLStack368);
        //lib::L2CValue::_L2CValue(aLStack80);
        //lib::L2CValue::_L2CValue(aLStack304);
        //lib::L2CValue::_L2CValue(aLStack320);
        if WorkModule::is_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER) {
            //lib::L2CValue::L2CValue(aLStack304,0xfea97fe73);
            //lib::L2CValue::L2CValue(aLStack320,0xc75c684a4);
            //uVar6 = lib::L2CValue::as_integer(aLStack304);
            //uVar7 = lib::L2CValue::as_integer(aLStack320);
            //fVar10 = (float)WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power"));
            //lib::L2CValue::L2CValue(aLStack80,fVar10);
            //lib::L2CValue::operator_(aLStack160,aLStack80);
            WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power"));
            //lib::L2CValue::_L2CValue(aLStack80);
            //lib::L2CValue::_L2CValue(aLStack320);
            //lib::L2CValue::_L2CValue(aLStack304);
            //lib::L2CValue::L2CValue(aLStack304,0xfea97fe73);
            //lib::L2CValue::L2CValue(aLStack320,0x13a029afd1);
            //uVar6 = lib::L2CValue::as_integer(aLStack304);
            //uVar7 = lib::L2CValue::as_integer(aLStack320);
            //fVar10 = (float)WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power_charge"));
            //lib::L2CValue::L2CValue(aLStack80,fVar10);
            //lib::L2CValue::operator_(aLStack256,aLStack80);
            WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("attack_power_charge"));
            //lib::L2CValue::_L2CValue(aLStack80);
            //lib::L2CValue::_L2CValue(aLStack320);
            //lib::L2CValue::_L2CValue(aLStack304);
            //lib::L2CValue::L2CValue(aLStack304,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
            //iVar3 = lib::L2CValue::as_integer(aLStack304);
            //fVar10 = (float)WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
            //lib::L2CValue::L2CValue(aLStack80,fVar10);
            WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
            //lib::L2CValue::L2CValue(aLStack336,0xfea97fe73);
            //lib::L2CValue::L2CValue(aLStack352,0xca3dc30e5);
            //uVar6 = lib::L2CValue::as_integer(aLStack336);
            //uVar7 = lib::L2CValue::as_integer(aLStack352);
            //fVar10 = (float)WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
            //lib::L2CValue::L2CValue(aLStack320,fVar10);
            //uVar6 = lib::L2CValue::operator__(aLStack320,aLStack80);
            //lib::L2CValue::_L2CValue(aLStack320);
            //lib::L2CValue::_L2CValue(aLStack352);
            //lib::L2CValue::_L2CValue(aLStack336);
            //lib::L2CValue::_L2CValue(aLStack80);
            //lib::L2CValue::_L2CValue(aLStack304);
            if WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame")) == false {
                //lib::L2CValue::L2CValue(aLStack304,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
                //iVar3 = lib::L2CValue::as_integer(aLStack304);
                //fVar10 = (float)WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
                //lib::L2CValue::L2CValue(aLStack80,fVar10);
                //lib::L2CValue::operator_(aLStack96,aLStack80);
                WorkModule::get_float(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLOAT_CHARGE);
                //pLVar5 = aLStack80;
            }
            else {
                //lib::L2CValue::L2CValue(aLStack304,0xfea97fe73);
                //lib::L2CValue::L2CValue(aLStack320,0xca3dc30e5);
                //uVar6 = lib::L2CValue::as_integer(aLStack304);
                //uVar7 = lib::L2CValue::as_integer(aLStack320);
                //fVar10 = (float)WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
                //lib::L2CValue::L2CValue(aLStack80,fVar10);
                //lib::L2CValue::operator_(aLStack96,aLStack80);
                //lib::L2CValue::_L2CValue(aLStack80);
                //pLVar5 = aLStack320;
                WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
            }
            //lib::L2CValue::_L2CValue(pLVar5);
            //lib::L2CValue::_L2CValue(aLStack304);
            //lib::L2CValue::operator_(aLStack256, aLStack160);
            //lib::L2CValue::L2CValue(aLStack336,0xfea97fe73);
            //lib::L2CValue::L2CValue(aLStack352,0xca3dc30e5);
            //uVar6 = lib::L2CValue::as_integer(aLStack336);
            //uVar7 = lib::L2CValue::as_integer(aLStack352);
            //fVar10 = (float)WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
            WorkModule::get_param_float(fighter.module_accessor, hash40("param_special_s"), hash40("charge_frame"));
            lib::L2CValue::L2CValue(aLStack320, fVar10);
            lib::L2CValue::operator_(aLStack304, aLStack320);
            lib::L2CValue::operator_(aLStack240, aLStack80);
            lib::L2CValue::_L2CValue(aLStack80);
            lib::L2CValue::_L2CValue(aLStack320);
            lib::L2CValue::_L2CValue(aLStack352);
            lib::L2CValue::_L2CValue(aLStack336);
            lib::L2CValue::_L2CValue(aLStack304);
            lib::L2CValue::operator_(aLStack240,aLStack96);
            lib::L2CValue::operator_(aLStack304, aLStack160);
            lib::L2CValue::operator_(aLStack272,aLStack80);
            lib::L2CValue::_L2CValue(aLStack80);
            lib::L2CValue::_L2CValue(aLStack304);
            lib::L2CValue::L2CValue(aLStack80, 10.0);
            lib::L2CValue::operator_(aLStack272,aLStack80);
            lib::L2CValue::_L2CValue(aLStack80);
            lib::L2CValue::operator_(aLStack288,aLStack304);
            lib::L2CValue::_L2CValue(aLStack304);
            fVar10 = (float)lib::L2CValue::as_number(aLStack288);
            AttackModule::set_power_mul_status(fighter.module_accessor, 10.0);
            lib::L2CValue::L2CValue(aLStack80,_FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER);
            iVar3 = lib::L2CValue::as_integer(aLStack80);
            WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_SET_ATTACK_POWER);
            goto LAB_710000ed60;
        }
    }
    else {
        lib::L2CValue::L2CValue(aLStack80,_GROUND_TOUCH_FLAG_LEFT);
        lib::L2CValue::operator_(aLStack112,aLStack80);
        lib::L2CValue::_L2CValue(aLStack80);
        bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack304);
        if ((bVar2 & 1U) == 0) {
            LAB_710000e494:
            lib::L2CValue::L2CValue(aLStack80,GROUND_TOUCH_FLAG_RIGHT);
            lib::L2CValue::operator_(aLStack112,aLStack80);
            lib::L2CValue::_L2CValue(aLStack80);
            bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack320);
            if ((bVar2 & 1U) == 0) {
                lib::L2CValue::_L2CValue(aLStack320);
                lib::L2CValue::_L2CValue(aLStack304);
            }
            else {
                lib::L2CValue::L2CValue(aLStack80,1.0);
                uVar6 = lib::L2CValue::operator__(aLStack192,aLStack80);
                lib::L2CValue::_L2CValue(aLStack80);
                lib::L2CValue::_L2CValue(aLStack320);
                lib::L2CValue::_L2CValue(aLStack304);
                if ((uVar6 & 1) != 0) goto LAB_710000e4fc;

            }
            goto LAB_710000e820;
        }
        lib::L2CValue::L2CValue(aLStack80,-1.0);
        uVar6 = lib::L2CValue::operator__(aLStack192,aLStack80);
        lib::L2CValue::_L2CValue(aLStack80);
        if ((uVar6 & 1) == 0) goto LAB_710000e494;
            lib::L2CValue::_L2CValue(aLStack304);
            LAB_710000e4fc:
            lib::L2CValue::L2CValue(aLStack80,_FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_END);
            lib::L2CValue::operator_(aLStack128,aLStack80);
            lib::L2CValue::_L2CValue(aLStack80);
            lib::L2CValue::L2CValue(aLStack304,_KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
            iVar3 = lib::L2CValue::as_integer(aLStack304);
            fVar10 = (float)app::lua_bind::KineticModule__get_sum_speed_x_impl(fighter.module_accessor,iVar3);
            lib::L2CValue::L2CValue(aLStack80,fVar10);
            lib::L2CValue::operator_(aLStack144,aLStack80);
            lib::L2CValue::_L2CValue(aLStack80);
            lib::L2CValue::_L2CValue(aLStack304);
            GVar4 = lib::L2CValue::as_integer(aLStack112);
            bVar1 = app::lua_bind::GroundModule__is_attachable_impl(fighter.module_accessor,GVar4);
            lib::L2CValue::L2CValue(aLStack80,(bool)(bVar1 & 1));
            bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack80);
            lib::L2CValue::_L2CValue(aLStack80);
            if ((bVar2 & 1U) != 0) {
                lib::L2CValue::L2CValue(aLStack336,0xfea97fe73);
                lib::L2CValue::L2CValue(aLStack352,0x99a7ac083);
                uVar6 = lib::L2CValue::as_integer(aLStack336);
                uVar7 = lib::L2CValue::as_integer(aLStack352);
                fVar10 = (float)app::lua_bind::WorkModule__get_param_float_impl(fighter.module_accessor,uVar6,uVar7);
                lib::L2CValue::L2CValue(aLStack320,fVar10);
                lib::L2CValue::L2CValue(aLStack80,100.0);
                lib::L2CValue::operator_(aLStack320,aLStack80);
                lib::L2CValue::_L2CValue(aLStack80);
                lib::L2CValue::operator_(aLStack176,aLStack304);
                lib::L2CValue::_L2CValue(aLStack304);
                lib::L2CValue::_L2CValue(aLStack320);
                lib::L2CValue::_L2CValue(aLStack352);
                lib::L2CValue::_L2CValue(aLStack336);
                lib::L2CValue::L2CValue(aLStack304,0x77a08c3fc);
                HVar8 = lib::L2CValue::as_hash(aLStack304);
                fVar10 = (float)app::sv_math::randf(HVar8,1.0);
                lib::L2CValue::L2CValue(aLStack80,fVar10);
                lib::L2CValue::operator_(aLStack208,aLStack80);
                lib::L2CValue::_L2CValue(aLStack80);
                lib::L2CValue::_L2CValue(aLStack304);
                lib::L2CValue::L2CValue(aLStack320,aLStack144);
                lib::L2CValue::L2CValue(aLStack80,0.0);
                uVar6 = lib::L2CValue::operator_(aLStack320,aLStack80);
                lib::L2CValue::_L2CValue(aLStack80);
                if ((uVar6 & 1) != 0) {
                    lib::L2CValue::operator_(aLStack320);
                    lib::L2CValue::operator_(aLStack320,aLStack80);
                    lib::L2CValue::_L2CValue(aLStack80);
                }
                lib::L2CValue::L2CValue(aLStack304,aLStack320);
                lib::L2CValue::L2CValue(aLStack336,0xfea97fe73);
                lib::L2CValue::L2CValue(aLStack352,0xce07c73da);
                uVar6 = lib::L2CValue::as_integer(aLStack336);
                uVar7 = lib::L2CValue::as_integer(aLStack352);
                fVar10 = (float)app::lua_bind::WorkModule__get_param_float_impl(fighter.module_accessor,uVar6,uVar7);
                lib::L2CValue::L2CValue(aLStack80,fVar10);
                uVar6 = lib::L2CValue::operator__(aLStack80,aLStack304);
                if ((uVar6 & 1) == 0) {
                    lib::L2CValue::_L2CValue(aLStack80);
                    lib::L2CValue::_L2CValue(aLStack352);
                    lib::L2CValue::_L2CValue(aLStack336);
                    lib::L2CValue::_L2CValue(aLStack304);
                    pLVar5 = aLStack320;
                    LAB_710000ed2c:
                    lib::L2CValue::_L2CValue(pLVar5);
                }
            else {
                uVar6 = lib::L2CValue::operator__(aLStack208,aLStack176);
                lib::L2CValue::_L2CValue(aLStack80);
                lib::L2CValue::_L2CValue(aLStack352);
                lib::L2CValue::_L2CValue(aLStack336);
                lib::L2CValue::_L2CValue(aLStack304);
                lib::L2CValue::_L2CValue(aLStack320);
                if ((uVar6 & 1) != 0) {
                    lib::L2CValue::L2CValue(aLStack80,_GROUND_TOUCH_FLAG_LEFT);
                    lib::L2CValue::operator_(aLStack112,aLStack80);
                    lib::L2CValue::_L2CValue(aLStack80);
                    bVar2 = lib::L2CValue::operator_cast_to_bool(aLStack304);
                    lib::L2CValue::_L2CValue(aLStack304);
                    if ((bVar2 & 1U) == 0) {
                        lib::L2CValue::L2CValue(aLStack80,1.0);
                        uVar6 = lib::L2CValue::operator__(aLStack192,aLStack80);
                        lib::L2CValue::_L2CValue(aLStack80);
                        if ((uVar6 & 1) != 0) {
                            lib::L2CValue::L2CValue(aLStack80,_FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL);
                            lib::L2CValue::operator_(aLStack128,aLStack80);
                            goto LAB_710000ee38;
                        }
                    }
                    else {
                        lib::L2CValue::L2CValue(aLStack80,-1.0);
                        uVar6 = lib::L2CValue::operator__(aLStack192,aLStack80);
                        lib::L2CValue::_L2CValue(aLStack80);
                        if ((uVar6 & 1) != 0) {
                            lib::L2CValue::L2CValue(aLStack80,_FIGHTER_LUIGI_STATUS_KIND_SPECIAL_S_WALL);
                            lib::L2CValue::operator_(aLStack128,aLStack80);
                            LAB_710000ee38:
                            pLVar5 = aLStack80;
                            goto LAB_710000ed2c;
                        }
                    }
                }
            }
        }
        lib::L2CValue::L2CValue(aLStack80,aLStack128);
        lib::L2CValue::L2CValue(aLStack304,false);
        lua2cpp::L2CFighterBase::change_status(this,(L2CValue)0xb0,(L2CValue)0xd0);
        lib::L2CValue::_L2CValue(aLStack304);
        LAB_710000ed60:
        pLVar5 = aLStack80;
        LAB_710000ed64:
        lib::L2CValue::_L2CValue(pLVar5);
        //cum back later
    }
    lib::L2CValue::_L2CValue(aLStack288);
    lib::L2CValue::_L2CValue(aLStack272);
    lib::L2CValue::_L2CValue(aLStack256);
    lib::L2CValue::_L2CValue(aLStack240);
    lib::L2CValue::_L2CValue(aLStack224);
    lib::L2CValue::_L2CValue(aLStack208);
    lib::L2CValue::_L2CValue(aLStack192);
    lib::L2CValue::_L2CValue(aLStack176);
    lib::L2CValue::_L2CValue(aLStack160);
    lib::L2CValue::_L2CValue(aLStack144);
    lib::L2CValue::_L2CValue(aLStack128);
    lib::L2CValue::_L2CValue(aLStack112);
    pLVar5 = aLStack96;
    LAB_710000edcc:
    lib::L2CValue::_L2CValue(pLVar5);
    lib::L2CValue::L2CValue((L2CValue *)return_value,0);
    return;
}*/


unsafe extern "C" fn luigi_stepjump_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_DAMAGE_FALL, false);
    }
}

unsafe extern "C" fn luigi_jab_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 361, 25, 20, 20, 2.9, 0.0, 6.6, 6.5, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.2, 361, 25, 10, 15, 3.1, 0.0, 6.6, 9.1, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.2, 180, 20, 15, 20, 3.6, 0.0, 6.6, 12.3, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 3, 0, Hash40::new("top"), 2.2, 361, 20, 15, 20, 3.6, 0.0, 6.6, 12.3, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 3.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 3, 3.0, false);
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

unsafe extern "C" fn luigi_jab2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.2, 361, 25, 20, 20, 2.9, 0.0, 5.2, 5.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.2, 361, 20, 15, 20, 3.8, 0.0, 5.8, 9.5, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 2.2, 180, 20, 15, 25, 4.5, 0.0, 6.2, 14.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 2, 2.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

unsafe extern "C" fn luigi_jab3_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 6.1, 28, 130, 0, 60, 5.0, 0.0, 8.5, 9.2, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 6.1, 28, 130, 0, 60, 3.5, 0.0, 7.0, 4.0, None, None, None, 1.6, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 6, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_HIP);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.25);
    }
}

unsafe extern "C" fn luigi_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.0, 0, 10, 38, 38, 3.6, 0.0, 8.2, 8.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.0, 0, 10, 60, 60, 4.0, 0.0, 8.2, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 2.0, 0, 10, 38, 38, 3.6, 0.0, 8.2, 8.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 2.0, 0, 10, 60, 60, 4.0, 0.0, 8.2, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.0, 0, 10, 38, 38, 3.6, 0.0, 8.2, 8.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 3.0, 0, 10, 60, 60, 4.0, 0.0, 8.2, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 4.0, 15, 10, 24, 24, 3.6, 0.0, 8.2, 8.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 4.0, 10, 10, 48, 48, 4.0, 0.0, 8.2, 2.0, None, None, None, 0.5, 0.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, true, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 5.0, 361, 134, 0, 77, 5.0, 0.0, 8.0, 12.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 5.0, 361, 134, 0, 77, 4.0, 0.0, 8.0, 5.0, None, None, None, 2.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KAMEHIT, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.1);
    }
}

unsafe extern "C" fn luigi_ftilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.4, 361, 95, 0, 50, 5.2, 4.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 9.4, 361, 95, 0, 50, 4.2, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

unsafe extern "C" fn luigi_ftilt_lw_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 9.4, 0, 70, 0, 20, 5.2, 4.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legl"), 9.4, 0, 70, 0, 20, 4.2, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.5, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 2, 0, Hash40::new("kneel"), 9.4, 0, 81, 0, 20, 5.2, 4.3, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 3, 0, Hash40::new("legl"), 9.4, 0, 81, 0, 20, 4.2, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_A, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 5.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 5.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

unsafe extern "C" fn luigi_utilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 8.2, 99, 89, 0, 39, 3.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("arml"), 8.2, 99, 89, 0, 39, 4.3, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 2, 0, Hash40::new("handl"), 8.2, 99, 89, 0, 39, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
}

unsafe extern "C" fn luigi_dtilt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 6.4, 0, 60, 27, 27, 4.8, 3.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legr"), 6.4, 0, 60, 27, 27, 3.8, 2.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 1.5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 2.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 2.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_fsmash_charge_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        LUIGI_FSMASH_ELECTRIC[entry_id] = true;
    }
}

unsafe extern "C" fn luigi_fsmash_charge_effect_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        LUIGI_FSMASH_ELECTRIC[entry_id] = true;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    for _ in 0..i32::MAX {
        if macros::is_excute(fighter) {
            macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_damage_elec"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.6, false);
        }
        sv_animcmd::wait(fighter.lua_state_agent, 8.0);
    }
}

unsafe extern "C" fn luigi_fsmash_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let is_electric;
    let is_electric_sound;

    if LUIGI_FSMASH_ELECTRIC[entry_id] {
        is_electric = Hash40::new("collision_attr_elec");
        is_electric_sound = *COLLISION_SOUND_ATTR_ELEC;
    }
    else {
        is_electric = Hash40::new("collision_attr_sting");
        is_electric_sound = *COLLISION_SOUND_ATTR_LUIGI_SMASH;
    }

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        LUIGI_FSMASH_ELECTRIC[entry_id] = false;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 2.5, y: 0.0, z: 0.0});
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.02, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 16.2, 42, 117, 0, 28, 4.2, 8.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, is_electric, *ATTACK_SOUND_LEVEL_L, is_electric_sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 16.2, 42, 117, 0, 28, 4.6, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, is_electric, *ATTACK_SOUND_LEVEL_L, is_electric_sound, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_fsmash2_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let is_electric;
    let is_electric_sound;

    if LUIGI_FSMASH_ELECTRIC[entry_id] {
        is_electric = Hash40::new("collision_attr_elec");
        is_electric_sound = *COLLISION_SOUND_ATTR_ELEC;
    }
    else {
        is_electric = Hash40::new("collision_attr_sting");
        is_electric_sound = *COLLISION_SOUND_ATTR_LUIGI_SMASH;
    }

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        LUIGI_FSMASH_ELECTRIC[entry_id] = false;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 2.5, y: 0.0, z: 0.0});
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.02, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 16.2, 73, 121, 0, 28, 4.2, 8.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, is_electric, *ATTACK_SOUND_LEVEL_L, is_electric_sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 16.2, 73, 121, 0, 28, 4.6, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, is_electric, *ATTACK_SOUND_LEVEL_L, is_electric_sound, *ATTACK_REGION_PUNCH);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_fsmash3_smash_script(fighter: &mut L2CAgentBase) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    let is_electric;
    let is_electric_sound;

    if LUIGI_FSMASH_ELECTRIC[entry_id] {
        is_electric = Hash40::new("collision_attr_elec");
        is_electric_sound = *COLLISION_SOUND_ATTR_ELEC;
    }
    else {
        is_electric = Hash40::new("collision_attr_sting");
        is_electric_sound = *COLLISION_SOUND_ATTR_LUIGI_SMASH;
    }

    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        LUIGI_FSMASH_ELECTRIC[entry_id] = false;
    }
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        KineticModule::add_speed(fighter.module_accessor, &smash::phx::Vector3f{x: 2.5, y: 0.0, z: 0.0});
    }
    sv_animcmd::frame(fighter.lua_state_agent, 12.0);
    if macros::is_excute(fighter) {
        macros::SET_SPEED_EX(fighter, 0.02, 0, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
        macros::ATTACK(fighter, 0, 0, Hash40::new("shoulderl"), 16.2, 27, 110, 0, 28, 4.2, 8.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, is_electric, *ATTACK_SOUND_LEVEL_L, is_electric_sound, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 16.2, 27, 110, 0, 28, 4.6, 2.5, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, is_electric, *ATTACK_SOUND_LEVEL_L, is_electric_sound, *ATTACK_REGION_PUNCH);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_usmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("head"), HitStatus(*HIT_STATUS_XLU), 0);
        HitModule::set_status_joint(fighter.module_accessor, Hash40::new("snout"), HitStatus(*HIT_STATUS_XLU), 0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 15.3, 103, 108, 0, 35, 5.2, 2.8, -1.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 15.3, 103, 108, 0, 35, 4.4, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_dsmash_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.7, 120, 120, 0, 40, 4.5, 0.0, 3.6, 11.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 14.5, 120, 120, 0, 40, 4.0, 0.0, 3.6, 6.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 14.7, 120, 127, 0, 40, 4.5, 0.0, 3.6, -10.8, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 16.5, 120, 127, 0, 40, 4.0, 0.0, 3.6, -6.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_nair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, crate::consts::FIGHTER_STATUS_ATTACK_WORK_FLAG_HITFALL);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 13.1, 90, 90, 0, 20, 4.9, 1.1, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 13.1, 90, 90, 0, 20, 4.4, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 3.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.23);
        WorkModule::on_flag(fighter.module_accessor, crate::consts::FIGHTER_STATUS_ATTACK_WORK_FLAG_HITFALL);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneel"), 7.8, 80, 100, 0, 20, 4.3, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 7.8, 80, 100, 0, 20, 4.0, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 4.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 4.0, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 32.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        WorkModule::off_flag(fighter.module_accessor, crate::consts::FIGHTER_STATUS_ATTACK_WORK_FLAG_HITFALL);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn luigi_fair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("arml"), 7.0, 55, 64, 0, 40, 5.6, 3.4, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("shoulderl"), 7.0, 55, 64, 0, 40, 3.84, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_SLAP, *ATTACK_REGION_PUNCH);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 0, 1.0, false);
        AttackModule::set_add_reaction_frame_revised(fighter.module_accessor, 1, 1.0, false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_bair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 14.6, 361, 100, 0, 18, 4.5, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legr"), 14.6, 361, 100, 0, 18, 5.8, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("kneer"), 9.0, 361, 100, 0, 18, 4.0, 4.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("legr"), 9.0, 361, 100, 0, 18, 5.0, 1.6, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.3);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 33.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn luigi_uair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 8.1, 68, 74, 0, 32, 4.0, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 8.1, 68, 74, 0, 32, 4.0, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("legr"), 10.3, 260, 80, 0, 15, 3.5, 1.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("kneer"), 10.3, 260, 80, 0, 15, 3.5, 3.2, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn luigi_dair_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 7.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("hip"), 12.5, 361, 103, 0, 21, 4.8, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 12.5, 270, 80, 0, 30, 5.3, 0.0, -0.5, 2.9, None, None, None, 1.3, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_KICK);    
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING);
    }
}

unsafe extern "C" fn luigi_grab_smash_script(fighter: &mut L2CAgentBase) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, smash::app::ArticleOperationTarget(0));
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, smash::app::ArticleOperationTarget(0));
        }
    }
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("catch"), false, -1.0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, smash::app::ArticleOperationTarget(0));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
        macros::SEARCH(fighter, 0, 0, Hash40::new("throw"), 3.7, 0.0, 0.0, -1.5, None, None, None, *COLLISION_KIND_MASK_HIT, *HIT_STATUS_MASK_NORMAL, 1, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_FIGHTER, *COLLISION_PART_MASK_BODY_HEAD, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.7, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.5, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(8.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(fighter);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, smash::app::ArticleOperationTarget(0));
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
        search!(fighter, *MA_MSC_CMD_SEARCH_SEARCH_SCH_CLR_ALL);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn luigi_grabd_smash_script(fighter: &mut L2CAgentBase) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, smash::app::ArticleOperationTarget(0));
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, smash::app::ArticleOperationTarget(0));
        }
    }
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("catch_dash"), false, -1.0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, smash::app::ArticleOperationTarget(0));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.7, 0.0, 0.0, -1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.4, 0.0, 6.6, 4.0, Some(0.0), Some(6.6), Some(10.4), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(fighter);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, smash::app::ArticleOperationTarget(0));
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn luigi_grabp_smash_script(fighter: &mut L2CAgentBase) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, smash::app::ArticleOperationTarget(0));
        }
    }
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER) {
        if macros::is_excute(fighter) {
            ArticleModule::remove_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, smash::app::ArticleOperationTarget(0));
        }
    }
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, false, -1);
        ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("catch_turn"), false, -1.0);
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, false, smash::app::ArticleOperationTarget(0));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        GrabModule::set_rebound(fighter.module_accessor, true);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::CATCH(fighter, 0, Hash40::new("throw"), 3.7, 0.0, 0.0, 1.5, None, None, None, *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
        macros::CATCH(fighter, 1, Hash40::new("top"), 3.6, 0.0, 6.6, -4.0, Some(0.0), Some(6.6), Some(-14.7), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_G);
    }
    macros::game_CaptureCutCommon(fighter);
    if macros::is_excute(fighter) {
        ArticleModule::set_visibility_whole(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, true, smash::app::ArticleOperationTarget(0));
        ArticleModule::shoot(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_PLUNGER, smash::app::ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        GrabModule::clear(fighter.module_accessor, 1);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        GrabModule::clear_all(fighter.module_accessor);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT);
        GrabModule::set_rebound(fighter.module_accessor, false);
    }
}

unsafe extern "C" fn luigi_zair_projectile(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("plungerrope26"), 7.5, 38, 67, 0, 44, 3.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
}

unsafe extern "C" fn luigi_fthrow_smash_script(fighter: &mut L2CAgentBase) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU){
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_f"), false, -1.0);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 285, 90, 0, 23, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 13.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, 18, 4);
        //FighterCutInManager::set_throw_finish_zoom_rate(1)
        //FighterCutInManager::set_throw_finish_offset(12, -1, 0)
        fighter.clear_lua_stack();
        lua_args!(fighter, 5, 1);
        sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }   
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
    sv_animcmd::frame(fighter.lua_state_agent, 26.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_bthrow_smash_script(fighter: &mut L2CAgentBase) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU){
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_b"), false, -1.0);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 8.8, 41, 124, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_REVERSE_LR_FINISH_CAMERA_THROW_ORBIT);
        macros::CHECK_FINISH_CAMERA(fighter, 21, 3);
        //FighterCutInManager::set_throw_finish_zoom_rate(1);
        //FighterCutInManager::set_throw_finish_offset(10, -1, 0);
        fighter.clear_lua_stack();
        lua_args!(fighter, 5, 1);
        sv_animcmd::FT_CATCH_STOP(fighter.lua_state_agent);
        fighter.clear_lua_stack();
    }   
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        PostureModule::reverse_lr(fighter.module_accessor);
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn luigi_uthrow_smash_script(fighter: &mut L2CAgentBase) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU){
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_hi"), false, -1.0);
        }
    }
    if macros::is_excute(fighter) {
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 11.6, 160, 72, 0, 50, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK_IGNORE_THROW(fighter, 0, 0, Hash40::new("top"), 6.0, 88, 55, 0, 95, 8.5, 0.0, 8.0, 10.0, None, None, None, 0.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_THROW);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 16.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        macros::CHECK_FINISH_CAMERA(fighter, -2, 24);
        //FighterCutInManager::set_throw_finish_zoom_rate(1.5)
        //FighterCutInManager::set_throw_finish_offset(0, 7, 0)
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
    }
}

unsafe extern "C" fn luigi_dthrow_smash_script(fighter: &mut L2CAgentBase) {
    if ArticleModule::is_exist(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU) {
        if macros::is_excute(fighter) {
            ArticleModule::change_motion(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_OBAKYUMU, Hash40::new("throw_lw"), false, -1.0);
        }
    }
    if macros::is_excute(fighter) {
        macros::FT_LEAVE_NEAR_OTTOTTO(fighter, -1, 1);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 3.0, 65, 110, 0, 45, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 3.2, 361, 100, 0, 30, 5.0, 0.0, 3.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 100, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_HIP);
        AttackModule::set_catch_only_all(fighter.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(fighter, 0, 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        macros::ATK_HIT_ABS(fighter, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP), WorkModule::get_int64(fighter.module_accessor,*FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO));
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_neutralb_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 17.0);
    if macros::is_excute(fighter) {
        /*let rand_num = smash::app::sv_math::rand(hash40("fighter"), 6);

        if rand_num == 0 {
            ArticleModule::generate_article(fighter.module_accessor, FIGHTER_LUIGI_GENERATE_ARTICLE_MISFIRE_AXE, false, -1);
        }
        else {
            ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
    
        }*/
        ArticleModule::generate_article(fighter.module_accessor, *FIGHTER_LUIGI_GENERATE_ARTICLE_FIREBALL, false, -1);
    
    }
}


unsafe extern "C" fn luigi_fireball(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 8.0, 122, 40, 0, 60, 2.4, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_SPEED, false, 3, 0.0, 0, true, true, false, false, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_ice"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FREEZE, *ATTACK_REGION_NONE);
        AttackModule::set_ice_frame_mul(fighter.module_accessor, 0, 1.1, false);
        AttackModule::enable_safe_pos(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_fireball_effect(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        if PostureModule::lr(fighter.module_accessor) < 0.0 {
		    macros::EFFECT_FOLLOW(fighter, Hash40::new("luigi_fb_bullet_l"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
	    }
	    else {
			macros::EFFECT_FOLLOW(fighter, Hash40::new("luigi_fb_bullet_r"), Hash40::new("rot"), 0, 0, 0, 0, 0, 0, 1, true);
		}
        macros::LAST_EFFECT_SET_COLOR(fighter, 1.0, 1.0, 50.0);
	}
}

unsafe extern "C" fn luigi_sideb_fly_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_AIR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 11.4, 361, 80, 0, 30, 5.5, 3.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
        JostleModule::set_status(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 11.4, 361, 80, 0, 30, 5.5, 3.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 5, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}
    
unsafe extern "C" fn luigi_sideb_misfire_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::SA_SET(fighter, *SITUATION_KIND_AIR);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_LUIGI_SPECIAL_S_RAM);
        macros::CORRECT(fighter, *GROUND_CORRECT_KIND_AIR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("neck"), 30.4, 361, 100, 0, 20, 6.5, 3.0, 0.0, 0.0, None, None, None, 1.2, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_SPEED, false, 30, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_FLOOR, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_FIRE, *ATTACK_REGION_HEAD);
        AttackModule::set_attack_keep_rumble(fighter.module_accessor, 0, true);
        JostleModule::set_status(fighter.module_accessor, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_GROUND_CHECK);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_TRANSITION_TERM_ID_GROUND);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_S_RAM_FLAG_BRAKE);
    }
}

unsafe extern "C" fn luigi_upb_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 25.3, 88, 80, 0, 50, 3.9, 1.2, 5.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 4.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 4.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
        //sys_club_tornado
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_upb_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.5);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 6.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 19.0, 88, 80, 0, 40, 3.9, 1.2, 5.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_fire"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_PUNCH);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 4.0, 80, 1, 0, 1, 5.8, 2.0, 2.2, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        macros::ATTACK(fighter, 1, 0, Hash40::new("hip"), 4.0, 80, 1, 0, 1, 4.7, 0.0, 4.8, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_coin"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_COIN, *ATTACK_REGION_PUNCH);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        macros::SA_SET(fighter, *SITUATION_KIND_AIR);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        notify_event_msc_cmd!(fighter, Hash40::new_raw(0x2127e37c07), GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_fall_special_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        JostleModule::set_status(fighter.module_accessor, false);
        macros::ATTACK(fighter, 0, 0, Hash40::new("handl"), 10.9, 270, 93, 0, 25, 5.0, 0.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_THRU, *ATTACK_LR_CHECK_POS, false, 8, 0.0, 15, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_PUNCH);
        macros::EFFECT_FOLLOW(fighter, Hash40::new("sys_speedbooster"), Hash40::new("rot"), 0, -5, 0, 0, 0, 0, 0.6, true);
    }
}

unsafe extern "C" fn luigi_upb_landing_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 37.0);
    if macros::is_excute(fighter) {
        CancelModule::enable_cancel(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_downb_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 367, 30, 50, 85, 6.0, 0.0, 10.0, -5.5, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 367, 30, 50, 85, 6.0, 0.0, 10.0, 5.5, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 367, 60, 40, 35, 6.5, 0.0, 5.5, 0.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 0.0, 180, 100, 50, 0, 19.5, 0.0, 8.5, 0.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &smash::phx::Vector2f{x: 0.0, y:8.0}, 9, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &smash::phx::Vector2f{x: 0.0, y:8.0}, 9, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40::new("top"), &smash::phx::Vector2f{x: 0.0, y:8.0}, 9, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.58823529);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 77, 130, 0, 85, 7.5, 0.0, 11.0, -9.5, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 77, 130, 0, 85, 7.5, 0.0, 11.0, 9.5, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 90, 130, 0, 85, 6.5, 0.0, 4.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_downb_air_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_XLU), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
        HitModule::set_status_all(fighter.module_accessor, HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 10.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 1.6, 367, 30, 50, 85, 6.0, 0.0, 10.0, -5.5, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 1.6, 367, 30, 50, 85, 6.0, 0.0, 10.0, 5.5, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 1.6, 367, 60, 40, 35, 6.5, 0.0, 5.5, 0.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 5, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_rush"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 4, 1, Hash40::new("top"), 0.0, 180, 100, 50, 0, 19.5, 0.0, 8.5, 0.0, None, None, None, 0.6, 0.8, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, true, 0, 0.0, 10, false, false, true, true, false, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_NO_ITEM, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_NONE);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 0, Hash40::new("top"), &smash::phx::Vector2f{x: 0.0, y:8.0}, 8, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 1, Hash40::new("top"), &smash::phx::Vector2f{x: 0.0, y:8.0}, 8, false);
        AttackModule::set_vec_target_pos(fighter.module_accessor, 2, Hash40::new("top"), &smash::phx::Vector2f{x: 0.0, y:8.0}, 8, false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 34.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 0.58823529);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 40.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 44.0);
    if macros::is_excute(fighter) {
        macros::FT_MOTION_RATE(fighter, 1.0);
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 7.0, 74, 109, 0, 67, 7.5, 0.0, 11.0, -9.5, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 1, 0, Hash40::new("top"), 7.0, 74, 109, 0, 67, 7.5, 0.0, 11.0, 9.5, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
        macros::ATTACK(fighter, 2, 0, Hash40::new("top"), 7.0, 90, 109, 0, 67, 6.5, 0.0, 4.0, 0.0, None, None, None, 1.7, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_BODY);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 2.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_LIMIT_X_DEC);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_LUIGI_STATUS_SPECIAL_LW_FLAG_RISE);
        WorkModule::on_flag(fighter.module_accessor, *FIGHTER_LUIGI_INSTANCE_WORK_ID_FLAG_SPECIAL_LW_BUOYANCY);
        AttackModule::clear_all(fighter.module_accessor);
    }
}

unsafe extern "C" fn luigi_downtaunt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 2.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 42.0);
    if macros::is_excute(fighter) {
        MotionModule::set_rate(fighter.module_accessor, 1.0);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 45.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("top"), 10.0, 270, 100, 0, 80, 4.0, 0.0, -1.0, 5.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 15, 0.0, 8, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_sting"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_BAT, *ATTACK_REGION_KICK);
        AttackModule::set_attack_height_all(fighter.module_accessor, AttackHeight(*ATTACK_HEIGHT_LOW), false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 46.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
        MotionModule::set_rate(fighter.module_accessor, 1.2);
    }
}

unsafe extern "C" fn luigi_sidetaunt_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 21.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, 0, 0, Hash40::new("head"), 7.7, 330, 90, 0, 40, 5.2, 2.8, 0.0, 0.0, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_POS, false, 10, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_HEAVY, *ATTACK_REGION_HEAD);
    }
    sv_animcmd::wait(fighter.lua_state_agent, 19.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//#[skyline::main(name = "tr4sh_rebuffed")]
pub fn install() {
    Agent::new("luigi")
    .on_line(Main, luigi_frame) //opff
    .game_acmd("game_stepjump", luigi_stepjump_smash_script, Low)
    .game_acmd("game_attack11", luigi_jab_smash_script, Low)
    .game_acmd("game_attack12", luigi_jab2_smash_script, Low)
    .game_acmd("game_attack13", luigi_jab3_smash_script, Low)
    .game_acmd("game_attackdash", luigi_dashattack_smash_script, Low)
    .game_acmd("game_attacks3", luigi_ftilt_smash_script, Low)
    .game_acmd("game_attacks3hi", luigi_ftilt_smash_script, Low)
    .game_acmd("game_attacks3lw", luigi_ftilt_lw_smash_script, Low)
    .game_acmd("game_attackhi3", luigi_utilt_smash_script, Low)
    .game_acmd("game_attacklw3", luigi_dtilt_smash_script, Low)
    .game_acmd("game_attacks4charge", luigi_fsmash_charge_smash_script, Low)
    .effect_acmd("effect_attacks4charge", luigi_fsmash_charge_effect_script, Low)
    .game_acmd("game_attacks4", luigi_fsmash_smash_script, Low)
    .game_acmd("game_attacks4hi", luigi_fsmash2_smash_script, Low)
    .game_acmd("game_attacks4lw", luigi_fsmash3_smash_script, Low)
    .game_acmd("game_attackhi4", luigi_usmash_smash_script, Low)
    .game_acmd("game_attacklw4", luigi_dsmash_smash_script, Low)
    .game_acmd("game_attackairn", luigi_nair_smash_script, Low)
    .game_acmd("game_attackairf", luigi_fair_smash_script, Low)
    .game_acmd("game_attackairb", luigi_bair_smash_script, Low)
    .game_acmd("game_attackairhi", luigi_uair_smash_script, Low)
    .game_acmd("game_attackairlw", luigi_dair_smash_script, Low)
    .game_acmd("game_catch", luigi_grab_smash_script, Low)
    .game_acmd("game_catchdash", luigi_grabd_smash_script, Low)
    .game_acmd("game_catchturn", luigi_grabp_smash_script, Low)
    .game_acmd("game_throwf", luigi_fthrow_smash_script, Low)
    .game_acmd("game_throwb", luigi_bthrow_smash_script, Low)
    .game_acmd("game_throwhi", luigi_uthrow_smash_script, Low)
    .game_acmd("game_throwlw", luigi_dthrow_smash_script, Low)
    .game_acmd("game_specialn", luigi_neutralb_smash_script, Low)
    .game_acmd("game_specialairn", luigi_neutralb_smash_script, Low)
    .game_acmd("game_specials", luigi_sideb_fly_smash_script, Low)
    .game_acmd("game_specialsdischarge", luigi_sideb_misfire_smash_script, Low)
    .game_acmd("game_specialhi", luigi_upb_smash_script, Low)
    .game_acmd("game_specialairhi", luigi_upb_air_smash_script, Low)
    .game_acmd("game_specialhifall", luigi_fall_special_script, Low)
    .game_acmd("game_specialhilandingfall", luigi_upb_landing_smash_script, Low)
    .game_acmd("game_speciallw", luigi_downb_smash_script, Low)
    .game_acmd("game_specialairlw", luigi_downb_air_smash_script, Low)
    .game_acmd("game_appealsl", luigi_sidetaunt_smash_script, Low)
    .game_acmd("game_appealsr", luigi_sidetaunt_smash_script, Low)
    .game_acmd("game_appeallwl", luigi_downtaunt_smash_script, Low)
    .game_acmd("game_appeallwr", luigi_downtaunt_smash_script, Low)
    .install();
  
    Agent::new("luigi_fireball")
    .game_acmd("game_regular", luigi_fireball, Low)
    .effect_acmd("effect_regular", luigi_fireball_effect, Low)
    .install();

    /*Agent::new("luigi_plunger")
    .game_acmd("game_aircatch", luigi_zair_projectile)
    .install();*/

    //smashline::clone_weapon("richter", "axe", "luigi", "misfire_axe", true);

}