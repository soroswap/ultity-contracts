use soroban_sdk::{self, contracterror};
use soroswap_library::SoroswapLibraryError;

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
pub enum CombinedLiquidityTimelockError {
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

impl From<SoroswapLibraryError> for CombinedLiquidityTimelockError {
    fn from(err: SoroswapLibraryError) -> Self {
        match err {
            SoroswapLibraryError::InsufficientAmount => {
                CombinedLiquidityTimelockError::LibraryInsufficientAmount
            }
            SoroswapLibraryError::InsufficientLiquidity => {
                CombinedLiquidityTimelockError::LibraryInsufficientLiquidity
            }
            SoroswapLibraryError::InsufficientInputAmount => {
                CombinedLiquidityTimelockError::LibraryInsufficientInputAmount
            }
            SoroswapLibraryError::InsufficientOutputAmount => {
                CombinedLiquidityTimelockError::LibraryInsufficientOutputAmount
            }
            SoroswapLibraryError::InvalidPath => CombinedLiquidityTimelockError::LibraryInvalidPath,
            SoroswapLibraryError::SortIdenticalTokens => {
                CombinedLiquidityTimelockError::LibrarySortIdenticalTokens
            }
        }
    }
}

impl From<LiquidityTimelockError> for CombinedLiquidityTimelockError {
    fn from(err: LiquidityTimelockError) -> Self {
        match err {
            LiquidityTimelockError::NotInitialized => {
                CombinedLiquidityTimelockError::TimelockNotInitialized
            }
            LiquidityTimelockError::AlreadyInitialized => {
                CombinedLiquidityTimelockError::TimelockAlreadyInitialized
            }
            LiquidityTimelockError::NegativeNotAllowed => {
                CombinedLiquidityTimelockError::TimelockNegativeNotAllowed
            }
            LiquidityTimelockError::ProtocolAddressNotFound => {
                CombinedLiquidityTimelockError::TimelockDeadlineExpired
            }
            LiquidityTimelockError::DeadlineExpired => {
                CombinedLiquidityTimelockError::TimelockDeadlineExpired
            }
            LiquidityTimelockError::NeedToWait => {
                CombinedLiquidityTimelockError::TimelockNeedToWait
            }
            LiquidityTimelockError::WrongTimestamp => {
                CombinedLiquidityTimelockError::TimelockWrongTimestamp
            }
            LiquidityTimelockError::InsufficientBAmount => {
                CombinedLiquidityTimelockError::TimelockInsufficientBAmount
            }
            LiquidityTimelockError::InsufficientAAmount => {
                CombinedLiquidityTimelockError::TimelockInsufficientAAmount
            }
        }
    }
}
