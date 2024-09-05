use crate::{genesis_builder, parse_json};
use diem_genesis::config::{HostAndPort, ValidatorConfiguration};
use lotus_config::validator_config;
use lotus_types::{core_types::fixtures::TestPersona, exports::NamedChain};
use std::{fs, net::Ipv4Addr, path::PathBuf, thread, time};
use std::fmt::Display;

// Sets up the environment for the given test persona.
pub async fn setup(
    me: &TestPersona,
    ip_list: &[Ipv4Addr],
    chain: NamedChain,
    data_path: PathBuf,
    legacy_data_path: Option<PathBuf>,
    keep_legacy_address: &Option<Vec<TestPersona>>,
) -> anyhow::Result<()> {
    let db_path = data_path.join("data");
    if db_path.exists() {
        println!("WARN: deleting {}, in 5 secs", db_path.display());
        let delay = time::Duration::from_secs(5);
        thread::sleep(delay);
        fs::remove_dir_all(db_path)?;
    }

    // create the local files for my_persona
    let index = me.idx();
    let format_host_str = format!(
        "{}:6180",
        ip_list.get(index).expect("could not get an IP and index")
    );
    println!(
        "your persona {me:?} is expected to use IP: {}",
        format_host_str
    );
    let my_host: HostAndPort = format_host_str
        .parse()
        .expect("could not parse IP address for host");

    // let mut keep_legacy_addr = false;
    // if let Some(keep_legacy_addresses) = keep_legacy_address {
    //     keep_legacy_addr = keep_legacy_addresses.contains(me);
    // }

    // Initializes the validator configuration.
    validator_config::initialize_validator(
        Some(data_path.clone()),
        Some(&me.to_string()),
        my_host,
        Some(me.get_persona_mnem()),
        keep_legacy_address
            .as_ref().map_or(false, |addresses| addresses.contains(me)),
        Some(chain),
    )
    .await?;

    // create validator configurations from fixtures
    // without needing to use a github repo to register and read
    let val_cfg: Vec<ValidatorConfiguration> = ip_list
        .iter()
        .enumerate()
        .filter_map(|(idx, ip)| {
            let format_host_str = format!("{}:6180", ip);
            let host: HostAndPort = format_host_str
                .parse()
                .expect("could not parse IP address for host");
            let p = TestPersona::from(idx).ok()?;

            // Only keep legacy address for the keep_legacy addresses.
            genesis_builder::testnet_validator_config(&p, &host, keep_legacy_address.as_ref().map_or(false, |addresses| addresses.contains(&p))).ok()
        })
        .collect();

    // Determines the path for the recovery data.
    let p = legacy_data_path.unwrap_or(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/sample_export_recovery.json"),
    );

    let mut recovery = parse_json::recovery_file_parse(p)?;

    // Builds the genesis block with the specified configurations.
    genesis_builder::build(
        "none".to_string(), // when is testnet is ignored
        "none".to_string(),
        "none".to_string(),
        data_path,
        true,
        &mut recovery,
        chain,
        Some(val_cfg),
    )?;
    Ok(())
}
