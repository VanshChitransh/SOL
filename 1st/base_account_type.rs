#[derive(PartialEq, Eq, Clone, Default)]
pub struct Account {
    pub lamports: u64,
    pub executable: bool,
    pub rent_epoch: Epoch,
    #[cfg_attr(feature = "serde", serde(with = "serde_bytes"))]
    pub data: Vec<u8>, 
    pub owner: Pubkey, 
}