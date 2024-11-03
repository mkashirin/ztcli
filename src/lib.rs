use core::panic;
use std::env;
use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::net::Ipv4Addr;
use std::ops::Deref;
use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{Ok, Result};
use central::types::{ApiToken, Member, Network};
use clap::{command, value_parser, Arg, ArgAction, ArgMatches, Command};
use one::types::{
    ControllerNetworkMember, ControllerNetworkMemberRequest,
    ControllerNetworkRequest, ControllerNetworkRequestDns,
    ControllerNetworkRequestIpAssignmentPoolsItem as IpAssignmentPool,
    ControllerNetworkRequestIpAssignmentPoolsItemIpRangeEnd as IpAssignmentPoolRangeEnd,
    ControllerNetworkRequestIpAssignmentPoolsItemIpRangeStart as IpAssignmentPoolRangeStart,
    ControllerNetworkRequestRoutesItem as Route,
    ControllerNetworkRequestV4AssignMode as V4AssignMode,
    ControllerNetworkRequestV6AssignMode as V6AssignMode, EmptyArrayItem, IPv4,
    IpSlashPort, JoinedNetworkRequest, ZtAddress, ZtNetworkId,
};
use reqwest::header;
use serde_json::{Map, Value};
use {zerotier_central as central, zerotier_one as one};

const CENTRAL_BASE_URL: &str = "https://api.zerotier.com/api/v1";
const ONE_BASE_URL: &str = "http://localhost:9993";

/// Entry point of the CLI. Initializes clients and delegates execution.
pub async fn cli() -> Result<()> {
    let matches = command!()
        .about("A mininal CLI combining ZeroTier Central and ZeroTier One Service essential functionality")
        .arg_required_else_help(true)
        .subcommands([central_cli(), one_cli()])
        .get_matches();
    match matches.subcommand() {
        Some(("central", subs)) => {
            let central_client = zerotier_central::Client::from_env();
            let handler = CentralCliHandler::new(&central_client);
            handler
                .handle(subs)
                .await
                .expect("Failed to handle Central CLI input");
        }
        Some(("one", subs)) => {
            let one_client = zerotier_one::Client::from_env();
            let handler = OneCliHandler::new(&one_client);
            handler
                .handle(subs)
                .await
                .expect("Failed to handle One CLI input");
        }
        _ => {}
    }
    Ok(())
}

/// Trait for creating instances from environment variables.
pub trait FromEnv<T> {
    fn from_env() -> Self;
}

/// Implementation for creating a ZeroTier Central client from environment
/// variables.
impl FromEnv<central::Client> for central::Client {
    fn from_env() -> Self {
        let token = "token ".to_string()
            + env::var("ZTC_API_TOKEN")
                .expect("Unable to obtain API token (ZTC_API_TOKEN is not set)")
                .as_str();
        let api_token = ApiToken {
            token: Some(token),
            token_name: None,
        };
        let mut headers = header::HeaderMap::new();
        let value = header::HeaderValue::from_str(&api_token.token.unwrap())
            .expect("Unacceptable API token");
        headers.insert("Authorization", value);

        let rqwst_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("No ZeroTier Central API token has been provided");

        central::Client::new_with_client(CENTRAL_BASE_URL, rqwst_client)
    }
}

