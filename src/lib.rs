use std::{collections::HashMap, sync::Arc};
use wonnx::utils::{InputTensor, OutputTensor};
use web_sys::console;
use tokenizers::tokenizer::Tokenizer;
use wasm_bindgen::JsValue;
use wasm_bindgen::prelude::wasm_bindgen;

const DIM: usize = 384;

static MODEL_DATA: &'static [u8] = include_bytes!("../model/all-MiniLM-L6-v2/onnx/model.onnx",);
static TOKENIZER_DATA: &'static [u8] = include_bytes!("../model/all-MiniLM-L6-v2/tokenizer.json",);

fn average_pool(last_hidden_layer: &[f32], mask: &[i32]) -> Vec<f32> {
    // input 1,512,emb_d , len = 1x512
    // mask is 1,512
    // let mut avg: Vec<f32> = vec![0.0; 384];
    let mask_sum: i32 = mask.iter().sum();

    let avg = last_hidden_layer
        .chunks(DIM)
        .enumerate()
        .filter(|(idx, _)| mask[*idx] == 1)
        .fold(vec![0.0; DIM], |acc, (_, layer)| {
            dbg!(&layer.len());
            acc.into_iter()
                .zip(layer)
                .map(|(l, &r)| l + r)
                .collect::<Vec<_>>()
        });
    dbg!(&avg[..10]);
    avg.into_iter().map(|e| e / mask_sum as f32).collect()
}

pub struct Embedder {
    session: Arc<wonnx::Session>,
    tokenizer: Tokenizer,
}
impl Embedder {
    pub async fn load() -> Result<Embedder, String> {
        console::log_1(&"Starting to load tokenizer".into());
        let tokenizer = Tokenizer::from_bytes(TOKENIZER_DATA)
            .map_err(|e| format!("Failed to load tokenizer: {}", e))?;
        console::log_1(&"Tokenizer loaded successfully".into());

        console::log_1(&"Starting to load model".into());
        let session = wonnx::Session::from_bytes(MODEL_DATA)
            .await
            .map_err(|e| {
                let error_msg = format!("Failed to load model: {}", e);
                console::error_1(&error_msg.clone().into());
                error_msg
            })?;
        console::log_1(&"Model loaded successfully".into());

        Ok(Self {
            session: Arc::new(session),
            tokenizer,
        })
    }

    pub async fn embed_query(&self, txt: String) -> Result<Vec<f32>, String> {
        let mut input: HashMap<String, InputTensor> = HashMap::new();
        let encoding = self.tokenizer.encode(txt, true).unwrap();
        let tokens: Vec<i32> = encoding
            .get_ids()
            .iter()
            .map(|&e| e as i32)
            .collect::<Vec<_>>();
        let token_type_ids = encoding
            .get_type_ids()
            .iter()
            .map(|&e| e as i32)
            .collect::<Vec<_>>();
        let attention_mask = encoding
            .get_attention_mask()
            .iter()
            .map(|&e| e as i32)
            .collect::<Vec<_>>();

        input.insert("input_ids".to_string(), tokens[..].into());
        input.insert("attention_mask".to_string(), attention_mask[..].into());
        input.insert("token_type_ids".to_string(), token_type_ids[..].into());
        let output = self.session.clone().run(&input).await.unwrap();

        match output.get(&"last_hidden_state".to_string()).unwrap() {
            OutputTensor::F32(last_hidden_layer) => {
                dbg!(&last_hidden_layer[..10]);
                let emb = average_pool(last_hidden_layer, &attention_mask);
                Ok(emb)
            }
            _ => Err("can't have other type".to_string()),
        }
    }
}

pub struct EmbeddingService {
    embedder: Embedder,
}

impl EmbeddingService {
    pub async fn new() -> Result<EmbeddingService, JsValue> {
        let embedder = Embedder::load()
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to load embedder: {}", e)))?;

        Ok(EmbeddingService { embedder })
    }

    pub async fn embed_text(&self, text: String) -> Result<Vec<f32>, JsValue> {
        self.embedder
            .embed_query(text)
            .await
            .map_err(|e| JsValue::from_str(&format!("Failed to embed text: {}", e)))
    }
}

#[wasm_bindgen]
pub async fn embed() -> Result<js_sys::Float32Array, JsValue> {
    let service = EmbeddingService::new().await
        .map_err(|e| JsValue::from(e))?;

    let embedding = service.embed_text("Your text here".to_string()).await
        .map_err(|e| JsValue::from(e))?;

    Ok(js_sys::Float32Array::from(&embedding[..]))
}
