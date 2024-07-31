mod commands;
mod handlers;
mod config;
mod events;
mod database;

use anyhow::Context as _;
use crate::handlers::anti_spam::message_tracker_cleaner;

// User data, which is stored and accessible in all command invocations
struct Data;

#[shuttle_runtime::main]
async fn main(#[shuttle_runtime::Secrets] secret_store: shuttle_runtime::SecretStore) -> shuttle_serenity::ShuttleSerenity {
    database::DB.connect::<surrealdb::engine::local::Mem>(()).await.unwrap_or_else(|why| {
        eprintln!("Could not connect to database: {why}");
        panic!("Could not connect to database: {why}");
    });

    println!("Connected to database.");

    // Borrar mensajes de la Base de Datos cada 24 horas
    database::clean_database_loop();
    
    // Limpiar el Tracker de mensajes de spam cada 5 segundos
    message_tracker_cleaner();

    // Get the discord token set in `Secrets.toml`
    let discord_token = secret_store
        .get("DISCORD_TOKEN")
        .context("'DISCORD_TOKEN' was not found")?;

    let intents = poise::serenity_prelude::GatewayIntents::all() | poise::serenity_prelude::GatewayIntents::MESSAGE_CONTENT;
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: config::command_loader::load_commands(),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("$".into()),
                edit_tracker: Some(std::sync::Arc::from(poise::EditTracker::for_timespan(tokio::time::Duration::from_secs(3600)))),
                ..Default::default()
            },
            on_error: |error| Box::pin(events::error::err_handler(error)),
            event_handler: |ctx, event, framework, _data| {
                Box::pin(handlers::event_handler(ctx, event, framework))
            },
            allowed_mentions: Some(poise::serenity_prelude::CreateAllowedMentions::default()
                .all_users(true)
                .replied_user(true)),

            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data)
            })
        })
        .build();

    let client = poise::serenity_prelude::ClientBuilder::new(discord_token, intents)
        .framework(framework)
        .await
        .map_err(shuttle_runtime::CustomError::new)?;

    Ok(client.into())
}

