use json::JsonValue;
use libcma_binding_rust::parser::{
    cma_decode_advance, cma_decode_inspect, CmaParserError, CmaParserInput, CmaParserInputType,
};
use libcmt_binding_rust::cmt_rollup_finish_t;
use libcmt_binding_rust::rollup::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Portals {
    ERC1155BatchPortal,
    ERC1155SinglePortal,
    ERC20Portal,
    ERC721Portal,
    EtherPortal,
    None,
}

pub fn match_portal(address: &str) -> Portals {
    if address.eq_ignore_ascii_case("0xe246Abb974B307490d9C6932F48EbE79de72338A") {
        Portals::ERC1155BatchPortal
    } else if address.eq_ignore_ascii_case("0x18558398Dd1a8cE20956287a4Da7B76aE7A96662") {
        Portals::ERC1155SinglePortal
    } else if address.eq_ignore_ascii_case("0xACA6586A0Cf05bD831f2501E7B4aea550dA6562D") {
        Portals::ERC20Portal
    } else if address.eq_ignore_ascii_case("0x9E8851dadb2b77103928518846c4678d48b5e371") {
        Portals::ERC721Portal
    } else if address.eq_ignore_ascii_case("0xA632c5c05812c6a6149B7af5C56117d1D2603828") {
        Portals::EtherPortal
    } else {
        Portals::None
    }
}

/// Build the JSON envelope expected by `cma_decode_advance` / `cma_decode_inspect`.
fn create_parser_input(msg_sender: &str, payload_hex: &str) -> JsonValue {
    let mut input = JsonValue::new_object();
    input["data"]["metadata"]["msg_sender"] = msg_sender.into();
    input["data"]["payload"] = payload_hex.into();
    input
}

fn parser_err(err: CmaParserError) -> Box<dyn std::error::Error> {
    err.message().into()
}

pub async fn handle_advance(rollup: &mut Rollup) -> Result<bool, Box<dyn std::error::Error>> {
    let advance = rollup.read_advance_state()?;
    println!("Received advance request data {:?}", &advance);

    let msg_sender = advance.msg_sender;
    let input = create_parser_input(&msg_sender, &advance.payload);

    let decoded_req = match match_portal(&msg_sender) {
        Portals::ERC1155BatchPortal => {
            let req_type = CmaParserInputType::CmaParserInputTypeErc1155BatchDeposit;
            let decoded_req = cma_decode_advance(req_type, input.clone()).map_err(parser_err);
            println!(" ERC1155BatchPortal Deposit: {:?}", decoded_req);
            decoded_req
        }
        Portals::ERC1155SinglePortal => {
            let req_type = CmaParserInputType::CmaParserInputTypeErc1155SingleDeposit;
            let decoded_req = cma_decode_advance(req_type, input.clone()).map_err(parser_err);
            println!(" ERC1155SinglePortal Deposit: {:?}", decoded_req);
            decoded_req
        }
        Portals::ERC20Portal => {
            let req_type = CmaParserInputType::CmaParserInputTypeErc20Deposit;
            let decoded_req = cma_decode_advance(req_type, input.clone()).map_err(parser_err);
            println!(" ERC20Portal Deposit: {:?}", decoded_req);
            decoded_req
        }
        Portals::ERC721Portal => {
            let req_type = CmaParserInputType::CmaParserInputTypeErc721Deposit;
            let decoded_req = cma_decode_advance(req_type, input.clone()).map_err(parser_err);
            println!(" ERC721Portal Deposit: {:?}", decoded_req);
            decoded_req
        }
        Portals::EtherPortal => {
            let req_type = CmaParserInputType::CmaParserInputTypeEtherDeposit;
            let decoded_req = cma_decode_advance(req_type, input.clone()).map_err(parser_err);
            println!(" EtherPortal Deposit: {:?}", decoded_req);
            decoded_req
        }
        Portals::None => {
            let req_type = CmaParserInputType::CmaParserInputTypeAuto;
            let decoded_req = cma_decode_advance(req_type, input).map_err(parser_err);
            println!(
                " Unknown portal. User Input detected with body: {:?}",
                decoded_req
            );
            decoded_req
        }
    };

    match decoded_req {
        Ok(decoded) => dispatch_decoded_advance(&decoded),
        Err(e) => {
            println!("Error decoding advance request: {:?}", e);
            return Ok(false);
        }
    }

    Ok(true)
}

