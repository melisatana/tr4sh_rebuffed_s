//Originally Knuckles Code
#![allow(unused_macros)]
use {
	skyline::hooks::{
		InlineCtx,
		Region,
		getRegionAddress
    },
    smash::{
        app::{
			lua_bind::*,
			utility::*,
			*
		},
		hash40,
        lib::{
			L2CAgent,
			lua_const::*,
		},
		lua2cpp::L2CFighterCommon,
        phx::*,
    },
	smashline::*,
};

//use smash2;

//BomaExt, helps with various things
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

pub static mut FLOAT_OFFSET: usize = 0x4E53C0;

pub static FLOAT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x19, 0x40, 0xf9,
];

pub static mut INT_OFFSET : usize = 0x4E5380;

//pub static mut INT_OFFSET: usize = 0x4DED80;

pub static INT_SEARCH_CODE: &[u8] = &[
    0x00, 0x1c, 0x40, 0xf9, 0x08, 0x00, 0x40, 0xf9, 0x03, 0x11, 0x40, 0xf9,
];




//Related to Param Edits
fn find_subsequence(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|window| window == needle)
}

//Related to Param Edits
#[skyline::hook(offset=0x3f0028, inline)]
pub unsafe fn offset_dump(ctx: &InlineCtx) {
	let text = skyline::hooks::getRegionAddress(skyline::hooks::Region::Text) as u64;
	println!("Function Offset: {:#X}", ctx.registers[8].x.as_ref() - text);
}

