use crate::*;
use neon::prelude::*;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("titlecase", titlecase)?;
    Ok(())
}

fn titlecase(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("foo"))
}
