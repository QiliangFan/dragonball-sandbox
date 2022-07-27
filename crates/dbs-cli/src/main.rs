
#![allow(unused)]

pub mod parser;
pub mod vmm;
pub mod utils;


use std::os::unix::net::UnixStream;
use clap::Parser;
use std::thread;
use libc::exit;
use dbs_utils::net::net_gen::connect;
use serde_json::{json, Error as JsonError};
use std::io::{BufReader, Read, ErrorKind, Write};

use parser::{Cli, CliSub};
use crate::utils::info::InstanceInfo;
use crate::vmm::vm::{CpuTopology, VmConfigInfo};
use crate::utils::sock_api::{bind};
use crate::utils::SOCK_PATH;


#[macro_use]
extern crate lazy_static;
extern crate core;

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();

    match &cli.command {
        CliSub::Bind(bind_args) => {
            let listener = bind()?;

            for mut stream in listener.incoming() {
                // test the existence of socket file
                match UnixStream::connect(SOCK_PATH) {
                    Ok(sc) => {  /* do nothing */  },
                    Err(e) => {
                        // Exit when socket file deleted
                        return Ok(());
                    }
                }

                // used for match
                let create = json!("create");

                match stream  {
                    Ok(soc) => {
                        let mut reader = BufReader::new(&soc);
                        let mut buf = String::new();
                        reader.read_to_string(&mut buf);
                        let request_body: serde_json::Value = serde_json::from_str(buf.as_str())?;
                        match &request_body["operation"] {
                            create => {
                                let vm_config_info: VmConfigInfo = serde_json::from_str(
                                    request_body["data"].as_str().ok_or(ErrorKind::UnexpectedEof)?
                                )?;
                                println!(r#"receive config: \
                                    {:?}"#, vm_config_info)
                            },
                            _ => {

                            }
                        };
                    },
                    Err(err) => {
                        return Err(err);
                    }
                }


            }
        },
        CliSub::Create(create_args) =>  {

            let mut vm_config = VmConfigInfo {
                vcpu: create_args.vcpu.clone(),
                max_vcpu: create_args.max_vcpu.clone(),
                cpu_pm: create_args.cpu_pm.clone(),
                vpmu_feature: create_args.vpmu_feature.clone(),
                cpu_topology: CpuTopology {
                    threads_per_core: create_args.cpu_topology.threads_per_core.clone(),
                    cores_per_die: create_args.cpu_topology.cores_per_die.clone(),
                    dies_per_socket: create_args.cpu_topology.dies_per_socket.clone(),
                    sockets: create_args.cpu_topology.sockets.clone(),
                },
                mem_type: create_args.mem_type.clone(),
                mem_file_path: create_args.mem_file_path.clone(),
                mem_size: create_args.mem_size.clone(),
                serial_path: None
            };

            let mut request_data = serde_json::to_string(&json!({
                "operation": "create",
                "data": vm_config
            }))?;
            let mut request_data = request_data.as_bytes();

            // send request
            let mut con = UnixStream::connect(SOCK_PATH)?;
            con.write_all(request_data.clone());

        },
        CliSub::Config(config_args) => {

        },
        _ => {
            // Nothing to do, error handled by clap:Parser.
        }
    }

    Ok(())
}
