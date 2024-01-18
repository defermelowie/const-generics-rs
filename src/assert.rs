pub(crate) struct Assert<const P: bool>;

pub(crate) trait True {}
pub(crate) trait False {}

impl True for Assert<true>{}
impl False for Assert<false>{}