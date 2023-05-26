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

    #[msg("NFT is not Eligble For Staking")]
    TokenNotEligble,

    #[msg("Account Has Already Been Initiliazed")]
    AlreadyInit,

    #[msg("You don't have premmision to withdraw the funds")]
    NoPrem
}