use bevy::prelude::*;
use crate::settings::settings::*;

#[derive(Resource)]
pub struct Enemies {
    pub spawned_this_wave: u32,
    pub alive: u32,
    pub cooldown: f32, //zeit seit letzte wave vorbei ist
    pub is_active: bool, //ob welle gerade aktiv ist
}

impl Default for Enemies {
    fn default() -> Self {
        Self {
            spawned_this_wave: 0,
            alive: 0,
            cooldown: 0.0,
            is_active: false,
        }
    }
}

//setzt wave aktivität und spawnt entsprechend gegner
pub fn set_waves(
    mut enemy_res: ResMut<Enemies>,
    s: Res<GameSettings>,
    time: Res<Time>,
) {
    let setting = &s.gameplay.enemy_waves;

    //bei aktiver welle
    if enemy_res.is_active {
        //neustart der welle wenn alle nötigen gegner gespawnt und alle tot
        if enemy_res.spawned_this_wave >= setting.enemy_num && enemy_res.alive == 0 {
            enemy_res.is_active = false;
            enemy_res.cooldown = 0.0;
        }
    } else {//inaktive welle...
        //pause hochzählen, wenn welle inaktive
        enemy_res.cooldown += time.delta_secs();

        //welle starten, wenn zeit vorbei
        if enemy_res.cooldown >= setting.free_time {
            enemy_res.is_active = true;
            enemy_res.spawned_this_wave = 0; //gespawnte gegner zurücksetzen
        }
    }    
}