use app::{App, SiteName};
use bevy::prelude::*;
use async_bevy_web::prelude::ABWConfigPlugin;
use async_bevy_web::prelude::LeptosAppPlugin;

fn main(){
    App::new()
        .insert_resource(SiteName("Bevy + Leptos".to_owned()))
        .add_systems(PostStartup, print_running)
        .add_plugins(ABWConfigPlugin::default())
        .add_plugins(LeptosAppPlugin::new(App))
        .run();
}

fn print_running(){
    println!("Running!")
}
