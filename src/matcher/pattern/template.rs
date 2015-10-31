pub trait Template: Debug {
    fn render(&mut self) -> String;
}
