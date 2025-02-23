#![allow(unused_macros)]

use smash::phx::Hash40;
use smash::lib::lua_const::*;
use smash::app::*;
use smash::app::lua_bind::*;
use smash::lua2cpp::{L2CFighterCommon, L2CAgentBase};
use smashline::*;
use smash_script::*;
use crate::custom::global_fighter_frame;
use smashline::Priority::*;



//Pokemon Trainer should switch faster


unsafe extern "C" fn ptrainer_switch_smash_script(fighter: &mut L2CAgentBase) {
    if macros::is_excute(fighter) {
        ArticleModule::generate_article(fighter.module_accessor, *WEAPON_PTRAINER_PTRAINER_GENERATE_ARTICLE_MBALL, false, -1);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_PTRAINER_PTRAINER_GENERATE_ARTICLE_MBALL, false, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("weapon").hash as i64, Hash40::new("m_ball_lr").hash as i64);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 4.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("weapon").hash as i64, Hash40::new("m_ball_l").hash as i64);
        ArticleModule::set_visibility_whole(fighter.module_accessor, *WEAPON_PTRAINER_PTRAINER_GENERATE_ARTICLE_MBALL, true, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL));
        ArticleModule::shoot(fighter.module_accessor, *WEAPON_PTRAINER_PTRAINER_GENERATE_ARTICLE_MBALL, ArticleOperationTarget(*ARTICLE_OPE_TARGET_ALL), false);
    }
    sv_animcmd::frame(fighter.lua_state_agent, 20.0);
    macros::FT_MOTION_RATE(fighter, 0.5);
    sv_animcmd::frame(fighter.lua_state_agent, 103.0);
    if macros::is_excute(fighter) {
        VisibilityModule::set_int64(fighter.module_accessor, Hash40::new("weapon").hash as i64, Hash40::new("m_ball_r").hash as i64);
    }
    
}


pub fn install() {
    Agent::new("ptrainer_ptrainer")
    .game_acmd("game_pchange", ptrainer_switch_smash_script, Low)
    .install();

}