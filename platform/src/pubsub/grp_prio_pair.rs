use std::fmt::Display;

use common::socket::cmd_message_grp_ids::GroupId;

pub type PrioId = i32;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct GrpPrioPair {
    pub grp_id: GroupId,
    pub priority_id: PrioId,
}

impl GrpPrioPair {
    pub fn new(grp_id: GroupId, priority_id: PrioId) -> Self {
        Self {
            grp_id,
            priority_id,
        }
    }
}

impl Display for GrpPrioPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(grp_id: {}, priority_id: {})",
            self.grp_id, self.priority_id
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let grp_prio_pair = GrpPrioPair::new(1, 2);
        assert_eq!(grp_prio_pair.grp_id, 1);
        assert_eq!(grp_prio_pair.priority_id, 2);
    }

    #[test]
    fn test_display() {
        let grp_prio_pair = GrpPrioPair::new(1, 2);
        assert_eq!(format!("{}", grp_prio_pair), "(grp_id: 1, priority_id: 2)");
    }
}