//Param Adjustments
#[skyline::hook(offset=FLOAT_OFFSET)]
pub unsafe fn get_param_float_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> f32 {
	let mut boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let fighter_kind = boma_reference.kind();
	let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if boma_reference.is_fighter() {
		if fighter_kind == *FIGHTER_KIND_PIKACHU {
			if param_type == hash40("dash_speed") {
				if crate::pikachu::PIKACHU_DOWNB_STATIC_IS_HIT[entry_id] {
					return 2.4;
				}
			}
			if param_type == hash40("run_speed_max") {
				if crate::pikachu::PIKACHU_DOWNB_STATIC_IS_HIT[entry_id] {
					return 2.33;
				}
			}
			if param_type == hash40("air_speed_x_stable") {
				if crate::pikachu::PIKACHU_DOWNB_STATIC_IS_HIT[entry_id] {
					return 1.19;
				}
			}
			if param_type == hash40("weight") {
				if crate::pikachu::PIKACHU_DOWNB_STATIC_IS_HIT[entry_id] {
					return 92.0;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_KIRBY {
			if param_type == hash40("dash_speed") {
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 2.1;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 2.15;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 2.2;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 2.3;
				}
			}
			if param_type == hash40("run_speed_max") {
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 2.0;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 2.2;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 2.4;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 2.8;
				}
			}
			if param_type == hash40("run_accel_mul") {
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 0.111;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 0.12;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 0.128;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 0.13;
				}
			}
			if param_type == hash40("walk_speed_max") {
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 1.0;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 1.15;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 1.25;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 1.35;
				}
			}
			if param_type == hash40("air_speed_x_stable") {
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 1.08;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 1.11;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 1.14;
				}
				if crate::sonic::SKIRBY_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 1.2;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_LUIGI {
			if param_type == hash40("dive_speed_y") {
				if crate::luigi::LUIGI_UPB_SUPERFAST_FALL[entry_id] {
					return 4.5;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_PZENIGAME {
			if param_type == hash40("dash_speed") {
				if crate::pzenigame::SQUIRTLE_TORRENT[entry_id] {
					return 2.5;
				}
			}
			if param_type == hash40("run_speed_max") {
				if crate::pzenigame::SQUIRTLE_TORRENT[entry_id] {
					return 2.4;
				}
			}
			if param_type == hash40("param_special_hi") {
				if param_hash == hash40("pass_mul") {
					if WorkModule::get_int(boma, *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE) == 75 {
						return 1.25;
					}
				}
				if param_hash == hash40("air_pass_mul") {
					if WorkModule::get_int(boma, *FIGHTER_PZENIGAME_INSTANCE_WORK_ID_INT_SPECIAL_N_CHARGE) == 75 {
						return 1.5;
					}
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_SONIC {
			if param_type == hash40("run_speed_max") {
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 3.4;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 3.5;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 3.6;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 3.9;
				}
			}
			if param_type == hash40("walk_speed_max") {
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 1.5;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 1.55;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 1.65;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 1.8;
				}
			}
			if param_type == hash40("run_accel_mul") {
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 0.17;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 0.18;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 0.2;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 0.22;
				}
			}
			if param_type == hash40("dash_speed") {
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 2.5;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 2.6;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 2.7;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 2.8;
				}
			}
			if param_type == hash40("air_speed_x_stable") {
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 1 {
					return 1.24;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 2 {
					return 1.26;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 3 {
					return 1.28;
				}
				if crate::sonic::SONIC_NEUTRALB_CURRENT_SPEEDUP_LEVEL[entry_id] == 4 {
					return 1.31;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_KAMUI {
			if param_type == hash40("landing_attack_air_frame_lw") {
				if crate::kamui::CORRIN_SLOW_DOWN_AIR[entry_id] {
					return 32.0;
				}
			}
    	}
		if fighter_kind == *FIGHTER_KIND_GAOGAEN {
			if param_type == hash40("param_special_n") {
				if param_hash == hash40("special_n_speed_x_max") {
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 5 {
						return 1.6;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 4 {
						return 1.4;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 3 {
						return 1.2;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 2 {
						return 1.0;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 1 {
						return 0.8;
					}
				}
				if param_hash == hash40("special_n_stick_accel_x") {
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 5 {
						return 0.1;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 4 {
						return 0.095;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 3 {
						return 0.09;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 2 {
						return 0.085;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 1 {
						return 0.08;
					}
				}
				if param_hash == hash40("special_air_n_speed_x_max") {
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 5 {
						return 1.6;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 4 {
						return 1.4;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 3 {
						return 1.2;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 2 {
						return 1.0;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 1 {
						return 0.8;
					}
				}
				if param_hash == hash40("special_air_n_stick_accel_x") {
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 5 {
						return 0.1;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 4 {
						return 0.095;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 3 {
						return 0.09;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 2 {
						return 0.085;
					}
					if crate::gaogaen::GAOGAEN_REVENGE_CURRENT_POWER_LEVEL[entry_id] == 1 {
						return 0.08;
					}
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_ELIGHT {
			if param_type == hash40("dash_speed") {
				if crate::elight::MYTHRA_UTHROW_SPEED[entry_id] {
					return 2.45;
				}
			}
			if param_type == hash40("run_speed_max") {
				if crate::elight::MYTHRA_UTHROW_SPEED[entry_id] {
					return 2.41;
				}
			}
			if param_type == hash40("run_accel_add") {
				if crate::elight::MYTHRA_UTHROW_SPEED[entry_id] {
					return 0.1;
				}
			}
			if param_type == hash40("air_speed_x_stable") {
				if crate::elight::MYTHRA_UTHROW_SPEED[entry_id] {
					return 1.28;
				}
			}
		}
		if fighter_kind == *FIGHTER_KIND_TRAIL {
			if param_type == hash40("landing_attack_air_frame_lw") {
				if crate::trail::SORA_SLOW_DOWN_AIR[entry_id] {
					return 29.0;
				}
			}
		}
    }
	else if boma_reference.is_weapon() {
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let entry_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *WEAPON_KIND_MARIO_FIREBALL {
			if param_type == hash40("param_fireball") {
				if param_hash == hash40("speed") {
					if crate::mario::MARIO_GIANT_FIREBALL[entry_id] {
						return 0.9;
					}
				}
				if param_hash == hash40("angle") {
					if crate::mario::MARIO_GIANT_FIREBALL[entry_id] {
						return 15.0;
					}
				}
				if param_hash == hash40("gravity_accel") {
					if crate::mario::MARIO_GIANT_FIREBALL[entry_id] {
						return 0.03;
					}
				}
				if param_hash == hash40("bounded_speed_y_min") {
					if crate::mario::MARIO_GIANT_FIREBALL[entry_id] {
						return 0.6;
					}
				}
				if param_hash == hash40("bounded_speed_y_max") {
					if crate::mario::MARIO_GIANT_FIREBALL[entry_id] {
						return 0.6;
					}
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_MARIOD_DRCAPSULE {
			if param_type == hash40("param_drcapsule") {
				if param_hash == hash40("speed") {
					if crate::mariod::DR_SLOW_PILL[entry_id] {
					return 1.0;
					}
				}
			}
		}
		if fighter_kind == *WEAPON_KIND_REFLET_GIGAFIRE {
			if param_type == hash40("param_gigafire") {
				if param_hash == hash40("shoot_angle") {
					if crate::reflet::REFLET_SIDEB_DOWN_ANGLE[entry_id] {
						return 50.0;
					}
					if crate::reflet::REFLET_SIDEB_UP_ANGLE[entry_id] {
						return -25.0;
					}
				}
			}
		}
		

		
	}
	
	original!()(module_accessor, param_type, param_hash)
}


#[skyline::hook(offset=INT_OFFSET)]
pub unsafe fn get_param_int_replace(module_accessor: u64, param_type: u64, param_hash: u64) -> i32 {
	let mut boma = *((module_accessor as *mut u64).offset(1)) as *mut BattleObjectModuleAccessor;
	let boma_reference = &mut *boma;
	let fighter_kind = boma_reference.kind();
	let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
	if boma_reference.is_fighter() {
		if fighter_kind == *FIGHTER_KIND_EFLAME {
			if param_type == hash40("jump_count_max") {
				if crate::eflame::PYRA_FTHROW_ARMOR[entry_id] {
					return 10;
				}
			}
		}
	}
	else if boma_reference.is_weapon() {
		let owner_module_accessor = &mut *sv_battle_object::module_accessor((WorkModule::get_int(boma, *WEAPON_INSTANCE_WORK_ID_INT_LINK_OWNER)) as u32);
		let entry_id = WorkModule::get_int(owner_module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
		if fighter_kind == *WEAPON_KIND_MARIO_FIREBALL {
			if param_type == hash40("param_fireball") {
				if param_hash == hash40("life") {
					if crate::mario::MARIO_GIANT_FIREBALL[entry_id] {
						return 240;
					}
				}
				if param_hash == hash40("is_penetration") {
					if crate::mario::MARIO_GIANT_FIREBALL[entry_id] {
						return 1;
					}
				}
				if param_hash == hash40("bound_count_max") {
					if crate::mario::MARIO_GIANT_FIREBALL[entry_id] {
						return 20;
					}
				}
			}
		}
	}

	original!()(module_accessor, param_type, param_hash)
}


//Installation
pub fn install() {
	unsafe {
        let text_ptr = getRegionAddress(Region::Text) as *const u8;
        let text_size = (getRegionAddress(Region::Rodata) as usize) - (text_ptr as usize);
        let text = std::slice::from_raw_parts(text_ptr, text_size);
        if let Some(offset) = find_subsequence(text, FLOAT_SEARCH_CODE) {
            FLOAT_OFFSET = offset;
        }
        /*if let Some(offset) = find_subsequence(text, INT_SEARCH_CODE) {
            INT_OFFSET = offset;
        }*/
    }
	skyline::install_hooks!(get_param_float_replace);
	skyline::install_hooks!(get_param_int_replace);
	skyline::install_hooks!(offset_dump);
}

