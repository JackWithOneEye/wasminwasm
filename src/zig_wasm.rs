use js_sys::{BigInt, Function, Uint8Array, WebAssembly};
use wasm_bindgen::{JsCast, JsValue};

#[derive(Clone)]
pub struct ZigWasm {
    free_u8: Function,
    malloc_u8: Function,
    reverse_string: Function,
    wasm_memory: WebAssembly::Memory,
}

impl ZigWasm {
    pub fn new(
        free_u8: Function,
        malloc_u8: Function,
        reverse_string: Function,
        wasm_memory: WebAssembly::Memory,
    ) -> ZigWasm {
        ZigWasm {
            free_u8,
            malloc_u8,
            reverse_string,
            wasm_memory,
        }
    }

    pub fn reverse_string(&self, input: &String) -> Result<String, JsValue> {
        let call_ctx = JsValue::undefined();
        let input_len = input.as_bytes().len() as u32;
        let input_dest_ptr = self
            .malloc_u8
            .call1(&call_ctx, &input.len().into())?
            .as_f64()
            .unwrap() as u32;

        let input_buffer = self.wasm_mem_subarray(input_dest_ptr, input_dest_ptr + input_len);
        input_buffer.set(&Uint8Array::from(input.as_bytes()), 0);

        let output_ptr_and_len: BigInt = self
            .reverse_string
            .call2(&call_ctx, &input_dest_ptr.into(), &input_len.into())?
            .dyn_into()?;
        let output_ptr_and_len = Self::bigint_to_u64(output_ptr_and_len)?;
        let output_ptr = (output_ptr_and_len >> 32) as u32;
        let output_len = (output_ptr_and_len & 0x0000ffff) as u32;

        let output_buffer = self.wasm_mem_subarray(output_ptr, output_ptr + output_len);

        self.free_u8
            .call2(&call_ctx, &input_dest_ptr.into(), &input_len.into())?;

        String::from_utf8(output_buffer.to_vec()).map_err(|e| JsValue::from(&e.to_string()))
    }

    fn wasm_mem_subarray(&self, begin: u32, end: u32) -> Uint8Array {
        Uint8Array::new(&self.wasm_memory.buffer()).subarray(begin, end)
    }

    fn bigint_to_u64(bigint: BigInt) -> Result<u64, JsValue> {
        String::from(bigint.to_string(10)?)
            .parse::<u64>()
            .map_err(|e| e.to_string().into())
    }
}
