use app::pages::blog_post::{BlogYear, DraftPost, Post, PostTitle, TestFontMatter};
use app::pages::home_page::{Age, PersonName};
use app::{App, SiteName};
use bevy_ecs::system::EntityCommands;
use cinnog::loaders::markdown::{ConvertMarkdownToHtml, MarkdownDataLayer};
use cinnog::{default_bundle_from_path, DataLayer, Ingest};
use leptos::serde;
use regex::Regex;
use std::io;
use std::path::Path;
use cinnog::loaders::ron::RonDataLayer;
use bevy::prelude::*;
use async_bevy_web::prelude::ABWConfigPlugin;
use async_bevy_web::prelude::LeptosAppPlugin;

fn main(){
    let mut binding = DataLayer::new();
    let app = binding
                                .insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()))
                                .add_markdown_directory::<PostFrontMatter>("blog")
                                .add_ron_directory::<PersonData>("people")
                                .add_plugins(ConvertMarkdownToHtml)
                                .add_plugins(ABWConfigPlugin::default())
                                .add_plugins(LeptosAppPlugin::new(App));
    
    app.app.add_systems(PostStartup, print_running);

    let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                // .unwrap();
                .expect("Could not start a runtime to load static assets");

    let _res = rt.block_on(async {
                app.build(App).await
            });

    app.app.run();
}


fn print_running(){
    println!("Running!")
}

#[derive(serde::Deserialize)]
struct PersonData {
    name: String,
    age: u8,
}

impl Ingest for PersonData {
    fn ingest(self, commands: &mut EntityCommands) {
        commands.insert((PersonName(self.name), Age(self.age)));
    }
}

#[derive(serde::Deserialize, Default)]
#[serde(default)]
pub struct PostFrontMatter {
    pub test: String,
    pub title: String,
    pub draft: bool,
}

impl Ingest for PostFrontMatter {
    fn ingest(self, commands: &mut EntityCommands) {
        commands.insert((TestFontMatter(self.test), PostTitle(self.title), Post));
        if self.draft {
            commands.insert(DraftPost);
        }
    }

    fn ingest_path(&self, commands: &mut EntityCommands, path: &Path) {
        let reg = Regex::new(r"/blog/(<year>[0-9]+)/\.*").unwrap();
        if let Some(caps) = reg.captures(&path.to_string_lossy()) {
            let year = &caps["year"];
            commands.insert(BlogYear(year.to_owned()));
        };
        commands.insert(default_bundle_from_path(path));
    }
}
