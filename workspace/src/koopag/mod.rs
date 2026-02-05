#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use smash::hash40;
use singletons::FighterManager;
use smash::phx::Vector3f;
use crate::custom::global_fighter_frame;
use smashline::Priority::*;


// A Once-Per-Fighter-Frame that only applies to Giga Bowser
unsafe extern "C" fn koopag_frame(fighter: &mut L2CFighterCommon) {
    unsafe {

        global_fighter_frame(fighter);
        //println!("gwaaaaaaah!");

        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let motion_kind = MotionModule::motion_kind(module_accessor);
        let status_kind = StatusModule::status_kind(fighter.module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let mut globals = fighter.globals_mut().clone();
        let GIGA_DTILT = &mut FIGHTER_BOOL_1[get_player_number(module_accessor)];
        let GIGA_DASH_ATTACK = &mut FIGHTER_BOOL_2[get_player_number(module_accessor)];
        let DTILT_INPUT = &mut FIGHTER_BOOL_3[get_player_number(module_accessor)];

        if let L2CValueType::Void = globals["giga_globals_set"].val_type {
            globals["giga_buffer_timer"] = 0.0.into();
            *GIGA_DTILT = false;
            *GIGA_DASH_ATTACK = false;
            globals["giga_dash"] = false.into();
            globals["giga_neutral_b"] = false.into();
            globals["giga_side_b"] = false.into();
            globals["giga_lr"] = 0.0.into();
            globals["giga_guard_grab"] = false.into();
            globals["giga_hitlag"] = false.into();
            globals["giga_gravity"] = 0.0.into();
            globals["giga_situation"] = 0.into();
            globals["giga_globals_set"] = true.into();
        }
        if ControlModule::check_button_trigger(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if dtilt_input(module_accessor) {
                *DTILT_INPUT = true;
                globals["giga_buffer_timer"] = 6.0.into();
            }
            else {
                *DTILT_INPUT = false;
            }
        }
        if ControlModule::check_button_off(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) {
            if globals["giga_buffer_timer"].get_num() > 0.0 {
                globals["giga_buffer_timer"] = (globals["giga_buffer_timer"].get_num() - 1.0).into();
            }
            else {
                *DTILT_INPUT = false;
            }
        }
        if situation_kind == SITUATION_KIND_GROUND {
            if *DTILT_INPUT && WorkModule::is_enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_ATTACK_HI3) 
            && motion_kind != hash40("attack_lw3") 
            && status_kind != *FIGHTER_STATUS_KIND_ATTACK_S3 
            && motion_kind != hash40("rebound") {
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                ControlModule::clear_command(module_accessor, true);
                *GIGA_DTILT = true;
            }
        }
        if status_kind == *FIGHTER_STATUS_KIND_RUN {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_ATTACK) && ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
                StatusModule::change_status_force(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_ATTACK_S3, true);
                *GIGA_DASH_ATTACK = true;
            }
        }
        if motion_kind == hash40("attack_dash") {
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            if MotionModule::frame(fighter.module_accessor) >= 58.0 {
                if *DTILT_INPUT {
                    MotionModule::change_motion(fighter.module_accessor, Hash40::new("attack_lw3"), 0.0, 1.0, false, 0.0, false, false);
                    *DTILT_INPUT = false;	
                }
                else {
                    if ControlModule::get_stick_y(fighter.module_accessor) < -0.66 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("squat"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);							
                    }
                    else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40::new("wait"), 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                }
            }
        }
        if motion_kind == hash40("attack_lw3") {
            PostureModule::update_rot_y_lr(fighter.module_accessor);
            if MotionModule::frame(fighter.module_accessor) >= 36.0 {
                if *DTILT_INPUT {
                    MotionModule::change_motion(fighter.module_accessor, Hash40{hash: hash40("attack_lw3")}, 0.0, 1.0, false, 0.0, false, false);
                    *DTILT_INPUT = false;
                }
                else {
                    if ControlModule::get_stick_y(fighter.module_accessor) < -0.66 {
                        MotionModule::change_motion(fighter.module_accessor, Hash40{hash: hash40("squat")}, 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_SQUAT, true);							
                    }
                    else {
                        MotionModule::change_motion(fighter.module_accessor, Hash40{hash: hash40("wait")}, 0.0, 1.0, false, 0.0, false, false);
                        StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_WAIT, true);
                    }
                    CancelModule::enable_cancel(fighter.module_accessor);
                }
            }
        }
        //Allows Air Neutral and Side B to cancel on landing (and prevents the animations from restarting on landing)
        if motion_kind == hash40("special_s_catch") || motion_kind == hash40("special_s_air_catch") {
            if motion_kind == hash40("special_s_air_catch") {
                globals["giga_side_b"] = true.into();
            }
            if motion_kind == hash40("special_s_catch") && globals["giga_side_b"].get_bool() {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
            }
        }
        else {
            globals["giga_side_b"] = false.into();
        }
        if motion_kind == hash40("special_air_n_start") || motion_kind == hash40("special_n_start") {
            if motion_kind == hash40("special_air_n_start") {
                globals["giga_neutral_b"] = true.into();
            }
            if motion_kind == hash40("special_n_start") && globals["giga_neutral_b"].get_bool() {
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_LANDING, true);
            }
        }
        else {
            globals["giga_neutral_b"] = false.into();
        }
        //Enabling certain status changes
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_GUARD);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_RUN);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_SLIP);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_OTTOTTO);
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_DISABLE_MINI_JUMP);	
        Other_Function(fighter);	


    
    }
}


