use base::{GameState, Manager};
use neon::prelude::*;
use neon::{class_definition, declare_types, impl_managed, register_module};

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
                // TODO: more efficient moving of Manager value
                // have to clone manager because it can't derive clone
                // due to hashmap. I don't know how to move around this
                // immediately without just wrapping it in an Rc so the clone
                // is at least cheaper to perform.
                mngr.clone().get_board()
            };

            let js_board_arr = JsArray::new(&mut ctx, board.len());


            for (k, v) in board {

                let str_k = ctx.string(k.as_string());
                let str_v = ctx.string(v.as_string());

                let _ = js_board_arr.set(&mut ctx, str_k, str_v);
            }
            Ok(js_board_arr.as_value(&mut ctx))

        }
    }
}

register_module!(mut cx, { cx.export_class::<JsManager>("Manager") });

// register_module!(mut cx, { cx.export_function("hello", hello) });
