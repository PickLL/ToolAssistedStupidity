use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::*;
use smash::lib::lua_const::*;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd;
use smash::hash40;

use smashline::*;

#[acmd_script(agent = "ness",scripts = ["game_specialairnend","game_specialnend"],category = ACMD_GAME)]
pub fn ness_neutralbend(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(frame=1)
        FT_MOTION_RATE(0.2)
    });
}

#[acmd_script(agent = "ness",script = "game_specialairs",category = ACMD_GAME)]
pub fn ness_airsideb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.85)
        frame(Frame=20)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=21)
        if(is_excute){
        ArticleModule::generate_article(FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE,false,0)
        WorkModule::on_flag(Flag=FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT)
        }
        FT_MOTION_RATE(FSM=0.5)
    });
}

#[acmd_script(agent = "ness",scripts = ["game_speciallwstart","game_specialairlwstart"],category = ACMD_GAME)]
pub fn ness_downbstart(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(3)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=90, KBG=100, FKB=40, BKB=20, Size=8.5, X=0.0, Y=6.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=10, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
        }
        wait(Frames=6)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "ness",scripts = ["game_jumpfront","game_jumpback","game_jumpminifront","game_jumpminiback"],category = ACMD_GAME)]
pub fn ness_jump(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_ITEM_SCREW_JUMP, true)
    });
}

#[acmd_script(agent = "ness",script = "game_screw",category = ACMD_GAME)]
pub fn ness_screw(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        rust{
            let speedVec = Vector3f{x: 0.0, y: -1.5, z: 0.0};
            KineticModule::add_speed(module_accessor, &speedVec);
        }
        frame(Frame=1)
        for(10 Iterations){
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=0.5, Angle=84, KBG=100, FKB=60, BKB=0, Size=3.2, X=0.0, Y=2.0, Z=5.0, X2=0.0, Y2=2.0, Z2=5.0, Hitlag=0.5, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUSD_SCREW, Type=ATTACK_REGION_BODY)
                ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=0.5, Angle=90, KBG=100, FKB=60, BKB=0, Size=3.2, X=0.0, Y=2.0, Z=-5.0, X2=0.0, Y2=2.0, Z2=-5.0, Hitlag=0.5, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUSD_SCREW, Type=ATTACK_REGION_BODY)
                ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=0.5, Angle=84, KBG=100, FKB=15, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=5.0, X2=0.0, Y2=9.0, Z2=5.0, Hitlag=0.5, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUSD_SCREW, Type=ATTACK_REGION_BODY)
                ATTACK(ID=3, Part=0, Bone=hash40("top"), Damage=0.5, Angle=90, KBG=100, FKB=15, BKB=0, Size=3.0, X=0.0, Y=9.0, Z=-5.0, X2=0.0, Y2=9.0, Z2=-5.0, Hitlag=0.5, SDI=0.5, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=3, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_rush"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SAMUSD_SCREW, Type=ATTACK_REGION_BODY)            
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
        }
    });
}

#[acmd_script(agent = "ness",script = "game_attackdash",category = ACMD_GAME)]
pub fn ness_dashattack(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=8)
        if(is_excute){
            rust{
                let speedVec = Vector3f{x: 0.5, y: 0.0, z: 0.0};
                KineticModule::add_speed(module_accessor, &speedVec);
            }
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=0, KBG=100, FKB=57, BKB=0, Size=5.0, X=0.0, Y=5.5, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=80, KBG=70, FKB=0, BKB=60, Size=5.0, X=0.0, Y=5.5, Z=9.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_A, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
            ATTACK(ID=2, Part=0, Bone=hash40("top"), Damage=3.0, Angle=45, KBG=100, FKB=0, BKB=80, Size=3.0, X=0.0, Y=4.3, Z=6.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
        }
        wait(Frames=1)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=2.0, Angle=20, KBG=100, FKB=70, BKB=0, Size=5.0, X=0.0, Y=4.8, Z=16.0, X2=0.0, Y2=4.8, Z2=13.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        wait(Frames=6)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=6.0, Angle=70, KBG=113, FKB=0, BKB=85, Size=6.0, X=0.0, Y=4.8, Z=21.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=1, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_magic"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_MAGIC, Type=ATTACK_REGION_PSI)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "ness_pkflash",script = "game_bang",category = ACMD_GAME)]
pub fn pkflash_bang(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=1.0, Angle=70, KBG=65, FKB=0, BKB=50, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_OFF, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=true, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_paralyze"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_PSI)
        }
        frame(Frame=5)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "ness",script = "game_specials",category = ACMD_GAME)]
pub fn ness_pkfire(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=1)
        FT_MOTION_RATE(FSM=2)
        frame(Frame=20)
        FT_MOTION_RATE(FSM=1)
        frame(Frame=21)
        if(is_excute){
            ArticleModule::generate_article(FIGHTER_NESS_GENERATE_ARTICLE_PK_FIRE,false,0)
            WorkModule::on_flag(Flag=FIGHTER_NESS_STATUS_SPECIAL_S_FLAG_SHOOT)
        }
        FT_MOTION_RATE(FSM=0.1)
    });
}

pub fn install() {
    install_acmd_scripts!(
        ness_neutralbend,
        ness_airsideb,
        ness_downbstart,
        ness_jump,
        ness_screw,
        ness_dashattack,
        pkflash_bang,
        ness_pkfire
    );
}