
use std::path::PathBuf;
use fatcat_cli::ApiModelSer;
use atty;
use std::io::Write;
use termcolor::{ColorChoice, StandardStream, Color, ColorSpec, WriteColor};
use anyhow::{Result, Context, anyhow};
#[allow(unused_imports)]
use log::{self,info,debug};
use structopt::StructOpt;
use fatcat_cli::*;
use fatcat_openapi::{client, models, ApiNoContext};


#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case", about = "CLI interface to Fatcat API" )]
struct Opt {

    #[structopt(long = "--api-host", env = "FATCAT_API_HOST", default_value = "https://api.fatcat.wiki")]
    api_host: String,

    #[structopt(long = "--api-token", env = "FATCAT_API_AUTH_TOKEN", hide_env_values = true)]
    api_token: Option<String>,

    #[structopt(long = "--search-host", env = "FATCAT_SEARCH_HOST", default_value = "https://search.fatcat.wiki")]
    search_host: String,

    /// Pass many times for more log output
    ///
    /// By default, it'll only report errors. Passing `-v` one time also prints
    /// warnings, `-vv` enables info logging, `-vvv` debug, and `-vvvv` trace.
    #[structopt(long, short = "v", parse(from_occurrences))]
    verbose: i8,

    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt)]
enum EditgroupCommand {
    Create {
        #[structopt(long, short)]
        description: String,
    },
    List {
        #[structopt(long = "--editor-id", short)]
        editor_id: Option<String>,

        #[structopt(long, short, default_value = "20")]
        limit: i64,

        #[structopt(long)]
        json: bool,
    },
    Reviewable {
        #[structopt(long, short, default_value = "20")]
        limit: i64,

        #[structopt(long)]
        json: bool,
    },
    Accept {
        #[structopt(env = "FATCAT_EDITGROUP", hide_env_values = true)]
        editgroup_id: String,
    },
    Submit {
        #[structopt(env = "FATCAT_EDITGROUP", hide_env_values = true)]
        editgroup_id: String,
    },
    Unsubmit {
        #[structopt(env = "FATCAT_EDITGROUP", hide_env_values = true)]
        editgroup_id: String,
    },
}

#[derive(StructOpt)]
enum Command {
    Status {
        #[structopt(long)]
        json: bool,
    },
    Get {
        specifier: Specifier,

        #[structopt(long)]
        toml: bool,
    },
    Create {
        entity_type: EntityType,

        /// Input file, "-" for stdin.
        #[structopt(long = "--file", short = "-f", parse(from_os_str))]
        input_path: Option<PathBuf>,

        #[structopt(long = "--editgroup-id", short, env = "FATCAT_EDITGROUP", hide_env_values = true)]
        editgroup_id: String,
    },
    Update {
        specifier: Specifier,

        /// Input file, "-" for stdin.
        #[structopt(long = "--file", short = "-f", parse(from_os_str))]
        input_path: Option<PathBuf>,

        #[structopt(long = "--editgroup-id", short, env = "FATCAT_EDITGROUP", hide_env_values = true)]
        editgroup_id: String,

        mutations: Vec<Mutation>,
    },
    Edit {
        specifier: Specifier,

        #[structopt(long = "--editgroup-id", short, env = "FATCAT_EDITGROUP", hide_env_values = true)]
        editgroup_id: String,

        #[structopt(long)]
        json: bool,

        #[structopt(long = "--editing-command", env = "EDITOR")]
        editing_command: String,
    },
    Delete {
        specifier: Specifier,

        #[structopt(long = "--editgroup-id", short, env = "FATCAT_EDITGROUP", hide_env_values = true)]
        editgroup_id: String,
    },
    Editgroup {
        #[structopt(subcommand)]
        cmd: EditgroupCommand,
    },
    //Changelog
    //Download
    //History
    Search {

        entity_type: EntityType,

        terms: Vec<String>,

        #[structopt(long, short, default_value = "20")]
        limit: i64,

        #[structopt(long = "--search-schema")]
        search_schema: bool,
    },
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let log_level = match opt.verbose {
        std::i8::MIN..=-1 => "none",
        0 => "error",
        1 => "warn",
        2 => "info",
        3 => "debug",
        4..=std::i8::MAX => "trace",
    };
    // hyper logging is very verbose, so crank that down even if everything else is more verbose
    let log_filter = format!("{},hyper=error", log_level);
    env_logger::from_env(env_logger::Env::default().default_filter_or(log_filter))
        .format_timestamp(None)
        .init();

