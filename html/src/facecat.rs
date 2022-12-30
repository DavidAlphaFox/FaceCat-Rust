pub static mut M_NOW_ID:i32 = 0;
pub static mut M_MOUSE_DOWN_VIEW:i32 = -1;
pub static mut M_MOUSE_MOVE_VIEW:i32 = -1;
pub static mut M_FOCUSED_VIEW:i32 = -1;
pub static mut M_MOUSE_DOWN_POINT:FCPoint = FCPoint{x:0.0, y:0.0};
pub static mut M_TOUCH_POINT:FCPoint =FCPoint{x:0.0, y:0.0};
pub static mut M_FIRST_TOUCH:bool = false;
pub static mut M_SECOND_TOUCH:bool = false;
pub static mut M_TOUCH_FIRST_POINT:FCPoint = FCPoint{x:0.0, y:0.0};
pub static mut M_TOUCH_SECOND_POINT:FCPoint = FCPoint{x:0.0, y:0.0};
pub static mut M_RATIO:f32 = 1.0;
pub static mut M_CANCEL_CLICK:bool = false;
pub static mut M_IS_MOBILE:bool = false;
pub static mut M_CANVAS_WIDTH:f32 = 900.0;
pub static mut M_CANVAS_HEIGHT:f32 = 800.0;
pub static mut M_ADDING_PLOT:i32 = -1;

pub static mut M_DRAG_BEGIN_POINT:FCPoint = FCPoint{x:0.0, y:0.0};
pub static mut M_DRAG_BEGIN_RECT:FCRect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
pub static mut M_DRAGGING_VIEW:i32 = -1;

#[derive(Clone)]
pub struct FCPoint{
	x: f32,
	y: f32
}

#[derive(Clone)]
pub struct FCSize{
	cx: f32,
	cy: f32
}

#[derive(Clone)]
pub struct FCRect{
	left: f32,
	top: f32,
	right: f32,
	bottom: f32
}

#[derive(Clone)]
pub struct FCPadding{
	left: f32,
	top: f32,
	right: f32,
	bottom: f32
}

#[derive(Clone)]
pub struct FCPaint{
	m_move_to:bool,
	m_offset_x:f32,
	m_offset_y:f32,
	m_default_ui_style:String,
	m_scale_factor_x:f32,
	m_scale_factor_y:f32
}

impl FCPaint{
	fn new()->Self{
		Self{
			m_move_to:false, 
			m_offset_x:0.0, 
			m_offset_y:0.0, 
			m_default_ui_style:String::from("dark"),
			m_scale_factor_x:1.0, 
			m_scale_factor_y:1.0
		}
	}
	fn add_line(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, x1:f32, y1:f32, x2:f32, y2:f32){
		if !self.m_move_to {
			self.m_move_to = true;
			context.move_to(((x1 + self.m_offset_x) * self.m_scale_factor_x) as f64, ((y1 + self.m_offset_y) * self.m_scale_factor_y) as f64);
		}
		context.line_to(((x2 + self.m_offset_x) * self.m_scale_factor_x) as f64, ((y2 + self.m_offset_y) * self.m_scale_factor_y) as f64);
	}

	fn begin_path(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>){
		context.begin_path();
	}

	fn begin_paint(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>){
		self.m_move_to = false;
		self.m_offset_x = 0.0;
		self.m_offset_y = 0.0;
	}

	fn close_figure(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>){
		context.close_path();
	}

	fn close_path(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>){
		self.m_move_to = false;
	}

	fn draw_line(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, width:f32, style:Vec<f32>, x1:f32, y1:f32, x2:f32, y2:f32){
		if color != "none"{
			let mut l_width = self.m_scale_factor_x.min(self.m_scale_factor_y) * width;
			context.begin_path();
			if l_width < 1.0{
				l_width = 1.0;
			}
			context.set_line_width(l_width as f64);
			let c = JsValue::from(String::from(color));
			context.set_stroke_style(&c);  
		    
			context.move_to(((x1 + self.m_offset_x) * self.m_scale_factor_x) as f64, ((y1 + self.m_offset_y) * self.m_scale_factor_y) as f64);
			context.line_to(((x2 + self.m_offset_x) * self.m_scale_factor_x) as f64, ((y2 + self.m_offset_y) * self.m_scale_factor_y) as f64);
			context.stroke();
		}
	}

	fn draw_path(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, width:f32, style:Vec<f32>){
		if color != "none"{
			let mut l_width = self.m_scale_factor_x.min(self.m_scale_factor_y) * width;
			if l_width < 1.0{
				l_width = 1.0;
			}
			context.set_line_width(l_width as f64);
			let c = JsValue::from(String::from(color));
			context.set_stroke_style(&c);  
			context.stroke();
		}
	}

	fn draw_rect(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, width:f32, style:Vec<f32>, left:f32, top:f32, right:f32, bottom:f32){
		if color != "none"{
			let mut l_width = self.m_scale_factor_x.min(self.m_scale_factor_y) * width;
			let w = right - left;
			let h = bottom - top;
			context.begin_path();
			if l_width < 1.0{
				l_width = 1.0;
			}
			context.set_line_width(l_width as f64);
			let c = JsValue::from(String::from(color));
			context.set_stroke_style(&c);  
			context.stroke_rect(((left + self.m_offset_x) * self.m_scale_factor_x) as f64, ((top + self.m_offset_y) * self.m_scale_factor_y) as f64, (w * self.m_scale_factor_x) as f64, (h * self.m_scale_factor_y) as f64);
		}
	}

