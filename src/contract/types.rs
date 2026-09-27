use soroban_sdk::{contracttype, Address, String};

/// Resolution outcome for a dispute.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Resolution {
    /// Full reward goes to the contributor.
    Contributor,
    /// Full reward is returned to the creator.
    Creator,
    /// Reward is split between contributor and creator.
    /// `contributor_share_bps` is the contributor's share in basis points
    /// (1 bps = 0.01%). The creator receives the remainder.
    Split { contributor_share_bps: u32 },
}

impl Resolution {
    /// Maximum valid basis points (100%).
    pub const MAX_BPS: u32 = 10_000;

    /// Returns true when the resolution carries a valid basis point share.
    pub fn is_valid(&self) -> bool {
        match self {
            Resolution::Contributor | Resolution::Creator => true,
            Resolution::Split {
                contributor_share_bps,
            } => *contributor_share_bps <= Self::MAX_BPS,
        }
    }

    /// Computes the contributor and creator payouts for a given reward.
    ///
    /// The two amounts always sum exactly to `reward`; any remainder from
    /// integer division is assigned to the creator.
    pub fn payouts(&self, reward: i128) -> (i128, i128) {
        match self {
            Resolution::Contributor => (reward, 0),
            Resolution::Creator => (0, reward),
            Resolution::Split {
                contributor_share_bps,
            } => {
                let contributor = reward * (*contributor_share_bps as i128) / (Self::MAX_BPS as i128);
                let creator = reward - contributor;
                (contributor, creator)
            }
        }
    }
}

/// A dispute raised against a bounty submission.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Dispute {
    pub bounty_id: u64,
    pub submission_id: u64,
    pub raised_by: Address,
    /// Short, human-readable reason supplied when the dispute was raised.
    /// Required and must be non-empty so the arbitrator has an on-chain
    /// record tying the dispute to its cause.
    pub reason: String,
    pub resolved: bool,
    pub resolution: Option<Resolution>,
}
