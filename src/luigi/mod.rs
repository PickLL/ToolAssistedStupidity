use smash::hash40;
use smash::lib::lua_const::*;
use smash::lua2cpp::*;
use smash::phx::*;
use smash::app::lua_bind::*;

use smashline::*;

pub static mut upcharge : [i32; 8] = [0; 8];

pub static mut downcharge : [i32; 8] = [0; 8];

pub static mut backcharge : [i32; 8] = [0; 8];

pub static mut frontcharge : [i32; 8] = [0; 8];

pub static chargemax : i32 = 20;

pub static flicktime : i32 = 10;

#[acmd_script(agent = "luigi",script = "game_specialhi",category = ACMD_GAME)]
pub fn luigi_upb(fighter: &mut L2CAgentBase) {
    unsafe{
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        if downcharge[player_no] < 0{
            acmd!(lua_state,{
                frame(Frame=1)
                frame(Frame=8)
                if(is_excute){
                    WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=55.0, Angle=88, KBG=88, FKB=0, BKB=50, Size=2.2, X=1.2, Y=6.0, Z=7.0, X2=-1.2, Y2=6.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_PUNCH)
                    WorkModule::on_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
                }
                wait(Frames=1)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=1.0, Angle=80, KBG=1, FKB=0, BKB=1, Size=5.8, X=2.0, Y=2.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=1.0, Angle=80, KBG=1, FKB=0, BKB=1, Size=4.7, X=0.0, Y=4.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                    WorkModule::off_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
                }
                wait(Frames=1)
                if(is_excute){
                    WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
                }
                wait(Frames=1)
                if(is_excute){
                    SA_SET(State=SITUATION_KIND_AIR)
                }
                frame(Frame=10)
                if(is_excute){
                    sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                }
                frame(Frame=24)
                if(is_excute){
                    AttackModule::clear_all()
                }
            });
        } else{
            acmd!(lua_state,{
                frame(Frame=8)
                if(is_excute){
                    WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_REVERSE_LR)
                    ATTACK(ID=0, Part=0, Bone=hash40("top"), Damage=25.0, Angle=88, KBG=88, FKB=0, BKB=50, Size=2.2, X=1.2, Y=6.0, Z=7.0, X2=-1.2, Y2=6.0, Z2=7.0, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_fire"), SFXLevel=ATTACK_SOUND_LEVEL_L, SFXType=COLLISION_SOUND_ATTR_BAT, Type=ATTACK_REGION_PUNCH)
                    WorkModule::on_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
                }
                wait(Frames=1)
                if(is_excute){
                    ATTACK(ID=0, Part=0, Bone=hash40("head"), Damage=1.0, Angle=80, KBG=1, FKB=0, BKB=1, Size=5.8, X=2.0, Y=2.2, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                    ATTACK(ID=1, Part=0, Bone=hash40("hip"), Damage=1.0, Angle=80, KBG=1, FKB=0, BKB=1, Size=4.7, X=0.0, Y=4.8, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_coin"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_COIN, Type=ATTACK_REGION_PUNCH)
                    WorkModule::off_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_HI_FLAG_CRITICAL_HIT)
                }
                wait(Frames=1)
                if(is_excute){
                    WorkModule::on_flag(Flag=FIGHTER_STATUS_SUPER_JUMP_PUNCH_FLAG_MOVE_TRANS)
                }
                wait(Frames=1)
                if(is_excute){
                    SA_SET(State=SITUATION_KIND_AIR)
                }
                frame(Frame=10)
                if(is_excute){
                    sv_battle_object::notify_event_msc_cmd(0x2127e37c07, GROUND_CLIFF_CHECK_KIND_ALWAYS)
                }
                frame(Frame=24)
                if(is_excute){
                    AttackModule::clear_all()
                }
            });
        }
        
    }
}

#[acmd_script(agent = "luigi",script = "game_specialsstart",category = ACMD_GAME)]
pub fn luigi_sidebstart(fighter: &mut L2CAgentBase) {
    unsafe{
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        if upcharge[player_no] < 0{
            acmd!(lua_state,{
                rust{
                    println!("{}",upcharge[player_no]);
                }
            });
        }else{
            acmd!(lua_state,{
                frame(Frame=3)
                if(is_excute){
                    WorkModule::on_flag(Flag=FIGHTER_LUIGI_STATUS_SPECIAL_S_FLAG_REVERSE_LR)
                }
            });
        }
    }
}

pub fn install() {
    install_acmd_scripts!(
        luigi_upb,
        luigi_sidebstart
    );
}