use app::{MyApp, SiteName};
use bevy::prelude::*;
use async_bevy_web::prelude::ABWConfigPlugin;
use async_bevy_web::prelude::LeptosAppPlugin;

fn main(){
    App::new()
        .insert_resource(SiteName("Bevy + Leptos".to_owned()))
        .add_systems(Startup, print_running)
        .add_plugins(ABWConfigPlugin::new(60.0))
        .add_plugins(LeptosAppPlugin::new(MyApp))
        .run();
}

fn print_running(){
    println!("Running!")
}
