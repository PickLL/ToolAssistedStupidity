use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash::app::lua_bind::*;

use smashline::*;

use crate::custom::*;

#[acmd_script(agent = "yoshi",script = "game_speciallw",category = ACMD_GAME)]
pub fn yoshi_downb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=4.0, Angle=80, KBG=80, FKB=0, BKB=90, Size=7.0, X=0.0, Y=7.0, Z=10.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.5, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_FIRE, Type=ATTACK_REGION_HIP)
        }
        frame(Frame=8)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=9)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_LW_FLAG_LANDING_ENABLE)
        }
        frame(Frame=13)
        if(is_excute){
            StatusModule::change_status_request_from_script(*FIGHTER_STATUS_KIND_FALL, true);
            rust{
                let speedVec = Vector3f{x: -2.0, y: -2.5, z: 0.0};
                KineticModule::add_speed(module_accessor, &speedVec);
            }
        }
    });
}

#[acmd_script(agent = "yoshi_tamago",script = "game_burst",category = ACMD_GAME)]
pub fn egg_burst(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=1)
        if(is_excute){
            rust{
                const vecRot : Vector3f = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                const vecZero : Vector3f = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                EffectModule::req_on_joint(module_accessor,Hash40::new("sys_bomb_b"), Hash40::new("top"),&vecRot,&vecZero,1.0,
                &vecZero,&vecZero,true,0,0,0);
            }
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=270, KBG=50, FKB=0, BKB=60, Size=10.0, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_YOSHI_EGG_HIT, Type=ATTACK_REGION_OBJECT)
            //as_hash__const(hash40("rbkind_explosion"), 0, false)
            //ControlModule::set_rumble()
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "yoshi_tamago",script = "game_throwed",category = ACMD_GAME)]
pub fn egg_throwed(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=7.0, Angle=270, KBG=50, FKB=0, BKB=60, Size=2.5, X=0.0, Y=-1.0, Z=0.0, X2=0.0, Y2=0.0, Z2=0.0, Hitlag=0.6, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_THRU, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=-3, Trip=0.0, Rehit=0, Reflectable=true, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=false, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_YOSHI_EGG_HIT, Type=ATTACK_REGION_OBJECT)
        }
    });
}

#[acmd_script(agent = "yoshi",script = "game_attackairhi",category = ACMD_GAME)]
pub fn yoshi_upair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        FT_MOTION_RATE(0.9)
        frame(Frame=5)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            ATTACK(ID=0, Part=0, Bone=hash40("tail2"), Damage=12.0, Angle=90, KBG=90, FKB=0, BKB=65, Size=7.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=1, Part=0, Bone=hash40("tail2"), Damage=12.0, Angle=90, KBG=90, FKB=0, BKB=65, Size=7.5, X=6.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
        }
        wait(Frames=2)
        FT_MOTION_RATE(0.9)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=10)
        FT_MOTION_RATE(FSM=0.6)
        frame(Frame=33)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "yoshi",script = "game_specialhi",category = ACMD_GAME)]
pub fn yoshi_upb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.6)
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_APPEAR)
        }
        frame(Frame=18)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_SHOOT)
        }
    });
}

#[acmd_script(agent = "yoshi",script = "game_specialairhi",category = ACMD_GAME)]
pub fn yoshi_airupb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.6)
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_APPEAR)
        }
        frame(Frame=18)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_HI_FLAG_EGG_SHOOT)
        }
    });
}

#[acmd_script(agent = "yoshi",script = "game_attacklw3",category = ACMD_GAME)]
pub fn yoshi_downtilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.4)
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("tail1"), Damage=5.0, Angle=28, KBG=30, FKB=0, BKB=67, Size=3.5, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=1, Part=0, Bone=hash40("tail2"), Damage=4.5, Angle=28, KBG=30, FKB=0, BKB=67, Size=3.5, X=3.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.3, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=2, Part=0, Bone=hash40("tail3"), Damage=4.0, Angle=361, KBG=30, FKB=0, BKB=67, Size=3.0, X=3.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.4, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        frame(Frame=10)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "yoshi",script = "game_jumpaerialfront",category = ACMD_GAME)]
pub fn yoshi_jumpf(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        FT_MOTION_RATE(0.2)
        frame(Frame=7)
        if(is_excute){
            WorkModule::enable_transition_term_group(FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING)
        }
        frame(Frame=45)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_JUMP_AERIAL_ARMOR)
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        }
    });
}

