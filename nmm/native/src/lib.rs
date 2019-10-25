use neon::prelude::*;
use neon::register_module;

use base::base_hello;


fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string(base_hello()))
}

register_module!(mut cx, { cx.export_function("hello", hello) });
