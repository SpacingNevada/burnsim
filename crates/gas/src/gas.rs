use std::rc::Rc;

#[derive(Debug, PartialEq, Clone)]
pub struct Gas {
    pub name: Rc<str>,
    pub moles: f32,
    archived_moles: f32,
    pub specific_heat: f32,
}

impl Default for Gas {
    fn default() -> Self {
        Self {
            name: "".into(),
            moles: 0.0,
            archived_moles: 0.0,
            specific_heat: 0.0,
        }
    }
}

impl Gas {
    pub fn new() -> Self {Gas::default()}

    pub fn name(mut self, name: Rc<str>) -> Self {
        self.name = name;
        self
    }

    pub fn moles(mut self, moles:f32) -> Self {
        self.moles = moles;
        self
    }

    fn archive_moles(mut self) {
        self.archived_moles = self.moles;
    }

    pub fn heat(mut self, heat:f32) -> Self {
        self.specific_heat = heat;
        self
    }

    pub fn zero(mut self) -> Self {
        self.moles = 0.0;
        self.specific_heat = 0.0;
        self
    }
}