pub unsafe extern "C" fn dtilt_input(module_accessor: &mut smash::app::BattleObjectModuleAccessor) -> bool {
	let stick_y = ControlModule::get_stick_y(module_accessor);
	let flick_y = ControlModule::get_flick_y(module_accessor);
	if ControlModule::check_button_on(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) 
	&& stick_y <= -0.25 && (stick_y > -0.72 || flick_y > 4) 
	&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_GUARD) 
	&& ControlModule::check_button_off(module_accessor, *CONTROL_PAD_BUTTON_CATCH) {
		return true;
	}
	else {
		return false;
	}
}

unsafe fn Other_Function(fighter: &mut L2CFighterCommon) {
    let entry_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	let fighter_info = FighterManager::get_fighter_information(FighterManager(), FighterEntryID(entry_id as i32));
	let status_kind = StatusModule::status_kind(fighter.module_accessor);
	let hit_point = FighterInformation::hit_point(fighter_info);
	let stock_count = FighterInformation::stock_count(fighter_info);
	if FighterUtil::is_hp_mode(fighter.module_accessor) && !is_training_mode() {
		if hit_point <= 0.0 && WorkModule::is_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES) {
			fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(), false.into());
			WorkModule::on_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES);
		}
		if status_kind != *FIGHTER_STATUS_KIND_DEAD && stock_count == 1 {
			if hit_point <= 0.0 {
				fighter.change_status(FIGHTER_STATUS_KIND_DEAD.into(), false.into());
			}
		}
	}
}


pub unsafe extern "C" fn frame_koopag_breath_main(weapon : &mut L2CFighterBase) {
	let status_kind = StatusModule::status_kind(weapon.module_accessor);
	if status_kind == *WEAPON_KOOPA_BREATH_STATUS_KIND_MOVE {
		let pos = PostureModule::pos(weapon.module_accessor);
		if GroundModule::is_touch(weapon.module_accessor, *GROUND_TOUCH_FLAG_SIDE as u32) 
		|| AttackModule::is_infliction_status(weapon.module_accessor, *COLLISION_KIND_MASK_SHIELD) {
			EffectModule::req(weapon.module_accessor, Hash40::new("brave_fire2_hit"), pos, &Vector3f{x: 0.0, y: 0.0, z: 0.0}, 2.0, 0, -1, false, 0);
			SoundModule::play_se(weapon.module_accessor, Hash40::new("se_common_bomb_m"), true, false, false, false, enSEType(0));
			SoundModule::play_se(weapon.module_accessor, Hash40::new("se_koopag_fireball_impact01"), true, false, false, false, enSEType(0));
			notify_event_msc_cmd!(weapon, Hash40::new_raw(0x199c462b5d));
		}
	}
}



