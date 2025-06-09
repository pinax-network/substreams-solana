mod calls;

use std::collections::HashSet;

use calls::batch_balance_of;
use proto::pb::evm::erc20::balances::v1::{BalanceByAccount, Events};
use proto::pb::evm::erc20::transfers::v1::Events as ERC20Transfers;

#[substreams::handlers::map]
fn map_events(params: String, erc20: ERC20Transfers) -> Result<Events, substreams::errors::Error> {
    let mut events = Events::default();
    let chunk_size = params.parse::<usize>().expect("Failed to parse chunk_size");

    // Collect unique tokens by owners
    let contracts_by_owner = erc20
        .transfers
        .iter()
        .flat_map(|t| vec![(&t.contract, &t.from), (&t.contract, &t.to)])
        .collect::<HashSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();

    // Fetch RPC calls for Balance Of
    let balance_ofs = batch_balance_of(&contracts_by_owner, chunk_size);

    for (contract, owner) in &contracts_by_owner {
        if let Some(amount) = balance_ofs.get(&(contract, owner)) {
            events.balances_by_account.push(BalanceByAccount {
                contract: contract.to_vec(),
                account: owner.to_vec(),
                amount: amount.to_string(),
            });
        };
    }
    Ok(events)
}
