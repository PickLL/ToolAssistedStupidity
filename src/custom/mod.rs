use smash::app::lua_bind::*;
use smash::app::utility::get_kind;
use smash::lua2cpp::{L2CFighterCommon, L2CFighterBase};
use smash::lib::lua_const::*;
use smash::phx::*;
use smash::app::lua_bind::*;
use smash::app::sv_animcmd;
use smash::hash40;

use crate::kirby::upb;

use crate::luigi::*;

use crate::pikachu::*;

use crate::ness::*;

use smashline::*;

pub static mut status_de : [u32; 8] = [0; 8];

pub static mut seen_de : [[bool ; 8]; 8] = [[false ; 8]; 8];

pub static mut llc : [bool; 8] = [false; 8];

fn is_button_at_all(module_accessor : &mut smash::cpp::root::app::BattleObjectModuleAccessor) -> bool{
    unsafe{
        let cat = ControlModule::get_command_flag_cat(module_accessor, 0);
        if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_ATTACK) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SPECIAL) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_CATCH) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_SMASH) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_LW) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_HI) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_L) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_APPEAL_S_R) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD) || (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 && ControlModule::is_enable_flick_jump(module_accessor)){
            true
        }
        else{
            false
        }
    }
}

#[smashline::fighter_frame(global)]
pub fn global_fighter_frame(fighter : &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let fighter_kind = smash::app::utility::get_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        let cat = ControlModule::get_command_flag_cat(module_accessor, 0);

        //Universals ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------

        // (MoveState, Character, Frame, Air/Ground/Both)
        // Cancels on hit                0   1      2

        let cancel = [("special_lw",*FIGHTER_KIND_KIRBY,0.0,2),("special_air_lw",*FIGHTER_KIND_KIRBY,0.0,2),("special_air_s_end",*FIGHTER_KIND_CAPTAIN,0.0,2)];
        for i in &cancel{
            if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){
                if MotionModule::motion_kind(module_accessor) == smash::hash40(i.0) && fighter_kind == i.1 && MotionModule::frame(module_accessor) >= i.2{
                    if (situation_kind == SITUATION_KIND_GROUND && (i.3 == 1 || i.3 == 2) || situation_kind == SITUATION_KIND_AIR && (i.3 == 0 || i.3 == 2)){
                        CancelModule::enable_cancel(module_accessor);
                    }
                }
            }
        }

        // (MoveState, Character, Frame)
        // Cancels if you press any button other than shield

        let cancel_any = [("special_air_n",*FIGHTER_KIND_MARIO,13.0),("special_s",*FIGHTER_KIND_FOX,0.0),("special_air_s",*FIGHTER_KIND_FOX,0.0),("special_s_end",*FIGHTER_KIND_FOX,0.0),
        ("special_air_s_end",*FIGHTER_KIND_FOX,0.0)];
        for i in &cancel_any{
            if MotionModule::motion_kind(module_accessor) == smash::hash40(i.0) && fighter_kind == i.1 && MotionModule::frame(module_accessor) >= i.2{
                if is_button_at_all(module_accessor) && !ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_GUARD){
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }

        //(MoveState, Character, Frame)
        //Cancels if you press jump

        let cancel_jump = [("attack_dash",*FIGHTER_KIND_MARIO,0.0),("special_air_s",*FIGHTER_KIND_DONKEY,0.0),("special_s1",*FIGHTER_KIND_LINK,28.0),("special_air_s1",*FIGHTER_KIND_LINK,28.0),
        ("special_air_lw",*FIGHTER_KIND_CAPTAIN,0.0),("special_lw",*FIGHTER_KIND_CAPTAIN,0.0),("special_s_loop",*FIGHTER_KIND_YOSHI,0.0),("special_air_s_loop",*FIGHTER_KIND_YOSHI,0.0),
        ("special_air_s_start",*FIGHTER_KIND_YOSHI,5.0),("special_air_s",*FIGHTER_KIND_PURIN,0.0),("special_s",*FIGHTER_KIND_PURIN,0.0)];
        for i in &cancel_jump{
            if MotionModule::motion_kind(module_accessor) == smash::hash40(i.0) && fighter_kind == i.1 && MotionModule::frame(module_accessor) >= i.2{
                if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) != WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                    let cat = ControlModule::get_command_flag_cat(module_accessor, 0);
                    if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI) || (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 && ControlModule::is_enable_flick_jump(module_accessor)){
                        if situation_kind == SITUATION_KIND_GROUND{
                            StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                        } else{
                            StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_JUMP, true);
                        }
                    }
                }
            }
        }

        //Cancel on hit on ground after hitstun
        //Blacklist
        let cancel_black = [("special_hi_hold",*FIGHTER_KIND_FOX,f32::INFINITY),("special_hi",*FIGHTER_KIND_DONKEY,62.0),("attack_100",999,f32::INFINITY),("attack_100_sub",999,f32::INFINITY),
        ("attack_hi3",*FIGHTER_KIND_GANON,60.0),("special_hi",*FIGHTER_KIND_MARIO,f32::INFINITY),("attack_hi4",*FIGHTER_KIND_LINK,41.0),("attack_hi4",*FIGHTER_KIND_SAMUS,27.0),("attack_hi4",*FIGHTER_KIND_SAMUSD,27.0),
        ("attack_hi3",*FIGHTER_KIND_SAMUS,17.0),("attack_hi3",*FIGHTER_KIND_SAMUSD,17.0),("attack_hi4",*FIGHTER_KIND_CAPTAIN,f32::INFINITY),("attack_hi3",*FIGHTER_KIND_CAPTAIN,23.0),
        ("special_lw_l",*FIGHTER_KIND_PURIN,f32::INFINITY),("special_lw_r",*FIGHTER_KIND_PURIN,f32::INFINITY),("special_air_lw_l",*FIGHTER_KIND_PURIN,f32::INFINITY),("special_air_lw_r",*FIGHTER_KIND_PURIN,f32::INFINITY),
        ("attack_lw4",*FIGHTER_KIND_PIKACHU,22.0),("attack_dash",*FIGHTER_KIND_NESS,25.0),("special_s_start",*FIGHTER_KIND_CAPTAIN,f32::INFINITY)];
        let check = (MotionModule::motion_kind(module_accessor),fighter_kind);
        let mut found = false;
        for i in &cancel_black{
            if MotionModule::motion_kind(module_accessor) == smash::hash40(i.0) && (fighter_kind == i.1 || i.1 == 999) && MotionModule::frame(module_accessor) <= i.2{
                found = true;
                break;
            }
        }  
        if found == false{
            if MotionModule::frame(module_accessor) >= 10.0{
                if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && !CatchModule::is_catch(module_accessor) && situation_kind == SITUATION_KIND_GROUND && status_kind != *FIGHTER_STATUS_KIND_FINAL{
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
        //short hop when pressing down
        if ControlModule::get_stick_y(module_accessor) < -0.5 && situation_kind == SITUATION_KIND_GROUND{
            WorkModule::set_flag(module_accessor,true,*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_JUMP_MINI);
            // let cat = ControlModule::get_command_flag_cat(module_accessor, 0);
            // if (cat & *FIGHTER_PAD_CMD_CAT1_FLAG_JUMP != 0 && ControlModule::is_enable_flick_jump(module_accessor)) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) || ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP_MINI){
            //     StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_JUMP, true);
            // }
        }

        //extra hit stop to attackers in air or landing when hitting shield
        if AttackModule::is_infliction(module_accessor, *COLLISION_KIND_MASK_SHIELD) && (situation_kind == SITUATION_KIND_AIR || status_kind == FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR){
           StopModule::set_hit_stop_frame(module_accessor,4+StopModule::get_hit_stop_real_frame(module_accessor) as i32,false);
        }

        //respawn/spawn
        if status_de[player_no] == 0{
            if status_kind == *FIGHTER_STATUS_KIND_ENTRY{
                status_de[player_no] = 1;
            }else if status_kind == *FIGHTER_STATUS_KIND_DEAD{
                status_de[player_no] = 2;
            }
        } else if !([*FIGHTER_STATUS_KIND_ENTRY,*FIGHTER_STATUS_KIND_DEAD].contains(&status_kind)){
            status_de[player_no] = 0;
            for x in 0..8{
                seen_de[x][player_no] = false;
            }
        }
        //grab ranges
        if fighter_kind == *FIGHTER_KIND_MARTH{
            GrabModule::set_size_mul(module_accessor,100.0);
        } else{
            GrabModule::set_size_mul(module_accessor,2.5);
        }

        //airdodge from jumpsquat
        if status_kind == FIGHTER_STATUS_KIND_JUMP_SQUAT && ControlModule::check_button_trigger(module_accessor,*CONTROL_PAD_BUTTON_GUARD){
            WorkModule::set_float(module_accessor,ControlModule::get_stick_y(module_accessor),*FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_Y);
            WorkModule::set_float(module_accessor,ControlModule::get_stick_x(module_accessor),*FIGHTER_STATUS_ESCAPE_AIR_SLIDE_WORK_FLOAT_DIR_X);
            
            let speedVec = Vector3f{x: 0.0, y: 0.01, z: 0.0};
            KineticModule::add_speed(module_accessor, &speedVec);
            StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_ESCAPE_AIR,false);
        }

        //ll cancel
        if situation_kind == SITUATION_KIND_AIR && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){
            llc[player_no] = true;
        }else if [*FIGHTER_STATUS_KIND_LANDING,*FIGHTER_STATUS_KIND_LANDING_ATTACK_AIR].contains(&status_kind) && llc[player_no]{
            CancelModule::enable_cancel(module_accessor);
            llc[player_no] = false;
        }else if situation_kind == SITUATION_KIND_GROUND && llc[player_no]{
            llc[player_no] = false;
        }

        WorkModule::set_flag(module_accessor,false,*FIGHTER_STATUS_WORK_ID_FLAG_RESERVE_ATTACK_DISABLE_MINI_JUMP_ATTACK);
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_DONKEY)]
fn donkey_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if DamageModule::damage(module_accessor,0) == 999.0{
            AttackModule::set_power_mul(module_accessor,2.0)
            } else{
                AttackModule::set_power_mul(module_accessor,1.0)
            }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_FOX)]