/// Implementation for creating a ZeroTier One client from environment
/// variables.
impl FromEnv<one::Client> for one::Client {
    fn from_env() -> Self {
        let mut path_buf = match env::consts::OS {
            "linux" => PathBuf::from("/var/lib/zerotier-one/"),
            "macos" => PathBuf::from("~/Library/Application Support/ZeroTier/"),
            "windows" => PathBuf::from(r"\ProgramData\ZeroTier\One\"),
            _ => panic!("Cannot find path to the authorization token"),
        };
        path_buf.push("authtoken.secret");
        let auth_token = read_to_string(path_buf).unwrap();
        let mut headers = header::HeaderMap::new();
        let value = header::HeaderValue::from_str(&auth_token)
            .expect("Unacceptable authorization token");
        headers.insert("X-ZT1-Auth", value);

        let rqwst_client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("No ZeroTierOne authentication token has been provided");

        one::Client::new_with_client(ONE_BASE_URL, rqwst_client)
    }
}

/// This part is responsible for ZeroTier Central CLI.
///
/// Defines subcommands and arguments for the ZeroTier Central CLI.
fn central_cli() -> Command {
    Command::new("central")
        .about("A minimal ZeroTier Central CLI")
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommands([
            Command::new("status").about("Get the client status"),
            Command::new("network")
                .about("Interface for actions related to ZeroTier networks")
                .args([
                    Arg::new("id")
                        .long("id")
                        .help("Identifier of the network")
                        .value_name("NWID"),
                    Arg::new("as-json")
                        .short('j')
                        .long("as-json")
                        .help("Print the requsted network as JSON")
                        .value_name("NWID")
                        .requires("id"),
                ])
                .subcommands([
                    Command::new("create")
                        .about("Create new network")
                        .arg_required_else_help(true)
                        .args([
                            Arg::new("name")
                                .short('N')
                                .long("name")
                                .help("Give network member a name")
                                .value_name("NAME")
                                .required(true),
                            Arg::new("private")
                                .short('p')
                                .long("private")
                                .help("Network access policy (default is public)")
                                .action(ArgAction::SetTrue),
                        ]),
                    Command::new("update")
                        .about("Update existing network")
                        .arg_required_else_help(true)
                        .args([
                            Arg::new("network-id")
                                .short('n')
                                .long("network-id")
                                .help("Identifier of the network")
                                .value_name("NWID")
                                .required(true),
                            Arg::new("data")
                                .short('d')
                                .long("data")
                                .help("File with the body to update with")
                                .value_name("FILE")
                                .requires("id")
                                .value_parser(value_parser!(
                                    std::path::PathBuf
                                )),
                        ]),
                    Command::new("delete")
                        .about("Delete existing network by id")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("id")
                                .long("id")
                                .help("Identifier of the network")
                                .value_name("NWID")
                                .required(true),
                        ),
                    Command::new("list").about("Lists available networks"),
                ]),
            Command::new("member")
                .about("Network memeber(s) related actions")
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("update")
                        .about("Update network member(s)")
                        .args([
                            Arg::new("network-id")
                                .short('n')
                                .long("network-id")
                                .help("Identifier of the network")
                                .value_name("NWID")
                                .required(true),
                            Arg::new("member-id")
                                .short('m')
                                .long("member-id")
                                .help("Identifier of the member ot update")
                                .value_name("MEMID")
                                .requires("network-id"),
                            Arg::new("data")
                                .short('d')
                                .long("data")
                                .help("File with the body to update with")
                                .value_name("FILE")
                                .requires("network-id")
                                .requires("member-id")
                                .required_unless_present_any([
                                    "authorize-all",
                                    "name",
                                ])
                                .value_parser(value_parser!(
                                    std::path::PathBuf
                                )),
                            Arg::new("authorize-all")
                                .short('A')
                                .long("authorize-all")
                                .help("Authorize all members of the network")
                                .action(ArgAction::SetTrue)
                                .requires("network-id")
                                .required_unless_present_any([
                                    "data",
                                    "name",
                                    "member-id",
                                ]),
                            Arg::new("name")
                                .short('N')
                                .long("name")
                                .help("Give network member a name")
                                .value_name("NAME")
                                .requires("network-id")
                                .requires("member-id")
                                .required_unless_present_any([
                                    "authorize-all",
                                    "data",
                                ]),
                        ]),
                    Command::new("delete")
                        .about("Delete network member")
                        .arg_required_else_help(true)
                        .args([
                            Arg::new("network-id")
                                .short('n')
                                .long("network-id")
                                .help("Identifier of the network")
                                .value_name("NWID")
                                .required(true),
                            Arg::new("member-id")
                                .short('m')
                                .long("member-id")
                                .help("Identifier of the network member to delete")
                                .value_name("MEMID")
                                .requires("network-id"),
                        ]),
                    Command::new("list")
                        .about("List the network members")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("network-id")
                                .short('n')
                                .long("network-id")
                                .help("ID of the network to list members of it")
                                .value_name("NWID")
                                .required(true),
                        ),
                ]),
        ])
}

/// Handler for ZeroTier Central CLI commands.
struct CentralCliHandler<'a> {
    client: &'a central::Client,
}

