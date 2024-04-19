#[allow(unused_variables)]
use crate::Data;

pub type CommandResult = Result<(), Error>;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

pub async fn err_handler(error: poise::FrameworkError<'_, Data, Error>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            eprintln!("Error al iniciar el Bot: {error:?}");
            panic!()
        },

        poise::FrameworkError::Command { error, ctx, ..} => {
            eprintln!("Error en comando `{}` : {:?}", &ctx.command().name, error);
        },

        poise::FrameworkError::EventHandler { error, event, .. } => {
            eprintln!("Error en el evento: {:?} Causa del error: {:?}", event.snake_case_name(), error.source());
        },

        error => {
            if let Err(why) = poise::builtins::on_error(error).await {
                eprintln!("Error al manejar el error: {why}");
            }
        }
    }
}