fn dispatch_decoded_advance(decoded: &CmaParserInput) {
    match decoded.req_type {
        CmaParserInputType::CmaParserInputTypeEtherDeposit => {
            // handle_ether_deposit(&decoded.input)
        }
        CmaParserInputType::CmaParserInputTypeErc20Deposit => {
            // handle_erc20_deposit(&decoded.input)
        }
        CmaParserInputType::CmaParserInputTypeErc721Deposit => {
            // handle_erc721_deposit(&decoded.input)
        }
        CmaParserInputType::CmaParserInputTypeErc1155SingleDeposit => {
            // handle_erc1155_single_deposit(&decoded.input)
        }
        CmaParserInputType::CmaParserInputTypeErc1155BatchDeposit => {
            // handle_erc1155_batch_deposit(&decoded.input)
        }
        CmaParserInputType::CmaParserInputTypeEtherWithdrawal
        | CmaParserInputType::CmaParserInputTypeErc20Withdrawal
        | CmaParserInputType::CmaParserInputTypeErc721Withdrawal
        | CmaParserInputType::CmaParserInputTypeErc1155SingleWithdrawal
        | CmaParserInputType::CmaParserInputTypeErc1155BatchWithdrawal
        | CmaParserInputType::CmaParserInputTypeEtherTransfer
        | CmaParserInputType::CmaParserInputTypeErc20Transfer
        | CmaParserInputType::CmaParserInputTypeErc721Transfer
        | CmaParserInputType::CmaParserInputTypeErc1155SingleTransfer
        | CmaParserInputType::CmaParserInputTypeErc1155BatchTransfer => {
            // handle_withdrawal_or_transfer(&decoded.input)
        }
        CmaParserInputType::CmaParserInputTypeUnidentified => {
            // handle_application_defined_methods(&decoded.input)
        }
        _ => {}
    }
}

fn dispatch_decoded_inspect(decoded: &CmaParserInput) {
    match decoded.req_type {
        CmaParserInputType::CmaParserInputTypeBalance => {
            // handle_ledger_get_balance(&decoded.input)
        }
        CmaParserInputType::CmaParserInputTypeSupply => {
            // handle_ledger_get_total_supply(&decoded.input)
        }
        _ => {}
    }
}

fn handle_application_defined_inspect(
    _rollup: &mut Rollup,
    payload_hex: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // handle_application_defined_inspect(payload_hex)
    let _ = payload_hex;
    Ok(())
}

pub async fn handle_inspect(rollup: &mut Rollup) -> Result<bool, Box<dyn std::error::Error>> {
    let inspect = rollup.read_inspect_state()?;
    println!("Received inspect request data {:?}", &inspect);
    println!(
        "Received inspect request length {}",
        inspect.payload.len()
    );

    let input = create_parser_input(
        "0x0000000000000000000000000000000000000000",
        &inspect.payload,
    );

    match cma_decode_inspect(input) {
        Ok(decoded) => {
            println!("Inspect decoded {:?}", decoded);
            dispatch_decoded_inspect(&decoded);
            Ok(true)
        }
        Err(CmaParserError::IncompatibleInput) => {
            println!("Unrecognized ledger inspect; delegating to application handler");
            handle_application_defined_inspect(rollup, &inspect.payload)?;
            Ok(true)
        }
        Err(e) => {
            println!("Failed to decode inspect: {:?}", e);
            Ok(false)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut accept_previous_request = true;
    let mut rollup: Rollup = Rollup::new().expect("Failed to create Rollup instance");

    loop {
        println!("Sending finish");
        let mut finish = cmt_rollup_finish_t {
            accept_previous_request,
            next_request_type: 0,
            next_request_payload_length: 0,
        };
        rollup.finish(&mut finish)?;

        accept_previous_request = match finish.next_request_type {
            0 => {
                println!("Received next input of type: advance_state");
                handle_advance(&mut rollup).await?
            }
            1 => {
                println!("Received next input of type: inspect_state");
                handle_inspect(&mut rollup).await?
            }
            _ => {
                eprintln!("Unknown request type: {}", finish.next_request_type);
                false
            }
        };
    }
}
