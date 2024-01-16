pub(crate) struct If<const P: bool>;

pub trait True {}
pub trait False {}

impl True for If<true>{}
impl False for If<false>{}