    debug!("Args parsed, starting up");

    if let Err(err) = run(opt) {
        // Be graceful about some errors
        if let Some(io_err) = err.root_cause().downcast_ref::<std::io::Error>() {
            match io_err.kind() {
                std::io::ErrorKind::BrokenPipe => {
                    // presumably due to something like writing to stdout and piped to `head -n10` and
                    // stdout was closed
                    debug!("got BrokenPipe error, assuming stdout closed as expected and exiting with success");
                    std::process::exit(0);
                },
                _ => (),
            }
        }
        let mut color_stderr = StandardStream::stderr(
            if atty::is(atty::Stream::Stderr) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        );
        color_stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red)).set_bold(true))?;
        eprintln!("Error: {:?}", err);
        color_stderr.set_color(&ColorSpec::new())?;
        std::process::exit(1);
    }
    Ok(())
}

fn run(opt: Opt) -> Result<()> {
    let client = if opt.api_host.starts_with("https://") {
        // Using Simple HTTPS
        client::Client::try_new_https(&opt.api_host).context("Failed to create HTTPS client")?
    } else if opt.api_host.starts_with("http://") {
        // Using HTTP
        client::Client::try_new_http(&opt.api_host).context("Failed to create HTTP client")?
    } else {
        Err(anyhow!("unsupported API Host prefix: {}", opt.api_host))?
    };

    let mut api_client = FatcatApiClient::new(&client, opt.api_host.clone(), opt.api_token.clone())?;

    match opt.cmd {
        Command::Get {toml, specifier} => {
            let result = specifier.get_from_api(&mut api_client)?;
            if toml {
                writeln!(&mut std::io::stdout(), "{}", result.to_toml_string()?)?
            } else {
                writeln!(&mut std::io::stdout(), "{}", result.to_json_string()?)?
            }
        },
        Command::Create { entity_type, input_path, editgroup_id } => {
            let json_str = read_entity_file(input_path)?;
            let ee = api_client.create_entity_from_json(entity_type, &json_str, editgroup_id)?;
            println!("{}", serde_json::to_string(&ee)?);
        },
        Command::Update { specifier, input_path, editgroup_id, mutations } => {
            let (json_str, exact_specifier): (String, Specifier) = match (&input_path, mutations.len()) {
                // input path or no mutations: read from path or stdin
                (Some(_), _) | (None, 0) => {
                    (read_entity_file(input_path)?, specifier.into_entity_specifier(&mut api_client)?)
                },
                // no input path *and* mutations: fetch from API
                (None, _) => {
                    let mut entity = specifier.get_from_api(&mut api_client)?;
                    entity.mutate(mutations)?;
                    (entity.to_json_string()?, entity.specifier())
                },
            };
            let ee = api_client.update_entity_from_json(exact_specifier, &json_str, editgroup_id)?;
            println!("{}", serde_json::to_string(&ee)?);
        },
        Command::Edit { specifier, editgroup_id, json, editing_command } => {
            // TODO: fetch editgroup, check if this entity is already being updated in it. If so,
            // need to fetch that revision, do the edit, parse that synatx is good, then delete the
            // existing edit and update with the new one.
            let original_entity = specifier.get_from_api(&mut api_client)?;
            let exact_specifier = original_entity.specifier();
            let tmp_file = tempfile::Builder::new()
                .suffix( if json { ".json" } else { ".toml"} )
                .tempfile()?;
            if json {
                writeln!(&tmp_file, "{}", original_entity.to_json_string()?)?
            } else {
                writeln!(&tmp_file, "{}", original_entity.to_toml_string()?)?
            }
            let mut editor_cmd = std::process::Command::new(&editing_command)
                .arg(tmp_file.path())
                .spawn()
                .expect("failed to execute process");
            let cmd_status = editor_cmd.wait()?;
            if !cmd_status.success() {
                Err(anyhow!("editor ({}) exited with non-success status code ({}), bailing on edit", editing_command, cmd_status.code().map(|v| v.to_string()).unwrap_or("N/A".to_string())))?;
            };
            let json_str = read_entity_file(Some(tmp_file.path().to_path_buf()))?;
            // for whatever reason api_client's TCP connection is broken after spawning, so try a
            // dummy call, expected to fail, but connection should re-establish after this
            specifier.get_from_api(&mut api_client).context("re-fetch").ok();
            let ee = api_client.update_entity_from_json(exact_specifier, &json_str, editgroup_id).context("updating after edit")?;
            println!("{}", serde_json::to_string(&ee)?);
        },
        Command::Search { entity_type, terms, limit, search_schema } => {
            let limit: Option<u64> = match limit {
                l if l < 0 => None,
                l => Some(l as u64),
            };
            let results = fatcat_cli::crude_search(&opt.search_host, entity_type, limit, terms)
                .context(format!("searching for {:?}", entity_type))?;
            eprintln!("Got {} hits in {}ms", results.count, results.took_ms);
            for hit in results {
                let hit = hit?;
                match (search_schema, entity_type) {
                    (true, _) => writeln!(&mut std::io::stdout(), "{}", hit.to_string())?,
                    (false, EntityType::Release) => {
                        let specifier = Specifier::Release(hit["ident"].as_str().unwrap().to_string());
                        let entity = specifier.get_from_api(&mut api_client)?;
                        writeln!(&mut std::io::stdout(), "{}", entity.to_json_string()?)?
                    },
                    (false, _) => unimplemented!("searching other entity types"),
                }
            }
        },
        Command::Delete { specifier, editgroup_id } => {
            let result = api_client.delete_entity(specifier.clone(), editgroup_id)
                .context(format!("delete entity: {:?}", specifier.clone()))?;
            println!("{}", serde_json::to_string(&result)?);
        },
        Command::Editgroup { cmd: EditgroupCommand::List { editor_id, limit, json } } => {
            let editor_id = match editor_id.or(api_client.editor_id) {
                Some(eid) => eid,
                None => Err(anyhow!("require either working auth token or --editor-id"))?,
            };
            let result = api_client.rt.block_on(
                api_client.api.get_editor_editgroups(editor_id.clone(), Some(limit), None, None)
            ).context("fetch editgroups")?;
            match result {
                fatcat_openapi::GetEditorEditgroupsResponse::Found(eg_list) => {
                    print_editgroups(eg_list, json)?;
                },
                other => Err(anyhow!("{:?}", other)).context(format!("failed to fetch editgroups for editor_{}", editor_id))?,
            }
        },
        Command::Editgroup { cmd: EditgroupCommand::Reviewable { limit, json } } => {
            let result = api_client.rt.block_on(
                api_client.api.get_editgroups_reviewable(Some("editors".to_string()), Some(limit), None, None)
            ).context("fetch reviewable editgroups")?;
            match result {
                fatcat_openapi::GetEditgroupsReviewableResponse::Found(eg_list) => {
                    print_editgroups(eg_list, json)?;
                },
                other => Err(anyhow!("{:?}", other)).context(format!("failed to fetch reviewable editgroups"))?,
            }
        },
        Command::Editgroup { cmd: EditgroupCommand::Create { description }} => {
            let mut eg = models::Editgroup::new();
            eg.description = Some(description);
            eg.extra = Some({
                let mut extra = std::collections::HashMap::new();
                extra.insert("agent".to_string(), serde_json::Value::String("fatcat-cli".to_string()));
                extra
            });
            let result = api_client.rt.block_on(
                api_client.api.create_editgroup(eg))?;
            match result {
                fatcat_openapi::CreateEditgroupResponse::SuccessfullyCreated(eg) =>
                    println!("{}", serde_json::to_string(&eg)?),
                other => Err(anyhow!("{:?}", other)).context(format!("failed to create editgroup"))?,
            }
        },
        Command::Editgroup { cmd: EditgroupCommand::Accept { editgroup_id } } => {
            let result = api_client.rt.block_on(
                api_client.api.accept_editgroup(editgroup_id.clone())
            ).context("accept editgroup")?;
            match result {
                fatcat_openapi::AcceptEditgroupResponse::MergedSuccessfully(msg) =>
                    println!("{}", serde_json::to_string(&msg)?),
                other => Err(anyhow!("failed to accept editgroup {}: {:?}", editgroup_id, other))?,
            }
        },
        Command::Editgroup { cmd: EditgroupCommand::Submit{ editgroup_id } } => {
            let eg = api_client.update_editgroup_submit(editgroup_id, true)?;
            println!("{}", eg.to_json_string()?);
        },
        Command::Editgroup { cmd: EditgroupCommand::Unsubmit { editgroup_id } } => {
            let eg = api_client.update_editgroup_submit(editgroup_id, false)?;
            println!("{}", eg.to_json_string()?);
        },
        Command::Status { json } => {
            let status = api_client.status()?;
            if json {
                println!("{}", serde_json::to_string(&status)?)
            } else {
                status.pretty_print()?;
            }
        },
    }
    Ok(())
}