	fn draw_ellipse(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, width:f32, style:Vec<f32>, left:f32, top:f32, right:f32, bottom:f32){
		if color != "none"{
			let mut l_width = self.m_scale_factor_x.min(self.m_scale_factor_y) * width;
			let w = right - left;
			let h = bottom - top;
			context.begin_path();
			if l_width < 1.0{
				l_width = 1.0;
			}
			context.set_line_width(l_width as f64);
			let c = JsValue::from(String::from(color));
			context.set_stroke_style(&c);  
		    
			context.begin_path();
			context.ellipse((((left + (right - left) / 2.0) + self.m_offset_x) * self.m_scale_factor_x) as f64, (((top + (bottom - top) / 2.0) + self.m_offset_y) * self.m_scale_factor_y) as f64, ((w / 2.0) * self.m_scale_factor_x) as f64, ((h / 2.0) * self.m_scale_factor_y) as f64, 0.0, 0.0, core::f64::consts::PI * 2.0);
			context.stroke();
		}
	}

	fn draw_text(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, text:String, color:String, font:String, x:f32, y:f32){
		if color != "none"{
			context.set_font(&font);
			let c = JsValue::from(String::from(color));
			context.set_fill_style(&c);  
			context.set_text_align("left");
			context.set_text_baseline("middle");
			context.fill_text(&text, ((x + self.m_offset_x) * self.m_scale_factor_x) as f64,  ((y + self.m_offset_y) * self.m_scale_factor_y) as f64);
		}
	}
	
	fn draw_text_auto_ellipsis(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, text:String, color:String, font:String, left:f32, top:f32, right:f32, bottom:f32){
		if color != "none"{
			context.set_font(&font);
			let c = JsValue::from(String::from(color));
			context.set_fill_style(&c);  
			context.set_text_align("left");
			context.set_text_baseline("middle");
			context.fill_text(&text, ((left + self.m_offset_x) * self.m_scale_factor_x) as f64,  ((top + self.m_offset_y) * self.m_scale_factor_y) as f64);
		}
	}

	fn end_paint(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>){
	}

	fn fill_path(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String){
		if color != "none"{
			let c = JsValue::from(String::from(color));
			context.set_fill_style(&c);  
			context.fill();
		}
	}

	fn fill_rect(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, left:f32, top:f32, right:f32, bottom:f32){
		if color != "none"{
			let c = JsValue::from(String::from(color));
			context.set_fill_style(&c);
			let w = right - left;
			let h = bottom - top;  
			context.fill_rect(((left + self.m_offset_x) * self.m_scale_factor_x) as f64, ((top + self.m_offset_y) * self.m_scale_factor_y) as f64, (w * self.m_scale_factor_x) as f64, (h * self.m_scale_factor_y) as f64);
		}
	}

	fn fill_ellipse(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, left:f32, top:f32, right:f32, bottom:f32){
		if color != "none"{
			let c = JsValue::from(String::from(color));
			context.set_fill_style(&c);
			let w = right - left;
			let h = bottom - top;  
			context.begin_path();
			context.ellipse((((left + (right - left) / 2.0) + self.m_offset_x) * self.m_scale_factor_x) as f64, (((top + (bottom - top) / 2.0) + self.m_offset_y) * self.m_scale_factor_y) as f64, ((w / 2.0) * self.m_scale_factor_x) as f64, ((h / 2.0) * self.m_scale_factor_y) as f64, 0.0, 0.0, core::f64::consts::PI * 2.0);
			context.fill();
		}
	}

	fn set_clip(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, left:f32, top:f32, right:f32, bottom:f32){
		let w = right - left;
		let h = bottom - top;
		context.begin_path(); 
		context.rect(((left + self.m_offset_x) * self.m_scale_factor_x) as f64, ((top + self.m_offset_y) * self.m_scale_factor_y) as f64, (w * self.m_scale_factor_x) as f64, (h * self.m_scale_factor_y) as f64);
		context.clip();
	}

	fn set_offset(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, offset_x:f32, offset_y:f32){
		self.m_offset_x = offset_x;
		self.m_offset_y = offset_y;
	}

	fn text_size(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, text:String, font:String)->FCSize{
		if text.len() > 0 {
		    context.set_font(&font);
		    let metrics = context.measure_text(&text).expect("REASON");
		    let actual_height = metrics.actual_bounding_box_ascent() + metrics.actual_bounding_box_descent(); 
		    let t_size = FCSize{cx:metrics.width() as f32, cy:actual_height as f32};
		    return t_size;
		}else{
			return FCSize{cx:0.0, cy:0.0};
		}
	}

	fn save(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>){
		context.save();
	}

	fn restore(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>){
		context.restore();
	}
}

#[derive(Clone)]
pub struct FCView{
	m_back_color:String,
	m_border_color:String,
	m_text_color:String,
	m_location:FCPoint,
	m_name:String,
	m_size:FCSize,
	m_text:String,
	m_visible:bool,
    m_scroll_v:f32,
	m_scroll_h:f32,
	m_scroll_size:f32,
	m_show_hscrollbar:bool,
	m_show_vscrollbar:bool,
	m_scroll_barcolor:String,
	m_allow_drag_scroll:bool,
	m_down_scroll_hbutton:bool,
	m_down_scroll_vbutton:bool,
	m_start_scroll_h:f32,
	m_start_scroll_v:f32,
	m_start_point:FCPoint,
	m_mouse_down_time:f64,
	m_display_offset:bool,
	m_padding:FCPadding,
	m_margin:FCPadding,
	m_dock:String,
	m_back_image:String,
	m_top_most:bool,
	m_has_clip:bool,
	m_clip_rect:FCRect,
	m_font:String,
	m_type:String,
	m_id:i32,
	m_hovered_color:String,
	m_pushed_color:String,
	m_allow_drag:bool
}

