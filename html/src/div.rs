pub fn draw_div_border(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, div:&mut FCView, clip_rect:FCRect){
	if div.m_border_color != "none"{
        M_PAINT.lock().unwrap().draw_rect(&context, div.m_border_color.clone(), 1.0, Vec::new(), 0.0, 0.0, div.m_size.cx, div.m_size.cy);
    }
}

pub fn draw_div(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, div:&mut FCView, clip_rect:FCRect){
	if div.m_back_color != "none"{
        M_PAINT.lock().unwrap().fill_rect(&context, div.m_back_color.clone(), 0.0, 0.0, div.m_size.cx, div.m_size.cy);
    }
}

pub fn get_div_content_width(div:&mut FCView)->f32{
	let mut c_width : f32 = 0.0;
	let sub_views = get_sub_views(div.clone());
	if sub_views.len() > 0{
		for i in 0..sub_views.len(){
			let sub_view = &sub_views[i];
			if sub_view.m_visible {
			    if c_width < sub_view.m_location.x + sub_view.m_size.cx{
			        c_width = sub_view.m_location.x + sub_view.m_size.cx;
			    }
		    }
		}
	}
	return c_width;
}

pub fn get_div_content_height(div:&mut FCView)->f32{
	let mut c_height : f32 = 0.0;
	let sub_views = get_sub_views(div.clone());
	if sub_views.len() > 0{
		for i in 0..sub_views.len(){
			let sub_view = &sub_views[i];
			if sub_view.m_visible {
			    if c_height < sub_view.m_location.y + sub_view.m_size.cy{
			        c_height = sub_view.m_location.y + sub_view.m_size.cy;
			    }
		    }
		}
	}
	return c_height;
}

pub fn mouse_wheel_div(div:&mut FCView, delta:i32){
	let mut old_scroll_v = div.m_scroll_v;
    if delta > 0 {
	    old_scroll_v = old_scroll_v - 10.0;
    } else if delta < 0 {
	    old_scroll_v = old_scroll_v + 10.0;
    }
    let content_height = get_div_content_height(div);
    if content_height < div.m_size.cy {
        div.m_scroll_v = 0.0;
    } else {
        if old_scroll_v < 0.0 {
	        old_scroll_v = 0.0;
	    } else if old_scroll_v > content_height - div.m_size.cy {
		    old_scroll_v = content_height - div.m_size.cy;
        }
        div.m_scroll_v = old_scroll_v;
    }
}

pub fn draw_div_scroll_bar(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, div:&mut FCView, clip_rect:FCRect){
	if div.m_show_hscrollbar {
		let content_width = get_div_content_width(div);
		if content_width > div.m_size.cx {
			let s_left = div.m_scroll_h / content_width * div.m_size.cx;
			let mut s_right = (div.m_scroll_h + div.m_size.cx) / content_width * div.m_size.cx;
			if s_right - s_left < div.m_scroll_size {
				s_right = s_left + div.m_scroll_size;
			}
			M_PAINT.lock().unwrap().fill_rect(&context, div.m_scroll_barcolor.clone(), s_left, div.m_size.cy - div.m_scroll_size, s_right, div.m_size.cy);
		}
	}
	if div.m_show_vscrollbar{
	    let content_height = get_div_content_height(div);		
		if content_height > div.m_size.cy {
			let s_top = div.m_scroll_v / content_height * div.m_size.cy;
			let mut s_bottom  = s_top + (div.m_size.cy / content_height * div.m_size.cy);
			if s_bottom  - s_top < div.m_scroll_size {
				s_bottom  = s_top + div.m_scroll_size;
			}
			M_PAINT.lock().unwrap().fill_rect(&context, div.m_scroll_barcolor.clone(), div.m_size.cx - div.m_scroll_size, s_top, div.m_size.cx, s_bottom );
		}
	}
}

pub fn mouse_up_div(div:&mut FCView, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	div.m_down_scroll_hbutton = false;
	div.m_down_scroll_vbutton = false;
}

