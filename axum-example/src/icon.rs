pub enum SVG {
	BurgerMenu,
}

impl hypertext::Renderable for SVG {
	fn render_to(&self, buffer: &mut String) {
		buffer.push_str(match self {
			SVG::BurgerMenu => include_str!("../static/svg/burger.svg"),
		})
	}
}
