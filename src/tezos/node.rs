use async_trait::async_trait;
use tezos_api::apis::{configuration::Configuration, default_api::context_big_maps_big_map_id_script_expr_get};
use anyhow::Result;

pub struct TezosNode(Configuration);

#[async_trait]
impl TezosNode {
    pub fn new(url: &str) -> Self {
        let mut cfg = Configuration::new();
        cfg.base_path = url.to_owned();
        cfg
    }

    async fn get_big_map_entry(big_map_id: &str, key_scriptexpr: &str) -> Result<String> {
        contect_big_maps_big_map_id_script_expr_get(&self.0, big_map_id, key_scriptexpr).await
    }
}
