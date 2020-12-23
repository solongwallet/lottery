use console::Emoji;
use clap::{
    crate_description, 
    crate_name, 
    crate_version, 
    value_t_or_exit, 
    App, 
    AppSettings, 
    Arg,
    ArgMatches, 
    SubCommand,
};

use solana_clap_utils::{
    fee_payer::fee_payer_arg,
    input_parsers::{
        pubkey_of_signer, 
        pubkeys_of_multiple_signers, 
        signer_of, 
        value_of
    },
    input_validators::{
        is_amount, 
        is_parsable, 
        is_url, 
        is_valid_pubkey, 
        is_valid_signer
    },
    keypair::{
        pubkey_from_path, 
        signer_from_path,
        DefaultSigner
    },
    nonce::*,
    offline::{
        self, 
        *
    },
    ArgConstant,
};
use std::fmt::Display;
use std::str::FromStr;


static WARNING: Emoji = Emoji("⚠️", "!");

pub fn is_fund<T>(found: T) -> Result<(), String>
where
    T: AsRef<str> + Display,
{
    if found.as_ref().parse::<u64>().is_ok() {
        Ok(())
    } else {
        Err(format!(
            "Unable to parse input fundamental as integer , provided: {}",
            found
        ))
    }
}

fn main() {
    let app_matches = App::new(crate_name!())
    .about(crate_description!())
    .version(crate_version!())
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .arg(
        Arg::with_name("verbose")
            .long("verbose")
            .short("v")
            .takes_value(false)
            .global(true)
            .help("Show additional information"),
    )
    .arg(
        Arg::with_name("json_rpc_url")
            .long("url")
            .value_name("URL")
            .takes_value(true)
            .global(true)
            .validator(is_url)
            .help("JSON RPC URL for the cluster.  Default from the configuration file."),
    )
    .arg({
        let arg = Arg::with_name("config_file")
            .short("C")
            .long("config")
            .value_name("PATH")
            .takes_value(true)
            .global(true)
            .help("Configuration file to use");
        if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
            arg.default_value(&config_file)
        } else {
            arg
        }
    })
    .subcommand(SubCommand::with_name("initialize").about("Initialize lottery")
        .arg(
            Arg::with_name("fund")
                .long("fundamental")
                .validator(is_fund)
                .value_name("FUND")
                .takes_value(true)
                .help("fundaental for pool"),
        ),
    )
    .get_matches();
        
    let (sub_command, sub_matches) = app_matches.subcommand();
    let matches = sub_matches.unwrap();
    let verbose = matches.is_present("verbose");
    println!("verbose:{}", verbose);

    let _ = match (sub_command, sub_matches) {
        _ => println!("default subcommand"),
    };


    println!("{} Hello, world!",WARNING);
}
