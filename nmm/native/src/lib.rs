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
    }
}

register_module!(mut cx, { cx.export_class::<JsManager>("Manager") });
