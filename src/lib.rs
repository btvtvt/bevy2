pub mod demo_fetch_api;
pub mod demo_heavy_compute;
pub mod demo_my_plugin;

use std::time::Duration;

// SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 5000);

pub const DELTA_TIME: Duration = Duration::from_millis(16);