pub fn mouse_move_div(div:&mut FCView, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	if first_touch {
		let mp = first_point.clone();
		if div.m_show_hscrollbar || div.m_show_vscrollbar {
			if div.m_down_scroll_hbutton {
				let content_width = get_div_content_width(div);
				let sub_x = (mp.x - div.m_start_point.x) / div.m_size.cx * content_width;
				let mut new_scrollh = div.m_start_scroll_h + sub_x;
				if new_scrollh < 0.0 {
					new_scrollh = 0.0;
				} else if new_scrollh > content_width - div.m_size.cx {
					new_scrollh = content_width - div.m_size.cx;
				}
				div.m_scroll_h = new_scrollh;
				unsafe{
					M_CANCEL_CLICK = true;
				}
				return;

			} else if div.m_down_scroll_vbutton {
				let content_height = get_div_content_height(div);
				let sub_y = (mp.y - div.m_start_point.y) / div.m_size.cy * content_height;
				let mut new_scroll_v = div.m_start_scroll_v + sub_y;
				if new_scroll_v < 0.0 {
					new_scroll_v = 0.0;
				} else if new_scroll_v > content_height - div.m_size.cy {
					new_scroll_v = content_height - div.m_size.cy;
				}
				div.m_scroll_v = new_scroll_v;
				unsafe{
					M_CANCEL_CLICK = true;
				}
				return;
			}
		}
		if div.m_allow_drag_scroll {
			let content_width = get_div_content_width(div);
			if content_width > div.m_size.cx {
				let sub_x = div.m_start_point.x - mp.x;
				let mut new_scrollh = div.m_start_scroll_h + sub_x;
				if new_scrollh < 0.0 {
					new_scrollh = 0.0;
				} else if new_scrollh > content_width - div.m_size.cx {
					new_scrollh = content_width - div.m_size.cx;
				}
				div.m_scroll_h = new_scrollh;
				if sub_x.abs() > 5.0{
					unsafe{
						M_CANCEL_CLICK = true;
				    }
				}
			}
			let content_height = get_div_content_height(div);
			if content_height > div.m_size.cy {
				let sub_y = div.m_start_point.y - mp.y;
				let mut new_scroll_v = div.m_start_scroll_v + sub_y;
				if new_scroll_v < 0.0 {
					new_scroll_v = 0.0;
				} else if new_scroll_v > content_height - div.m_size.cy {
					new_scroll_v = content_height - div.m_size.cy;
				}
				div.m_scroll_v = new_scroll_v;
				if sub_y.abs() > 5.0{
					unsafe{
						M_CANCEL_CLICK = true;
				    }
				}
			}
		}
	}
}

pub fn mouse_down_div(div:&mut FCView, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	let mp = first_point.clone();
	div.m_start_point = mp.clone();
	div.m_down_scroll_hbutton = false;
	div.m_down_scroll_vbutton = false;
	if div.m_show_hscrollbar {
		let content_width = get_div_content_width(div);
		if content_width > div.m_size.cx {
		    let s_left = div.m_scroll_h / content_width * div.m_size.cx;
			let mut s_right = (div.m_scroll_h + div.m_size.cx) / content_width * div.m_size.cx;
			if s_right - s_left < div.m_scroll_size {
				s_right = s_left + div.m_scroll_size;
			}
			if mp.x >= s_left && mp.x <= s_right && mp.y >= div.m_size.cy - div.m_scroll_size && mp.y <= div.m_size.cy {
				div.m_down_scroll_hbutton = true;
				div.m_start_scroll_h = div.m_scroll_h;
				return;
			}
		}
	}
	if div.m_show_vscrollbar {
	    let content_height = get_div_content_height(div);
		if content_height > div.m_size.cy {
			let s_top = div.m_scroll_v / content_height * div.m_size.cy;
			let mut s_bottom  = (div.m_scroll_v + div.m_size.cy) / content_height * div.m_size.cy;
			if s_bottom  - s_top < div.m_scroll_size {
				s_bottom  = s_top + div.m_scroll_size;
			}
			if mp.x >= div.m_size.cx - div.m_scroll_size && mp.x <= div.m_size.cx && mp.y >= s_top && mp.y <= s_bottom  {
				div.m_down_scroll_vbutton = true;
				div.m_start_scroll_v = div.m_scroll_v;
				return;
			}
		}
	}
	if div.m_allow_drag_scroll {
		div.m_start_scroll_h = div.m_scroll_h;
		div.m_start_scroll_v = div.m_scroll_v;
	}
}