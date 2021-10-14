use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::*;
use smash::lib::lua_const::*;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd;
use smash::hash40;

use smashline::*;

pub static mut lucina_timer : [i32; 8] = [0; 8];

pub static mut lucina_mul : [f32; 8] = [1.0; 8];

#[acmd_script(agent = "lucina",script = "game_catch",category = ACMD_GAME)]
pub fn lucina_grab(fighter: &mut L2CAgentBase) {
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let lua_state = fighter.lua_state_agent;
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        acmd!(lua_state,{
            FT_MOTION_RATE(5.0)
            frame(Frame=5)
            if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
            }
            frame(Frame=6)
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=8.0, Z=4.0, X2=0.0, Y2=8.0, Z2=7.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=8.0, Z=2.35, X2=0.0, Y2=8.0, Z2=9.35, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
            }
            wait(Frames=2)
            if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
                FT_MOTION_RATE(1.0)
            }
        });
    }
}

#[acmd_script(agent = "lucina",script = "game_catchdash",category = ACMD_GAME)]
pub fn lucina_dashgrab(fighter: &mut L2CAgentBase) {
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let lua_state = fighter.lua_state_agent;
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        acmd!(lua_state,{
            FT_MOTION_RATE(5.0)
            frame(Frame=8)
            if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
            }
            frame(Frame=9)
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=3.0, X=0.0, Y=7.25, Z=4.0, X2=0.0, Y2=7.25, Z2=9.4, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.3, X=0.0, Y=7.0, Z=2.7, X2=0.0, Y2=7.0, Z2=10.7, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
            }
            wait(Frames=2)
            if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
                FT_MOTION_RATE(1.0)
            }
        });
    }
}

#[acmd_script(agent = "lucina",script = "game_catchturn",category = ACMD_GAME)]
pub fn lucina_pivotgrab(fighter: &mut L2CAgentBase) {
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let lua_state = fighter.lua_state_agent;
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        acmd!(lua_state,{
            FT_MOTION_RATE(5.0)
            frame(Frame=9)
            if(is_excute){
                GrabModule::set_rebound(CanCatchRebound=true)
            }
            frame(Frame=10)
            if(is_excute){
                CATCH(ID=0, Bone=hash40("top"), Size=3.3, X=0.0, Y=7.0, Z=-4.0, X2=0.0, Y2=7.0, Z2=-13.2, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_G)
                CATCH(ID=1, Bone=hash40("top"), Size=1.65, X=0.0, Y=7.0, Z=-2.35, X2=0.0, Y2=7.0, Z2=-14.85, Status=FIGHTER_STATUS_KIND_CAPTURE_PULLED, Ground_or_Air=COLLISION_SITUATION_MASK_A)
            }
            wait(Frames=2)
            if(is_excute){
                sv_module_access::grab(MA_MSC_CMD_GRAB_CLEAR_ALL)
                WorkModule::on_flag(Flag=FIGHTER_STATUS_CATCH_FLAG_CATCH_WAIT)
                GrabModule::set_rebound(CanCatchRebound=false)
                FT_MOTION_RATE(1.0)
            }
        });
    }
}

#[acmd_script(agent = "lucina",script = "game_attackairf",category = ACMD_GAME)]
pub fn lucina_fair(fighter: &mut L2CAgentBase) {
    let lua_state = fighter.lua_state_agent;
    acmd!(lua_state,{
        if(is_excute){
            WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }
        frame(Frame=6)
        if(is_excute){
            rust{
                let speedVec = Vector3f{x: 0.0, y: 2.0, z: 0.0};
                KineticModule::add_speed(module_accessor, &speedVec);
            }
            ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=10.5, Angle=361, KBG=77, FKB=0, BKB=40, Size=3.0, X=1.0, Y=0.0, Z=3.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=10.5, Angle=361, KBG=77, FKB=0, BKB=40, Size=3.8, X=1.0, Y=0.0, Z=0.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=10.5, Angle=361, KBG=77, FKB=0, BKB=40, Size=3.0, X=1.0, Y=0.0, Z=7.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_F, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
        }
        wait(Frames=3)
        if(is_excute){
            AttackModule::clear_all()
        }
        FT_MOTION_RATE(0.6)
        frame(Frame=36)
        if(is_excute){
            WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
        }            
    });
}