impl FCView{
	fn new()->Self{
		Self{
		m_back_color:String::from("rgb(255,255,255)"), 
		m_border_color:String::from("rgb(100,100,100)"),
		m_text_color:String::from("rgb(0,0,0)"),
		m_location:FCPoint{x:0.0,y:0.0},
		m_name:String::from(""),
		m_size:FCSize{cx:0.0, cy:0.0},
		m_text:String::from(""),
		m_visible:true,
		m_scroll_v:0.0,
		m_scroll_h:0.0,
		m_scroll_size:8.0,
		m_show_hscrollbar:false,
		m_show_vscrollbar:false,
		m_scroll_barcolor:String::from("rgb(100,100,100)"),
		m_allow_drag_scroll:false,
		m_down_scroll_hbutton:false,
		m_down_scroll_vbutton:false,
		m_start_scroll_h:0.0,
		m_start_scroll_v:0.0,
		m_start_point:FCPoint{x:0.0,y:0.0},
		m_mouse_down_time:0.0,
		m_display_offset:true,
		//M_PAINT:FCPaint,
		m_padding:FCPadding{left:0.0, top:0.0, right:0.0, bottom:0.0},
		m_margin:FCPadding{left:0.0, top:0.0, right:0.0, bottom:0.0},
		m_dock:String::from("none"),
		m_back_image:String::from(""),
		m_top_most:false,
		m_has_clip:false,
		m_clip_rect:FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0},
		m_font:String::from("12px Arial"),
		m_type:String::from(""),
		m_id:-1,
		m_hovered_color:String::from("none"),
		m_pushed_color:String::from("rgb(100,100,100)"),
		m_allow_drag:false
		}
	}
}

pub fn to_fixed(value:f64, digit:i32)->String{
	if digit == 1{
		return format!("{:.1}", value);
	}else if digit == 2{
		return format!("{:.2}", value);
	}else if digit == 3{
		return format!("{:.3}", value);
	}else if digit == 4{
		return format!("{:.4}", value);
	}else if digit == 5{
		return format!("{:.5}", value);
	}else if digit == 6{
		return format!("{:.6}", value);
	}else if digit == 7{
		return format!("{:.7}", value);
	}else if digit == 8{
		return format!("{:.8}", value);
	}else if digit == 9{
		return format!("{:.9}", value);
	}else{
		return (value as i64).to_string();
	}
}

pub fn create_new_id()->i32{
	let mut id:i32 = 0;
	unsafe{
		M_NOW_ID = M_NOW_ID + 1;
		id = M_NOW_ID;
    }
    return id;
}

pub fn add_view(view:FCView)->i32{
	let new_id = create_new_id();
	let mut add_view = view.clone();
	add_view.m_id = new_id;
	M_VIEW_MAP.lock().unwrap().insert(new_id, add_view);
	return new_id;
}

pub fn add_view_to_parent(view:FCView, parent:FCView)->i32{
	let new_id = create_new_id();
	let mut add_view = view.clone();
	add_view.m_id = new_id;
	let parent_id = parent.m_id;
	M_VIEW_MAP.lock().unwrap().insert(new_id, add_view);
	M_PARENT_VIEW_MAP.lock().unwrap().insert(new_id, parent_id);
	return new_id;
}

pub fn get_sub_views(parent:FCView)->Vec<FCView>{
	let mut sub_views:Vec<FCView> = Vec::new();
	for (k, v) in &*M_VIEW_MAP.lock().unwrap() {
		match M_PARENT_VIEW_MAP.lock().unwrap().get(&k) {
			Some(x) => {
				if x == &parent.m_id{
					sub_views.push(v.clone());
				}
			},
			None => {
			}
		}
    }
	return sub_views;
}

pub fn get_top_views()->Vec<FCView>{
	let mut sub_views:Vec<FCView> = Vec::new();
	for (k, v) in &*M_VIEW_MAP.lock().unwrap() {
		match M_PARENT_VIEW_MAP.lock().unwrap().get(&k) {
			Some(x) => {
			},
			None => {
				sub_views.push(v.clone());
			}
		}
    }
	return sub_views;
}

pub fn client_x(view:FCView)->f32{
	if view.m_id != -1{
		let c_left = view.m_location.x;
		let mut find = false;
		match M_PARENT_VIEW_MAP.lock().unwrap().get(&view.m_id) {
			Some(x) => {
				find = true;	
			},
			None => {
				find = false;
			}
		}
		if find{
			let p_id = M_PARENT_VIEW_MAP.lock().unwrap()[&view.m_id];
			let parent = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
			if parent.m_display_offset {
				return c_left - parent.m_scroll_h + client_x(parent);
			}else{
				return c_left + client_x(parent);
			}
		}else{
			return c_left;
		}
	}else{
		return 0.0;
	}
}

pub fn client_y(view:FCView)->f32{
	if view.m_id != -1 {
		let c_top = view.m_location.y;
		let mut find = false;
		match M_PARENT_VIEW_MAP.lock().unwrap().get(&view.m_id) {
			Some(x) => {
				find = true;	
			},
			None => {
				find = false;
				
			}
		}
		if find {
			let p_id = M_PARENT_VIEW_MAP.lock().unwrap()[&view.m_id];
			let parent = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
			if parent.m_display_offset{
				return c_top - parent.m_scroll_v + client_y(parent);
			}else{
				return c_top + client_y(parent);
			}
		}else{
			return c_top;
		}
	}else{
		return 0.0;
	}
}

