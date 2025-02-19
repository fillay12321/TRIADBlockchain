use crate::block::Block;

pub trait ConsensusPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn validate(&self, block: &Block) -> bool;
}

pub struct PluginManager {
    plugins: Vec<Box<dyn ConsensusPlugin>>,
}

impl PluginManager {
    pub fn new() -> Self {
        PluginManager { plugins: Vec::new() }
    }

    pub fn register_plugin(&mut self, plugin: Box<dyn ConsensusPlugin>) {
        self.plugins.push(plugin);
    }

    pub fn validate_block(&self, block: &Block) -> bool {
        self.plugins.iter().all(|plugin| {
            println!("Plugin {} validating block {}", plugin.name(), block.index);
            plugin.validate(block)
        })
    }
}
