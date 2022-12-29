#[derive(Clone)]
pub struct FCLayoutDiv{
	m_view:FCView,
	m_layout_style:String,
	m_auto_wrap:bool
}

impl FCLayoutDiv{
	fn new()->Self{
		let view:FCView = FCView::new();
		Self{
			m_view:view,
			m_layout_style:String::from("lefttoright"),
			m_auto_wrap:false
		}
	}
}

pub fn reset_layout_div(layout:&mut FCLayoutDiv)->bool{
	let mut reset:bool = false;
    let padding = layout.m_view.m_padding.clone();
    let mut v_pos:i32 = 0;
    let mut left = padding.left;
    let mut top = padding.top;
    let width = layout.m_view.m_size.cx - padding.left - padding.right;
    let height = layout.m_view.m_size.cy - padding.top - padding.bottom;
    let sub_views = get_sub_views(layout.m_view.clone());
    for i in 0..sub_views.len(){
        let mut view = (&sub_views[i]).clone();
        if view.m_visible{
            let size = view.m_size.clone();
            let margin = view.m_margin.clone();
            let c_left = view.m_location.x;
            let c_top = view.m_location.y;
            let c_width = size.cx;
            let c_height = size.cy;
            let mut n_left = c_left;
            let mut n_top = c_top;
            let mut n_width = c_width;
            let mut n_height = c_height;
            if layout.m_layout_style == "bottomtotop"{
                if i == 0{
                    top = height - padding.top;
                }
                let mut l_width:f32 = 0.0;
                if layout.m_auto_wrap{
                    l_width = size.cx;
                    let l_top = top - margin.top - c_height - margin.bottom;
                    if l_top < padding.top{
                        if v_pos != 0{
                            left += c_width + margin.left;
                        }
                        top = height - padding.top;
                    }
                }
                else{
                    l_width = width - margin.left - margin.right;
                }
                top -= c_height + margin.bottom;
                n_left = left + margin.left;
                n_width = l_width;
                n_top = top;
            }else if layout.m_layout_style == "lefttoright"{
                let mut l_height:f32 = 0.0;
                if layout.m_auto_wrap{
                    l_height = size.cy;
                    let l_right = left + margin.left + c_width + margin.right;
                    if l_right > width{
                        left = padding.left;
                        if v_pos != 0{
                            top += c_height + margin.top;
                        }
                    }
                }
                else{
                    l_height = height - margin.top - margin.bottom;
                }
                left += margin.left;
                n_left = left;
                n_top = top + margin.top;
                n_height = l_height;
                left += c_width + margin.right;
            }else if layout.m_layout_style == "righttoleft"{
                if i == 0{
                    left = width - padding.left;
                }
                let mut l_height:f32 = 0.0;
                if layout.m_auto_wrap{
                    l_height = size.cy;
                    let l_left = left - margin.left - c_width - margin.right;
                    if l_left < padding.left{
                        left = width - padding.left;
                        if v_pos != 0 {
                            top += c_height + margin.top;
                        }
                    }
                }
                else{
                    l_height = height - margin.top - margin.bottom;
                }
                left -= c_width + margin.left;
                n_left = left;
                n_top = top + margin.top;
                n_height = l_height;
            }else if layout.m_layout_style == "toptobottom"{
                let mut l_width:f32 = 0.0;
                if layout.m_auto_wrap{
                    l_width = size.cx;
                    let l_bottom = top + margin.top + c_height + margin.bottom;
                    if l_bottom > height{
                        if v_pos != 0{
                            left += c_width + margin.left + margin.right;
                        }
                        top = padding.top;
                    }
                }
                else{
                    l_width = width - margin.left - margin.right;
                }
                top += margin.top;
                n_top = top;
                n_left = left + margin.left;
                n_width = l_width;
                top += c_height + margin.bottom;
            }
            if c_left != n_left || c_top != n_top || c_width != n_width || c_height != n_height{
                view.m_location = FCPoint{x:n_left, y:n_top};
                view.m_size = FCSize{cx:n_width, cy:n_height};
				M_VIEW_MAP.lock().unwrap().insert(view.m_id, view.clone());
                reset = true;
            }
            v_pos = v_pos + 1;
        }
    }
    return reset;
}