fn fox_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_LOOP, *FIGHTER_FOX_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
            if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) != WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    if situation_kind == SITUATION_KIND_GROUND{
                        StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    } else{
                        StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_JUMP, true);
                    }
                }
            }
        }

        if [*FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH,*FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_BOUND,*FIGHTER_FOX_STATUS_KIND_SPECIAL_HI_RUSH_END].contains(&status_kind) && AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT){
            if situation_kind == SITUATION_KIND_GROUND{
                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_WAIT, true);
            } else{
                StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_FALL, true);
            }
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_FALCO)]
fn falco_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if [hash40("special_lw"),hash40("special_lw_r"),hash40("special_air_lw"),hash40("special_air_lw_r")].contains(&MotionModule::motion_kind(module_accessor)){
            if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) != WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    CancelModule::enable_cancel(module_accessor);
                }
            }
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_SAMUS)]
fn samus_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let cshot = WorkModule::get_int(module_accessor,*FIGHTER_SAMUS_INSTANCE_WORK_ID_INT_SPECIAL_N_COUNT);
        AttackModule::set_power_mul(module_accessor,1.0+(cshot as f32/1350.0));
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_MARIO)]
fn mario_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        if status_kind == *FIGHTER_MARIO_STATUS_KIND_SPECIAL_LW_SHOOT{
            if is_button_at_all(module_accessor){
                CancelModule::enable_cancel(module_accessor);
            }
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_KIRBY)]
fn kirb_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        if situation_kind == SITUATION_KIND_GROUND{
        upb[player_no] = false;
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_NESS)]
fn ness_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        let situation_kind = StatusModule::situation_kind(module_accessor);

        if [*FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_END, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HOLD, *FIGHTER_NESS_STATUS_KIND_SPECIAL_LW_HIT, *FIGHTER_STATUS_KIND_SPECIAL_LW].contains(&status_kind) {
            if WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) != WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX){
                if ControlModule::check_button_trigger(module_accessor, *CONTROL_PAD_BUTTON_JUMP) {
                    if situation_kind == SITUATION_KIND_GROUND{
                        StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    } else{
                        StatusModule::change_status_request_from_script(module_accessor,*FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    }
                }
            }
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_CAPTAIN)]
fn falcon_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        if AttackModule::is_infliction_status(module_accessor, *COLLISION_KIND_MASK_HIT) && WorkModule::get_int(module_accessor,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) != 0{
            WorkModule::set_int(module_accessor,1,*FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT);
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_DUCKHUNT)]
fn duckhunt_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let situation_kind = StatusModule::situation_kind(module_accessor);
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);
        let frame = smash::app::lua_bind::MotionModule::frame(module_accessor) as i32;
        if frame == 1{
            DamageModule::add_damage(module_accessor,1.0,0);
        }
        AttackModule::set_power_mul(module_accessor,0.333);

        if status_kind == *FIGHTER_STATUS_KIND_DEAD{
            GroundModule::set_collidable(module_accessor,true);
        }

        if smash::app::sv_math::rand(hash40("fighter"), 500) == 0 {
            if situation_kind == SITUATION_KIND_GROUND{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_SLEEP_START, true); 
            }
        }
        if smash::app::sv_math::rand(hash40("fighter"), 500) == 0 {
            if situation_kind == SITUATION_KIND_GROUND{
                GroundModule::set_collidable(module_accessor,false);
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL, true);
            }
        }
        if MotionModule::end_frame(module_accessor) as i32 == frame + 15{
            if situation_kind == SITUATION_KIND_AIR{
                StatusModule::change_status_request_from_script(module_accessor, *FIGHTER_STATUS_KIND_FALL_SPECIAL, true); 
            }
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_PIKACHU)]
fn pikachu_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        let status_kind = smash::app::lua_bind::StatusModule::status_kind(module_accessor);

        //Static Charge
        //println!("{}",staticCharge[player_no]);
        for x in 0..8{
            if status_de[x] == 2 && seen_de[player_no][x] == false{
                staticCharge[player_no] += 1;
                seen_de[player_no][x] = true;
            } else if status_de[x] == 1 && seen_de[player_no][x] == false && x == player_no{
                staticCharge[player_no] = staticDef;
                seen_de[player_no][x] = true;
            }
        }

        if smash::app::smashball::is_training_mode(){
             staticCharge[player_no] = 999;
        }
    }
}

