#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
//#![feature()]
//#![feature(asm)]


pub mod custom;
pub mod customparam;
mod mario;
mod mariod;
mod donkey;
mod link;
mod samus;
mod kirby;
mod fox;
mod pichu;
mod pikachu;
mod luigi;
mod ness;
mod captain;
mod purin;
mod yoshi;
mod samusd;
mod wario;
mod ike;
mod pit;
mod pitb;
mod peach;
mod daisy;
mod master;
mod koopa;
mod rosetta;
mod wiifit;
mod marth;
mod roy;
mod buddy;
mod mewtwo;
mod falco;
mod miifighter;
mod miiswordsman;
mod miigunner;
mod gaogaen;
mod dedede;
mod ganon;
mod inkling;
mod brave;
mod lucario;
mod shulk;
mod reflet;
mod simon;
mod richter;
mod younglink;
mod toonlink;
mod wolf;
mod zelda;
mod sheik;
mod lucina;
mod littlemac;
mod trail;
mod cloud;
mod edge;
mod lucas;
mod snake;
mod dolly;
mod ryu;
mod ken;
mod krool;
mod gekkouga;
mod metaknight;
mod rockman;
mod kamui;
mod pikmin;
mod chrom;
mod sonic;
mod pacman;
mod packun;
mod szerosuit;
mod shizue;
mod bayonetta;
mod diddy;
mod palutena;
mod duckhunt;
mod eflame;
mod elight;
mod koopajr;
mod tantan;
mod robot;
mod jack;
mod demon;
mod murabito;
mod plizardon;
mod pfushigisou;
mod pzenigame;
mod gamewatch;
mod pickel;
mod popo;
mod ridley;
mod miienemyf;
mod miienemys;
mod miienemyg;
//mod koopag;

std::arch::global_asm!(
    r#"
    .section .nro_header
    .global __nro_header_start
    .word 0
    .word _mod_header
    .word 0
    .word 0
    
    .section .rodata.module_name
        .word 0
        .word 14
        .ascii "tr4sh_rebuffed"
    .section .rodata.mod0
    .global _mod_header
    _mod_header:
        .ascii "MOD0"
        .word __dynamic_start - _mod_header
        .word __bss_start - _mod_header
        .word __bss_end - _mod_header
        .word __eh_frame_hdr_start - _mod_header
        .word __eh_frame_hdr_end - _mod_header
        .word __nx_module_runtime - _mod_header // runtime-generated module object offset
    .global IS_NRO
    IS_NRO:
        .word 1
    
    .section .bss.module_runtime
    __nx_module_runtime:
    .space 0xD0
    "#
);

#[no_mangle]
pub extern "C" fn main() {
    
    custom::install();
    mario::install();
    mariod::install();
    donkey::install();
    link::install();
    samus::install();
    samusd::install();
    kirby::install();
    fox::install();
    pichu::install();
    pikachu::install();
    luigi::install();
    ness::install();
    captain::install();
    purin::install();
    yoshi::install();
    wario::install();
    ike::install();
    pit::install();
    pitb::install();
    peach::install();
    daisy::install();
    master::install();
    koopa::install();
    rosetta::install();
    wiifit::install();
    marth::install();
    roy::install();
    buddy::install();
    mewtwo::install();
    falco::install();
    miifighter::install();
    miiswordsman::install();
    miigunner::install();
    gaogaen::install();
    dedede::install();
    ganon::install();
    inkling::install();
    brave::install();
    lucario::install();
    shulk::install();
    reflet::install();
    simon::install();
    richter::install();
    younglink::install();
    toonlink::install();
    wolf::install();
    zelda::install();
    sheik::install();
    lucina::install();
    littlemac::install();
    trail::install();
    cloud::install();
    edge::install();
    lucas::install();
    snake::install();
    dolly::install();
    ryu::install();
    ken::install();
    krool::install();
    gekkouga::install();
    metaknight::install();
    rockman::install();
    kamui::install();
    pikmin::install();
    chrom::install();
    sonic::install();
    pacman::install();
    packun::install();
    szerosuit::install();
    shizue::install();
    bayonetta::install();
    diddy::install();
    palutena::install();
    duckhunt::install();
    eflame::install();
    elight::install();
    koopajr::install();
    tantan::install();
    robot::install();
    jack::install();
    demon::install();
    murabito::install();
    plizardon::install();
    pfushigisou::install();
    pzenigame::install();
    gamewatch::install();
    pickel::install();
    popo::install();
    ridley::install();
    miienemyf::install();
    miienemys::install();
    miienemyg::install();
    customparam::install();
    //koopag::install();


}