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

static WARNING: Emoji = Emoji("⚠️", "!");


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
    .subcommand(SubCommand::with_name("initialize").about("Initialize lottery")
            .arg(
                Arg::with_name("config")
                    .long("config")
                    .validator(is_mint_decimals)
                    .value_name("DECIMALS")
                    .takes_value(true)
                    .help("Account address for config file"),
            )


    println!("{} Hello, world!",WARNING);
}
