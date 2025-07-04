use std::{error::Error, time::Duration};

use log::info;
use yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient, Interceptor};

const CONNECT_TIMEOUT: u64 = 10;
const KEEP_ALIVE_TIMEOUT: u64 = 60;

pub struct YellowstoneGrpc {
    endpoint: String,
    #[allow(dead_code)]
    x_token: Option<String>,
}

impl YellowstoneGrpc {
    pub fn new(endpoint: String, x_token: Option<String>) -> Self {
        Self { endpoint, x_token }
    }

    pub async fn build_client(
        self,
    ) -> Result<GeyserGrpcClient<impl Interceptor>, Box<dyn std::error::Error>> {
        let client = GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .connect_timeout(Duration::from_secs(CONNECT_TIMEOUT))
            .keep_alive_while_idle(true)
            .timeout(Duration::from_secs(KEEP_ALIVE_TIMEOUT))
            .connect()
            .await?;
        Ok(client)
    }

    #[allow(unused)]
    pub async fn subscribe(&self, program_id: String) -> Result<(), Box<dyn Error>> {
        info!("subscribe!");

        Ok(())
    }
}
