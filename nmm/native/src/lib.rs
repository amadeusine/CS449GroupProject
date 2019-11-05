use base::{Action, Agent, Coord, GameOpts, GameState, Manager};
use neon::prelude::*;
use neon::{class_definition, declare_types, impl_managed, register_module};

fn conv_js_opts(
    ctx: &mut MethodContext<JsManager>,
    action: Action,
    opts: &mut JsObject,
) -> GameOpts {
    match action {
        Action::Piece => GameOpts::new_piece_opt(
            conv_player_option(ctx, opts, "sender"),
            conv_position_option(ctx, opts),
        ),
        Action::Menu => GameOpts::new_menu_opt(
            conv_player_option(ctx, opts, "user"),
            conv_player_option(ctx, opts, "opponent"),
            conv_agent_option(ctx, opts),
        ),
        _ => unreachable!(),
    }
}

fn conv_from_menu(mut cx: FunctionContext, options: JsObject) -> GameOpts {
    unimplemented!()
}

declare_types! {
    pub class JsManager for Manager {
        init(mut ctx) {
            Ok(
                Manager::new()
            )
        }

        method get_board(mut ctx) {
            let this = ctx.this();
            let mut board = {
                let guard = ctx.lock();
                let mngr = this.borrow(&guard);
                mngr.get_board()
            };

            let js_board_arr = JsArray::new(&mut ctx, board.len());


            for (k, v) in board {

                let str_k = ctx.string(k.as_string());
                let str_v = ctx.string(v.as_string());

                let _ = js_board_arr.set(&mut ctx, str_k, str_v);
            }
            Ok(js_board_arr.as_value(&mut ctx))
        }

        method poll(mut ctx) {
            let this = ctx.this();
            let (res_handle, res_trigger, res_board) = {
                let guard = ctx.lock();
                let mngr = this.borrow(&guard);
                mngr.poll()
            };
            let result_obj = JsObject::new(&mut ctx);
            let str_handle = ctx.string(res_handle.to_string());
            let str_trigger = ctx.string(res_trigger.to_string());

            let js_board_arr = JsArray::new(&mut ctx, res_board.len());

            for (k, v) in res_board {

                let str_k = ctx.string(k.as_string());
                let str_v = ctx.string(v.as_string());

                let _ = js_board_arr.set(&mut ctx, str_k, str_v);
            }

            // TODO: Handle error cases here, because result of .set() is technicall a NeonResult type.
            // Really, nothing should go wrong here, but I could at least generically return a
            // JsObject with Handle to err and everything else None to signal failure.
            // NeonResult is just a bool value, so could simply check on returned result as well.
            result_obj.set(&mut ctx, "handle", str_handle);
            result_obj.set(&mut ctx, "trigger", str_trigger);
            result_obj.set(&mut ctx, "board", js_board_arr);

            Ok(result_obj.as_value(&mut ctx))

        }

        method get_user(mut ctx) {
            let mut this = ctx.this();
            let mut opts = ctx.argument::<JsObject>(0)?;
            let user = conv_player_option(&mut ctx, &mut opts, "user");
            let user = ctx.number(user as f64);

            Ok(user.upcast())
        }
        method get_opponent(mut ctx) {
            let mut this = ctx.this();
            let mut opts = ctx.argument::<JsObject>(0)?;
            let opponent = conv_player_option(&mut ctx, &mut opts, "opponent");
            let opponent = ctx.number(opponent as f64);

            Ok(opponent.upcast())
        }

        method get_agent(mut ctx) {
            let mut this = ctx.this();
            let mut opts = ctx.argument::<JsObject>(0)?;
            let agent = conv_agent_option(&mut ctx, &mut opts);
            let opponent = ctx.string(agent.to_string());
            Ok(opponent.upcast())
        }

        method get_req_type(mut ctx) {
            let mut this = ctx.this();
            let mut _type = ctx.argument::<JsString>(0)?;
            let _type = conv_type(&mut ctx, &mut _type);
            let _type = ctx.string(_type.to_string());
            Ok(_type.upcast())

        }

        method get_position(mut ctx) {
            let mut this = ctx.this();
            let mut opts = ctx.argument::<JsObject>(0)?;
            let coord = conv_position_option(&mut ctx, &mut opts);
            let coord = ctx.string(coord.as_string());
            Ok(coord.upcast())
        }
    }
}

fn conv_position_option(ctx: &mut MethodContext<JsManager>, opts: &mut JsObject) -> Coord {
    match opts.get(ctx, "position") {
        Ok(js_handle) if js_handle.is_a::<JsArray>() => match js_handle.downcast::<JsArray>() {
            Ok(arr) => {
                let _arr = arr
                    .to_vec(ctx)
                    .expect("Failed to convert 'position' array to Rust vector");
                let uuuuh: Vec<String> = _arr
                    .iter()
                    .map(|c| {
                        let res = c
                            .downcast::<JsString>()
                            .expect("Failed to downcast internal element of position array");
                        res.value()
                    })
                    .collect();
                let mut s: String = String::from(&uuuuh[0]);
                s.push_str(&uuuuh[1]);
                Coord::from_str(&s)
            }
            _ => panic!("Failed to downcast 'position' property to JsArray"),
        },
        Ok(_) => panic!("Property 'position' does not contain a JsArray"),
        Err(_) => panic!("Could not get 'position' property from optoins object"),
    }
}

fn conv_player_option(
    ctx: &mut MethodContext<JsManager>,
    opts: &mut JsObject,
    player: &str,
) -> u32 {
    match opts.get(ctx, player) {
        Ok(js_handle) if js_handle.is_a::<JsNumber>() => match js_handle.downcast::<JsNumber>() {
            Ok(num) => num.value() as u32,
            Err(e) => panic!("Failed to convert JsNumber: {:#?}", e),
        },
        Ok(_) => {
            // let js_handle = js_handle.upcast();
            panic!(
                "Property \"{}\" did not contain a JsNumber",
                String::from(player)
            )
        }
        Err(_) => panic!(
            "Could not get \"{}\" property from options object.",
            String::from(player)
        ),
    }
}

fn conv_agent_option(ctx: &mut MethodContext<JsManager>, opts: &mut JsObject) -> Agent {
    match opts.get(ctx, "agent") {
        Ok(js_handle) if js_handle.is_a::<JsString>() => match js_handle.downcast::<JsString>() {
            Ok(s) if s.value() == String::from("auto") => Agent::Auto,
            Ok(s) if s.value() == String::from("human") => Agent::Human,
            Ok(_) => panic!("Invalid value for 'agent' property found."),
            Err(_) => unreachable!(),
        },
        Ok(_) => panic!("Property 'agent' did not contain a valid agent value."),
        Err(_) => panic!("Could not get 'agent' property from options object"),
    }
}

fn conv_type(ctx: &mut MethodContext<JsManager>, _type: &mut JsString) -> Action {
    if _type.value() == "Menu" {
        return Action::Menu;
    } else if _type.value() == "Piece" {
        return Action::Piece;
    } else {
        panic!("Invalid value for ElementType: {:#?}", _type.value())
    }
}

register_module!(mut cx, { cx.export_class::<JsManager>("Manager") });

#[cfg(test)]
mod nmm_tests {}
