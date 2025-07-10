use alloy_sol_types::sol;

/// Declare ABIs for all AMM Factories
pub mod factories {
    use super::*;

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV2Factory,
        "src/abis/dex/factories/uniswap_v2_factory.json"
    }
}

pub mod pools {
    use super::*;

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV2Pair,
        "src/abis/dex/pools/uniswap_v2_pair.json"
    }
}
