#[derive(Clone)]
pub struct FCTabPage{
	m_view:FCView,
	m_header_button:FCView
}

impl FCTabPage{
	fn new()->Self{
		let header_button:FCView = FCView::new();
		let view:FCView = FCView::new();
		Self{
			m_view:view,
			m_header_button:header_button
		}
	}
}

#[derive(Clone)]
pub struct FCTabView{
	m_view:FCView,
	m_layout:String,
	m_tab_pages:Vec<FCTabPage>,
	m_under_line_color:String,
	m_under_line_size:f32,
	m_under_point:FCPoint,
	m_use_animation:bool,
	m_animation_speed:f32
}

impl FCTabView{
	fn new()->Self{
		let tab_pages:Vec<FCTabPage> = Vec::new();
		let view:FCView = FCView::new();
		Self{
			m_view:view,
			m_layout:String::from("top"),
			m_tab_pages:tab_pages,
			m_under_line_color:String::from("rgb(255,255,255)"),
			m_under_line_size:5.0,
			m_under_point:FCPoint{x:0.0, y:0.0},
			m_use_animation:false,
			m_animation_speed:20.0
		}
	}
}

pub fn updata_page_layout(tab_view:&mut FCTabView, tab_page:&mut FCTabPage, left:f32, top:f32, width:f32, height:f32, tw:f32, th:f32){
	let mut new_bounds = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
	if(tab_view.m_layout == "bottom"){
		new_bounds.left = 0.0;
		new_bounds.top = 0.0;
		new_bounds.right = width;
		new_bounds.bottom = height - th;
		tab_page.m_header_button.m_location = FCPoint{x:left, y:height - th};
	}else if(tab_view.m_layout == "left"){
		new_bounds.left = tw;
		new_bounds.top = 0.0;
		new_bounds.right = width;
		new_bounds.bottom = height;
		tab_page.m_header_button.m_location = FCPoint{x:0.0, y:top};
	}else if(tab_view.m_layout == "right"){
		new_bounds.left = 0.0;
		new_bounds.top = 0.0;
		new_bounds.right = width - tw;
		new_bounds.bottom = height;
		tab_page.m_header_button.m_location = FCPoint{x:width - tw, y:top};
	}else if(tab_view.m_layout == "top"){
		new_bounds.left = 0.0;
		new_bounds.top = th;
		new_bounds.right = width;
		new_bounds.bottom = height;
		tab_page.m_header_button.m_location = FCPoint{x:left, y:0.0};
	}
	tab_page.m_view.m_location = FCPoint{x:new_bounds.left, y:new_bounds.top};
	tab_page.m_view.m_size = FCSize{cx:new_bounds.right - new_bounds.left, cy:new_bounds.bottom - new_bounds.top};
}

pub fn update_tab_layout(tab_view:&mut FCTabView){
	let width = tab_view.m_view.m_size.cx;
	let height = tab_view.m_view.m_size.cy;
	let mut left :f32 = 0.0;
	let mut top : f32 = 0.0;
	for i in 0..tab_view.m_tab_pages.len(){
		let mut tp = tab_view.m_tab_pages[i].clone();
		let header_button = tp.m_header_button.clone();
		if(header_button.m_visible){
			let tw = header_button.m_size.cx;
			let th = header_button.m_size.cy;
			updata_page_layout(tab_view, &mut tp, left, top, width, height, tw, th);
			left = left + tw;
			top = top + th;
		}else{
			tp.m_view.m_visible = false;
		}
		M_VIEW_MAP.lock().unwrap().insert(tp.m_view.m_id, tp.m_view.clone());
		M_VIEW_MAP.lock().unwrap().insert(tp.m_header_button.m_id, tp.m_header_button.clone());
	}
}

pub fn draw_tabview_border(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, tab_view:&mut FCTabView, clip_rect:FCRect){
	if(tab_view.m_under_line_color != "none"){
        for i in 0..tab_view.m_tab_pages.len(){
	        let tp = tab_view.m_tab_pages[i].clone();
	        if(tp.m_view.m_visible){
	            let header_button = tp.m_header_button;
	            let mut location = FCPoint{x:header_button.m_location.x, y:header_button.m_location.y};
	            let size = header_button.m_size;
	            if(tab_view.m_use_animation){
					location.x = tab_view.m_under_point.x;
                    location.y = tab_view.m_under_point.y;
                }
	            if(tab_view.m_layout == "bottom"){
	                M_PAINT.lock().unwrap().fill_rect(&context, tab_view.m_under_line_color.clone(), location.x, location.y, location.x + size.cx, location.y + tab_view.m_under_line_size);
                }else if(tab_view.m_layout == "left"){
                    M_PAINT.lock().unwrap().fill_rect(&context, tab_view.m_under_line_color.clone(), location.x + size.cx - tab_view.m_under_line_size, location.y, location.x + size.cx, location.y + size.cy);
                }else if(tab_view.m_layout == "top"){
                    M_PAINT.lock().unwrap().fill_rect(&context, tab_view.m_under_line_color.clone(), location.x, location.y + size.cy - tab_view.m_under_line_size, location.x + size.cx, location.y + size.cy);
                }
                else if(tab_view.m_layout == "right"){
                    M_PAINT.lock().unwrap().fill_rect(&context, tab_view.m_under_line_color.clone(), location.x, location.y, location.x + tab_view.m_under_line_size, location.y + size.cy);
                }
	            break;
	        }
        }
    }
}