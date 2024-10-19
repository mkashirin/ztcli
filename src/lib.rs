use std::env;
use std::fs::{read_to_string, File};
use std::io::BufReader;
use std::path::PathBuf;
use std::str::FromStr;

use central::types::{ApiToken, Member, Network};
use clap::{arg, command, value_parser, ArgAction, ArgMatches, Command};
use one::types::{JoinedNetworkRequest, ZtNetworkId};
use reqwest::header;
use serde_json::{Map, Value};
use {zerotier_central as central, zerotier_one as one};

const CENTRAL_BASE_URL: &str = "https://api.zerotier.com/api/v1";
const ONE_BASE_URL: &str = "http://localhost:9993";

/// Entry point of the CLI. Initializes clients and delegates execution.
pub async fn cli(central_client: &central::Client, one_client: &one::Client) {
    let matches = command!()
        .about(
            "A mininal CLI combining ZeroTier Central and ZeroTier One \
             Service essential functionality",
        )
        .arg_required_else_help(true)
        .subcommands([central_cli(), one_cli()])
        .get_matches();
    match matches.subcommand() {
        Some(("central", subs)) => {
            let handler = CentralCliHandler::new(central_client);
            handler.handle(subs).await.unwrap();
        }
        Some(("one", subs)) => {
            let handler = OneCliHandler::new(one_client);
            handler.handle(subs).await;
        }
        _ => {}
    }
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
            .map_err(|_| "No ZeroTier Central API token has been provided")
            .unwrap();

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
            _ => todo!("Handle unsopported OS case properly"),
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
            .map_err(
                |_| "No ZeroTierOne authentication token has been provided",
            )
            .unwrap();

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
                .arg_required_else_help(true)
                .args([
                    arg!(--id <ID> "Get network by ID"),
                    arg!(-j --"as-json" "Print the requsted network as JSON")
                        .requires("id"),
                ])
                .subcommands([
                    Command::new("create")
                        .about("Create new network")
                        .arg_required_else_help(true)
                        .args([
                            arg!(
                                -n --name <NAME>
                                "Name of the network"
                            )
                            .required(true),
                            arg!(
                                -p --private
                                "Network access policy (default is public)"
                            )
                            .action(ArgAction::SetTrue),
                        ]),
                    Command::new("update")
                        .about("Update existing network")
                        .arg_required_else_help(true)
                        .args([
                            arg!(--id <ID> "Identifier of the network")
                                .required(true),
                            arg!(
                                -d --data <FILE>
                                "File with the body to update with"
                            )
                            .requires("id")
                            .value_parser(value_parser!(std::path::PathBuf)),
                        ]),
                    Command::new("delete")
                        .about("Delete existing network by id")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--id <ID> "Identifier of the network")
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
                            arg!(
                                -n --"network-id" <ID>
                                "Identifier of the network"
                            )
                            .required(true),
                            arg!(
                                -m --"member-id" <ID>
                                "Identifier of the member to update"
                            )
                            .requires("network-id"),
                            arg!(
                                -d --data <FILE>
                                "File with the body to update with"
                            )
                            .requires("network-id")
                            .requires("member-id")
                            .required_unless_present("authorize-all")
                            .value_parser(value_parser!(std::path::PathBuf)),
                            arg!(
                                -A --"authorize-all"
                                "Authorize all members of the network"
                            )
                            .action(ArgAction::SetTrue)
                            .requires("network-id")
                            .required_unless_present("data")
                            .required_unless_present("member-id"),
                        ]),
                    Command::new("delete")
                        .about("Delete netword member")
                        .arg_required_else_help(true)
                        .args([
                            arg!(
                                -n --"network-id" <ID>
                                "Identifier of the network"
                            )
                            .required(true),
                            arg!(
                                -m --"member-id" <ID>
                                "Identifier of the network member to delete"
                            )
                            .requires("network-id"),
                        ]),
                    Command::new("list")
                        .about("List the network members")
                        .arg_required_else_help(true)
                        .arg(arg!(
                            -n --"network-id" <ID>
                            "ID of the network to list members of it"
                        )),
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
    async fn handle(&self, matches: &ArgMatches) -> Result<(), central::Error> {
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
    async fn handle_network(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), central::Error> {
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
    async fn network_long(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), central::Error> {
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
            println!("Network (ID: {network_id})\n{network}");
        }
        Ok(())
    }

    /// Creates a new network.
    async fn network_create(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), central::Error> {
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
    async fn network_update(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), central::Error> {
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
    async fn network_delete(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), central::Error> {
        let network_id = matches.get_one::<String>("id").unwrap();
        self.client.delete_network(network_id).await?;
        println!("Network (ID: {}) deleted", network_id);
        Ok(())
    }

    /// Lists available networks.
    async fn network_list(&self) -> Result<(), central::Error> {
        let networks = self.client.get_network_list().await?;
        println!("List of the networks available (short):");
        for (n, net) in networks.to_vec().iter().enumerate() {
            println!("\n{}. {}", n + 1, net);
        }
        Ok(())
    }

    /// Handles member-related subcommands.
    async fn handle_member(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), central::Error> {
        match matches.subcommand() {
            Some(("update", subs)) => {
                self.member_update(subs).await?;
            }
            Some(("delete", subs)) => self.member_delete(subs).await?,
            Some(("list", subs)) => {
                let _ = self.member_list(subs, true).await?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Updates network members.
    async fn member_update(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), central::Error<()>> {
        let network_id =
            matches.get_one::<String>("network-id").unwrap().to_owned();
        if matches.get_flag("authorize-all") {
            self.member_update_authorize_all(matches, &network_id)
                .await?;
            return Ok(());
        }

        let member_id =
            matches.get_one::<String>("member-id").unwrap().to_owned();
        let path = matches.get_one::<PathBuf>("data").unwrap().to_owned();
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
        let member: Member = serde_json::from_reader(reader).unwrap();

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
    ) -> Result<(), central::Error> {
        let members = self.member_list(matches, false).await?;
        println!("Authorized member(s) (short):");
        for member in members {
            let mut new_member = member.clone();
            new_member.config.as_mut().unwrap().authorized = Some(true);
            let member_id = member.node_id.as_ref().unwrap();
            let member_updated = self
                .client
                .update_network_member(network_id, member_id, &new_member)
                .await?
                .into_inner();
            println!("\n1. {member_updated}");
        }
        Ok(())
    }

    /// Deletes a network member.
    async fn member_delete(
        &self,
        matches: &ArgMatches,
    ) -> Result<(), central::Error> {
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
    ) -> Result<Vec<Member>, central::Error<()>> {
        let network_id =
            matches.get_one::<String>("network-id").unwrap().to_owned();
        let members = self
            .client
            .get_network_member_list(&network_id)
            .await?
            .into_inner();

        if display {
            println!("Network (ID: {network_id}) member list (short):");
            for (n, memb) in members.to_vec().iter().enumerate() {
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
        .arg_required_else_help(true)
        .subcommand_required(true)
        .subcommand(Command::new("status").about("Gets the node status"))
        .subcommand(
            Command::new("network")
                .about("Interface for actions related to networking")
                .arg_required_else_help(true)
                .subcommands([
                    Command::new("post")
                        .about(
                            "Join or update the network with the given ID. \
                             All flags default to true",
                        )
                        .arg_required_else_help(true)
                        .args([
                            arg!(
                                --id <ID> "Network ID to send POST request to"
                            )
                            .required(true),
                            arg!(
                                --"allow-dns"
                                "Whether DNS addresses would be allowed"
                            )
                            .action(ArgAction::SetFalse),
                            arg!(
                                -d --"allow-default"
                                "Whether default addresses would be allowed"
                            )
                            .action(ArgAction::SetFalse),
                            arg!(
                                -m --"allow-managed"
                                "Whether managed addresses would be allowed"
                            )
                            .action(ArgAction::SetFalse),
                            arg!(
                                -g --"allow-global"
                                "Whether global addresses would be allowed"
                            )
                            .action(ArgAction::SetFalse),
                        ]),
                    Command::new("leave")
                        .about("Leave the network with the given ID")
                        .arg_required_else_help(true)
                        .arg(
                            arg!(--id <ID> "ID of the network to leave")
                                .required(true),
                        ),
                    Command::new("list").about(
                        "List all the networks that this node is joined to",
                    ),
                ]),
        )
}

/// Handler for ZeroTier One CLI commands.
struct OneCliHandler<'a> {
    client: &'a one::Client,
}

impl<'a> OneCliHandler<'a> {
    /// Creates a new OneCliHandler.
    fn new(client: &'a one::Client) -> OneCliHandler<'a> {
        Self { client }
    }

    /// Handles subcommands for the ZeroTier One CLI.
    async fn handle(&self, matches: &ArgMatches) {
        match matches.subcommand() {
            Some(("status", _)) => {
                let status = self
                    .client
                    .node_status_read_status()
                    .await
                    .unwrap()
                    .into_inner();
                println!("{status}");
            }
            Some(("network", subs)) => self.handle_network(subs).await,
            _ => {}
        }
    }

    /// Handles network-related subcommands.
    async fn handle_network(&self, matches: &ArgMatches) {
        match matches.subcommand() {
            Some(("post", subs)) => self.handle_network_post(subs).await,
            Some(("leave", subs)) => self.handle_network_leave(subs).await,
            Some(("list", _)) => self.handle_network_list().await,
            _ => {}
        }
    }

    /// Joins or updates a network.
    async fn handle_network_post(&self, matches: &ArgMatches) {
        let id = matches.get_one::<String>("id").unwrap().to_owned();
        let network_id = ZtNetworkId::from_str(id.as_str()).unwrap();

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
            .network_membership_set_network(&network_id, &body)
            .await
            .unwrap()
            .into_inner();
        println!("{network}");
    }

    /// Leaves a network.
    async fn handle_network_leave(&self, matches: &ArgMatches) {
        let id = matches.get_one::<String>("id").unwrap().to_owned();
        let network_id = ZtNetworkId::from_str(id.as_str()).unwrap();

        match self
            .client
            .network_membership_del_network(&network_id)
            .await
        {
            Ok(_) => println!("Left network (ID: {id})"),
            Err(e) => eprintln!("Failed to leave network (ID: {id}): {e}"),
        };
    }

    /// Lists joined networks.
    async fn handle_network_list(&self) {
        let networks = self
            .client
            .network_membership_read_networks()
            .await
            .unwrap();
        println!("List of the networks available (short):");
        for (n, net) in networks.to_vec().iter().enumerate() {
            println!("\n{}. {}", n + 1, net);
        }
    }
}