pub fn contains_point(view:FCView, mp:FCPoint)->bool{
	let clx = client_x(view.clone());
	let cly = client_y(view.clone());
	let size = view.m_size;
	let cp = FCPoint{x:mp.x - clx, y:mp.y - cly};
	if cp.x >= 0.0 && cp.x <= size.cx &&
		cp.y >= 0.0 && cp.y <= size.cy {
		return true;
	} else {
		return false;
	}
}

pub fn find_view_by_name(name:String, views:Vec<FCView>)->FCView{
	let none_view = M_NONE_VIEW.lock().unwrap().clone();
	let copy_name = name.clone();
	 for i in 0..views.len(){
		let view = &views[i];
		if view.m_name == copy_name{
		    return view.clone();
		}else{
			let sub_views = get_sub_views(view.clone());
			if sub_views.len() > 0{
				let sub_view = find_view_by_name(name.clone(), sub_views);
				if sub_view.m_id != -1 {
					return sub_view;
				}
			}
		}
	}
	return none_view;
}

pub fn is_paint_visible(view:FCView)->bool{
	if view.m_visible{
		let mut find = false;
        match M_PARENT_VIEW_MAP.lock().unwrap().get(&view.m_id) {
			Some(x) => {
				find = true;
			},
			None => {
				find = false;
			}
		}
		if find{
			let p_id = M_PARENT_VIEW_MAP.lock().unwrap()[&view.m_id];
			let parent = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
			if parent.m_visible {
				return is_paint_visible(parent);
			}else{
				return false;
			}
		}else{
			return true;
		}
    }else{
        return false;
     }
}

pub fn get_intersect_rect(lp_dest_rect:&mut FCRect, lp_src1_rect:FCRect, lp_src2_rect:FCRect)->i32{
	lp_dest_rect.left = lp_src1_rect.left.max(lp_src2_rect.left);
    lp_dest_rect.right = lp_src1_rect.right.min(lp_src2_rect.right);
    lp_dest_rect.top = lp_src1_rect.top.max(lp_src2_rect.top);
    lp_dest_rect.bottom = lp_src1_rect.bottom.min(lp_src2_rect.bottom);
    if lp_dest_rect.right > lp_dest_rect.left && lp_dest_rect.bottom > lp_dest_rect.top{
        return 1;
    }
    else{
        lp_dest_rect.left = 0.0;
        lp_dest_rect.right = 0.0;
        lp_dest_rect.top = 0.0;
        lp_dest_rect.bottom = 0.0;
        return 0;
    }
}

pub fn click(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, div: &web_sys::HtmlElement, c: &str) {
    /*let context = context.clone();
    let c = JsValue::from(String::from(c));
    let closure = Closure::wrap(Box::new(move || {           
    }) as Box<dyn FnMut()>);

    div.set_onclick(Some(closure.as_ref().unchecked_ref()));
    closure.forget();*/
}

