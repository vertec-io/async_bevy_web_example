use app::pages::blog_post::{BlogYear, DraftPost, Post, PostTitle, TestFontMatter};
use app::pages::home_page::{Age, PersonName};
use app::{App, SiteName};
use bevy_ecs::system::EntityCommands;
use cinnog::loaders::markdown::{ConvertMarkdownToHtml, MarkdownDataLayer};
use cinnog::{default_bundle_from_path, DataLayer, Ingest};
use leptos::{get_configuration, provide_context, serde, IntoView};
use regex::Regex;
use std::io;
use std::path::Path;
use std::sync::{Arc, Mutex};
use cinnog::loaders::ron::RonDataLayer;
use bevy::prelude::*;
use async_bevy_web::prelude::ABWConfigPlugin;
use async_bevy_web::prelude::LeptosAppPlugin;
use tokio::task;
use leptos_axum::generate_route_list_with_exclusions_and_ssg_and_context;
use leptos_router::build_static_routes_with_additional_context;

fn main(){
    let mut binding = DataLayer::new();
    let app = binding
                                .insert_resource(SiteName("Bevy ECS + Leptos = 💕".to_owned()))
                                .add_markdown_directory::<PostFrontMatter>("blog")
                                .add_ron_directory::<PersonData>("people")
                                .add_plugins(ConvertMarkdownToHtml);
                                
    let app = std::mem::take(app);
    let arc_app = Arc::new(Mutex::new(app));
    let arc_app_clone = arc_app.clone();

    let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                // .unwrap();
                .expect("Could not start a runtime to load static assets");

    let _res = rt.block_on(async {
        let mut app_guard = arc_app_clone.lock().unwrap();
        // app_guard.build(App).await
        // let app = app_guard;
        println!("Building app...")
        app_guard.build_external(arc_app_clone.clone(), App).await
    });

    println!("Starting the application...");
    let mut app = arc_app.lock().unwrap();
    app.app.add_systems(PostStartup, print_running)
         .add_plugins(ABWConfigPlugin::default())
         .add_plugins(LeptosAppPlugin::new(App));

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

trait ExternalBuild{
    async fn build_external<IV>(
        &mut self, 
        data: Arc<Mutex<DataLayer>>,
        pp_fn: impl Fn() -> IV + Clone + Send + 'static,
    ) -> std::io::Result<()>
    where
        IV: IntoView + 'static;
    
}

impl ExternalBuild for DataLayer {
    async fn build_external<IV>(
        &mut self,
        data: Arc<Mutex<DataLayer>>,
        app_fn: impl Fn() -> IV + Clone + Send + 'static,
    ) -> std::io::Result<()>
    where
        IV: IntoView + 'static,
    {
        self.app.update();
        // let datalayer = std::mem::replace(self, DataLayer::new());
        // let data = Arc::new(Mutex::new(datalayer));
        let data_for_route_generation = data.clone();

        let conf = get_configuration(None).await.unwrap();
        let leptos_options = conf.leptos_options;

        let (routes, static_data_map) = generate_route_list_with_exclusions_and_ssg_and_context(
            app_fn.clone(),
            None,
            move || provide_context(data_for_route_generation.clone()),
        );

        let local = task::LocalSet::new();
        let app_fn_clone = app_fn.clone();
        let leptos_options_clone = leptos_options.clone();
        let routes_clone = routes.clone();
        local
            .run_until(async move {
                println!("Building static routes!") ;
                build_static_routes_with_additional_context(
                    &leptos_options_clone,
                    app_fn_clone,
                    move || provide_context(data.clone()),
                    &routes_clone,
                    &static_data_map,
                )
                .await
                .expect("Failed to build static routes")
            })
            .await;
        Ok(())
    }

}