#[acmd_script(agent = "lucina",script = "game_attackairn",category = ACMD_GAME)]
pub fn lucina_nair(fighter: &mut L2CAgentBase) {
    unsafe{
        let lua_state = fighter.lua_state_agent;
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        acmd!(lua_state,{
            frame(Frame=2)
            if(is_excute){
                WorkModule::on_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
            frame(Frame=6)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=4.275, Angle=75, KBG=50, FKB=0, BKB=45, Size=3.8, X=1.0, Y=-1.3, Z=1.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=4.275, Angle=80, KBG=50, FKB=0, BKB=45, Size=4.0, X=-1.5, Y=1.0, Z=-1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=4.275, Angle=90, KBG=50, FKB=0, BKB=35, Size=3.3, X=1.0, Y=-1.3, Z=7.2, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=true, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_S, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=2)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=14)
            if(is_excute){
                rust{
                    if ControlModule::check_button_on(module_accessor,*CONTROL_PAD_BUTTON_ATTACK){
                        MotionModule::change_motion(module_accessor,Hash40::new("attack_air_n"),5.0,1.0,false,0.0,false,false);
                    }
                }
            }
            frame(Frame=15)
            if(is_excute){
                ATTACK(ID=0, Part=0, Bone=hash40("sword1"), Damage=8.55, Angle=361, KBG=95, FKB=0, BKB=55, Size=3.8, X=1.2, Y=-1.1, Z=1.5, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=1, Part=0, Bone=hash40("armr"), Damage=8.55, Angle=361, KBG=95, FKB=0, BKB=55, Size=4.0, X=-2.0, Y=1.0, Z=-1.6, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
                ATTACK(ID=2, Part=0, Bone=hash40("sword1"), Damage=8.55, Angle=361, KBG=95, FKB=0, BKB=55, Size=3.4, X=0.8, Y=-1.1, Z=7.0, X2=LUA_VOID, Y2=LUA_VOID, Z2=LUA_VOID, Hitlag=1.0, SDI=1.0, Clang_Rebound=ATTACK_SETOFF_KIND_ON, FacingRestrict=ATTACK_LR_CHECK_POS, SetWeight=false, ShieldDamage=0, Trip=0.0, Rehit=0, Reflectable=false, Absorbable=false, Flinchless=false, DisableHitlag=false, Direct_Hitbox=true, Ground_or_Air=COLLISION_SITUATION_MASK_GA, Hitbits=COLLISION_CATEGORY_MASK_ALL, CollisionPart=COLLISION_PART_MASK_ALL, FriendlyFire=false, Effect=hash40("collision_attr_cutup"), SFXLevel=ATTACK_SOUND_LEVEL_M, SFXType=COLLISION_SOUND_ATTR_CUTUP, Type=ATTACK_REGION_SWORD)
            }
            wait(Frames=7)
            if(is_excute){
                AttackModule::clear_all()
            }
            frame(Frame=47)
            if(is_excute){
                WorkModule::off_flag(Flag=FIGHTER_STATUS_ATTACK_AIR_FLAG_ENABLE_LANDING)
            }
        });
    }
}

//Why can't original be used for multi hooking

#[acmd_script(agent = "lucina",script = "game_specialnendmax",category = ACMD_GAME)]
pub fn lucina_neutralbendmax(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 2.0, y: 0.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialnendmaxhi",category = ACMD_GAME)]
pub fn lucina_neutralbendmaxhi(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 2.0, y: 1.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialnendmaxlw",category = ACMD_GAME)]
pub fn lucina_neutralbendmaxlw(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 2.0, y: -1.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialairnendmax",category = ACMD_GAME)]
pub fn lucina_airneutralbendmax(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 1.0, y: 0.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialairnendmaxhi",category = ACMD_GAME)]
pub fn lucina_airneutralbendmaxhi(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 1.0, y: 1.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialairnendmaxlw",category = ACMD_GAME)]
pub fn lucina_airneutralbendmaxlw(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 1.0, y: -1.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

//that doesn't seem that hard to add

#[acmd_script(agent = "lucina",script = "game_specialnend",category = ACMD_GAME)]
pub fn lucina_neutralbend(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 2.0, y: 0.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialnendhi",category = ACMD_GAME)]
pub fn lucina_neutralbendhi(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 2.0, y: 1.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialnendlw",category = ACMD_GAME)]
pub fn lucina_neutralbendlw(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 2.0, y: -1.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialairnend",category = ACMD_GAME)]
pub fn lucina_airneutralbend(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 1.0, y: 0.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialairnendhi",category = ACMD_GAME)]
pub fn lucina_airneutralbendhi(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 1.0, y: 1.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

#[acmd_script(agent = "lucina",script = "game_specialairnendlw",category = ACMD_GAME)]
pub fn lucina_airneutralbendlw(fighter: &mut L2CAgentBase){
    unsafe{
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let speedVec = Vector3f{x: 1.0, y: -1.0, z: 0.0};
        KineticModule::add_speed(module_accessor, &speedVec);
        original!(fighter);
    }
}

pub fn install() {
    install_acmd_scripts!(
        lucina_grab,
        lucina_pivotgrab,
        lucina_dashgrab,
        lucina_fair,
        lucina_nair,

        lucina_neutralbendmax,
        lucina_neutralbendmaxhi,
        lucina_neutralbendmaxlw,
        lucina_airneutralbendmax,
        lucina_airneutralbendmaxhi,
        lucina_airneutralbendmaxlw,

        lucina_neutralbend,
        lucina_neutralbendhi,
        lucina_neutralbendlw,
        lucina_airneutralbend,
        lucina_airneutralbendhi,
        lucina_airneutralbendlw,
    );
}