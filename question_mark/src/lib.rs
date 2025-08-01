#[derive(Debug)]
pub struct One {
    pub first_layer: Option<Two>,
}

#[derive(Debug)]
pub struct Two {
    pub second_layer: Option<Three>,
}

#[derive(Debug)]
pub struct Three {
    pub third_layer: Option<Four>,
}

#[derive(Debug)]
pub struct Four {
    pub fourth_layer: Option<u16>,
}

impl One {
    pub fn get_fourth_layer(self) -> Option<u16> {
        self.first_layer
            .as_ref()
            .and_then(|two| two.second_layer.as_ref())
            .and_then(|three| three.third_layer.as_ref())
            .and_then(|four| four.fourth_layer)
    }
}