pub fn touch_start(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
        unsafe{
			M_IS_MOBILE = true;
			if M_IS_MOBILE {
				let touches = event.touches();
				let touch1 = touches.get(0).expect("REASON");
				let mp = FCPoint{x:touch1.client_x() as f32, y:touch1.client_y() as f32};
				M_TOUCH_POINT = FCPoint{x:mp.x, y:mp.y};
				M_MOUSE_DOWN_POINT = FCPoint{x:mp.x, y:mp.y};
				let top_views = get_top_views();
				let mouse_down_view = find_view(M_MOUSE_DOWN_POINT.clone(), top_views);
				M_MOUSE_DOWN_VIEW = mouse_down_view.m_id;
				if M_MOUSE_DOWN_VIEW != -1{
					M_FOCUSED_VIEW = M_MOUSE_DOWN_VIEW;
					M_FIRST_TOUCH = false;
					M_SECOND_TOUCH = false;
					M_TOUCH_FIRST_POINT = FCPoint{x:0.0, y:0.0};
					M_TOUCH_SECOND_POINT = FCPoint{x:0.0, y:0.0};
					let clx = client_x(mouse_down_view.clone());
					let cly = client_y(mouse_down_view.clone());
					if touches.length() >= 1 {
						M_FIRST_TOUCH = true;
						M_MOUSE_DOWN_POINT = FCPoint{x:touch1.client_x() as f32, y:touch1.client_y() as f32};
						M_TOUCH_FIRST_POINT = FCPoint{x:mp.x, y:mp.y};
						M_TOUCH_FIRST_POINT.x = M_TOUCH_FIRST_POINT.x - clx;
						M_TOUCH_FIRST_POINT.y = M_TOUCH_FIRST_POINT.y - cly;
					}
					if touches.length() >= 2 {
						let touch2 = touches.get(1).expect("REASON");
						M_SECOND_TOUCH = true;
						M_TOUCH_SECOND_POINT = FCPoint{x:touch2.client_x() as f32, y:touch2.client_y() as f32};
						M_TOUCH_SECOND_POINT.x = M_TOUCH_SECOND_POINT.x - clx;
						M_TOUCH_SECOND_POINT.y = M_TOUCH_SECOND_POINT.y - cly;
					}
					on_touch_start(&context, mouse_down_view.clone(), M_FIRST_TOUCH, M_SECOND_TOUCH, M_TOUCH_FIRST_POINT.clone(), M_TOUCH_SECOND_POINT.clone());
				}
			}
		}
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn touch_move(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
        unsafe{
			if M_IS_MOBILE{
				if M_MOUSE_DOWN_VIEW != -1{
					let mouse_down_view = M_VIEW_MAP.lock().unwrap()[&M_MOUSE_DOWN_VIEW].clone();
					let touches = event.touches();
					let touch1 = touches.get(0).expect("REASON");
					M_FIRST_TOUCH = false;
					M_SECOND_TOUCH = false;
					M_TOUCH_FIRST_POINT = FCPoint{x:0.0, y:0.0};
					M_TOUCH_SECOND_POINT = FCPoint{x:0.0, y:0.0};
					let clx = client_x(mouse_down_view.clone());
					let cly = client_y(mouse_down_view.clone());
					let mp = FCPoint{x:touch1.client_x() as f32, y:touch1.client_y() as f32};
					if touches.length() >= 1 {
						M_FIRST_TOUCH = true;
						M_TOUCH_FIRST_POINT = FCPoint{x:touch1.client_x() as f32, y:touch1.client_y() as f32};
						M_TOUCH_POINT = M_TOUCH_FIRST_POINT.clone();
						M_TOUCH_FIRST_POINT.x = M_TOUCH_FIRST_POINT.x - clx;
						M_TOUCH_FIRST_POINT.y = M_TOUCH_FIRST_POINT.y - cly;
					}
					if touches.length() >= 2 {
						let touch2 = touches.get(1).expect("REASON");
						M_SECOND_TOUCH = true;
						M_TOUCH_SECOND_POINT = FCPoint{x:touch2.client_x() as f32, y:touch2.client_y() as f32};
						M_TOUCH_SECOND_POINT.x = M_TOUCH_SECOND_POINT.x - clx;
						M_TOUCH_SECOND_POINT.y = M_TOUCH_SECOND_POINT.y - cly;
					}
					on_touch_move(&context, mouse_down_view.clone(), M_FIRST_TOUCH, M_SECOND_TOUCH, M_TOUCH_FIRST_POINT.clone(), M_TOUCH_SECOND_POINT.clone());
					if mouse_down_view.m_allow_drag{
						if (mp.x - M_MOUSE_DOWN_POINT.x).abs() > 5.0 || (mp.y - M_MOUSE_DOWN_POINT.y).abs() > 5.0 {
							M_DRAG_BEGIN_POINT = FCPoint{x:M_MOUSE_DOWN_POINT.x, y:M_MOUSE_DOWN_POINT.y};
							M_DRAGGING_VIEW = M_MOUSE_DOWN_VIEW;
							M_DRAG_BEGIN_RECT = FCRect{left:mouse_down_view.m_location.x, top:mouse_down_view.m_location.y,
							right:mouse_down_view.m_location.x + mouse_down_view.m_size.cx,
							bottom:mouse_down_view.m_location.y + mouse_down_view.m_size.cy};
							M_MOUSE_DOWN_VIEW = -1;
						}
					}
				} else if M_DRAGGING_VIEW != -1{
					let touches = event.touches();
					let touch1 = touches.get(0).expect("REASON");
					let mp = FCPoint{x:touch1.client_x() as f32, y:touch1.client_y() as f32};
					let offsetX:f32 = mp.x - M_DRAG_BEGIN_POINT.x;
					let offsetY:f32 = mp.y - M_DRAG_BEGIN_POINT.y;
					let newBounds = FCRect{left:M_DRAG_BEGIN_RECT.left + offsetX, top:M_DRAG_BEGIN_RECT.top + offsetY,
						right:M_DRAG_BEGIN_RECT.right + offsetX, bottom:M_DRAG_BEGIN_RECT.bottom + offsetY};
					let mut dragging_view = M_VIEW_MAP.lock().unwrap()[&M_DRAGGING_VIEW].clone();
					dragging_view.m_location = FCPoint{x:newBounds.left, y:newBounds.top};
					M_VIEW_MAP.lock().unwrap().insert(dragging_view.m_id, dragging_view.clone());
					let mut find:bool = false;
					let mut p_id:i32 = 0;
					match M_PARENT_VIEW_MAP.lock().unwrap().get(&dragging_view.m_id) {
						Some(x) => {
							find = true;
							p_id = *x;	
						},
						None => {
							find = false;
						}
					}
					if find{
						let parentView = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
						if parentView.m_type == "split"{
							for (id, v) in M_SPLIT_MAP.lock().unwrap().iter_mut(){
								if parentView.m_id == *id{
									(*v).m_splitter = dragging_view.clone();
									reset_split_layout_div(&mut *v);
								}
							}
						}
						invalidate_view(&context, parentView.clone());
					}else{
						invalidate(&context);
					}
				}
			}
		}
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn touch_end(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::TouchEvent| {
        unsafe{
			if M_IS_MOBILE{
				if M_MOUSE_DOWN_VIEW != -1{
					let mouse_down_view = M_VIEW_MAP.lock().unwrap()[&M_MOUSE_DOWN_VIEW].clone();
					let top_views = get_top_views();
					let view = find_view(M_TOUCH_POINT.clone(), top_views);
					if view.m_id == M_MOUSE_DOWN_VIEW{
						on_click(&context, mouse_down_view.clone(), M_TOUCH_FIRST_POINT.clone(), 1, 1, 0);
					}
					on_touch_end(&context, mouse_down_view.clone(), M_FIRST_TOUCH, M_SECOND_TOUCH, M_TOUCH_FIRST_POINT.clone(), M_TOUCH_SECOND_POINT.clone());
					M_MOUSE_DOWN_VIEW = -1;
				}
				M_DRAGGING_VIEW = -1;
			}
		}
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn mouse_wheel(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::WheelEvent| {
        unsafe{
			if !M_IS_MOBILE{
				let mp = FCPoint{x:event.offset_x() as f32, y:event.offset_y() as f32};
				let top_views = get_top_views();
				let find_view = find_view(mp.clone(), top_views);
				let cmp  = FCPoint{x:mp.x - client_x(find_view.clone()), y:mp.y - client_y(find_view.clone())};
				let delta_y = event.delta_y();
				let mut delta:i32 = 0;
				if delta_y > 0.0{
					delta = -1;
				}else if delta_y < 0.0{
					delta = 1;
				}
				on_mouse_wheel(&context, find_view.clone(), cmp.clone(), 0, 0, delta);
			}
		}
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousewheel", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn mouse_up(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement) {
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        pressed.set(false);
        unsafe{
			if !M_IS_MOBILE{
				if M_MOUSE_DOWN_VIEW != -1{
					let mp = FCPoint{x:event.offset_x() as f32, y:event.offset_y() as f32};
					let mouse_down_view = M_VIEW_MAP.lock().unwrap()[&M_MOUSE_DOWN_VIEW].clone();
					let cmp  = FCPoint{x:mp.x - client_x(mouse_down_view.clone()), y:mp.y - client_y(mouse_down_view.clone())};
					let top_views = get_top_views();
					let find_view = find_view(mp.clone(), top_views);
					if find_view.m_id == M_MOUSE_DOWN_VIEW{
						on_click(&context, mouse_down_view.clone(), cmp.clone(), 1, 1, 0);
					}
					M_MOUSE_DOWN_VIEW = -1;
					on_mouse_up(&context, mouse_down_view.clone(), cmp.clone(), 1, 1, 0);
				}
				M_DRAGGING_VIEW = -1;
			}
		}
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn mouse_move(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement){
    let context = context.clone();
    let pressed = pressed.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        unsafe{
			if !M_IS_MOBILE{
				let mp = FCPoint{x:event.offset_x() as f32, y:event.offset_y() as f32};
				if M_MOUSE_DOWN_VIEW != -1{
					let mouse_down_view = M_VIEW_MAP.lock().unwrap()[&M_MOUSE_DOWN_VIEW].clone();
					M_MOUSE_MOVE_VIEW = mouse_down_view.m_id;
					let cmp = FCPoint{x:mp.x - client_x(mouse_down_view.clone()), y:mp.y - client_y(mouse_down_view.clone())};
					on_mouse_move(&context, mouse_down_view.clone(), cmp.clone(), 1, 1, 0);
					if mouse_down_view.m_allow_drag{
						if ((mp.x - M_MOUSE_DOWN_POINT.x).abs() > 5.0 || (mp.y - M_MOUSE_DOWN_POINT.y).abs() > 5.0) {
							M_DRAG_BEGIN_POINT = FCPoint{x:M_MOUSE_DOWN_POINT.x, y:M_MOUSE_DOWN_POINT.y};
							M_DRAGGING_VIEW = M_MOUSE_DOWN_VIEW;
							M_DRAG_BEGIN_RECT = FCRect{left:mouse_down_view.m_location.x, top:mouse_down_view.m_location.y,
							right:mouse_down_view.m_location.x + mouse_down_view.m_size.cx,
							bottom:mouse_down_view.m_location.y + mouse_down_view.m_size.cy};
							M_MOUSE_DOWN_VIEW = -1;
						}
					}
				}
				else if M_DRAGGING_VIEW != -1{
					let offsetX:f32 = mp.x - M_DRAG_BEGIN_POINT.x;
					let offsetY:f32 = mp.y - M_DRAG_BEGIN_POINT.y;
					let newBounds = FCRect{left:M_DRAG_BEGIN_RECT.left + offsetX, top:M_DRAG_BEGIN_RECT.top + offsetY,
						right:M_DRAG_BEGIN_RECT.right + offsetX, bottom:M_DRAG_BEGIN_RECT.bottom + offsetY};
					let mut dragging_view = M_VIEW_MAP.lock().unwrap()[&M_DRAGGING_VIEW].clone();
					dragging_view.m_location = FCPoint{x:newBounds.left, y:newBounds.top};
					M_VIEW_MAP.lock().unwrap().insert(dragging_view.m_id, dragging_view.clone());
					let mut find:bool = false;
					let mut p_id:i32 = 0;
					match M_PARENT_VIEW_MAP.lock().unwrap().get(&dragging_view.m_id) {
						Some(x) => {
							find = true;
							p_id = *x;	
						},
						None => {
							find = false;
						}
					}
					if find {
						let parentView = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
						if parentView.m_type == "split"{
							for (id, v) in M_SPLIT_MAP.lock().unwrap().iter_mut(){
								if parentView.m_id == *id{
									(*v).m_splitter = dragging_view.clone();
									reset_split_layout_div(&mut *v);
								}
							}
						}
						invalidate_view(&context, parentView.clone());
					}else{
						invalidate(&context);
					}
				}
				else{
					let top_views = get_top_views();
					let find_view = find_view(mp.clone(), top_views);
					M_MOUSE_MOVE_VIEW = find_view.m_id;
					let cmp  = FCPoint{x:mp.x - client_x(find_view.clone()), y:mp.y - client_y(find_view.clone())};
					on_mouse_move(&context, find_view.clone(), cmp.clone(), 0, 0, 0);
				}
			}
		}
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

pub fn mouse_down(context: &std::rc::Rc<web_sys::CanvasRenderingContext2d>, pressed: &std::rc::Rc<std::cell::Cell<bool>>, canvas: &web_sys::HtmlCanvasElement){
    let pressed = pressed.clone();
    let context = context.clone();
    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        unsafe{
			if !M_IS_MOBILE{
				M_CANCEL_CLICK = false;
				M_MOUSE_DOWN_POINT = FCPoint{x:event.offset_x() as f32, y:event.offset_y() as f32};
				let top_views = get_top_views();
				let mouse_down_view = find_view(M_MOUSE_DOWN_POINT.clone(), top_views);
				M_MOUSE_DOWN_VIEW = mouse_down_view.m_id;
				if M_MOUSE_DOWN_VIEW != -1{
					M_FOCUSED_VIEW = M_MOUSE_DOWN_VIEW;
					let cmp  = FCPoint{x:M_MOUSE_DOWN_POINT.x - client_x(mouse_down_view.clone()), y:M_MOUSE_DOWN_POINT.y - client_y(mouse_down_view.clone())};
					on_mouse_down(&context, mouse_down_view.clone(), cmp.clone(), 1, 1, 0);
				}
			}
        }
        pressed.set(true);
    }) as Box<dyn FnMut(_)>);
    canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}

lazy_static! {
        pub static ref M_VIEW_MAP:Mutex<HashMap<i32,FCView>> = {
			let map:HashMap<i32,FCView> = HashMap::new();
			 Mutex::new(map)
        };
		pub static ref M_PARENT_VIEW_MAP:Mutex<HashMap<i32,i32>> = 
		{
			let map:HashMap<i32,i32> = HashMap::new();
			 Mutex::new(map)
		}; 
		pub static ref M_NONE_VIEW:Mutex<FCView> = {
			let view:FCView = FCView::new();
			 Mutex::new(view)
		};
		pub static ref M_PAINT:Mutex<FCPaint> = {
			let paint:FCPaint = FCPaint::new();
			 Mutex::new(paint)
		};
		
		pub static ref M_GRID_MAP:Mutex<HashMap<i32,FCGrid>> = {
			let grid_map:HashMap<i32,FCGrid> = HashMap::new();
			 Mutex::new(grid_map)
        };
        pub static ref M_CHART_MAP:Mutex<HashMap<i32,FCChart>> = {
			let chart_map:HashMap<i32,FCChart> = HashMap::new();
			 Mutex::new(chart_map)
        };
        pub static ref M_TAB_MAP:Mutex<HashMap<i32,FCTabView>> = {
			let tab_map:HashMap<i32,FCTabView> = HashMap::new();
			 Mutex::new(tab_map)
        };
        pub static ref M_LAYOUT_MAP:Mutex<HashMap<i32,FCLayoutDiv>> = {
			let layout_map:HashMap<i32,FCLayoutDiv> = HashMap::new();
			 Mutex::new(layout_map)
        };
        pub static ref M_SPLIT_MAP:Mutex<HashMap<i32,FCSplitLayoutDiv>> = {
			let split_map:HashMap<i32,FCSplitLayoutDiv> = HashMap::new();
			 Mutex::new(split_map)
        };
        pub static ref M_CHECK_BOX_MAP:Mutex<HashMap<i32,FCCheckBox>> = {
			let check_box_map:HashMap<i32,FCCheckBox> = HashMap::new();
			 Mutex::new(check_box_map)
        };
        pub static ref M_RADIO_BUTTON_MAP:Mutex<HashMap<i32,FCRadioButton>> = {
			let radio_button_map:HashMap<i32,FCRadioButton> = HashMap::new();
			 Mutex::new(radio_button_map)
        };
}

pub fn render_views(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, views:Vec<FCView>, rect:FCRect, has_rect:bool){
	let views_size = views.len();
	 for i in 0..views_size{
		let view = &views[i];
		let v_id = view.m_id;
		if !has_rect{
			let sub_views = get_sub_views(view.clone());
			let sub_views_size = sub_views.len();
		    if sub_views_size > 0 {
		        if sub_views_size > 0 {
			        render_views(context, sub_views, rect.clone(), false);
		        }
	        }
	        let mut change_view = M_VIEW_MAP.lock().unwrap()[&v_id].clone();
		    change_view.m_has_clip = false;
		    M_VIEW_MAP.lock().unwrap().insert(v_id, change_view);
		    continue;
		}
		if !view.m_top_most && is_paint_visible(view.clone()){
			let clx = client_x(view.clone());
			let cly = client_y(view.clone());
			let draw_rect = FCRect{left:0.0, top:0.0, right:view.m_size.cx, bottom:view.m_size.cy};
			let clip_rect = FCRect{left:clx, top:cly, right:clx + view.m_size.cx, bottom:cly + view.m_size.cy};
			let mut dest_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
			if get_intersect_rect(&mut dest_rect, rect.clone(), clip_rect.clone()) > 0{
				M_PAINT.lock().unwrap().save(context);
			    M_PAINT.lock().unwrap().set_offset(context, 0.0, 0.0);
			    M_PAINT.lock().unwrap().set_clip(context, dest_rect.left, dest_rect.top, dest_rect.right, dest_rect.bottom);
			    
			    let mut change_view = M_VIEW_MAP.lock().unwrap()[&v_id].clone();
				change_view.m_has_clip = true;
				change_view.m_clip_rect = dest_rect.clone();
				M_VIEW_MAP.lock().unwrap().insert(v_id, change_view);
			    M_PAINT.lock().unwrap().set_offset(context, clx, cly);
			    on_paint(context, view.clone(), draw_rect.clone());
			    let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if sub_views_size > 0 {
					render_views(context, sub_views, dest_rect.clone(), true);
				}
			    M_PAINT.lock().unwrap().set_offset(context, clx, cly);
			    on_paint_border(context, view.clone(), draw_rect.clone());
			    M_PAINT.lock().unwrap().restore(context);
			}else{
				let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if sub_views_size > 0 {
					render_views(context, sub_views, rect.clone(), false);
				}
				let mut change_view = M_VIEW_MAP.lock().unwrap()[&v_id].clone();
				change_view.m_has_clip = false;
				M_VIEW_MAP.lock().unwrap().insert(v_id, change_view);
			}
		}
	}
	for i in 0..views_size{
		let view = &views[i];
		let v_id = view.m_id;
		if !has_rect{
		    continue;
		}
		if view.m_top_most && is_paint_visible(view.clone()) {
			let clx = client_x(view.clone());
			let cly = client_y(view.clone());
			let draw_rect = FCRect{left:0.0, top:0.0, right:view.m_size.cx, bottom:view.m_size.cy};
			let clip_rect = FCRect{left:clx, top:cly, right:clx + view.m_size.cx, bottom:cly + view.m_size.cy};
			let mut dest_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
			if get_intersect_rect(&mut dest_rect, rect.clone(), clip_rect.clone()) > 0{
				M_PAINT.lock().unwrap().save(context);
			    M_PAINT.lock().unwrap().set_offset(context, 0.0, 0.0);
			    M_PAINT.lock().unwrap().set_clip(context, dest_rect.left, dest_rect.top, dest_rect.right, dest_rect.bottom);
			    
			    let mut change_view = M_VIEW_MAP.lock().unwrap()[&v_id].clone();
				change_view.m_has_clip = true;
				change_view.m_clip_rect = dest_rect.clone();
				M_VIEW_MAP.lock().unwrap().insert(v_id, change_view);
			    M_PAINT.lock().unwrap().set_offset(context, clx, cly);
			    on_paint(context, view.clone(), draw_rect.clone());
			    let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if sub_views_size > 0 {
					render_views(context, sub_views, dest_rect.clone(), true);
				}
			    
			    M_PAINT.lock().unwrap().set_offset(context, clx, cly);
			    on_paint_border(context, view.clone(), draw_rect.clone());
			    M_PAINT.lock().unwrap().restore(context);
			}else{
				let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if sub_views_size > 0 {
					render_views(context, sub_views, rect.clone(), false);
				}
				let mut change_view = M_VIEW_MAP.lock().unwrap()[&v_id].clone();
				change_view.m_has_clip = false;
				M_VIEW_MAP.lock().unwrap().insert(v_id, change_view);
			}
		}
	}
}

pub fn find_view(mp:FCPoint, views:Vec<FCView>)->FCView{
	let none_view = M_NONE_VIEW.lock().unwrap().clone();
	let views_size = views.len();
	for i in 0..views_size{
		let view = &views[views_size - i - 1];
		if view.m_visible && view.m_top_most {
			if contains_point(view.clone(), mp.clone()) {
			    if view.m_show_hscrollbar && view.m_scroll_size > 0.0{
			        let clx = client_x(view.clone());
	                if mp.x >= clx + view.m_size.cx - view.m_scroll_size{
	                    return view.clone();
	                }
			    }
			    if view.m_show_vscrollbar && view.m_scroll_size > 0.0{
			        let cly = client_y(view.clone());
	                if mp.y >= cly + view.m_size.cy - view.m_scroll_size{
	                    return view.clone();
	                }
			    }
			    let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if sub_views_size > 0 {
					let sub_view = find_view(mp.clone(), sub_views);
					if sub_view.m_id != -1 {
						return sub_view.clone();
					}
				}
				return view.clone();
			}
		}
	}
	for i in 0..views_size{
		let view = &views[views_size - i - 1];
		if view.m_visible && !view.m_top_most {
			if contains_point(view.clone(), mp.clone()){
			    if view.m_show_hscrollbar && view.m_scroll_size > 0.0{
			        let clx = client_x(view.clone());
	                if mp.x >= clx + view.m_size.cx - view.m_scroll_size{
	                    return view.clone();
	                }
			    }
			    if view.m_show_vscrollbar && view.m_scroll_size > 0.0{
			        let cly = client_y(view.clone());
	                if mp.y >= cly + view.m_size.cy - view.m_scroll_size{
	                    return view.clone();
	                }
			    }
			    let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if sub_views_size > 0 {
					let sub_view = find_view(mp.clone(), sub_views);
					if sub_view.m_id != -1 {
						return sub_view.clone();
					}
				}
				return view.clone();
			}
		}
	}
	return none_view;
}

pub fn invalidate(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>){
	unsafe{
		let top_views2 = get_top_views();
		render_views(&context, top_views2, FCRect{left:0.0, top:0.0, right:M_CANVAS_WIDTH, bottom:M_CANVAS_HEIGHT}, true);
	}
}


pub fn invalidate_view(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView){
	unsafe{
		if is_paint_visible(view.clone()){
			let top_views2 = get_top_views();
			let cl_x = client_x(view.clone());
	        let cl_y = client_y(view.clone());
	        let draw_rect = FCRect{left:cl_x, top:cl_y, right:cl_x + view.m_size.cx, bottom:cl_y + view.m_size.cy};
			render_views(&context, top_views2, draw_rect, true);
		}
	}
}