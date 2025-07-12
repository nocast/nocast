use egui::Color32;

pub struct Theme{
    //window
    pub WINDOW_BG: Color32,
    //item
    pub SELECTED_ITEM_COLOR: Color32,
    pub NORMAL_ITEM_COLOR: Color32,
    pub ITEM_TITLE_COLOR: Color32,
    pub ITEM_DESC_COLOR: Color32,
    //input
    pub INPUT_BG_COLOR: Color32,
    pub INPUT_TEXT_COLOR: Color32,
}

pub fn load_theme(dark_mode: bool) -> Theme {
    if dark_mode{
		return Theme {
			WINDOW_BG: Color32::from_rgba_unmultiplied(16, 16, 19, 191),
			SELECTED_ITEM_COLOR: Color32::from_rgba_unmultiplied(72,72,75, 153),
			NORMAL_ITEM_COLOR: Color32::from_rgba_unmultiplied(16, 16, 19, 0),
			ITEM_TITLE_COLOR: Color32::from_rgba_unmultiplied(255,255,255,153),
			ITEM_DESC_COLOR: Color32::from_rgba_unmultiplied(160,160,160,153),
			INPUT_BG_COLOR: Color32::from_rgba_unmultiplied(60, 60, 67, 224),
			INPUT_TEXT_COLOR: Color32::from_rgba_unmultiplied(255, 255, 255, 224),
    	}
	}
    else{
        return Theme {
			WINDOW_BG: Color32::from_rgba_unmultiplied(242, 242, 247, 191),
			SELECTED_ITEM_COLOR: Color32::from_rgba_unmultiplied(209,209,214, 153),
			NORMAL_ITEM_COLOR: Color32::from_rgba_unmultiplied(242, 242, 247, 0),
			ITEM_TITLE_COLOR: Color32::from_rgba_unmultiplied(0,0,0,153),
			ITEM_DESC_COLOR: Color32::from_rgba_unmultiplied(60,60,60,153),
			INPUT_BG_COLOR: Color32::from_rgba_unmultiplied(229, 229, 234, 224),
			INPUT_TEXT_COLOR: Color32::from_rgba_unmultiplied(0, 0, 0, 224),
    	}
    }
}