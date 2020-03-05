use crate::glimbot::modules::command::*;
use once_cell::sync::Lazy;
use serenity::prelude::{Context};
use serenity::model::prelude::Message;
use crate::glimbot::modules::command::ArgType::Str;
use log::{error};
use crate::glimbot::modules::command::CommanderError::Other;

fn ping(_cmd: &Commander, ctx: &Context, msg: &Message, args: &[Arg]) -> Result<()> {
    let response =
        if args.len() > 0 {
            if let Arg::Str(s) = &args[0] {
                s
            } else {
                return Err(CommanderError::BadParameter(0, Str));
            }
        } else {
            "Pong!"
        };

    let res = msg.channel_id.say(ctx, response);

    if let Err(e) = res {
        error!("{:?}", e);
        Err(Other)
    } else {
        Ok(())
    }
}

static PING: Lazy<Commander> = Lazy::new(
    || Commander::new(
        "ping",
        Some("Responds with pong OR echoes the single optional argument."),
        vec!["echo"],
        vec![],
        vec![ArgType::Str],
        ping
    )
);