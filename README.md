# wonnx-embeddings-repro
> When I try and build my project, I get these errors in the wgpu crate, which is a dependency of the wonnx crate that I am using.

## Relevant Posts
- https://stackoverflow.com/questions/78756708/getting-errors-in-wgpu-crate-when-used-as-a-dependency-of-wonnx
- https://github.com/webonnx/wonnx/issues/212
- Output of `cargo tree`: https://gist.github.com/jacobhq/8248f59c7f392a2d10a1722422641e9b

## My Setup
- Windows 11 Home
- All necessary deps for rust are installed and working (including visual studio)

## Steps to Repro
- `cargo init --lib`
- Add below to `cargo.toml`
  ```
  [lib]
  crate-type = ["cdylib", "rlib"]
  ```
- `cargo add web-sys wonnx wasm-bindgen`
- `$env:RUSTFLAGS="--cfg=web_sys_unstable_apis"; wasm-pack build --target bundler`
- (you may set env var in other ways if not using windows)
- `mkdir model && cd model`
- `git submodule add https://huggingface.co/sentence-transformers/all-MiniLM-L6-v2`

## Logs
```
wonnx-repro on  main [!+?] is 📦 v0.1.0 via 🦀 v1.79.0 took 18s
❯ $env:RUSTFLAGS="--cfg=web_sys_unstable_apis"; wasm-pack build --target bundler
[INFO]: Checking for the Wasm target...
[INFO]: Compiling to Wasm...
   Compiling memchr v2.7.4
   Compiling cfg-if v1.0.0
   Compiling wasm-bindgen v0.2.92
   Compiling num-traits v0.2.19
   Compiling crossbeam-utils v0.8.20                                                                                                                                                                                               
   Compiling log v0.4.22                                                                                                                                                                                                           
   Compiling regex-syntax v0.8.4                                                                                                                                                                                                   
   Compiling js-sys v0.3.69                                                                                                                                                                                                        
   Compiling aho-corasick v1.1.3                                                                                                                                                                                                   
   Compiling thiserror v1.0.62                                                                                                                                                                                                     
   Compiling regex-automata v0.4.7                                                                                                                                                                                                 
   Compiling web-sys v0.3.69                                                                                                                                                                                                       
   Compiling smallvec v1.13.2                                                                                                                                                                                                      
   Compiling slab v0.4.9                                                                                                                                                                                                           
   Compiling lock_api v0.4.12                                                                                                                                                                                                      
   Compiling crossbeam-epoch v0.9.18                                                                                                                                                                                               
   Compiling num-integer v0.1.46                                                                                                                                                                                                   
   Compiling bstr v1.9.1                                                                                                                                                                                                           
   Compiling scopeguard v1.2.0                                                                                                                                                                                                     
   Compiling futures-sink v0.3.30                                                                                                                                                                                                  
   Compiling bitflags v2.6.0                                                                                                                                                                                                       
   Compiling unic-common v0.9.0                                                                                                                                                                                                    
   Compiling futures-core v0.3.30                                                                                                                                                                                                  
   Compiling same-file v1.0.6                                                                                                                                                                                                      
   Compiling unic-char-range v0.9.0                                                                                                                                                                                                
   Compiling unic-char-property v0.9.0                                                                                                                                                                                             
   Compiling walkdir v2.5.0                                                                                                                                                                                                        
   Compiling futures-channel v0.3.30                                                                                                                                                                                               
   Compiling unic-ucd-version v0.9.0                                                                                                                                                                                               
   Compiling parking_lot_core v0.9.10                                                                                                                                                                                              
   Compiling serde v1.0.204                                                                                                                                                                                                        
   Compiling indexmap v1.9.3                                                                                                                                                                                                       
   Compiling globset v0.4.14                                                                                                                                                                                                       
   Compiling crossbeam-deque v0.8.5                                                                                                                                                                                                
   Compiling futures-task v0.3.30                                                                                                                                                                                                  
   Compiling pin-utils v0.1.0                                                                                                                                                                                                      
   Compiling futures-io v0.3.30                                                                                                                                                                                                    
   Compiling hashbrown v0.12.3                                                                                                                                                                                                     
   Compiling pin-project-lite v0.2.14                                                                                                                                                                                              
   Compiling bit-vec v0.6.3                                                                                                                                                                                                        
   Compiling bit-set v0.5.3                                                                                                                                                                                                        
   Compiling futures-util v0.3.30                                                                                                                                                                                                  
   Compiling instant v0.1.13                                                                                                                                                                                                       
   Compiling parking_lot_core v0.8.6                                                                                                                                                                                               
   Compiling serde_json v1.0.120                                                                                                                                                                                                   
   Compiling ignore v0.4.22                                                                                                                                                                                                        
   Compiling unic-ucd-segment v0.9.0                                                                                                                                                                                               
   Compiling num-bigint v0.4.6                                                                                                                                                                                                     
   Compiling ryu v1.0.18                                                                                                                                                                                                           
   Compiling rustc-hash v1.1.0                                                                                                                                                                                                     
   Compiling itoa v1.0.11                                                                                                                                                                                                          
   Compiling ucd-trie v0.1.6                                                                                                                                                                                                       
   Compiling bitflags v1.3.2                                                                                                                                                                                                       
   Compiling naga v0.12.3                                                                                                                                                                                                          
   Compiling protobuf v2.28.0                                                                                                                                                                                                      
   Compiling num-rational v0.4.2                                                                                                                                                                                                   
   Compiling pest v2.7.11                                                                                                                                                                                                          
   Compiling unic-segment v0.9.0                                                                                                                                                                                                   
   Compiling parking_lot v0.12.3                                                                                                                                                                                                   
   Compiling globwalk v0.9.1                                                                                                                                                                                                       
   Compiling futures-executor v0.3.30                                                                                                                                                                                              
   Compiling wgpu-types v0.16.1
   Compiling num-iter v0.1.45                                                                                                                                                                                                      
   Compiling regex v1.10.5                                                                                                                                                                                                         
   Compiling wasm-bindgen-futures v0.4.42                                                                                                                                                                                          
   Compiling num-complex v0.4.6                                                                                                                                                                                                    
   Compiling static_assertions v1.1.0                                                                                                                                                                                              
   Compiling profiling v1.0.15                                                                                                                                                                                                     
   Compiling bytes v1.6.1                                                                                                                                                                                                          
   Compiling arrayvec v0.7.4                                                                                                                                                                                                       
   Compiling raw-window-handle v0.5.2                                                                                                                                                                                              
   Compiling lazy_static v1.5.0                                                                                                                                                                                                    
   Compiling tera v1.20.0                                                                                                                                                                                                          
   Compiling wgpu v0.16.3                                                                                                                                                                                                          
error[E0061]: this function takes 1 argument but 2 arguments were supplied                                                                                                                                                         
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:1523:13
     |
1523 |             web_sys::GpuVertexState::new(desc.vertex.entry_point, &module.0);
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ -------------------------
     |                                          |
     |                                          unexpected argument of type `&str`
     |                                          help: remove the extra argument
     |
note: associated function defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuVertexState.rs:27:12
     |
27   |     pub fn new(module: &GpuShaderModule) -> Self {
     |            ^^^

error[E0061]: this function takes 2 arguments but 3 arguments were supplied                                                                                                                                                        
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:1598:17
     |
1598 |                 web_sys::GpuFragmentState::new(frag.entry_point, &module.0, &targets);
     |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ----------------             -------- unexpected argument of type `&Array`
     |                                                |
     |                                                expected `&GpuShaderModule`, found `&str`
     |
     = note: expected reference `&GpuShaderModule`
                found reference `&str`
note: associated function defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuFragmentState.rs:27:12
     |
27   |     pub fn new(module: &GpuShaderModule, targets: &::wasm_bindgen::JsValue) -> Self {
     |            ^^^
help: remove the extra argument
     |
1598 -                 web_sys::GpuFragmentState::new(frag.entry_point, &module.0, &targets);
1598 +                 web_sys::GpuFragmentState::new(/* &GpuShaderModule */, &module.0);
     |

error[E0061]: this function takes 1 argument but 2 arguments were supplied
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:1623:13
     |
1623 |             web_sys::GpuProgrammableStage::new(desc.entry_point, &shader_module.0);
     |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ------------------
     |                                                |
     |                                                unexpected argument of type `&str`
     |                                                help: remove the extra argument
     |
note: associated function defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuProgrammableStage.rs:27:12
     |
27   |     pub fn new(module: &GpuShaderModule) -> Self {
     |            ^^^

error[E0599]: no method named `write_timestamp` found for struct `GpuCommandEncoder` in the current scope                                                                                                                          
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:2304:14
     |
2302 | /         encoder_data
2303 | |             .0
2304 | |             .write_timestamp(&query_set_data.0, query_index);
     | |             -^^^^^^^^^^^^^^^ method not found in `GpuCommandEncoder`
     | |_____________|
     | 

error[E0308]: mismatched types                                                                                                                                                                                                     
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:2556:17
     |
2554 |             .set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
     |              --------------------------------------------------------------------- arguments to this method are incorrect
2555 |                 index,
2556 |                 &bind_group_data.0,
     |                 ^^^^^^^^^^^^^^^^^^ expected `Option<&GpuBindGroup>`, found `&GpuBindGroup`
     |
     = note:   expected enum `Option<&GpuBindGroup>`
             found reference `&GpuBindGroup`
note: method defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuComputePassEncoder.rs:201:12
     |
201  |     pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: try wrapping the expression in `Some`
     |
2556 |                 Some(&bind_group_data.0),
     |                 +++++                  +

error[E0308]: mismatched types
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:2681:17
     |
2679 |             .set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
     |              --------------------------------------------------------------------- arguments to this method are incorrect
2680 |                 index,
2681 |                 &bind_group_data.0,
     |                 ^^^^^^^^^^^^^^^^^^ expected `Option<&GpuBindGroup>`, found `&GpuBindGroup`
     |
     = note:   expected enum `Option<&GpuBindGroup>`
             found reference `&GpuBindGroup`
note: method defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuRenderBundleEncoder.rs:131:12
     |
131  |     pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: try wrapping the expression in `Some`
     |
2681 |                 Some(&bind_group_data.0),
     |                 +++++                  +

error[E0308]: mismatched types
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:2731:21
     |
2729 |                 encoder_data.0.set_vertex_buffer_with_f64_and_f64(
     |                                ---------------------------------- arguments to this method are incorrect
2730 |                     slot,
2731 |                     &buffer_data.0,
     |                     ^^^^^^^^^^^^^^ expected `Option<&GpuBuffer>`, found `&GpuBuffer`
     |
     = note:   expected enum `Option<&GpuBuffer>`
             found reference `&GpuBuffer`
note: method defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuRenderBundleEncoder.rs:617:12
     |
617  |     pub fn set_vertex_buffer_with_f64_and_f64(
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: try wrapping the expression in `Some`
     |
2731 |                     Some(&buffer_data.0),
     |                     +++++              +

error[E0308]: mismatched types
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:2739:55
     |
2739 |                     .set_vertex_buffer_with_f64(slot, &buffer_data.0, offset as f64);
     |                      --------------------------       ^^^^^^^^^^^^^^ expected `Option<&GpuBuffer>`, found `&GpuBuffer`
     |                      |
     |                      arguments to this method are incorrect
     |
     = note:   expected enum `Option<&GpuBuffer>`
             found reference `&GpuBuffer`
note: method defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuRenderBundleEncoder.rs:546:12
     |
546  |     pub fn set_vertex_buffer_with_f64(
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^
help: try wrapping the expression in `Some`
     |
2739 |                     .set_vertex_buffer_with_f64(slot, Some(&buffer_data.0), offset as f64);
     |                                                       +++++              +

error[E0308]: mismatched types
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:2896:17
     |
2894 |             .set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
     |              --------------------------------------------------------------------- arguments to this method are incorrect
2895 |                 index,
2896 |                 &bind_group_data.0,
     |                 ^^^^^^^^^^^^^^^^^^ expected `Option<&GpuBindGroup>`, found `&GpuBindGroup`
     |
     = note:   expected enum `Option<&GpuBindGroup>`
             found reference `&GpuBindGroup`
note: method defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuRenderPassEncoder.rs:218:12
     |
218  |     pub fn set_bind_group_with_u32_array_and_f64_and_dynamic_offsets_data_length(
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: try wrapping the expression in `Some`
     |
2896 |                 Some(&bind_group_data.0),
     |                 +++++                  +

error[E0308]: mismatched types
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:2946:21
     |
2944 |                 pass_data.0.set_vertex_buffer_with_f64_and_f64(
     |                             ---------------------------------- arguments to this method are incorrect
2945 |                     slot,
2946 |                     &buffer_data.0,
     |                     ^^^^^^^^^^^^^^ expected `Option<&GpuBuffer>`, found `&GpuBuffer`
     |
     = note:   expected enum `Option<&GpuBuffer>`
             found reference `&GpuBuffer`
note: method defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuRenderPassEncoder.rs:704:12
     |
704  |     pub fn set_vertex_buffer_with_f64_and_f64(
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
help: try wrapping the expression in `Some`
     |
2946 |                     Some(&buffer_data.0),
     |                     +++++              +

error[E0308]: mismatched types
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\wgpu-0.16.3\src\backend\web.rs:2954:55
     |
2954 |                     .set_vertex_buffer_with_f64(slot, &buffer_data.0, offset as f64);
     |                      --------------------------       ^^^^^^^^^^^^^^ expected `Option<&GpuBuffer>`, found `&GpuBuffer`
     |                      |
     |                      arguments to this method are incorrect
     |
     = note:   expected enum `Option<&GpuBuffer>`
             found reference `&GpuBuffer`
note: method defined here
    --> C:\Users\jacob\.cargo\registry\src\index.crates.io-6f17d22bba15001f\web-sys-0.3.69\src\features\gen_GpuRenderPassEncoder.rs:633:12
     |
633  |     pub fn set_vertex_buffer_with_f64(
     |            ^^^^^^^^^^^^^^^^^^^^^^^^^^
help: try wrapping the expression in `Some`
     |
2954 |                     .set_vertex_buffer_with_f64(slot, Some(&buffer_data.0), offset as f64);
     |                                                       +++++              +

   Compiling num v0.4.3                                                                                                                                                                                                            
Some errors have detailed explanations: E0061, E0308, E0599.                                                                                                                                                                       
For more information about an error, try `rustc --explain E0061`.
   Compiling futures v0.3.30
error: could not compile `wgpu` (lib) due to 11 previous errors                                                                                                                                                                    
warning: build failed, waiting for other jobs to finish...
Error: Compiling your crate to WebAssembly failed                                                                                                                                                                                  
Caused by: Compiling your crate to WebAssembly failed
Caused by: failed to execute `cargo build`: exited with exit code: 101
  full command: "cargo" "build" "--lib" "--release" "--target" "wasm32-unknown-unknown"

wonnx-repro on  main [!+?] is 📦 v0.1.0 via 🦀 v1.79.0 took 2m40s
❯
```