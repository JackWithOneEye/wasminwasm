use js_sys::{Function, Object, Reflect, WebAssembly};
use leptos::{leptos_dom::logging::console_log, mount_to_body, spawn_local, view, window};
use wasm_bindgen::{prelude::Closure, JsCast, JsValue, UnwrapThrowExt};
use wasm_bindgen_futures::JsFuture;
use wasminwasm::{zig_wasm::ZigWasm, App};

const GO_WASM: &[u8] = include_bytes!("../wasm/go.wasm");
const ZIG_WASM: &[u8] = include_bytes!("../wasm/zig.wasm");

async fn init_go_wasm_func() -> Result<Function, JsValue> {
    let window = window();
    let create_go: Function = Reflect::get(&window, &"createGo".into())?.dyn_into()?;
    let go = create_go.call0(&JsValue::undefined())?;
    let import_object: Object = Reflect::get(&go, &"importObject".into())?.dyn_into()?;
    let result_obj =
        JsFuture::from(WebAssembly::instantiate_buffer(GO_WASM, &import_object)).await?;
    let wasm_instance = Reflect::get(&result_obj, &"instance".into())?;

    let run: Function = Reflect::get(&go, &"run".into())?.dyn_into()?;
    run.call1(&go, &wasm_instance)?;

    Reflect::get(&window, &"stringLenInfo".into())?.dyn_into()
}

async fn init_zig_wasm() -> Result<ZigWasm, JsValue> {
    let import_object_env = Object::new();
    let log: Closure<dyn FnMut(JsValue)> =
        Closure::new(|arg: JsValue| web_sys::console::log_1(&arg));
    Reflect::set(
        &import_object_env,
        &"consoleLog".into(),
        log.as_ref().unchecked_ref(),
    )?;
    let import_object = Object::new();
    Reflect::set(&import_object, &"env".into(), &import_object_env)?;
    let result_obj =
        JsFuture::from(WebAssembly::instantiate_buffer(ZIG_WASM, &import_object)).await?;
    let wasm_instance: WebAssembly::Instance =
        Reflect::get(&result_obj, &"instance".into())?.dyn_into()?;
    let exports = Reflect::get(&wasm_instance, &"exports".into())?;

    let say_hello: Function = Reflect::get(&exports, &"sayHello".into())?.dyn_into()?;
    say_hello.call0(&JsValue::undefined())?;

    let malloc_u8: Function = Reflect::get(&exports, &"mallocu8".into())?.dyn_into()?;
    let free_u8: Function = Reflect::get(&exports, &"freeu8".into())?.dyn_into()?;
    let reverse_string: Function = Reflect::get(&exports, &"reverseString".into())?.dyn_into()?;
    let memory: WebAssembly::Memory = Reflect::get(&exports, &"memory".into())?.dyn_into()?;

    Ok(ZigWasm::new(free_u8, malloc_u8, reverse_string, memory))
}

fn main() {
    console_log("HELLO RUST!!!!");

    spawn_local(async {
        let go_wasm_func = init_go_wasm_func().await.unwrap_throw();
        let zig_wasm = init_zig_wasm().await.unwrap_throw();

        mount_to_body(
            move || view! {  <App go_wasm_func=go_wasm_func.clone() zig_wasm=zig_wasm.clone()/> },
        );
    });
}
