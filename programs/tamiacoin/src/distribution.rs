pub const TOTAL_SUPPLY: u64 = 100_000_000_000_000; //100 trillions tokens

// Token distribution in percentage
pub const LIQUIDITY_SUPPLY: u64 = (TOTAL_SUPPLY * 50) / 100; // 50%
pub const P2E_SUPPLY: u64 = (TOTAL_SUPPLY * 18) / 100; // 18%
pub const MARKETING_SUPPLY: u64 = (TOTAL_SUPPLY * 10) / 100; // 10%
pub const TEAM_SUPPLY: u64 = (TOTAL_SUPPLY * 7) / 100; // 7%
pub const BURN_SUPPLY: u64 = (TOTAL_SUPPLY * 10) / 100; // 10%
pub const OWNER_SUPPLY: u64 = (TOTAL_SUPPLY * 5) / 100; // 5%

// Vesting (optional) - Time before tokens can be retrieved (in seconds)
#[allow(dead_code)]
pub const OWNER_VESTING_PERIOD: u64 = 31536000; // 1 year (365 days)