#[smashline::fighter_frame(agent = FIGHTER_KIND_LUIGI)]
fn luigi_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let player_no : usize = CameraModule::get_player_no(module_accessor,0) as usize;
        let sy = ControlModule::get_stick_y(module_accessor);
        let sx = ControlModule::get_stick_x(module_accessor)*PostureModule::lr(module_accessor);

        if upcharge[player_no] >= 0 && upcharge[player_no] < chargemax && sy >= 0.5{
            upcharge[player_no] += 1;
        } else if upcharge[player_no] == chargemax && !(sy >= 0.5){
            upcharge[player_no] = flicktime * -1;
        } else if upcharge[player_no] < 0 && !(sy >= 0.5){
            upcharge[player_no] +=1;
        } else if ( upcharge[player_no] > 0 && !(sy >= 0.5) ) || upcharge[player_no] <0 && sy >= 0.5{
            upcharge[player_no] = 0;
        }

        if downcharge[player_no] >= 0 && downcharge[player_no] < chargemax && sy <= -0.5{
            downcharge[player_no] += 1;
        } else if downcharge[player_no] == chargemax && !(sy <= -0.5){
            downcharge[player_no] = flicktime * -1;
        } else if downcharge[player_no] < 0 && !(sy <= -0.5){
            downcharge[player_no] +=1;
        } else if ( downcharge[player_no] > 0 && !(sy <= -0.5) ) || downcharge[player_no] <0 && sy >= 0.5{
            downcharge[player_no] = 0;
        }

        if frontcharge[player_no] >= 0 && frontcharge[player_no] < chargemax && sx >= 0.5{
            frontcharge[player_no] += 1;
        } else if frontcharge[player_no] == chargemax && !(sx >= 0.5){
            frontcharge[player_no] = flicktime * -1;
        } else if frontcharge[player_no] < 0 && !(sx >= 0.5){
            frontcharge[player_no] +=1;
        } else if ( frontcharge[player_no] > 0 && !(sx >= 0.5) ) || frontcharge[player_no] <0 && sx >= 0.5{
            frontcharge[player_no] = 0;
        }

        if backcharge[player_no] >= 0 && backcharge[player_no] < chargemax && sx <= -0.5{
            backcharge[player_no] += 1;
        } else if backcharge[player_no] == chargemax && !(sx <= -0.5){
            backcharge[player_no] = flicktime * -1;
        } else if backcharge[player_no] < 0 && !(sx <= -0.5){
            backcharge[player_no] +=1;
        } else if ( backcharge[player_no] > 0 && !(sx <= -0.5) ) || backcharge[player_no] <0 && sx <= -0.5{
            backcharge[player_no] = 0;
        }
    }
}

pub fn install() {
    install_agent_frames!(
        global_fighter_frame,
        donkey_frame,
        fox_frame,
        falco_frame,
        samus_frame,
        mario_frame,
        kirb_frame,
        falcon_frame,
        duckhunt_frame,
        pikachu_frame,
        luigi_frame,
        ness_frame
    );
}