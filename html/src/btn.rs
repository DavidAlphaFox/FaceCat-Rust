#[derive(Clone)]
pub struct FCCheckBox{
	m_view:FCView,
	m_checked:bool,
	m_button_size:FCSize
	
}

impl FCCheckBox{	
	fn new()->Self{
		let view:FCView = FCView::new();
		Self{
			m_view:view,
			m_checked:false,
			m_button_size:FCSize{cx:16.0, cy:16.0}
		}
	}
}

#[derive(Clone)]
pub struct FCRadioButton{
	m_view:FCView,
	m_checked:bool,
	m_button_size:FCSize,
	m_group_name:String
}

impl FCRadioButton{
	fn new()->Self{
		let view:FCView = FCView::new();
		Self{
			m_view:view,
			m_checked:false,
			m_button_size:FCSize{cx:16.0, cy:16.0},
			m_group_name:String::from("")
		}
	}
}

#[derive(Clone)]
pub struct FCImage{
	m_view:FCView,
	m_src:String
}

impl FCImage{
	fn new()->Self{
		let view:FCView = FCView::new();
		Self{
			m_view:view,
			m_src:String::from("")
		}
	}
}

pub fn draw_check_box(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, check_box:&mut FCCheckBox, clip_rect:FCRect){
	let width = check_box.m_view.m_size.cx;
	let height = check_box.m_view.m_size.cy;
    if(check_box.m_view.m_text_color != "none"){
        let e_right = check_box.m_button_size.cx + 10.0;
        let mut e_rect = FCRect{left:1.0, top:(height - check_box.m_button_size.cy) / 2.0, right:check_box.m_button_size.cx + 1.0, bottom:(height + check_box.m_button_size.cy) / 2.0};
        M_PAINT.lock().unwrap().draw_rect(&context, check_box.m_view.m_text_color.clone(), 1.0, Vec::new(), e_rect.left, e_rect.top, e_rect.right, e_rect.bottom);
        if(check_box.m_checked){
            e_rect.left += 2.0;
            e_rect.top += 2.0;
            e_rect.right -= 2.0;
            e_rect.bottom -= 2.0;
            M_PAINT.lock().unwrap().fill_rect(&context, check_box.m_view.m_text_color.clone(), e_rect.left, e_rect.top, e_rect.right, e_rect.bottom);
        }
        M_PAINT.lock().unwrap().draw_text(&context, check_box.m_view.m_text.clone(), check_box.m_view.m_text_color.clone(), check_box.m_view.m_font.clone(), e_right, height / 2.0 + 1.0);
    }
}

pub fn draw_radio_button(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, radio_button:&mut FCRadioButton, clip_rect:FCRect){
	let width = radio_button.m_view.m_size.cx;
	let height = radio_button.m_view.m_size.cy;
	if(radio_button.m_view.m_text_color != "none"){
		let e_right = radio_button.m_button_size.cx + 10.0;
        let mut e_rect = FCRect{left:1.0, top:(height - radio_button.m_button_size.cy) / 2.0, right:radio_button.m_button_size.cx + 1.0, bottom:(height + radio_button.m_button_size.cy) / 2.0};
        M_PAINT.lock().unwrap().draw_ellipse(&context, radio_button.m_view.m_text_color.clone(), 1.0, Vec::new(), e_rect.left, e_rect.top, e_rect.right, e_rect.bottom);
        if(radio_button.m_checked){
            e_rect.left += 2.0;
            e_rect.top += 2.0;
            e_rect.right -= 2.0;
            e_rect.bottom -= 2.0;
            M_PAINT.lock().unwrap().fill_ellipse(&context, radio_button.m_view.m_text_color.clone(), e_rect.left, e_rect.top, e_rect.right, e_rect.bottom);
        }
        M_PAINT.lock().unwrap().draw_text(&context, radio_button.m_view.m_text.clone(), radio_button.m_view.m_text_color.clone(), radio_button.m_view.m_font.clone(), e_right, height / 2.0 + 1.0);
	}
}

pub fn draw_button(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, button:&mut FCView, clip_rect:FCRect){
	unsafe{
		if(M_MOUSE_DOWN_VIEW == button.m_id){
			M_PAINT.lock().unwrap().fill_rect(&context, button.m_pushed_color.clone(), 0.0, 0.0, button.m_size.cx, button.m_size.cy);
		}else if(M_MOUSE_MOVE_VIEW == button.m_id){
			M_PAINT.lock().unwrap().fill_rect(&context, button.m_hovered_color.clone(), 0.0, 0.0, button.m_size.cx, button.m_size.cy);
		}
		else if(button.m_back_color != "none"){
			M_PAINT.lock().unwrap().fill_rect(&context, button.m_back_color.clone(), 0.0, 0.0, button.m_size.cx, button.m_size.cy);
		}
		if(button.m_text_color != "none"){
			let t_size = M_PAINT.lock().unwrap().text_size(&context, button.m_text.clone(), button.m_font.clone());
			M_PAINT.lock().unwrap().draw_text(&context, button.m_text.clone(), button.m_text_color.clone(), button.m_font.clone(), (button.m_size.cx - t_size.cx) / 2.0, button.m_size.cy / 2.0 + 1.0);
		}
		if(button.m_border_color != "none"){
			M_PAINT.lock().unwrap().draw_rect(&context, button.m_border_color.clone(), 1.0, Vec::new(), 0.0, 0.0, button.m_size.cx, button.m_size.cy);
		}
	}
}

pub fn click_check_box(check_box:&mut FCCheckBox, mp:FCPoint){
	check_box.m_checked = !check_box.m_checked;
}

pub fn click_radio_button(radio_button:&mut FCRadioButton, mp:FCPoint){
	radio_button.m_checked = true;
}