impl<'a> CentralCliHandler<'a> {
    /// Creates a new CentralCliHandler.
    fn new(client: &'a central::Client) -> CentralCliHandler<'a> {
        Self { client }
    }

    /// Handles subcommands for the ZeroTier Central CLI.
    async fn handle(&self, matches: &ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("status", _)) => {
                let status = self.client.get_status().await?.into_inner();
                println!("Central client status (short):\n\n{status}");
            }
            Some(("network", subs)) => self.handle_network(subs).await?,
            Some(("member", subs)) => self.handle_member(subs).await?,
            _ => {}
        }
        Ok(())
    }

    /// Handles network-related subcommands.
    async fn handle_network(&self, matches: &ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("create", subs)) => self.network_create(subs).await?,
            Some(("update", subs)) => self.network_update(subs).await?,
            Some(("delete", subs)) => self.network_delete(subs).await?,
            Some(("list", _)) => self.network_list().await?,
            _ => self.network_long(matches).await?,
        }
        Ok(())
    }

    /// Prints network full JSON by ID.
    async fn network_long(&self, matches: &ArgMatches) -> Result<()> {
        let network_id = matches.get_one::<String>("id").unwrap();
        let as_json = matches.get_flag("as-json");
        let network = self
            .client
            .get_network_by_id(network_id)
            .await?
            .into_inner();
        if as_json {
            let network_json = serde_json::to_string_pretty(&network).unwrap();
            println!("{network_json}");
        } else {
            println!("{network}");
        }
        Ok(())
    }

    /// Creates a new network.
    async fn network_create(&self, matches: &ArgMatches) -> Result<()> {
        let mut body = Map::new();
        let name = Value::String(
            matches.get_one::<String>("name").unwrap().to_owned(),
        );
        let mut config = Map::new();
        config.insert("name".to_string(), name.clone());
        let private = Value::Bool(matches.get_flag("private"));
        config.insert("private".to_string(), private);

        let config_value = Value::Object(config);
        body.insert("config".to_string(), config_value);

        let network = self.client.new_network(&body).await?.into_inner();
        let network_id = network.id.unwrap();
        println!("Network (ID: {network_id}) created");
        Ok(())
    }

    /// Updates an existing network.
    async fn network_update(&self, matches: &ArgMatches) -> Result<()> {
        let network_id = matches.get_one::<String>("id").unwrap().to_owned();
        let path = matches.get_one::<PathBuf>("data").unwrap().to_owned();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let network: Network = serde_json::from_reader(reader).unwrap();

        let network_updated = self
            .client
            .update_network(&network_id, &network)
            .await?
            .into_inner();
        println!("Updated settings for:\n{network_updated}");
        Ok(())
    }

    /// Deletes an existing network.
    async fn network_delete(&self, matches: &ArgMatches) -> Result<()> {
        let network_id = matches.get_one::<String>("id").unwrap();
        self.client.delete_network(network_id).await?;
        println!("Network (ID: {network_id}) deleted");
        Ok(())
    }

    /// Lists available networks.
    async fn network_list(&self) -> Result<()> {
        let networks = self.client.get_network_list().await?;
        println!("List of the networks available (short):");
        for (n, net) in networks.to_vec().iter().enumerate() {
            println!("\n{}. {}", n + 1, net);
        }
        Ok(())
    }

    /// Handles member-related subcommands.
    async fn handle_member(&self, matches: &ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("update", subs)) => self.member_update(subs).await?,
            Some(("delete", subs)) => self.member_delete(subs).await?,
            Some(("list", subs)) => _ = self.member_list(subs, true).await?,
            _ => {}
        }
        Ok(())
    }

    /// Updates network members.
    async fn member_update(&self, matches: &ArgMatches) -> Result<()> {
        let network_id =
            matches.get_one::<String>("network-id").unwrap().to_owned();

        if matches.get_flag("authorize-all") {
            self.member_update_authorize_all(matches, &network_id)
                .await?;
            return Ok(());
        }

        let member_id =
            matches.get_one::<String>("member-id").unwrap().to_owned();
        let member = 'memb: {
            if let Some(name) = matches.get_one::<String>("name") {
                let mut member: Member = self
                    .client
                    .get_network_member(&network_id, &member_id)
                    .await?
                    .into_inner();
                member.name = Some(name.to_string());
                break 'memb member;
            } else if let Some(path) = matches.get_one::<PathBuf>("data") {
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                let member: Member = serde_json::from_reader(reader).unwrap();
                break 'memb member;
            };
            let empty_member: Member = serde_json::from_str("{}").unwrap();
            empty_member
        };

        let member_updated = self
            .client
            .update_network_member(&network_id, &member_id, &member)
            .await?
            .into_inner();
        println!("Updated member (short):\n{member_updated}");
        Ok(())
    }

    /// Authorizes all members of a network.
    async fn member_update_authorize_all(
        &self,
        matches: &ArgMatches,
        network_id: &str,
    ) -> Result<()> {
        let members = self.member_list(matches, false).await?;
        println!("Authorized member(s) (short):");
        for (n, member) in members.iter().enumerate() {
            let mut new_member = member.clone();
            new_member.config.as_mut().unwrap().authorized = Some(true);
            let member_id = member.node_id.as_ref().unwrap();
            let member_updated = self
                .client
                .update_network_member(network_id, member_id, &new_member)
                .await?
                .into_inner();
            println!("\n{}. {}", n + 1, member_updated);
        }
        Ok(())
    }

    /// Deletes a network member.
    async fn member_delete(&self, matches: &ArgMatches) -> Result<()> {
        let network_id =
            matches.get_one::<String>("network-id").unwrap().to_owned();
        let member_id =
            matches.get_one::<String>("member-id").unwrap().to_owned();
        self.client
            .delete_network_member(&network_id, &member_id)
            .await?;
        println!("Member (ID: {member_id}) deleted");
        Ok(())
    }

    /// Lists network members.
    async fn member_list(
        &self,
        matches: &ArgMatches,
        display: bool,
    ) -> Result<Vec<Member>> {
        let network_id =
            matches.get_one::<String>("network-id").unwrap().to_owned();
        let members = self
            .client
            .get_network_member_list(&network_id)
            .await?
            .into_inner();

        if display {
            println!("Network (ID: {network_id}) member list (short):");
            for (n, memb) in members.iter().enumerate() {
                println!("\n{}. {}", n + 1, memb);
            }
        }
        Ok(members)
    }
}

