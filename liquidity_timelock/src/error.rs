use soroban_sdk::{self, contracterror};
use soroswap_library::{SoroswapLibraryError};


#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum LiquidityTimelockError {
    NotInitialized = 801,
    AlreadyInitialized = 802,
    NegativeNotAllowed = 803,
    ProtocolAddressNotFound = 804,
    DeadlineExpired = 805,
    NeedToWait = 806,
    WrongTimestamp = 807,
    InsufficientBAmount = 808,
    InsufficientAAmount = 809,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
// Define a new set of integer literals for the CombinedError enum
pub enum CombinedLiquidityTimelockErrror {
    TimelockNotInitialized = 901,
    TimelockAlreadyInitialized = 902,
    TimelockNegativeNotAllowed = 903,
    TimelockProtocolAddressNotFound = 904,
    TimelockDeadlineExpired = 905,
    TimelockNeedToWait = 906,
    TimelockWrongTimestamp = 907,
    TimelockInsufficientBAmount = 908,
    TimelockInsufficientAAmount = 909,

    LibraryInsufficientAmount = 910,
    LibraryInsufficientLiquidity = 911,
    LibraryInsufficientInputAmount = 912,
    LibraryInsufficientOutputAmount = 913,
    LibraryInvalidPath = 914,
    LibrarySortIdenticalTokens = 915,
}

impl From<SoroswapLibraryError> for CombinedLiquidityTimelockErrror {
    fn from(err: SoroswapLibraryError) -> Self {
        match err {
            SoroswapLibraryError::InsufficientAmount => CombinedLiquidityTimelockErrror::LibraryInsufficientAmount,
            SoroswapLibraryError::InsufficientLiquidity => CombinedLiquidityTimelockErrror::LibraryInsufficientLiquidity,
            SoroswapLibraryError::InsufficientInputAmount => CombinedLiquidityTimelockErrror::LibraryInsufficientInputAmount,
            SoroswapLibraryError::InsufficientOutputAmount => CombinedLiquidityTimelockErrror::LibraryInsufficientOutputAmount,
            SoroswapLibraryError::InvalidPath => CombinedLiquidityTimelockErrror::LibraryInvalidPath,
            SoroswapLibraryError::SortIdenticalTokens => CombinedLiquidityTimelockErrror::LibrarySortIdenticalTokens,
        }
    }
}

impl From<LiquidityTimelockError> for CombinedLiquidityTimelockErrror {
    fn from(err: LiquidityTimelockError) -> Self {
        match err {
            LiquidityTimelockError::NotInitialized => CombinedLiquidityTimelockErrror::TimelockNotInitialized,
            LiquidityTimelockError::AlreadyInitialized => CombinedLiquidityTimelockErrror::TimelockAlreadyInitialized,
            LiquidityTimelockError::NegativeNotAllowed => CombinedLiquidityTimelockErrror::TimelockNegativeNotAllowed,
            LiquidityTimelockError::ProtocolAddressNotFound => CombinedLiquidityTimelockErrror::TimelockDeadlineExpired,
            LiquidityTimelockError::DeadlineExpired => CombinedLiquidityTimelockErrror::TimelockDeadlineExpired,
            LiquidityTimelockError::NeedToWait => CombinedLiquidityTimelockErrror::TimelockNeedToWait,
            LiquidityTimelockError::WrongTimestamp => CombinedLiquidityTimelockErrror::TimelockWrongTimestamp,
            LiquidityTimelockError::InsufficientBAmount => CombinedLiquidityTimelockErrror::TimelockInsufficientBAmount,
            LiquidityTimelockError::InsufficientAAmount => CombinedLiquidityTimelockErrror::TimelockInsufficientAAmount,
        }
    }
}
