use anchor_lang::error_code;

#[error_code]
pub enum StakeError {
    #[msg("NFT already staked")]
    AlreadyStaked,

    #[msg("State account is uninitialized")]
    UninitializedAccount,

    #[msg("Stake state is invalid")]
    InvalidStakeState,

    #[msg("Nft not owned by user")]
    InvalidOwner,

    #[msg("NFT has not been prepaired to be unstaked")]
    NoPrepForUnstake,

    #[msg("Haven't Claiemd Last NFT")]
    UnclaimedNFT,

}