#[acmd_script(agent = "yoshi",script = "game_jumpaerialback",category = ACMD_GAME)]
pub fn yoshi_jumpb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        FT_MOTION_RATE(0.2)
        frame(Frame=7)
        if(is_excute){
            WorkModule::enable_transition_term_group(FIGHTER_STATUS_TRANSITION_GROUP_CHK_AIR_LANDING)
        }
        frame(Frame=45)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_YOSHI_INSTANCE_WORK_ID_FLAG_JUMP_AERIAL_ARMOR)
            sv_module_access::damage(MA_MSC_DAMAGE_DAMAGE_NO_REACTION, DAMAGE_NO_REACTION_MODE_NORMAL, 0)
        }
    });
}

#[acmd_script(agent = "yoshi",script = "game_attackairf",category = ACMD_GAME)]
pub fn yoshi_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=16)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=15.0, Angle=361, KBG=90, FKB=0, BKB=0, Size=4.2, X=1.5, Y=-0.5, Z=-0.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_G, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
            ATTACK(ID=1, Part=0, Bone=hash40("head"), Damage=14.0, Angle=270, KBG=90, FKB=0, BKB=0, Size=4.0, X=7.0, Y=1.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HEAD)
        }
        wait(Frames=5)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=40)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=67)
        if(is_excute){
            sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS_BOTH_SIDES)
        }
        
    });
}

#[acmd_script(agent = "yoshi",script = "game_attackhi3",category = ACMD_GAME)]
pub fn yoshi_uptilt(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=1)
        FT_MOTION_RATE(FSM=0.9)
        frame(Frame=7)
        FT_MOTION_RATE(FSM=1)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("tail2"), Damage=7.0, Angle=100, KBG=45, FKB=0, BKB=72, Size=4.0, X=4.5, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=1, Part=0, Bone=hash40("tail2"), Damage=7.0, Angle=100, KBG=45, FKB=0, BKB=72, Size=4.5, X=1.2, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=2, Part=0, Bone=hash40("tail1"), Damage=7.0, Angle=100, KBG=45, FKB=0, BKB=72, Size=2.5, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
        }
        wait(Frames=8)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(FSM=0.7)
    });
}

#[acmd_script(agent = "yoshi",script = "game_attacklw4",category = ACMD_GAME)]
pub fn yoshi_downsmash(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=4)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_FLAG_START_SMASH_HOLD)
        }
        frame(Frame=7)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=26, KBG=75, FKB=0, BKB=51, Size=3.5, X=0.0, Y=3.0, Z=7.0, X2=0.0, Y2=3.0, Z2=10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=26, KBG=63, FKB=0, BKB=46, Size=2.0, X=0.0, Y=2.0, Z=4.0, X2=0.0, Y2=2.0, Z2=14.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=22)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=15.0, Angle=26, KBG=83, FKB=0, BKB=51, Size=3.5, X=0.0, Y=3.0, Z=-7.0, X2=0.0, Y2=3.0, Z2=-10.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=15.0, Angle=26, KBG=75, FKB=0, BKB=44, Size=2.0, X=0.0, Y=2.0, Z=-3.0, X2=0.0, Y2=2.0, Z2=-13.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_B, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_SLAP, Type=ATTACK_REGION_TAIL)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_LOW, false)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear_all()
        }
    });
}

#[acmd_script(agent = "yoshi",script = "game_specialairlw",category = ACMD_GAME)]
pub fn yoshi_airdownb(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=15)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_LW_FLAG_LANDING_ENABLE)
        }
        frame(Frame=19)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=68, FKB=0, BKB=70, Size=6.4, X=0.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HIP)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
        }
        frame(Frame=20)
        if(is_excute){
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=12.0, Angle=270, KBG=68, FKB=0, BKB=70, Size=6.4, X=0.0, Y=5.5, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=0.7, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=10, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_HIP)
            //AttackModule::set_attack_height_all(ATTACK_HEIGHT_HIGH, false)
        }
    });
}

