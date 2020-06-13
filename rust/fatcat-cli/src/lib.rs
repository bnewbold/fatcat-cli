
use std::io::Read;
use std::path::PathBuf;
use std::io::BufRead;
use tabwriter::TabWriter;
use chrono_humanize::HumanTime;
use anyhow::{Result, anyhow, Context};
use std::io::Write;
use atty;
use termcolor::{ColorChoice, StandardStream, Color, ColorSpec, WriteColor};
use data_encoding::BASE64;
use macaroon::{Macaroon, Verifier};
use fatcat_openapi;
use fatcat_openapi::models;
#[allow(unused_imports)]
use log::{self,info,debug};
use std::str::FromStr;

mod entities;
mod search;
mod specifier;
mod api;

pub use entities::{ApiEntityModel,ApiModelSer,ApiModelIdent,Mutation};
pub use specifier::Specifier;
pub use api::FatcatApiClient;
pub use search::crude_search;

// Want to show:
// - whether api_token found
// - configured api_host we are connecting to
// - whether we can connect to remote host (eg, get most recent changelog)
// - whether our auth is valid
// - current active editgroup
#[derive(Debug, PartialEq, Clone, serde::Serialize)]
pub struct ClientStatus {
    pub has_api_token: bool,
    pub api_host: String,
    pub last_changelog: Option<i64>,
    pub account: Option<models::Editor>,
}

impl ClientStatus {

    pub fn pretty_print(self) -> Result<()> {

        let mut color_stdout = StandardStream::stdout(
            if atty::is(atty::Stream::Stdout) {
                ColorChoice::Auto
            } else {
                ColorChoice::Never
            }
        );
        let color_normal = ColorSpec::new();
        let mut color_bold = ColorSpec::new();
        color_bold.set_bold(true);
        let mut color_happy = ColorSpec::new();
        color_happy.set_fg(Some(Color::Green)).set_bold(true);
        let mut color_sad = ColorSpec::new();
        color_sad.set_fg(Some(Color::Red)).set_bold(true);

        color_stdout.set_color(&color_normal)?;
        write!(&mut color_stdout, "{:>16}: ", "API host")?;
        color_stdout.set_color(&color_bold)?;
        write!(&mut color_stdout, "{}", self.api_host)?;
        match self.last_changelog {
            Some(index) => {
                color_stdout.set_color(&color_happy)?;
                writeln!(&mut color_stdout, " [successfully connected]")?;
                color_stdout.set_color(&color_normal)?;
                write!(&mut color_stdout, "{:>16}: ", "Last changelog")?;
                color_stdout.set_color(&color_bold)?;
                writeln!(&mut color_stdout, "{}", index)?;
            },
            None => {
                color_stdout.set_color(&color_sad)?;
                writeln!(&mut color_stdout, " [Failed to connect]")?;
            }
        };
        color_stdout.set_color(&color_normal)?;
        write!(&mut color_stdout, "{:>16}: ", "API auth token")?;
        match self.has_api_token {
            true => {
                color_stdout.set_color(&color_happy)?;
                writeln!(&mut color_stdout, "[configured]")?;
            },
            false => {
                color_stdout.set_color(&color_sad)?;
                writeln!(&mut color_stdout, "[not configured]")?;
            },
        };
        if let Some(editor) = self.account {
            color_stdout.set_color(&color_normal)?;
            write!(&mut color_stdout, "{:>16}: ", "Account")?;
            color_stdout.set_color(&color_bold)?;
            write!(&mut color_stdout, "{}", editor.username)?;
            if editor.is_bot == Some(true) {
                color_stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)).set_bold(true))?;
                write!(&mut color_stdout, " [bot]")?;
            }
            if editor.is_admin == Some(true) {
                color_stdout.set_color(ColorSpec::new().set_fg(Some(Color::Magenta)).set_bold(true))?;
                write!(&mut color_stdout, " [admin]")?;
            }
            match editor.is_active {
                Some(true) => {
                    color_stdout.set_color(&color_happy)?;
                    writeln!(&mut color_stdout, " [active]")?;
                },
                Some(false) | None => {
                    color_stdout.set_color(&color_sad)?;
                    writeln!(&mut color_stdout, " [disabled]")?;
                },
            };
            color_stdout.set_color(&color_normal)?;
            writeln!(&mut color_stdout, "{:>16}  editor_{}", "", editor.editor_id.unwrap())?;
        };
        color_stdout.set_color(&color_normal)?;
        Ok(())
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum EntityType {
    Release,
    Work,
    Container,
    Creator,
    File,
    FileSet,
    WebCapture,
}