/// This part is responsible for ZeroTierOne Service CLI.
///
/// Defines subcommands and arguments for the ZeroTier One CLI.
fn one_cli() -> Command {
    Command::new("one")
        .about("A minimal ZeroTierOne Service CLI")
        .subcommand_required(true)
        .subcommands([
            Command::new("status").about("Gets the node status"),
            Command::new("peers")
                .about("List all the peers your node knows about"),
            Command::new("network")
                .about("Interface for actions related to networking")
                .subcommands([
                    Command::new("post")
                        .about("Join or update the network with the given ID. All flags default to true")
                        .arg_required_else_help(true)
                        .args([
                            Arg::new("id")
                                .long("id")
                                .help("Network ID to send POST request to")
                                .value_name("NWID")
                                .required(true),
                            Arg::new("allow-dns")
                                .short('d')
                                .long("allow-dns")
                                .help("Whether DNS addresses would be allowed")
                                .action(ArgAction::SetFalse),
                            Arg::new("allow-default")
                                .short('D')
                                .long("allow-default")
                                .help("Whether default addresses would be allowed")
                                .action(ArgAction::SetFalse),
                            Arg::new("allow-managed")
                                .short('m')
                                .long("allow-managed")
                                .help("Whether managed addresses would be allowed")
                                .action(ArgAction::SetFalse),
                            Arg::new("allow-global")
                                .short('g')
                                .long("allow-global")
                                .help("Whether global addresses would be allowed")
                                .action(ArgAction::SetFalse),
                        ]),
                    Command::new("leave")
                        .about("Leave the network with the given ID")
                        .arg_required_else_help(true)
                        .arg(
                            Arg::new("id")
                                .long("id")
                                .help("ID of the network to leave")
                                .value_name("NWID")
                                .required(true),
                        ),
                    Command::new("list").about("List all the networks that this node is joined to"),
                ]),
            Command::new("controller")
                .about("Interfaces controller-related functionality")
                .subcommand_required(true)
                .subcommands([
                    Command::new("status").about("Gets the controller status"),
                    Command::new("network")
                        .about("Controller networking manipulations")
                        .args([
                            Arg::new("id")
                                .long("id")
                                .help("Identifier of the network")
                                .value_name("NWID"),
                            Arg::new("as-json")
                                .short('j')
                                .long("as-json")
                                .help("Print the requsted network as JSON")
                                .action(ArgAction::SetTrue)
                                .requires("id"),
                        ])
                        .subcommands([
                            Command::new("post")
                                .about("Update an existing network or create a new one")
                                .args([
                                    Arg::new("id")
                                        .long("id")
                                        .help("ID of the network to be updated")
                                        .value_name("NWID")
                                        .requires("data"),
                                    Arg::new("data")
                                        .short('d')
                                        .long("data")
                                        .help("File with the body to update with")
                                        .value_name("FILE")
                                        .requires("id"),
                                    Arg::new("name")
                                        .long("name")
                                        .help("Name of the network to be created")
                                        .value_name("NAME")
                                        .required_unless_present_any([
                                            "id", "data",
                                        ]),
                                ]),
                            Command::new("delete")
                                .about("Delete network hosted by controller")
                                .arg(
                                    Arg::new("id")
                                        .long("id")
                                        .help("Identifier of the network")
                                        .value_name("NWID")
                                        .required(true),
                                ),
                            Command::new("list").about("List all the networks hosted by this controller"),
                        ]),
                    Command::new("member")
                        .about("Networks members-related actions")
                        .args([
                            Arg::new("id")
                                .long("id")
                                .help("Identifier of the member")
                                .value_name("MEMID"),
                            Arg::new("as-json")
                                .short('j')
                                .long("as-json")
                                .help("Print the requsted member as JSON")
                                .requires("id"),
                        ])
                        .subcommands([
                            Command::new("post")
                                .about("Update network member(s)")
                                .args([
                                    Arg::new("network-id")
                                        .short('n')
                                        .long("network-id")
                                        .help("Identifier of the network")
                                        .value_name("NWID")
                                        .required(true),
                                    Arg::new("member-id")
                                        .short('m')
                                        .long("member-id")
                                        .help("Identifier of the member to update")
                                        .value_name("MEMID")
                                        .requires("network-id"),
                                    Arg::new("data")
                                        .short('d')
                                        .long("data")
                                        .help("File with the body to update with")
                                        .value_name("FILE")
                                        .requires("network-id")
                                        .requires("member-id")
                                        .required_unless_present_any([
                                            "authorize-all",
                                            "name",
                                        ])
                                        .value_parser(value_parser!(
                                            std::path::PathBuf
                                        )),
                                    Arg::new("authorize-all")
                                        .short('A')
                                        .long("authorize-all")
                                        .help("Authorize all members of the network")
                                        .action(ArgAction::SetTrue)
                                        .requires("network-id")
                                        .required_unless_present_any([
                                            "data",
                                            "name",
                                            "member-id",
                                        ]),
                                    Arg::new("name")
                                        .short('N')
                                        .long("name")
                                        .help("Give network member a name")
                                        .value_name("NAME")
                                        .requires("network-id")
                                        .requires("member-id")
                                        .required_unless_present_any([
                                            "authorize-all",
                                            "data",
                                        ]),
                                ]),
                            Command::new("delete")
                                .about("Delete network member")
                                .arg_required_else_help(true)
                                .args([
                                    Arg::new("network-id")
                                        .short('n')
                                        .long("network-id")
                                        .help("Identifier of the network")
                                        .value_name("NWID")
                                        .required(true),
                                    Arg::new("member-id")
                                        .short('m')
                                        .long("member-id")
                                        .help("Identifier of the network member to delete")
                                        .value_name("MEMID")
                                        .requires("network-id"),
                                ]),
                            Command::new("list")
                                .about("List the network members")
                                .arg_required_else_help(true)
                                .arg(
                                    Arg::new("network-id")
                                        .short('n')
                                        .long("network-id")
                                        .help("ID of the network to list members of it")
                                        .value_name("NWID")
                                        .required(true),
                                ),
                        ]),
                ]),
        ])
}

