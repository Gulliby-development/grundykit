use crate::error::GameError;
use crate::io::Io;
use crate::text;

pub const USAGE: &str = "usage: grundykit grundy [--max N] RULE
       grundykit moves PART
       grundykit sum PART...
       grundykit win PART...
       grundykit play PART...
       grundykit period [--max N] [--cycles N] RULE
       grundykit misere HEAP...
       grundykit wythoff A B
       grundykit pairs [--max N]
       grundykit nimber A B";

pub fn run(args: &[String], io: &dyn Io) -> i32 {
    if args.len() == 1 && args[0] == "--help" {
        io.out(&format!("{USAGE}\n"));
        return 0;
    }

    let result = match args.first() {
        None => Err(GameError(
            "missing command (grundy, moves, sum, win, play, period, misere, wythoff, pairs or nimber)"
                .to_owned(),
        )),
        Some(command) => {
            let argv = &args[1..];
            match command.as_str() {
                "grundy" => crate::cmd_grundy::run(argv, io),
                "moves" => crate::cmd_moves::run(argv, io),
                "sum" => crate::cmd_sum::run(argv, io),
                "win" => crate::cmd_win::run(argv, io),
                "play" => crate::cmd_play::run(argv, io),
                "period" => crate::cmd_period::run(argv, io),
                "misere" => crate::cmd_misere::run(argv, io),
                "wythoff" => crate::cmd_wythoff::run(argv, io),
                "pairs" => crate::cmd_pairs::run(argv, io),
                "nimber" => crate::cmd_nimber::run(argv, io),
                _ => Err(GameError(format!(
                    "unknown command {}",
                    text::show(command)
                ))),
            }
        }
    };

    match result {
        Ok(code) => code,
        Err(GameError(message)) => {
            io.err(&format!("grundykit: {message}\n"));
            2
        }
    }
}
