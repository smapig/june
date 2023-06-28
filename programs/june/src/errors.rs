use crate::*;

#[error_code]
pub enum JuneTokenErrorCode {
  #[msg("Insufficient SOL balance")]
  InsufficientSOLBalance,
  #[msg("Insufficient JUNE balance")]
  InsufficientJUNEBalance,
  #[msg("Paused")]
  Paused,
  #[msg("Must be pool owner")]
  MustBePoolOwner,
  #[msg("Already paused/unpaused")]
  AlreadyPausedOrUnpaused,
  #[msg("Liquidity must greater than 0")]
  ZeroLiquidity,
  #[msg("Swap value must greater than 0")]
  ZeroSwap,
  #[msg("Invalid swap rate")]
  InvalidSwapRate,
}