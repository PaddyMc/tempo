use clap::Parser;
use reth_chainspec::ChainSpec;
use reth_cli::chainspec::ChainSpecParser;
use reth_cli_commands::node::NoArgs;
use std::sync::Arc;

/// Malachite chain spec parser
#[derive(Debug, Clone, Default)]
pub struct MalachiteChainSpecParser;

impl ChainSpecParser for MalachiteChainSpecParser {
    type ChainSpec = ChainSpec;

    const SUPPORTED_CHAINS: &'static [&'static str] = &["malachite"];

    fn parse(s: &str) -> eyre::Result<Arc<Self::ChainSpec>> {
        // For now, return a basic chain spec - this should be implemented properly
        match s {
            "malachite" => Ok(Arc::new(custom_malachite_chain())),
            _ => Err(eyre::eyre!("Unknown chain: {}", s)),
        }
    }
}

/// The main Malachite CLI interface
#[derive(Debug, Parser)]
#[command(author, version, about = "Malachite Node", long_about = None)]
pub struct Cli<
    Spec: ChainSpecParser = MalachiteChainSpecParser,
    Ext: clap::Args + std::fmt::Debug = NoArgs,
> {
    /// The command to run
    #[command(subcommand)]
    pub command: reth::cli::Commands<Spec, Ext>,

    #[command(flatten)]
    logs: reth::args::LogArgs,
}

impl<C, Ext> Cli<C, Ext>
where
    C: ChainSpecParser<ChainSpec = ChainSpec>,
    Ext: clap::Args + std::fmt::Debug,
{
    /// Execute the configured CLI command
    pub fn run<L, Fut>(self, launcher: L) -> eyre::Result<()>
    where
        L: FnOnce(
            reth::builder::WithLaunchContext<
                reth::builder::NodeBuilder<std::sync::Arc<reth_db::DatabaseEnv>, C::ChainSpec>,
            >,
            Ext,
        ) -> Fut,
        Fut: std::future::Future<Output = eyre::Result<()>>,
    {
        use reth::CliRunner;
        self.with_runner(CliRunner::try_default_runtime()?, launcher)
    }

    /// Execute the configured CLI command with the provided CliRunner
    pub fn with_runner<L, Fut>(mut self, runner: reth::CliRunner, launcher: L) -> eyre::Result<()>
    where
        L: FnOnce(
            reth::builder::WithLaunchContext<
                reth::builder::NodeBuilder<std::sync::Arc<reth_db::DatabaseEnv>, C::ChainSpec>,
            >,
            Ext,
        ) -> Fut,
        Fut: std::future::Future<Output = eyre::Result<()>>,
    {
        // Add network name if available to the logs dir
        if let Some(chain_spec) = self.command.chain_spec() {
            self.logs.log_file_directory = self
                .logs
                .log_file_directory
                .join(chain_spec.chain().to_string());
        }

        let _guard = self.init_tracing()?;
        tracing::info!(target: "malachite::cli", "Initialized tracing, debug log directory: {}", self.logs.log_file_directory);

        // Install the prometheus recorder
        let _ = reth::prometheus_exporter::install_prometheus_recorder();

        match self.command {
            reth::cli::Commands::Node(command) => runner.run_command_until_exit(|ctx| {
                command.execute(
                    ctx,
                    reth_cli_commands::launcher::FnLauncher::new::<C, Ext>(launcher),
                )
            }),
            _ => todo!("Other commands not implemented yet"),
        }
    }

    /// Initializes tracing with the configured options
    pub fn init_tracing(&self) -> eyre::Result<Option<reth_tracing::FileWorkerGuard>> {
        let guard = self.logs.init_tracing()?;
        Ok(guard)
    }
}

// Temporary custom chain implementation - should be replaced with proper chain spec
fn custom_malachite_chain() -> ChainSpec {
    use alloy_genesis::{Genesis, GenesisAccount};
    use alloy_primitives::{Address, B256, Bytes, U256};
    use reth_chainspec::{Chain, ChainSpecBuilder};
    use std::collections::BTreeMap;

    // Create a basic genesis block
    let genesis = Genesis {
        config: Default::default(),
        nonce: 0x42,
        timestamp: 0x0,
        extra_data: Bytes::from_static(b"SC"),
        gas_limit: 0xa388,
        difficulty: U256::from(0x400000000_u64),
        mix_hash: B256::ZERO,
        coinbase: Address::ZERO,
        number: Some(0),
        alloc: {
            let mut alloc = BTreeMap::new();
            alloc.insert(
                "0x6Be02d1d3665660d22FF9624b7BE0551ee1Ac91b"
                    .parse()
                    .unwrap(),
                GenesisAccount {
                    balance: U256::from_str_radix("4a47e3c12448f4ad000000", 16).unwrap(),
                    ..Default::default()
                },
            );
            alloc
        },
        ..Default::default()
    };

    ChainSpecBuilder::default()
        .chain(Chain::from_id(2600))
        .genesis(genesis)
        .paris_activated()
        .build()
}
