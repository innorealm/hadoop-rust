use clap::{Parser, Subcommand};
use hadoop_common::{
    conf::Configuration,
    fs::{common_configuration_keys_public, FileSystem, Path},
};
use hadoop_hdfs_client::fs::file_system;
use iref::{Iri, IriRefBuf};
use std::str::FromStr;

#[derive(Parser)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(subcommand)]
    Dfs(DfsCommands),
}

#[derive(Subcommand)]
enum DfsCommands {
    #[command(name = "-mkdir")]
    Mkdir {
        #[arg(short, default_value_t = false)]
        p: bool,
        paths: Vec<String>,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let fs = get_file_system()?;
    match &cli.command {
        Commands::Dfs(dfs_command) => match dfs_command {
            DfsCommands::Mkdir { p: _, paths } => {
                paths.iter().for_each(|path| _ = mkdir(&fs, path));
            }
        },
    }
    Ok(())
}

fn get_file_system() -> anyhow::Result<impl FileSystem> {
    let conf = Configuration {};
    let uri = Iri::new(conf.get_trimmed_with_default(
        common_configuration_keys_public::FS_DEFAULT_NAME_KEY,
        "hdfs://localhost:9000",
    ))?;
    file_system::get(&uri, &conf)
}

fn mkdir(fs: &impl FileSystem, path: &str) -> anyhow::Result<bool> {
    fs.mkdirs(&Path::from(IriRefBuf::from_str(path)?), None)
}