//Init
unsafe extern "C" fn start_koopag_init(fighter : &mut L2CFighterCommon) {
    let shield_max = WorkModule::get_param_float(fighter.module_accessor, hash40("common"), hash40("shield_max"));
    WorkModule::set_float(fighter.module_accessor, shield_max * 2.0, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD_MAX);
}
//End of Init


//Status Scripts
unsafe extern "C" fn status_koopag_attackair_main(fighter: &mut L2CFighterCommon) -> L2CValue {
	//L2CFighterCommon_status_AttackAir(fighter)
    fighter.status_AttackAir_Main()
}


unsafe extern "C" fn status_koopag_guardon_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_GuardOn()
}

unsafe extern "C" fn status_koopag_guard_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.sub_status_guard_common();
    fighter.sub_shift_status_main(L2CValue::Ptr(koopag_Guard_Main_loop as *const () as _))
}

unsafe extern "C" fn koopag_Guard_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if !fighter.status_guard_main_common_air().get_bool() {
        if !fighter.sub_guard_cont().get_bool() {
            fighter.status_guard_main_common();
        }
    }
    let shield_hp = WorkModule::get_float(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLOAT_GUARD_SHIELD);
    if shield_hp <= 0.0 {
        fighter.change_status(FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY.into(), false.into());
    }
    0.into()
}


unsafe extern "C" fn status_koopag_shieldbreakfly_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(fighter.module_accessor, SituationKind(*SITUATION_KIND_GROUND), *FIGHTER_KINETIC_TYPE_MOTION_BIND, *GROUND_CORRECT_KIND_GROUND_CLIFF_STOP as u32, GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), true, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT, 0);
    FighterStatusModuleImpl::set_fighter_status_data(fighter.module_accessor, false, *FIGHTER_TREADED_KIND_ENABLE, false, false, false, 0, (*FIGHTER_STATUS_ATTR_DISABLE_JUMP_BOARD_EFFECT | *FIGHTER_STATUS_ATTR_DAMAGE | *FIGHTER_STATUS_ATTR_DISABLE_SHIELD_RECOVERY) as u32, 0, 0);
    0.into()
}

unsafe extern "C" fn status_koopag_shieldbreakfly_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    fighter.status_ShieldBreakFly()
}


unsafe extern "C" fn status_koopag_dead_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let dead_mode = WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_DEAD_MODE);
    fighter.status_DeadSub();
    if dead_mode == *FIGHTER_DEAD_MODE_DEADUP_STAR {
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new("vc_koopag_damage_twinkle"), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 1.0, 0);
    } 
    else if dead_mode != *FIGHTER_DEAD_MODE_DEADUP_FALL {
        let rand_val = sv_math::rand(hash40("fighter"), 2);
        let sound = match rand_val {
            0 => "vc_koopag_missfoot01",
            _ => "vc_koopag_missfoot02",
        };
        let se_handle = SoundModule::play_se(fighter.module_accessor, Hash40::new(sound), true, false, false, false, enSEType(0));
        SoundModule::set_se_vol(fighter.module_accessor, se_handle as i32, 1.0, 0);
    }
    if FighterUtil::is_hp_mode(fighter.module_accessor) {
        if fighter.global_table[0x16].get_i32() != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("death_air"), 0.0, 1.0, false, 0.0, false, false);
        }
        else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("death"), 0.0, 1.0, false, 0.0, false, false);
        }
        fighter.sub_shift_status_main(L2CValue::Ptr(koopag_Dead_Main_loop as *const () as _))
    }
    else {
        fighter.sub_shift_status_main(L2CValue::Ptr(L2CFighterCommon_bind_address_call_status_Dead_Main as *const () as _))
    }
}

