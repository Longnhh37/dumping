mod commit;
mod diff;
mod index;
mod network;
mod object;
mod repo;
mod status;

use anyhow::Context;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "gitr")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Init {
        repo: String,
    },

    Add {
        paths: Vec<String>,
    },

    CatFile {
        mode: String,
        hash_prefix: String,
    },

    Commit {
        #[arg(short = 'm', long)]
        message: String,
        #[arg(short = 'a', long)]
        author: Option<String>,
    },

    Diff,

    HashObject {
        path: String,
        #[arg(short = 't', default_value = "blob")]
        obj_type: String,
        #[arg(short = 'w')]
        write: bool,
    },

    LsFiles {
        #[arg(short = 's', long)]
        stage: bool,
    },

    Push {
        git_url: String,
        #[arg(short = 'u', long)]
        username: Option<String>,
        #[arg(short = 'p', long)]
        password: Option<String>,
    },

    Status,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    use Command::*;
    match cli.command {
        Init { repo } => repo::init(&repo)?,
        Add { paths } => index::add(&paths)?,
        CatFile { mode, hash_prefix } => object::cat_file(&mode, &hash_prefix)?,
        Commit { message, author } => { commit::commit(&message, author)?; },
        Diff => diff::diff()?,
        HashObject {
            path,
            obj_type,
            write,
        } => {
            let data =
                std::fs::read(&path).with_context(|| format!("failed to read file '{}'", path))?;
            let ty = object::ObjectType::from_str(&obj_type)?;
            let sha1 = object::hash_object(&data, ty, write)?;
            println!("{}", sha1);
        }
        LsFiles { stage } => index::ls_files(stage)?,
        Push { git_url, username, password } => { network::push(&git_url, username, password)?; },
        Status => status::status()?,
    }

    Ok(())
}
