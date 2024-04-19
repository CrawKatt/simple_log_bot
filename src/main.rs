mod commands;
mod handlers;
mod config;
mod events;
mod database;

use std::net::SocketAddr;
use handlers::event_handler;
use events::error::err_handler;
use config::command_loader::load_commands;
use database::clean_database_loop;
use database::DB;

use anyhow::Context as _;
use poise::serenity_prelude as serenity;
use shuttle_runtime::SecretStore;
use shuttle_serenity::ShuttleSerenity;
use std::sync::Arc;
use tokio::time::Duration;
use surrealdb::engine::local::Mem;
use reqwest::Client;
use serenity::all::Http;

// User data, which is stored and accessible in all command invocations
struct Data {
    poise_mentions: String,
    client: Client,
}

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> ShuttleSerenity {
    DB.connect::<Mem>(()).await.unwrap_or_else(|why| {
        eprintln!("Could not connect to database: {why}");
        panic!("Could not connect to database: {why}");
    });

    println!("Connected to database.");

    // Borrar mensajes de la Base de Datos cada 24 horas
    clean_database_loop();

    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let intents = serenity::GatewayIntents::all() | serenity::GatewayIntents::MESSAGE_CONTENT;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: load_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("$".into()),
                edit_tracker: Some(Arc::from(poise::EditTracker::for_timespan(Duration::from_secs(3600)))),
                ..Default::default()
            },
            on_error: |error| Box::pin(err_handler(error)),
            event_handler: |ctx, event, framework, _data| {
                Box::pin(event_handler(ctx, event, framework))
            },
            allowed_mentions: Some(serenity::CreateAllowedMentions::default()
                .all_users(true)
                .replied_user(true)),

            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    poise_mentions: String::default(),
                    client: Client::default(),
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}

