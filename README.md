# wonnx-embeddings-repro
> I'm attempting to embed some text as vectors in the browser using rust, webassembly and wonnx. When I try and run my model in the demo, I get these errors in the console:

## Errors
```
wonnx_embeddings_repro_bg.js:820 Failed to load model: IR error: issue with data types: encountered parametrized dimensions 'unk__0'; this is not currently supported (this may be solved by running onnx-simplifier on the model first)
__wbg_error_fe807da27c4a4ced @ wonnx_embeddings_repro_bg.js:820
$func184 @ wonnx_embeddings_repro_bg.wasm?t=1721295649015:0x3f7a8
$func1067 @ wonnx_embeddings_repro_bg.wasm?t=1721295649015:0x219fe7
$func2910 @ wonnx_embeddings_repro_bg.wasm?t=1721295649015:0x29f7f3
$_dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hd5bdc993e1827b1c @ wonnx_embeddings_repro_bg.wasm?t=1721295649015:0x29f7e5
__wbg_adapter_33 @ wonnx_embeddings_repro_bg.js:227
real @ wonnx_embeddings_repro_bg.js:208
App.tsx:7 Uncaught (in promise) Failed to load embedder: Failed to load model: IR error: issue with data types: encountered parametrized dimensions 'unk__0'; this is not currently supported (this may be solved by running onnx-simplifier on the model first)
```

## Relevant Posts
- https://stackoverflow.com/questions/78763732/getting-console-errors-when-trying-to-run-model-in-browser-using-rust-wasm-and
- Output of `cargo tree`: https://gist.github.com/jacobhq/f20dfe14e5adf7a60843d29e5eccc6e2

## Related Code and Reading
I've read these, and they partly inspired the project. Attaching them in case they are useful.
- [Semantic search powered by WASM and WebGPU](https://medium.com/@aminedirhoussi1/semantic-search-powered-by-wasm-and-wgpu-492e900b8796)
- [AmineDiro/docvec](https://github.com/AmineDiro/docvec)

## My Setup
- Windows 11 Home
- All necessary deps for rust are installed and working (including visual studio)

> [!IMPORTANT]  
> To use wonnx in the browser, you must enable the [enable-unsafe-webgpu](chrome://flags/#enable-unsafe-webgpu) chrome flag.

## Steps to Run This Repro
- `pip install onnx-simplifier`
- `python -m onnxsim model/all-MiniLM-L6-v2/onnx/model.onnx model/all-MiniLM-L6-v2/onnx/model_sim.onnx`
- `$env:RUSTFLAGS="--cfg=web_sys_unstable_apis"; wasm-pack build --target bundler --out-dir ./web/wasm`
- `cd web`
  - `pnpm i`
  - `pnpm dev`

## Steps to Make This Repro Yourself
- `cargo init --lib`
- Add below to `cargo.toml`
  ```
  [lib]
  crate-type = ["cdylib", "rlib"]
  ```
- `cargo add web-sys wonnx wasm-bindgen`
- Copy and paste my `src/lib.rs`
- `$env:RUSTFLAGS="--cfg=web_sys_unstable_apis"; wasm-pack build --target bundler`
- (you may set env var in other ways if not using windows)
- `mkdir model && cd model`
- `git submodule add https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2`
- `pip install onnx-simplifier`
- `python -m onnxsim model/all-MiniLM-L6-v2/onnx/model.onnx model/all-MiniLM-L6-v2/onnx/model_sim.onnx`
- `pnpm create vite` (React, TypeScript + SWC), project name: `web`
- `$env:RUSTFLAGS="--cfg=web_sys_unstable_apis"; wasm-pack build --target bundler --out-dir ./web/wasm`
- `cd web`
  - `pnpm i --save-dev vite-plugin-wasm`
  - `pnpm i ./wasm`
  - (Changed files: `vite.config.ts`, `src/App.tsx`, `src/main.tsx`, delete everything else in `src`, except `vite-env.d.ts`)
  - `pnpm dev`
