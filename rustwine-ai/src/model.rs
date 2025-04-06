use anyhow::Result;
use tch::{Device, Tensor, nn};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ModelConfig {
    input_size: i64,
    hidden_size: i64,
    output_size: i64,
    learning_rate: f64,
}

pub struct Model {
    vs: nn::VarStore,
    net: nn::Sequential,
    config: ModelConfig,
}

impl Model {
    pub fn new(config: ModelConfig) -> Result<Self> {
        let vs = nn::VarStore::new(Device::Cpu);
        let net = Self::build_network(&vs.root(), &config);
        Ok(Self { vs, net, config })
    }

    fn build_network<'a>(vs: &nn::Path<'a>, config: &ModelConfig) -> nn::Sequential {
        nn::seq()
            .add(nn::linear(vs, config.input_size, config.hidden_size, Default::default()))
            .add_fn(|xs| xs.relu())
            .add(nn::linear(vs, config.hidden_size, config.output_size, Default::default()))
    }

    pub fn train(&mut self, inputs: &Tensor, targets: &Tensor) -> Result<()> {
        let opt = nn::Adam::default()
            .build(&self.vs, self.config.learning_rate)?;
        
        let output = self.net.forward(&inputs);
        let loss = output.mse_loss(&targets, tch::Reduction::Mean);
        
        opt.backward_step(&loss);
        Ok(())
    }

    pub fn predict(&self, inputs: &Tensor) -> Result<Tensor> {
        Ok(self.net.forward(&inputs))
    }

    pub fn save(&self, path: &str) -> Result<()> {
        self.vs.save(path)?;
        Ok(())
    }

    pub fn load(&mut self, path: &str) -> Result<()> {
        self.vs.load(path)?;
        Ok(())
    }
} 