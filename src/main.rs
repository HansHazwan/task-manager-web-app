mod error;
mod prelude;
mod config;

use crate::prelude::*;
use crate::config::*;

#[tokio::main]
async fn main() -> Result<()> {
    init_configuration();

    let ip_address = get_ip_address()?;

    log::info!("IP Address: {:?}", ip_address);

    Ok(())
}