#[acmd_script(agent = "yoshi",script = "game_attackairn",category = ACMD_GAME)]
pub fn yoshi_nair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        frame(Frame=3)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            rust{
                effect_on_joint(module_accessor,Hash40::new("sys_bomb_b"), Hash40::new("top"),&Vector3f{x:0.0,y:0.0,z:0.0},&Vector3f{x:90.0,y:0.0,z:0.0},1.0);
            }
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=86, FKB=0, BKB=50, Size=10.0, X=0.0, Y=4.3, Z=7.5, X2=0.0, Y2=3.0, Z2=-1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=5)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=7.6, X=0.0, Y=4.5, Z=5.0, X2=0.0, Y2=3.7, Z2=-1.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
            AttackModule::clear(0,false)
        }
        frame(Frame=12)
        if(is_excute){
            ATTACK(ID=1, Part=0, Bone=hash40("top"), Damage=5.0, Angle=361, KBG=100, FKB=0, BKB=30, Size=7.0, X=0.0, Y=4.8, Z=3.5, X2=0.0, Y2=4.0, Z2=-1.5, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        frame(Frame=26)
        if(is_excute){
            AttackModule::clear_all()
        }
        frame(Frame=38)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
    });
}

#[acmd_script(agent = "yoshi",scripts = ["game_specialn2","game_specialairn2"],category = ACMD_GAME)]
pub fn yoshi_neutralb2(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        if(is_excute){
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, ID=0, Damage=7.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=1.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_NONE, Type=ATTACK_REGION_NONE)
            ATTACK_ABS(Kind=FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, ID=0, Damage=4.0, Angle=361, KBG=100, FKB=0, BKB=0, Hitlag=0.0, Unk=1.0, FacingRestrict=ATTACK_LR_CHECK_F, Unk=1.0, Unk=true, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_NONE)
        }
        frame(Frame=7)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_N_FLAG_SWALLOW)
        }
        frame(Frame=21)
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_YOSHI_STATUS_SPECIAL_N_FLAG_SPIT)
            rust{
                const vecRot : Vector3f = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                const vecZero : Vector3f = Vector3f{x: 0.0, y: 0.0, z: 0.0};
                EffectModule::req_on_joint(module_accessor,Hash40::new("sys_bomb_b"), Hash40::new("top"),&vecRot,&vecZero,1.0,
                &vecZero,&vecZero,true,0,0,0);
            }
            ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=86, FKB=0, BKB=50, Size=10.0, X=0.0, Y=4.3, Z=7.5, X2=0.0, Y2=3.0, Z2=-1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
        }
        wait(Frames=2)
        if(is_excute){
            AttackModule::clear(0,false)
        }
    });
}


//EFFECT(hash40("sys_bomb_b"), hash40("top"), 0, 0, 0, 90, 0, 0, 1, 0, 0, 0, 0, 0, 0, true)

/*rust{
    const vecRot : Vector3f = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    const vecZero : Vector3f = Vector3f{x: 0.0, y: 0.0, z: 0.0};
    EffectModule::req_on_joint(module_accessor,Hash40::new("sys_bomb_b"), Hash40::new("top"),&vecRot,&vecZero,1.0,
    &vecZero,&vecZero,true,0,0,0);
}*/

//ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=10.0, Angle=361, KBG=86, FKB=0, BKB=50, Size=10.0, X=0.0, Y=4.3, Z=7.5, X2=0.0, Y2=3.0, Z2=-1.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_normal"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_KICK, Type=ATTACK_REGION_KICK)
//AttackModule::clear(0,false)

pub fn install() {
    install_acmd_scripts!(
        yoshi_downb,
        egg_burst,
        egg_throwed,
        yoshi_upair,
        yoshi_upb,
        yoshi_airupb,
        yoshi_downtilt,
        yoshi_jumpf,
        yoshi_jumpb,
        yoshi_fair,
        yoshi_uptilt,
        yoshi_downsmash,
        yoshi_airdownb,
        yoshi_nair,
        yoshi_neutralb2
    );
}