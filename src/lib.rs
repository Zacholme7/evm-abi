use alloy_sol_types::sol;

//// AMM Factories ABIs and Addresses
pub mod factories {
    use super::*;

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV1Factory,
        "src/abis/dex/factories/uniswap_v1.json"
    }

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV2Factory,
        "src/abis/dex/factories/uniswap_v2.json"
    }

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV3Factory,
        "src/abis/dex/factories/uniswap_v3.json"
    }
}
}

/// AMM Routers
pub mod routers {
    use super::*;

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV2Router,
        "src/abis/dex/routers/uniswap_v2.json"
    }

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV3Router_02,
        "src/abis/dex/routers/uniswap_v3_02.json"
    }

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV4Router,
        "src/abis/dex/routers/uniswap_v4.json"
    }
}

/// AMM Pairs/Pools
pub mod pools {
    use super::*;

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV2Pool,
        "src/abis/dex/pools/uniswap_v2.json"
    }

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV3Pool,
        "src/abis/dex/pools/uniswap_v3.json"
    }
}

/// Amm Quoters
pub mod quoters {
    use super::*;

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV3Quoter_02,
        "src/abis/dex/quoters/uniswap_v3_02.json"
    }

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        UniswapV4Quoter,
        "src/abis/dex/quoters/uniswap_v4.json"
    }
}
/// Tokens
pub mod tokens {
    use super::*;

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        ERC20,
        "src/abis/tokens/ERC20.json"
    }

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        ERC721,
        "src/abis/tokens/ERC721.json"
    }

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        ERC1155,
        "src/abis/tokens/ERC1155.json"
    }
}

/// Lending
pub mod lending {
    use super::*;

    pub mod aave {
        use super::*;

        sol! {
            #[sol(rpc)]
            #[derive(Debug)]
            AToken,
            "src/abis/lending/aave/atoken.json"
        }

        sol! {
            #[sol(rpc)]
            #[derive(Debug)]
            LendingPool,
            "src/abis/lending/aave/lendingpool.json"
        }

        sol! {
            #[sol(rpc)]
            #[derive(Debug)]
            LendingPoolAddressesProvider,
            "src/abis/lending/aave/LendingPoolAddressesProvider.json"
        }

        sol! {
            #[sol(rpc)]
            #[derive(Debug)]
            LendingPoolCore,
            "src/abis/lending/aave/LendingPoolCore.json"
        }
    }
}

/// Utils
pub mod utils {
    use super::*;

    sol! {
        #[sol(rpc)]
        #[derive(Debug)]
        Multicall3,
        "src/abis/utils/multicall3.json"
    }
}

/// Address book
pub mod addresses {

    /// Uniswap Addresses
    pub mod uniswap {
        use alloy_primitives::{Address, address};

        /// Uniswap - Ethereum
        pub mod eth {
            use super::*;

            /// UniswapV2 - Ethereum
            pub const V2_FACTORY: Address = address!("0x5C69bEe701ef814a2B6a3EDD4B1652CB9cc5aA6f");
            pub const V2_ROUTER: Address = address!("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D");

            /// UniswapV3 - Ethereum
            pub const V3_FACTORY: Address = address!("0x1F98431c8aD98523631AE4a59f267346ea31F984");
            pub const V3_ROUTER_02: Address =
                address!("0x68b3465833fb72A70ecDF485E0e4C7bD8665Fc45");
            pub const V3_QUOTER_02: Address =
                address!("0x61fFE014bA17989E743c5F6cB21bF9697530B21e");
        }

        /// Uniswap - Base
        pub mod base {
            use super::*;

            /// UniswapV2 - Base
            pub const V2_FACTORY: Address = address!("0x8909Dc15e40173Ff4699343b6eB8132c65e18eC6");
            pub const V2_ROUTER: Address = address!("0x4752ba5dbc23f44d87826276bf6fd6b1c372ad24");

            /// UniswapV3 - Base
            pub const V3_FACTORY: Address = address!("0x33128a8fC17869897dcE68Ed026d694621f6FDfD");
            pub const V3_ROUTER_02: Address =
                address!("0x2626664c2603336E57B271c5C0b26F421741e481");
            pub const V3_QUOTER_02: Address =
                address!("0x3d4e44Eb1374240CE5F1B871ab261CD16335B76a");
        }
    }
}
