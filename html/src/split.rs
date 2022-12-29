#[derive(Clone)]
pub struct FCSplitLayoutDiv{
	m_view:FCView,
	m_layout_style:String,
	m_first_view:FCView,
	m_second_view:FCView,
	m_split_mode:String,
	m_splitter:FCView,
	m_split_percent:f32,
	m_old_size:FCSize
}

impl FCSplitLayoutDiv{
	fn new()->Self{
		let first_view:FCView = FCView::new();
		let second_view:FCView = FCView::new();
		let splitter:FCView = FCView::new();
		let view:FCView = FCView::new();
		Self{
			m_view:view,
			m_layout_style:String::from("lefttoright"),
			m_first_view:first_view,
			m_second_view:second_view,
			m_split_mode:String::from("absolutesize"),
			m_splitter:splitter,
			m_split_percent:-1.0,
			m_old_size:FCSize{cx:0.0, cy:0.0}
		}
	}
}

pub fn reset_split_layout_div(split:&mut FCSplitLayoutDiv)->bool{
	let mut reset : bool = false;
    let mut split_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
    let width = split.m_view.m_size.cx;
    let height = split.m_view.m_size.cy;
    let mut f_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
    let mut s_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
    let mut splitter_size = FCSize{cx:0.0, cy:0.0};
    if split.m_splitter.m_visible{
        splitter_size.cx = split.m_splitter.m_size.cx;
        splitter_size.cy = split.m_splitter.m_size.cy;
    }
    let layout_style = split.m_layout_style.clone();
    if layout_style == "bottomtotop"{
        if split.m_split_mode == "absolutesize" || split.m_old_size.cy == 0.0{
            split_rect.left = 0.0;
            split_rect.top = height - (split.m_old_size.cy - split.m_splitter.m_location.y);
            split_rect.right = width;
            split_rect.bottom = split_rect.top + splitter_size.cy;
        }
        else if split.m_split_mode == "percentsize"{
            split_rect.left = 0.0;
            if split.m_split_percent == -1.0{
                split.m_split_percent = split.m_splitter.m_location.y / split.m_old_size.cy;
            }
            split_rect.top = height * split.m_split_percent;
            split_rect.right = width;
            split_rect.bottom = split_rect.top + splitter_size.cy;
        }
        f_rect.left = 0.0;
        f_rect.top = split_rect.bottom;
        f_rect.right = width;
        f_rect.bottom = height;
        s_rect.left = 0.0;
        s_rect.top = 0.0;
        s_rect.right = width;
        s_rect.bottom = split_rect.top;
    }
    else if layout_style == "lefttoright"{
        if split.m_split_mode == "absolutesize" || split.m_old_size.cx == 0.0{
            split_rect.left = split.m_splitter.m_location.x;
            split_rect.top = 0.0;
            split_rect.right = split_rect.left + splitter_size.cx;
            split_rect.bottom = height;
        }
        else if split.m_split_mode == "percentsize"{
            if split.m_split_percent == -1.0{
                split.m_split_percent = split.m_splitter.m_location.x / split.m_old_size.cx;
            }
            split_rect.left = width * split.m_split_percent;
            split_rect.top = 0.0;
            split_rect.right = split_rect.left + splitter_size.cx;
            split_rect.bottom = height;
        }
        f_rect.left = 0.0;
        f_rect.top = 0.0;
        f_rect.right = split_rect.left;
        f_rect.bottom = height;
        s_rect.left = split_rect.right;
        s_rect.top = 0.0;
        s_rect.right = width;
        s_rect.bottom = height;
    }
    else if layout_style == "righttoleft"{
        if split.m_split_mode == "absolutesize" || split.m_old_size.cx == 0.0{
            split_rect.left = width - (split.m_old_size.cx - split.m_splitter.m_location.x);
            split_rect.top = 0.0;
            split_rect.right = split_rect.left + splitter_size.cx;
            split_rect.bottom = height;
        }
        else if split.m_split_mode == "percentsize"{
            if split.m_split_percent == -1.0{
                split.m_split_percent = split.m_splitter.m_location.x / split.m_old_size.cx;
            }
            split_rect.left = width * split.m_split_percent;
            split_rect.top = 0.0;
            split_rect.right = split_rect.left + splitter_size.cx;
            split_rect.bottom = height;
        }
        f_rect.left = split_rect.right;
        f_rect.top = 0.0;
        f_rect.right = width;
        f_rect.bottom = height;
        s_rect.left = 0.0;
        s_rect.top = 0.0;
        s_rect.right = split_rect.left;
        s_rect.bottom = height;
    }
    else if layout_style == "toptobottom"{
        if split.m_split_mode == "absolutesize" || split.m_old_size.cy == 0.0{
            split_rect.left = 0.0;
            split_rect.top = split.m_splitter.m_location.y;
            split_rect.right = width;
            split_rect.bottom = split_rect.top + splitter_size.cy;
        }
        else if split.m_split_mode == "percentsize"{
            split_rect.left = 0.0;
            if split.m_split_percent == -1.0{
                split.m_split_percent = split.m_splitter.m_location.y / split.m_old_size.cy;
            }
            split_rect.top = height * split.m_split_percent;
            split_rect.right = width;
            split_rect.bottom = split_rect.top + splitter_size.cy;
        }
        f_rect.left = 0.0;
        f_rect.top = 0.0;
        f_rect.right = width;
        f_rect.bottom = split_rect.top;
        s_rect.left = 0.0;
        s_rect.top = split_rect.bottom;
        s_rect.right = width;
        s_rect.bottom = height;
    }
    if split.m_splitter.m_visible{
        let sp_rect = FCRect{left:split.m_splitter.m_location.x,  top:split.m_splitter.m_location.y, right:split.m_splitter.m_location.x + split.m_splitter.m_size.cx, bottom:split.m_splitter.m_location.y + split.m_splitter.m_size.cy};
        if sp_rect.left != split_rect.left || sp_rect.top != split_rect.top || sp_rect.right != split_rect.right || sp_rect.bottom != split_rect.bottom{
            split.m_splitter.m_location = FCPoint{x:split_rect.left, y:split_rect.top};
            split.m_splitter.m_size = FCSize{cx:split_rect.right - split_rect.left, cy:split_rect.bottom - split_rect.top};
            M_VIEW_MAP.lock().unwrap().insert(split.m_splitter.m_id, split.m_splitter.clone());
            reset = true;
        }
    }
    let fc_rect = FCRect{left:split.m_first_view.m_location.x,  top:split.m_first_view.m_location.y, right:split.m_first_view.m_location.x + split.m_first_view.m_size.cx, bottom:split.m_first_view.m_location.y + split.m_first_view.m_size.cy};
    if fc_rect.left != f_rect.left || fc_rect.top != f_rect.top || fc_rect.right != f_rect.right || fc_rect.bottom != f_rect.bottom{
        reset = true;
        split.m_first_view.m_location = FCPoint{x:f_rect.left, y:f_rect.top};
        split.m_first_view.m_size = FCSize{cx:f_rect.right - f_rect.left, cy:f_rect.bottom - f_rect.top};
        M_VIEW_MAP.lock().unwrap().insert(split.m_first_view.m_id, split.m_first_view.clone());
    }
    let sc_rect = FCRect{left:split.m_second_view.m_location.x,  top:split.m_second_view.m_location.y, right:split.m_second_view.m_location.x + split.m_second_view.m_size.cx, bottom:split.m_second_view.m_location.y + split.m_second_view.m_size.cy};
    if sc_rect.left != s_rect.left || sc_rect.top != s_rect.top || sc_rect.right != s_rect.right || sc_rect.bottom != s_rect.bottom{
        reset = true;
        split.m_second_view.m_location = FCPoint{x:s_rect.left, y:s_rect.top};
        split.m_second_view.m_size = FCSize{cx:s_rect.right - s_rect.left, cy:s_rect.bottom - s_rect.top};
        M_VIEW_MAP.lock().unwrap().insert(split.m_second_view.m_id, split.m_second_view.clone());
    }
    split.m_old_size = FCSize{cx:width, cy:height};
    return reset;
}