#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, poise::ChoiceParameter)]
pub enum Roles {
    RoleOne,
    RoleTwo,
}
impl Default for Roles {
    fn default() -> Self {
        Self::RoleOne
    }
}
impl std::fmt::Display for Roles {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