unsafe extern "C" fn koopag_Dead_Main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_AIR
    && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_GROUND {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("death"), -1.0, 1.0, 0.0);
    }
    if fighter.global_table[0x17].get_i32() == *SITUATION_KIND_GROUND
    && fighter.global_table[0x16].get_i32() == *SITUATION_KIND_AIR {
        MotionModule::change_motion_inherit_frame_keep_rate(fighter.module_accessor, Hash40::new("death_air"), -1.0, 1.0, 0.0);
    }
    if MotionModule::is_end(fighter.module_accessor) {
        WorkModule::off_flag(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_FLAG_ALLOT_STATUSES);
        fighter.change_status(FIGHTER_STATUS_KIND_STANDBY.into(), false.into());
    }
    0.into()
}


//End of Status Scripts


//General Movement
unsafe extern "C" fn koopag_jumpsquat(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
}

unsafe extern "C" fn koopag_turn(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
}

unsafe extern "C" fn koopag_dash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}

unsafe extern "C" fn koopag_turndash(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 14.0);
    if macros::is_excute(fighter) {
        WorkModule::enable_transition_term(fighter.module_accessor, *FIGHTER_STATUS_TRANSITION_TERM_ID_DASH_TO_RUN);
    }
}
//end of General Movement

unsafe extern "C" fn koopag_jab1_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.25);
    sv_animcmd::frame(fighter.lua_state_agent, 24.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handl"), /*Damage*/ 19.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ -0.1, /*Z*/ 0.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("arml"), /*Damage*/ 19.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 8.0, /*X*/ 8.0, /*Y*/ -0.7, /*Z*/ -0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderl"), /*Damage*/ 19.0, /*Angle*/ 80, /*KBG*/ 100, /*FKB*/ 70, /*BKB*/ 0, /*Size*/ 8.0, /*X*/ 4.0, /*Y*/ 0.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_cutup"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 29.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    sv_animcmd::frame(fighter.lua_state_agent, 36.0);
    if macros::is_excute(fighter) {
        WorkModule::on_flag(fighter.module_accessor, /*Flag*/ *FIGHTER_STATUS_ATTACK_FLAG_ENABLE_COMBO);
    }
}

//Attack12 
unsafe extern "C" fn koopag_jab2_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 0.7);
    sv_animcmd::frame(fighter.lua_state_agent, 8.0);
    macros::FT_MOTION_RATE(fighter, /*FSM*/ 1.0);
    sv_animcmd::frame(fighter.lua_state_agent, 15.0);
    if macros::is_excute(fighter) {
        macros::ATTACK(fighter, /*ID*/ 0, /*Part*/ 0, /*Bone*/ Hash40::new("handr"), /*Damage*/ 21.0, /*Angle*/ 45, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 6.0, /*Y*/ -0.1, /*Z*/ 0.6, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 1, /*Part*/ 0, /*Bone*/ Hash40::new("armr"), /*Damage*/ 21.0, /*Angle*/ 45, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 8.0, /*Y*/ -0.7, /*Z*/ -0.2, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
        macros::ATTACK(fighter, /*ID*/ 2, /*Part*/ 0, /*Bone*/ Hash40::new("shoulderr"), /*Damage*/ 21.0, /*Angle*/ 45, /*KBG*/ 55, /*FKB*/ 0, /*BKB*/ 70, /*Size*/ 8.0, /*X*/ 4.0, /*Y*/ 0.1, /*Z*/ 0.0, /*X2*/ None, /*Y2*/ None, /*Z2*/ None, /*Hitlag*/ 1.0, /*SDI*/ 1.0, /*Clang_Rebound*/ *ATTACK_SETOFF_KIND_OFF, /*FacingRestrict*/ *ATTACK_LR_CHECK_F, /*SetWeight*/ false, /*ShieldDamage*/ 0, /*Trip*/ 0.0, /*Rehit*/ 0, /*Reflectable*/ false, /*Absorbable*/ false, /*Flinchless*/ false, /*DisableHitlag*/ false, /*Direct_Hitbox*/ true, /*Ground_or_Air*/ *COLLISION_SITUATION_MASK_GA, /*Hitbits*/ *COLLISION_CATEGORY_MASK_ALL, /*CollisionPart*/ *COLLISION_PART_MASK_ALL, /*FriendlyFire*/ false, /*Effect*/ Hash40::new("collision_attr_purple"), /*SFXLevel*/ *ATTACK_SOUND_LEVEL_L, /*SFXType*/ *COLLISION_SOUND_ATTR_CUTUP, /*Type*/ *ATTACK_REGION_NONE);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 18.0);
    if macros::is_excute(fighter) {
        AttackModule::clear_all(fighter.module_accessor);
    }
}