/// Handler for ZeroTier One CLI commands.
struct OneCliHandler<'a> {
    client: &'a one::Client,
}

impl<'a> OneCliHandler<'a> {
    /// Creates a new OneCliHandler instance.
    fn new(client: &'a one::Client) -> OneCliHandler<'a> {
        Self { client }
    }

    /// Handles subcommands for the ZeroTier One CLI.
    async fn handle(&self, matches: &ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("status", _)) => self.handle_status().await?,
            Some(("peers", _)) => self.handle_peers().await?,
            Some(("network", subs)) => self.handle_network(subs).await?,
            Some(("controller", subs)) => self.handle_controller(subs).await?,
            _ => {}
        }
        Ok(())
    }

    /// Displays the node status.
    async fn handle_status(&self) -> Result<()> {
        let status = self.client.node_status_read_status().await?.into_inner();
        println!("{status}");
        Ok(())
    }

    /// Lists all available peers.
    async fn handle_peers(&self) -> Result<()> {
        let peers = self.client.node_peer_read_networks().await?.into_inner();
        println!("List of the peers available (short):");
        for (n, peer) in peers.iter().enumerate() {
            println!("\n{}. {}", n + 1, peer);
        }
        Ok(())
    }

    /// Handles network-related subcommands.
    async fn handle_network(&self, matches: &ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("post", subs)) => self.network_post(subs).await?,
            Some(("leave", subs)) => self.network_leave(subs).await?,
            Some(("list", _)) => self.network_list().await?,
            _ => {}
        }
        Ok(())
    }

    /// Joins or updates a network.
    async fn network_post(&self, matches: &ArgMatches) -> Result<()> {
        let network_id = matches.get_one::<String>("id").unwrap().to_owned();
        let zt_network_id = ZtNetworkId::from_str(network_id.as_str()).unwrap();

        let allow_dns = Some(matches.get_flag("allow-dns"));
        let allow_default = Some(matches.get_flag("allow-default"));
        let allow_managed = Some(matches.get_flag("allow-managed"));
        let allow_global = Some(matches.get_flag("allow-global"));

        let body = JoinedNetworkRequest {
            allow_dns,
            allow_default,
            allow_managed,
            allow_global,
        };
        let network = self
            .client
            .network_membership_set_network(&zt_network_id, &body)
            .await?
            .into_inner();
        println!("{network}");
        Ok(())
    }

    /// Leaves a network.
    async fn network_leave(&self, matches: &ArgMatches) -> Result<()> {
        let network_id = matches.get_one::<String>("id").unwrap().to_owned();
        let zt_network_id = ZtNetworkId::from_str(network_id.as_str()).unwrap();

        self.client
            .network_membership_del_network(&zt_network_id)
            .await?;

        Ok(())
    }

    /// Lists joined networks.
    async fn network_list(&self) -> Result<()> {
        let networks = self
            .client
            .network_membership_read_networks()
            .await?
            .into_inner();
        println!("List of the networks available (short):");
        for (n, net) in networks.iter().enumerate() {
            println!("\n{}. {}", n + 1, net);
        }
        Ok(())
    }

    /// Handles controller-related subcommands.
    async fn handle_controller(&self, matches: &ArgMatches) -> Result<()> {
        match matches.subcommand() {
            Some(("status", _)) => {
                let status = self
                    .client
                    .controller_read_controller_status()
                    .await?
                    .into_inner();
                println!("{status}");
            }
            Some(("network", subs)) => {
                self.handle_controller_network(subs).await?
            }
            Some(("member", subs)) => {
                self.handle_controller_member(subs).await?
            }
            _ => {}
        }
        Ok(())
    }

    /// Handles controller network subcommands.
    async fn handle_controller_network(
        &self,
        matches: &ArgMatches,
    ) -> Result<()> {
        match matches.subcommand() {
            Some(("post", subs)) => self.controller_network_post(subs).await?,
            Some(("delete", subs)) => {
                self.controller_network_delete(subs).await?
            }
            Some(("list", _)) => self.controller_network_list().await?,
            _ => self.controller_network_long(matches).await?,
        }
        Ok(())
    }

    /// Displays detailed network information.
    async fn controller_network_long(
        &self,
        matches: &ArgMatches,
    ) -> Result<()> {
        let network_id = matches.get_one::<String>("id").unwrap().to_owned();
        let zt_network_id = ZtNetworkId::from_str(network_id.as_str()).unwrap();
        let as_json = matches.get_flag("as-json");
        let network = self
            .client
            .network_read_network(&zt_network_id)
            .await?
            .into_inner();
        if as_json {
            let network_json = serde_json::to_string_pretty(&network).unwrap();
            println!("{network_json}");
        } else {
            println!("{network}");
        }
        Ok(())
    }

    /// Creates a new network.
    async fn controller_network_post(
        &self,
        matches: &ArgMatches,
    ) -> Result<()> {
        let name = matches.get_one::<String>("name").unwrap().to_owned();
        let body = 'body: {
            if let Some(path) = matches.get_one::<PathBuf>("data") {
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                let controller_network_request: ControllerNetworkRequest =
                    serde_json::from_reader(reader).unwrap();
                break 'body controller_network_request;
            } else {
                break 'body self._default_controller_network_request(name);
            }
        };

        let zt_network_id = self
            .client
            .random_network_random_network(&body)
            .await?
            .into_inner()
            .id;
        let controller_network = self
            .client
            .network_post_network(&zt_network_id, &body)
            .await?
            .into_inner();

        let network_id = controller_network.id.to_string();
        println!("Network (ID: {network_id}) now hosted by this controller");
        Ok(())
    }

    fn _default_controller_network_request(
        &self,
        name: String,
    ) -> ControllerNetworkRequest {
        let dns = Some(ControllerNetworkRequestDns::EmptyArrayItem(
            EmptyArrayItem(vec![serde_json::Value::Null]),
        ));
        let enable_broadcast = Some(true);

        let ip_range_start_addr =
            IPv4::from(Ipv4Addr::from_str("192.168.192.1").unwrap());
        let ip_range_end_addr =
            IPv4::from(Ipv4Addr::from_str("192.168.192.254").unwrap());
        let ip_assignment_pools = vec![IpAssignmentPool {
            ip_range_start: IpAssignmentPoolRangeStart::Subtype0(Some(
                ip_range_start_addr,
            )),
            ip_range_end: IpAssignmentPoolRangeEnd::Subtype0(Some(
                ip_range_end_addr,
            )),
        }];

        let mtu = Some(one::types::Mtu::from(2800));
        let multicast_limit = Some(one::types::USafeint::from(32));
        let name = Some(name);
        let private = Some(true);

        let route_target_addr =
            IpSlashPort::from_str("192.168.192.0/24").unwrap();
        let routes = vec![Route {
            target: Some(route_target_addr),
            via: None,
        }];

        let v4_assign_mode = Some(V4AssignMode { zt: Some(true) });
        let v6_assign_mode = Some(V6AssignMode {
            _6plane: Some(false),
            rfc4193: Some(false),
            zt: Some(false),
        });

        ControllerNetworkRequest {
            dns,
            enable_broadcast,
            ip_assignment_pools,
            mtu,
            multicast_limit,
            name,
            private,
            routes,
            v4_assign_mode,
            v6_assign_mode,
        }
    }

    /// Deletes a network.
    async fn controller_network_delete(
        &self,
        matches: &ArgMatches,
    ) -> Result<()> {
        let network_id = matches.get_one::<String>("id").unwrap().to_owned();
        let zt_network_id = ZtNetworkId::from_str(network_id.as_str()).unwrap();
        self.client
            .network_delete_network(&zt_network_id)
            .await?
            .into_inner();
        println!("Network (ID: {network_id}) deleted");
        Ok(())
    }

    /// Lists networks hosted by the controller.
    async fn controller_network_list(&self) -> Result<()> {
        let networks = self
            .client
            .network_read_networks()
            .await?
            .into_inner()
            .deref()
            .to_owned();

        println!("List of the networks hosted by this controller (IDs):");
        for (n, net) in networks.iter().enumerate() {
            let net = net.to_string();
            println!("\n{}. {}", n + 1, net);
        }
        Ok(())
    }

    /// Handles controller member subcommands.
    async fn handle_controller_member(
        &self,
        matches: &ArgMatches,
    ) -> Result<()> {
        match matches.subcommand() {
            Some(("post", subs)) => self.member_post(subs).await?,
            Some(("delete", subs)) => self.member_delete(subs).await?,
            Some(("list", subs)) => _ = self.member_list(subs, true).await?,
            _ => {}
        }
        Ok(())
    }

    /// Adds or updates a network member.
    async fn member_post(&self, matches: &ArgMatches) -> Result<()> {
        let network_id =
            matches.get_one::<String>("network-id").unwrap().to_owned();
        let zt_network_id = ZtNetworkId::from_str(network_id.as_str()).unwrap();

        if matches.get_flag("authorize-all") {
            self.member_post_authorize_all(matches, &zt_network_id)
                .await?;
            return Ok(());
        }

        let member_id =
            matches.get_one::<String>("member-id").unwrap().to_owned();
        let zt_member_id = ZtAddress::from_str(member_id.as_str()).unwrap();
        let member_request = 'membreq: {
            if let Some(name) = matches.get_one::<String>("name") {
                let mut member: ControllerNetworkMember = self
                    .client
                    .network_member_get_network_member(
                        &zt_network_id,
                        &zt_member_id,
                    )
                    .await?
                    .into_inner();
                member.name = Some(name.to_string());
                let member_request =
                    ControllerNetworkMemberRequest::from(&member);
                break 'membreq member_request;
            } else if let Some(path) = matches.get_one::<PathBuf>("data") {
                let file = File::open(path).unwrap();
                let reader = BufReader::new(file);
                let member_request: ControllerNetworkMemberRequest =
                    serde_json::from_reader(reader).unwrap();
                break 'membreq member_request;
            };
            let empty_member: ControllerNetworkMemberRequest =
                serde_json::from_str("{}").unwrap();
            empty_member
        };

        let member_updated = self
            .client
            .network_member_post_network_member(
                &zt_network_id,
                &zt_member_id,
                &member_request,
            )
            .await?
            .into_inner();
        println!("{member_updated}");
        Ok(())
    }

    /// Authorizes all network members.
    async fn member_post_authorize_all(
        &self,
        matches: &ArgMatches,
        network_id: &ZtNetworkId,
    ) -> Result<()> {
        let zt_network_id = ZtNetworkId::from_str(network_id.as_str()).unwrap();
        let members = self.member_list(matches, false).await?;
        println!("Authorized member(s) (short):");
        for (n, member) in members.iter().enumerate() {
            let mut new_member = member.clone();
            new_member.authorized = Some(true);
            let zt_member_id = member.address.to_owned();
            let new_member_request =
                ControllerNetworkMemberRequest::from(&new_member);
            let member_updated = self
                .client
                .network_member_post_network_member(
                    &zt_network_id,
                    &zt_member_id,
                    &new_member_request,
                )
                .await?
                .into_inner();
            println!("\n{}. {}", n + 1, member_updated);
        }
        Ok(())
    }

    /// Deletes a network member.
    async fn member_delete(&self, matches: &ArgMatches) -> Result<()> {
        let network_id =
            matches.get_one::<String>("network-id").unwrap().to_owned();
        let zt_network_id = ZtNetworkId::from_str(network_id.as_str()).unwrap();
        let member_id =
            matches.get_one::<String>("member-id").unwrap().to_owned();
        let zt_member_id = ZtAddress::from_str(member_id.as_str()).unwrap();
        self.client
            .network_member_del_network_member(&zt_network_id, &zt_member_id)
            .await?
            .into_inner();
        println!("Member (ID: {member_id}) deleted");
        Ok(())
    }

    /// Lists network members.
    async fn member_list(
        &self,
        matches: &ArgMatches,
        display: bool,
    ) -> Result<Vec<ControllerNetworkMember>> {
        let network_id =
            matches.get_one::<String>("network-id").unwrap().to_owned();
        let zt_network_id = ZtNetworkId::from_str(network_id.as_str()).unwrap();
        let members = self
            .client
            .member_list_network_members2(&zt_network_id)
            .await?
            .into_inner()
            .data;

        if display {
            println!("Network (ID: {network_id}) member list (short):");
            for (n, memb) in members.iter().enumerate() {
                println!("\n{}. {}", n + 1, memb);
            }
        }
        Ok(members)
    }
}
