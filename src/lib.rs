use js_sys::Function;
use leptos::*;
use rand::Rng;
use wasm_bindgen::JsValue;
pub mod zig_wasm;
use zig_wasm::ZigWasm;

#[component]
pub fn App(cx: Scope, go_wasm_func: Function, zig_wasm: ZigWasm) -> impl IntoView {
    let mut rng = rand::thread_rng();
    let (input, set_input) = create_signal(
        cx,
        if rng.gen::<f32>() < 0.5 {
            "WASM UP YOUR ASS!!!"
        } else {
            "Yup... it's WASM time!"
        }
        .to_string(),
    );
    view! {
        cx,
        <div class="m-4">
            <h1 class="font-bold text-4xl">"wasminwasm"</h1>
            <div class="p-4 flex flex-col gap-2">
                <div class="flex flex-col">
                    <label class="mb-1" for="input">"put in something:"</label>
                    <input
                        class="p-1 border-solid border rounded-md border-black dark:border-white dark:bg-slate-800"
                        id="input"
                        type="text"
                        prop:value=input
                        on:input=move |ev| { set_input(event_target_value(&ev)) }
                    />
                </div>
                <Go input go_wasm_func />
                <Zig input zig_wasm />
            </div>
        </div>
    }
}

#[component]
fn Go(cx: Scope, input: ReadSignal<String>, go_wasm_func: Function) -> impl IntoView {
    let call_ctx = JsValue::undefined();
    let go_wasm_says = move || {
        go_wasm_func
            .call1(&call_ctx, &input().into())
            .unwrap()
            .as_string()
            .unwrap()
    };
    view! {
       cx,
       <div class="flex gap-1">
           <span class="font-semibold">"Go WASM says:"</span>
           <span>"\""{go_wasm_says}"\""</span>
       </div>
    }
}

#[component]
fn Zig(cx: Scope, input: ReadSignal<String>, zig_wasm: ZigWasm) -> impl IntoView {
    let zig_wasm_says = move || match zig_wasm.reverse_string(&input()) {
        Ok(res) => res,
        Err(e) => {
            web_sys::console::error_1(&e);
            String::from("ERROR!")
        }
    };
    view! {
        cx,
        <div class="flex gap-1">
            <span class="font-semibold">"Zig WASM says:"</span>
            <span>"\""{zig_wasm_says}"\""</span>
        </div>
    }
}