//AttackDash
unsafe extern "C" fn koopag_dashattack_smash_script(fighter: &mut L2CAgentBase) {
    sv_animcmd::frame(fighter.lua_state_agent, 1.0);
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.5);
	}
    sv_animcmd::frame(fighter.lua_state_agent, 9.0);
    if macros::is_excute(fighter) {
		MotionModule::set_rate(fighter.module_accessor, 1.0);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 11.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 26.0, 80, 60, 0, 60, 16.0, 0.0, 6.0, 15.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 26.0, 80, 60, 0, 60, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 3, 0, Hash40::new("trans"), 26.0, 80, 60, 0, 60, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 4, 0, Hash40::new("trans"), 26.0, 80, 60, 0, 60, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 25.0, 80, 60, 0, 50, 16.0, 0.0, 6.0, 15.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 25.0, 80, 60, 0, 50, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 2, 0, Hash40::new("trans"), 25.0, 80, 60, 0, 50, 16.0, 0.0, 6.0, 15.2, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 3, 0, Hash40::new("trans"), 25.0, 80, 60, 0, 50, 16.0, 0.0, 6.0, 8.5, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_purple"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
	sv_animcmd::frame(fighter.lua_state_agent, 25.0);
    if macros::is_excute(fighter) {
		macros::ATTACK(fighter, 0, 0, Hash40::new("trans"), 24.0, 74, 80, 0, 30, 18.4, 0.0, 6.0, 16.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 1, 0, Hash40::new("trans"), 24.0, 74, 80, 0, 30, 18.4, 0.0, 6.0, 8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 2, 0, Hash40::new("trans"), 24.0, 74, 80, 0, 30, 18.4, 0.0, 6.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
		macros::ATTACK(fighter, 3, 0, Hash40::new("trans"), 24.0, 74, 80, 0, 30, 18.4, 0.0, 6.0, -8.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_F, false, 3, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_G, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_bury"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_KICK, *ATTACK_REGION_NONE);
	}
	sv_animcmd::wait(fighter.lua_state_agent, 5.0);
    if macros::is_excute(fighter) {
		AttackModule::clear_all(fighter.module_accessor);
	}
}




pub fn install() {
    Agent::new("koopag")
    .on_start(start_koopag_init)
    .on_line(Main, koopag_frame) //opff
    .status(Main, *FIGHTER_STATUS_KIND_ATTACK_AIR, status_koopag_attackair_main)
    .status(Main, *FIGHTER_STATUS_KIND_GUARD, status_koopag_guard_main)
    .status(Main, *FIGHTER_STATUS_KIND_GUARD_ON, status_koopag_guardon_main)
    .status(Pre, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, status_koopag_shieldbreakfly_pre)
    .status(Main, *FIGHTER_STATUS_KIND_SHIELD_BREAK_FLY, status_koopag_shieldbreakfly_main)
    .status(Main, *FIGHTER_STATUS_KIND_DEAD, status_koopag_dead_main)
    .game_acmd("game_jumpsquat", koopag_jumpsquat, Low)
    .game_acmd("game_turn", koopag_turn, Low)
    .game_acmd("game_dash", koopag_dash, Low)
    .game_acmd("game_turndash", koopag_turndash, Low)
    .game_acmd("game_attack11", koopag_jab1_smash_script, Low)
    .game_acmd("game_attack12", koopag_jab2_smash_script, Low)
    .game_acmd("game_attackdash", koopag_dashattack_smash_script, Low)
    .install();

    Agent::new("koopag_breath")
    .on_line(Main, frame_koopag_breath_main)
    .install();


}
