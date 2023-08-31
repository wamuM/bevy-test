use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App){
        app.add_systems(Startup,spawn_player)
           .add_systems(Update,player_movement);
    }
}


pub struct WizardStats {
    hp:u32, //Health Points
    mp:u32, //Mana Points 
    re:u32, //Renown Essence
    ka:u32, //Karma
    vr:u32, //Vril 
}
#[derive(Component)]
pub struct Player{
    stats:WizardStats,
    speed:f32,
    name:std::string:String,
}

enum Wizard{
    Player,
    Raider
}

fn spawn_player(mut commands:Commands){
    commands.spawn((
        Player{
            speed:23
            name:"No name for now".to_string()
            stats: WizardStats{

            }
        }
    ));
    
}

fn player_movement(){

}