impl FromStr for EntityType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "release" => Ok(EntityType::Release),
            "work" => Ok(EntityType::Work),
            "container" => Ok(EntityType::Container),
            "creator" => Ok(EntityType::Creator),
            "file" => Ok(EntityType::File),
            "fileset" => Ok(EntityType::FileSet),
            "webcapture" => Ok(EntityType::WebCapture),
            _ => Err(anyhow!("invalid entity type : {}", s)),
        }
    }
}

/// Takes a macaroon token (as base64-encoded string) and tries to parse out an editor id
pub fn parse_macaroon_editor_id(s: &str) -> Result<String> {
    let raw = BASE64.decode(s.as_bytes()).context("macaroon parsing failed")?;
    let mac = Macaroon::deserialize(&raw).map_err(|err| anyhow!("macaroon deserialization failed: {:?}", err))?;
    let mac = mac.validate().map_err(|err| anyhow!("macaroon validation failed: {:?}", err))?;
    let mut verifier = Verifier::new();
    let mut editor_id: Option<String> = None;
    for caveat in mac.first_party_caveats() {
        if caveat.predicate().starts_with("editor_id = ") {
            editor_id = Some(caveat.predicate().get(12..).context("parsing macaroon")?.to_string());
            break;
        }
    }
    let editor_id = match editor_id {
        Some(id) => id,
        None => Err(anyhow!("expected an editor_id caveat in macaroon token"))?,
    };
    verifier.satisfy_exact(&format!("editor_id = {}", editor_id.to_string()));
    Ok(editor_id)
}

pub fn print_editgroups(eg_list: Vec<models::Editgroup>, json: bool) -> Result<()> {
    if json {
        for eg in eg_list {
            writeln!(&mut std::io::stdout(), "{}", eg.to_json_string()?)?;
        }
    } else {
        let mut tw = TabWriter::new(std::io::stdout());
        writeln!(tw, "editgroup_id\tchangelog_index\tcreated\tsubmitted\tdescription")?;
        for eg in eg_list {
            writeln!(tw, "{}\t{}\t{}\t{}\t{}",
                eg.editgroup_id.unwrap(),
                eg.changelog_index.map_or("-".to_string(), |v| v.to_string()),
                eg.created.map_or("-".to_string(), |v| HumanTime::from(v).to_string()),
                eg.submitted.map_or("-".to_string(), |v| HumanTime::from(v).to_string()),
                eg.description.unwrap_or("-".to_string()))?;
        }
        tw.flush()?;
    }
    Ok(())
}

pub fn read_entity_file(input_path: Option<PathBuf>) -> Result<String> {
    // treat "-" as "use stdin"
    let input_path = match input_path {
        Some(s) if s.to_string_lossy() == "-" => None,
        _ => input_path,
    };
    match input_path {
        None => {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line)?;
            Ok(line)
        },
        Some(path) if path.extension().map(|v| v.to_str()) == Some(Some("toml")) => {
            info!("reading {:?} as TOML", path);
            // as a hack, read TOML but then serialize it back to JSON
            let mut contents = String::new();
            let mut input_file = std::fs::File::open(path).context("reading entity from TOML file")?;
            input_file.read_to_string(&mut contents)?;
            let value: toml::Value = contents.parse().context("parsing TOML file")?;
            Ok(serde_json::to_string(&value)?)
        },
        Some(path) => {
            let mut line = String::new();
            let input_file = std::fs::File::open(path)?;
            let mut buffered = std::io::BufReader::new(input_file);
            buffered.read_line(&mut line)?;
            Ok(line)
        },
    }
}
