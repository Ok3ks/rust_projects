use hf_hub::api::sync::{Api};
use hf_hub::{Repo, RepoType};
use tokenizers::{Tokenizer};

use anyhow::{Error as E, Result};
use candle_core::{ Device, Tensor, DType };
use candle_nn::VarBuilder;
// use candle_transformers::models::mimi::candle_nn::var_builder::VarBuilderArgs
// use candle_transformers::quantized_var_builder::VarBuilder;
use clap::Parser;
use candle_transformers::models::bert::{BertModel, Config, HiddenAct, DTYPE};

//goal is to load sbert from huggingface and compute embeddings

//load finetuned json
//load respective model type in Candle

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    
    #[arg(long)]
    cpu: bool,

    #[arg(long)]
    tracing: bool, 

    #[arg(long)]
    model_id: Option<String>,

    #[arg(long)]
    prompt: Option<String>,

    #[arg(long)]
    use_pth: bool, 

    #[arg(long, default_value_t = 1)]
    n: usize,

    #[arg(long, default_value = "true")]
    normalize_embeddings: bool,

    #[arg(long, default_value = "false")]
    approximate_gelu: bool,

}

impl Args {
    fn build_model_and_tokenizer(&self) -> Result<(BertModel, Tokenizer)> {
        let device = Device::Cpu;
        let model = "sentence-transformers/all-MiniLM-L6-v2".to_string();
        let revision = "refs/pr/21".to_string();

        let repo = Repo::with_revision(model, RepoType::Model, revision);
        let (config_filename, tokenizer_filename, weights_filename) = {
            let api = Api::new()?;
            let api = api.repo(repo);
            let config = api.get("config.json")?;
            let tokenizer = api.get("tokenizer.json")?;
            let weights = if self.use_pth {
                api.get("pytorch_model.bin")?
            } else {
                api.get("model.safetensors")?
            };
            (config, tokenizer, weights)
        };
        let config = std::fs::read_to_string(config_filename)?;
        let mut config: Config = serde_json::from_str(&config)?;
        let tokenizer = Tokenizer::from_file(tokenizer_filename).map_err(E::msg)?;

        let vb = if self.use_pth {
            VarBuilder::from_pth(&weights_filename, DType::F32, &device)?
        } else {
            unsafe { VarBuilder::from_mmaped_safetensors(&[weights_filename], DType::F32, &device)? }
        };
        if self.approximate_gelu {
            config.hidden_act = HiddenAct::GeluApproximate;
        }
        let model = BertModel::load(vb, &config)?;
        Ok((model, tokenizer))
        }
    }


pub fn main() -> Result<()> {
    use tracing_chrome::ChromeLayerBuilder;
    use tracing_subscriber::prelude::*;

    let args = Args::parse();
    let _guard = if args.tracing {
        println!("tracing...");
        let (chrome_layer, guard) = ChromeLayerBuilder::new().build();
        tracing_subscriber::registry().with(chrome_layer).init();
        Some(guard)
    } else {
        None
    };
    
    let start = std::time::Instant::now();
    let (model, mut tokenizer) = args.build_model_and_tokenizer()?;
    let device = &model.device;

    if let Some(prompt) = args.prompt{
        let tokenizer = tokenizer
            .with_padding(None)
            .with_truncation(None)
            .map_err(E::msg)?;
        let tokens = tokenizer
            .encode(prompt, true)
            .map_err(E::msg)?
            .get_ids()
            .to_vec();
        let token_ids = Tensor::new(&tokens[..], &Device::Cpu)?.unsqueeze(0)?;
        let token_type_ids = token_ids.zeros_like()?;
        println!("Loaded and encode {:?}", start.elapsed());

        for idx in 0..args.n {
            let start = std::time::Instant::now();
            let ys = model.forward(&token_ids, &token_type_ids, None)?;
            if idx == 0 {
                println!("{ys}");
            }
            println!("Took {:?}", start.elapsed());
        }
    } else {
        println!("You need a prompt");
    }
    Ok(())
}
