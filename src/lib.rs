/*
* FaceCat-Rust-Wasm(OpenSource)
* Shanghai JuanJuanMao Information Technology Co., Ltd
*/ 

use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::collections::HashMap;
use web_sys::DomParser;
use web_sys::SupportedType;
use web_sys::NamedNodeMap;
use web_sys::Attr;
use std::sync::Mutex; 

use std::error::Error;
use std::fs::File;
use std::io::Read;


// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
extern crate lazy_static; 

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
		if(!self.m_move_to) {
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
		if(color != "none"){
			let mut l_width = self.m_scale_factor_x.min(self.m_scale_factor_y) * width;
			context.begin_path();
			if(l_width < 1.0){
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
		if(color != "none"){
			let mut l_width = self.m_scale_factor_x.min(self.m_scale_factor_y) * width;
			if(l_width < 1.0){
				l_width = 1.0;
			}
			context.set_line_width(l_width as f64);
			let c = JsValue::from(String::from(color));
			context.set_stroke_style(&c);  
			context.stroke();
		}
	}

	fn draw_rect(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, width:f32, style:Vec<f32>, left:f32, top:f32, right:f32, bottom:f32){
		if(color != "none"){
			let mut l_width = self.m_scale_factor_x.min(self.m_scale_factor_y) * width;
			let w = right - left;
			let h = bottom - top;
			context.begin_path();
			if(l_width < 1.0){
				l_width = 1.0;
			}
			context.set_line_width(l_width as f64);
			let c = JsValue::from(String::from(color));
			context.set_stroke_style(&c);  
			context.stroke_rect(((left + self.m_offset_x) * self.m_scale_factor_x) as f64, ((top + self.m_offset_y) * self.m_scale_factor_y) as f64, (w * self.m_scale_factor_x) as f64, (h * self.m_scale_factor_y) as f64);
		}
	}

	fn draw_ellipse(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, width:f32, style:Vec<f32>, left:f32, top:f32, right:f32, bottom:f32){
		if(color != "none"){
			let mut l_width = self.m_scale_factor_x.min(self.m_scale_factor_y) * width;
			let w = right - left;
			let h = bottom - top;
			context.begin_path();
			if(l_width < 1.0){
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
		if(color != "none"){
			context.set_font(&font);
			let c = JsValue::from(String::from(color));
			context.set_fill_style(&c);  
			context.set_text_align("left");
			context.set_text_baseline("middle");
			context.fill_text(&text, ((x + self.m_offset_x) * self.m_scale_factor_x) as f64,  ((y + self.m_offset_y) * self.m_scale_factor_y) as f64);
		}
	}
	
	fn draw_text_auto_ellipsis(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, text:String, color:String, font:String, left:f32, top:f32, right:f32, bottom:f32){
		if(color != "none"){
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
		if(color != "none"){
			let c = JsValue::from(String::from(color));
			context.set_fill_style(&c);  
			context.fill();
		}
	}

	fn fill_rect(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, left:f32, top:f32, right:f32, bottom:f32){
		if(color != "none"){
			let c = JsValue::from(String::from(color));
			context.set_fill_style(&c);
			let w = right - left;
			let h = bottom - top;  
			context.fill_rect(((left + self.m_offset_x) * self.m_scale_factor_x) as f64, ((top + self.m_offset_y) * self.m_scale_factor_y) as f64, (w * self.m_scale_factor_x) as f64, (h * self.m_scale_factor_y) as f64);
		}
	}

	fn fill_ellipse(&mut self, context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, color:String, left:f32, top:f32, right:f32, bottom:f32){
		if(color != "none"){
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
		if(text.len() > 0) {
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

#[derive(Clone)]
pub struct FCTextBox{
	m_view:FCView
}

impl FCTextBox{
	fn new()->Self{
		let view:FCView = FCView::new();
		Self{
			m_view:view
		}
	}
}

#[derive(Clone)]
pub struct FCComboBox{
	m_view:FCView
}

impl FCComboBox{
	fn new()->Self{
		let view:FCView = FCView::new();
		Self{
			m_view:view
		}
	}
}

#[derive(Clone)]
pub struct FCSpin{
	m_view:FCView
}

impl FCSpin{
	fn new()->Self{
		let view:FCView = FCView::new();
		Self{
			m_view:view
		}
	}
}

#[derive(Clone)]
pub struct FCDatePicker{
	m_view:FCView
}

impl FCDatePicker{
	fn new()->Self{
		let view:FCView = FCView::new();
		Self{
			m_view:view
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

#[derive(Clone)]
pub struct FCGridColumn{
	m_name:String,
	m_text:String,
	m_type:String,
	m_width:f32,
	m_font:String,
	m_back_color:String,
	m_border_color:String,
	m_text_color:String,
	m_frozen:bool,
	m_sort:String,
	m_visible:bool,
	m_index:i32,
	m_bounds:FCRect
}

impl FCGridColumn{
	fn new()->Self{
		Self{
			m_name:String::from(""),
			m_text:String::from(""),
			m_type:String::from(""),
			m_width:100.0,
			m_font:String::from("12px Arial"),
			m_back_color:String::from("rgb(255,255,255)"),
			m_border_color:String::from("rgb(100,100,100)"),
			m_text_color:String::from("rgb(0,0,0)"),
			m_frozen:false,
			m_sort:String::from(""),
			m_visible:true,
			m_index:-1,
			m_bounds:FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0}
		}
	}
}

#[derive(Clone)]
pub struct FCGridCell{
	m_value:String,
	m_back_color:String,
	m_border_color:String,
	m_text_color:String,
	m_font:String,
	m_col_span:i32,
	m_row_span:i32,
	m_column:FCGridColumn
}

impl FCGridCell{
	fn new()->Self{
		let column:FCGridColumn = FCGridColumn::new();
		Self{
			m_value:String::from(""),
			m_back_color:String::from("rgb(255,255,255)"),
			m_border_color:String::from("rgb(100,100,100)"),
			m_text_color:String::from("rgb(0,0,0)"),
			m_font:String::from("12px Arial"),
			m_col_span:1,
			m_row_span:1,
			m_column:column
		}
	}
}

#[derive(Clone)]
pub struct FCGridRow{
	m_cells:Vec<FCGridCell>,
	m_selected:bool,
	m_visible:bool
}

impl FCGridRow{
	fn new()->Self{
		let cells:Vec<FCGridCell> = Vec::new();
		Self{
			m_cells:cells,
			m_selected:false,
			m_visible:true
		}
	}
}

#[derive(Clone)]
pub struct FCGrid{
	m_view:FCView,
	m_columns:Vec<FCGridColumn>,
	m_rows:Vec<FCGridRow>,
	m_row_height:f32,
	m_header_height:f32,
	m_selected_row_color:String
}

impl FCGrid{
	fn new()->Self{
		let columns:Vec<FCGridColumn> = Vec::new();
		let rows:Vec<FCGridRow> = Vec::new();
		let view:FCView = FCView::new();
		Self{
			m_view:view,
			m_columns:columns,
			m_rows:rows,
			m_row_height:30.0,
			m_header_height:30.0,
			m_selected_row_color:String::from("rgb(125,125,125)"),
		}
	}
}

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern {
	#[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn to_fixed(value:f64, digit:i32)->String{
	if(digit == 1){
		return format!("{:.1}", value);
	}else if(digit == 2){
		return format!("{:.2}", value);
	}else if(digit == 3){
		return format!("{:.3}", value);
	}else if(digit == 4){
		return format!("{:.4}", value);
	}else if(digit == 5){
		return format!("{:.5}", value);
	}else if(digit == 6){
		return format!("{:.6}", value);
	}else if(digit == 7){
		return format!("{:.7}", value);
	}else if(digit == 8){
		return format!("{:.8}", value);
	}else if(digit == 9){
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
				if(x == &parent.m_id){
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
	if(view.m_id != -1){
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
		if(find){
			let p_id = M_PARENT_VIEW_MAP.lock().unwrap()[&view.m_id];
			let parent = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
			if(parent.m_display_offset){
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
	if(view.m_id != -1){
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
		if(find){
			let p_id = M_PARENT_VIEW_MAP.lock().unwrap()[&view.m_id];
			let parent = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
			if(parent.m_display_offset){
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
	if(cp.x >= 0.0 && cp.x <= size.cx &&
		cp.y >= 0.0 && cp.y <= size.cy) {
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
		if(view.m_name == copy_name){
		    return view.clone();
		}else{
			let sub_views = get_sub_views(view.clone());
			if(sub_views.len() > 0){
				let sub_view = find_view_by_name(name.clone(), sub_views);
				if(sub_view.m_id != -1) {
					return sub_view;
				}
			}
		}
	}
	return none_view;
}

pub fn is_paint_visible(view:FCView)->bool{
	if(view.m_visible){
		let mut find = false;
        match M_PARENT_VIEW_MAP.lock().unwrap().get(&view.m_id) {
			Some(x) => {
				find = true;
			},
			None => {
				find = false;
			}
		}
		if(find){
			let p_id = M_PARENT_VIEW_MAP.lock().unwrap()[&view.m_id];
			let parent = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
			if(parent.m_visible){
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
    if(lp_dest_rect.right > lp_dest_rect.left && lp_dest_rect.bottom > lp_dest_rect.top){
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
			if(M_IS_MOBILE){
				let touches = event.touches();
				let touch1 = touches.get(0).expect("REASON");
				let mp = FCPoint{x:touch1.client_x() as f32, y:touch1.client_y() as f32};
				M_TOUCH_POINT = FCPoint{x:mp.x, y:mp.y};
				M_MOUSE_DOWN_POINT = FCPoint{x:mp.x, y:mp.y};
				let top_views = get_top_views();
				let mouse_down_view = find_view(M_MOUSE_DOWN_POINT.clone(), top_views);
				M_MOUSE_DOWN_VIEW = mouse_down_view.m_id;
				if(M_MOUSE_DOWN_VIEW != -1){
					M_FOCUSED_VIEW = M_MOUSE_DOWN_VIEW;
					M_FIRST_TOUCH = false;
					M_SECOND_TOUCH = false;
					M_TOUCH_FIRST_POINT = FCPoint{x:0.0, y:0.0};
					M_TOUCH_SECOND_POINT = FCPoint{x:0.0, y:0.0};
					let clx = client_x(mouse_down_view.clone());
					let cly = client_y(mouse_down_view.clone());
					if (touches.length() >= 1) {
						M_FIRST_TOUCH = true;
						M_MOUSE_DOWN_POINT = FCPoint{x:touch1.client_x() as f32, y:touch1.client_y() as f32};
						M_TOUCH_FIRST_POINT = FCPoint{x:mp.x, y:mp.y};
						M_TOUCH_FIRST_POINT.x = M_TOUCH_FIRST_POINT.x - clx;
						M_TOUCH_FIRST_POINT.y = M_TOUCH_FIRST_POINT.y - cly;
					}
					if (touches.length() >= 2) {
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
			if(M_IS_MOBILE){
				if(M_MOUSE_DOWN_VIEW != -1){
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
					if (touches.length() >= 1) {
						M_FIRST_TOUCH = true;
						M_TOUCH_FIRST_POINT = FCPoint{x:touch1.client_x() as f32, y:touch1.client_y() as f32};
						M_TOUCH_POINT = M_TOUCH_FIRST_POINT.clone();
						M_TOUCH_FIRST_POINT.x = M_TOUCH_FIRST_POINT.x - clx;
						M_TOUCH_FIRST_POINT.y = M_TOUCH_FIRST_POINT.y - cly;
					}
					if (touches.length() >= 2) {
						let touch2 = touches.get(1).expect("REASON");
						M_SECOND_TOUCH = true;
						M_TOUCH_SECOND_POINT = FCPoint{x:touch2.client_x() as f32, y:touch2.client_y() as f32};
						M_TOUCH_SECOND_POINT.x = M_TOUCH_SECOND_POINT.x - clx;
						M_TOUCH_SECOND_POINT.y = M_TOUCH_SECOND_POINT.y - cly;
					}
					on_touch_move(&context, mouse_down_view.clone(), M_FIRST_TOUCH, M_SECOND_TOUCH, M_TOUCH_FIRST_POINT.clone(), M_TOUCH_SECOND_POINT.clone());
					if(mouse_down_view.m_allow_drag){
						if ((mp.x - M_MOUSE_DOWN_POINT.x).abs() > 5.0 || (mp.y - M_MOUSE_DOWN_POINT.y).abs() > 5.0) {
							M_DRAG_BEGIN_POINT = FCPoint{x:M_MOUSE_DOWN_POINT.x, y:M_MOUSE_DOWN_POINT.y};
							M_DRAGGING_VIEW = M_MOUSE_DOWN_VIEW;
							M_DRAG_BEGIN_RECT = FCRect{left:mouse_down_view.m_location.x, top:mouse_down_view.m_location.y,
							right:mouse_down_view.m_location.x + mouse_down_view.m_size.cx,
							bottom:mouse_down_view.m_location.y + mouse_down_view.m_size.cy};
							M_MOUSE_DOWN_VIEW = -1;
						}
					}
				} else if(M_DRAGGING_VIEW != -1){
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
					if(find){
						let parentView = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
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
			if(M_IS_MOBILE){
				if(M_MOUSE_DOWN_VIEW != -1){
					let mouse_down_view = M_VIEW_MAP.lock().unwrap()[&M_MOUSE_DOWN_VIEW].clone();
					let top_views = get_top_views();
					let view = find_view(M_TOUCH_POINT.clone(), top_views);
					if(view.m_id == M_MOUSE_DOWN_VIEW){
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
			if(!M_IS_MOBILE){
				let mp = FCPoint{x:event.offset_x() as f32, y:event.offset_y() as f32};
				let top_views = get_top_views();
				let find_view = find_view(mp.clone(), top_views);
				let cmp  = FCPoint{x:mp.x - client_x(find_view.clone()), y:mp.y - client_y(find_view.clone())};
				let delta_y = event.delta_y();
				let mut delta:i32 = 0;
				if(delta_y > 0.0){
					delta = -1;
				}else if(delta_y < 0.0){
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
			if(!M_IS_MOBILE){
				if(M_MOUSE_DOWN_VIEW != -1){
					let mp = FCPoint{x:event.offset_x() as f32, y:event.offset_y() as f32};
					let mouse_down_view = M_VIEW_MAP.lock().unwrap()[&M_MOUSE_DOWN_VIEW].clone();
					let cmp  = FCPoint{x:mp.x - client_x(mouse_down_view.clone()), y:mp.y - client_y(mouse_down_view.clone())};
					let top_views = get_top_views();
					let find_view = find_view(mp.clone(), top_views);
					if(find_view.m_id == M_MOUSE_DOWN_VIEW){
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
			if(!M_IS_MOBILE){
				let mp = FCPoint{x:event.offset_x() as f32, y:event.offset_y() as f32};
				if(M_MOUSE_DOWN_VIEW != -1){
					let mouse_down_view = M_VIEW_MAP.lock().unwrap()[&M_MOUSE_DOWN_VIEW].clone();
					M_MOUSE_MOVE_VIEW = mouse_down_view.m_id;
					let cmp = FCPoint{x:mp.x - client_x(mouse_down_view.clone()), y:mp.y - client_y(mouse_down_view.clone())};
					on_mouse_move(&context, mouse_down_view.clone(), cmp.clone(), 1, 1, 0);
					if(mouse_down_view.m_allow_drag){
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
				else if(M_DRAGGING_VIEW != -1){
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
					if(find){
						let parentView = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
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
			if(!M_IS_MOBILE){
				M_CANCEL_CLICK = false;
				M_MOUSE_DOWN_POINT = FCPoint{x:event.offset_x() as f32, y:event.offset_y() as f32};
				let top_views = get_top_views();
				let mouse_down_view = find_view(M_MOUSE_DOWN_POINT.clone(), top_views);
				M_MOUSE_DOWN_VIEW = mouse_down_view.m_id;
				if(M_MOUSE_DOWN_VIEW != -1){
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

#[derive(Clone)]
pub struct SecurityData{
	m_amount : f64,
	m_close : f64,
	m_date : f64,
	m_high : f64,
	m_low : f64,
	m_open : f64,
	m_volume : f64
}

impl SecurityData{
	fn new()->Self{
		Self{
			m_amount : 0.0,
			m_close : 0.0,
			m_date : 0.0,
			m_high : 0.0,
			m_low : 0.0,
			m_open : 0.0,
			m_volume : 0.0
		}
	}
}

#[derive(Clone)]
pub struct FCPlot{
	m_plot_type : String, 
    m_line_color : String, 
    m_point_color : String,
    m_line_width : f32,
    m_key1 : f64, 
    m_value1 : f64,
    m_key2 : f64,
    m_value2 : f64,
    m_key3 : f64,
    m_value3 : f64,
    m_start_key1 : f64,
    m_start_value1 : f64,
    m_start_key2 : f64,
    m_start_value2 : f64,
    m_start_key3 : f64,
    m_start_value3 : f64,
    m_id: i32
}

impl FCPlot{
	fn new()->Self{
		Self{
		m_plot_type : String::from("Line"), 
		m_line_color : String::from("rgb(255,255,255)"), 
		m_point_color : String::from("rgba(255,255,255,0.5)"), 
		m_line_width : 1.0,
		m_key1 : 0.0, 
		m_value1 :  0.0, 
		m_key2 : 0.0, 
		m_value2 :  0.0, 
		m_key3 :  0.0, 
		m_value3 :  0.0, 
		m_start_key1 :  0.0, 
		m_start_value1 :  0.0, 
		m_start_key2 : 0.0, 
		m_start_value2 :  0.0, 
		m_start_key3 :  0.0, 
		m_start_value3 :  0.0, 
		m_id : -1
		}
	}
}

#[derive(Clone)]
pub struct FCChart{
	m_view:FCView,
	m_text_color : String,
	m_candle_distance : f32,
    m_hscale_pixel : f32,
	m_data : Vec<SecurityData>,
    m_down_color : String, 
    m_left_vscale_width : f32,
    m_right_vscale_width : f32, 
    m_up_color : String, 
    m_first_visible_index : i32,
    m_last_visible_index : i32, 
    m_hscale_height: f32, 
    m_scale_color : String, 
    m_candle_max : f64,
    m_candle_min : f64,  
    m_vol_max : f64,
    m_vol_min : f64,
    m_ind_max : f64,
    m_ind_min : f64, 
    m_cross_tip_color : String, 
    m_cross_line_color : String,
	m_font : String, 
    m_candle_digit : i32, 
    m_vol_digit : i32,
    m_ind_digit : i32, 
    m_last_record_is_visible : bool, 
    m_last_visible_key : f64, 
    m_auto_fill_hscale : bool, 
    m_candle_div_percent : f64,
    m_vol_div_percent : f64,
    m_ind_div_percent : f64, 
    m_show_indicator : String,
    m_main_indicator: String,
    m_grid_color : String,
    m_magnitude : i32,
    m_show_cross_line : bool,
    m_candle_padding_top : f32, 
    m_candle_padding_bottom : f32, 
    m_vol_padding_top : f32, 
    m_vol_padding_bottom : f32,
    m_ind_padding_top : f32, 
    m_ind_padding_bottom : f32, 
    m_vscale_distance : f32,
    m_vscale_type : String,
	m_indicator_colors : Vec<String>, 
	m_line_width : f32, 
	m_plots : Vec<FCPlot>, 
	m_select_plot_point : i32, 
	m_splot : FCPlot, 
    m_start_move_plot : bool,
    m_cross_stop_index : i32,
    m_cycle : String,
    m_first_index_cache : i32,
    m_first_touch_index_cache : i32,
    m_first_touch_point_cache : FCPoint,
    m_last_index_cache : i32,
    m_second_touch_index_cache : i32,
    m_second_touch_point_cache : FCPoint,
    m_mouse_position : FCPoint,
    m_mouse_down_position : FCPoint,
    m_right_space:f32,
    m_allema12 : Vec<f64>,
    m_allema26 : Vec<f64>,
    m_alldifarr : Vec<f64>,
    m_alldeaarr : Vec<f64>,
    m_allmacdarr : Vec<f64>,
    m_boll_up : Vec<f64>,
    m_boll_down : Vec<f64>,
    m_boll_mid : Vec<f64>,
    m_bias1 : Vec<f64>,
    m_bias2 : Vec<f64>,
    m_bias3 : Vec<f64>,
    m_kdj_k : Vec<f64>,
    m_kdj_d : Vec<f64>,
    m_kdj_j : Vec<f64>,
    m_rsi1 : Vec<f64>,
    m_rsi2 : Vec<f64>,
    m_rsi3 : Vec<f64>,
    m_roc : Vec<f64>,
    m_roc_ma : Vec<f64>,
    m_wr1 : Vec<f64>,
    m_wr2 : Vec<f64>,
    m_cci : Vec<f64>,
    m_bbi : Vec<f64>,
    m_trix : Vec<f64>,
    m_trix_ma : Vec<f64>,
    m_dma1 : Vec<f64>,
    m_dma2 : Vec<f64>,
    m_size : FCSize,
    m_ma5 : Vec<f64>,
    m_ma10 : Vec<f64>,
    m_ma20 : Vec<f64>,
    m_ma30 : Vec<f64>,
    m_ma120 : Vec<f64>,
    m_ma250 : Vec<f64>,
    m_select_shape:String,
    m_select_shape_ex:String,
}

impl FCChart{
	fn new()->Self{
		let security_datas:Vec<SecurityData> = Vec::new();
		let mut indicator_colors:Vec<String> = Vec::new();
		indicator_colors.push(String::from("rgb(255,255,255)"));
        indicator_colors.push(String::from("rgb(255,255,0)"));
        indicator_colors.push(String::from("rgb(255,0,255)"));
        indicator_colors.push(String::from("rgb(255,0,0)"));
        indicator_colors.push(String::from("rgb(0,255,255)"));
        indicator_colors.push(String::from("rgb(0,255,0)"));
        indicator_colors.push(String::from("rgb(255,255,0)"));
        indicator_colors.push(String::from("rgb(255,255,255)"));
		let plots:Vec<FCPlot> =  Vec::new();
		let splot:FCPlot = FCPlot::new();
		let allema12 = Vec::new();
		let allema26 = Vec::new();
		let alldifarr = Vec::new();
		let alldeaarr = Vec::new();
		let allmacdarr = Vec::new();
		let boll_up = Vec::new();
		let boll_down = Vec::new();
		let boll_mid = Vec::new();
		let bias1 = Vec::new();
		let bias2 = Vec::new();
		let bias3 = Vec::new();
		let kdj_k = Vec::new();
		let kdj_d = Vec::new();
		let kdj_j = Vec::new();
		let rsi1 = Vec::new();
		let rsi2 = Vec::new();
		let rsi3 = Vec::new();
		let roc = Vec::new();
		let roc_ma = Vec::new();
		let wr1 = Vec::new();
		let wr2 = Vec::new();
		let cci = Vec::new();
		let bbi = Vec::new();
		let trix = Vec::new();
		let trix_ma = Vec::new();
		let dma1 = Vec::new();
		let dma2 = Vec::new();
		let view:FCView = FCView::new();
		let ma5 = Vec::new();
		let ma10 = Vec::new();
		let ma20 = Vec::new();
		let ma30 = Vec::new();
		let ma120 = Vec::new();
		let ma250 = Vec::new();
		Self{
			m_view:view,
			m_text_color : String::from("rgb(255,255,255)"), 
			m_candle_distance : 0.0,
			m_hscale_pixel : 11.0,
			m_data : security_datas,
			m_down_color : String::from("rgb(15,193,118)"), 
			m_left_vscale_width : 100.0,
			m_right_vscale_width : 100.0, 
			m_up_color : String::from("rgb(219,68,83)"), 
			m_first_visible_index : -1,
			m_last_visible_index : -1, 
			m_hscale_height: 30.0, 
			m_scale_color : String::from("rgb(100,100,100)"), 
			m_candle_max : 0.0,
			m_candle_min : 0.0,
			m_vol_max : 0.0,
			m_vol_min : 0.0,
			m_ind_max : 0.0,
			m_ind_min : 0.0,
			m_cross_tip_color : String::from("rgb(50,50,50)"), 
			m_cross_line_color : String::from("rgb(100,100,100)"), 
			m_font : String::from("12px Arial"), 
			m_candle_digit : 2, 
			m_vol_digit : 0,
			m_ind_digit : 2, 
			m_last_record_is_visible : true, 
			m_last_visible_key : 0.0, 
			m_auto_fill_hscale : false, 
			m_candle_div_percent : 0.5,
			m_vol_div_percent : 0.2,
			m_ind_div_percent : 0.3, 
			m_show_indicator : String::from("MACD"), 
			m_main_indicator : String::from("MA"), 
			m_grid_color : String::from("rgba(100,100,100,0.5)"), 
			m_magnitude : 1,
			m_show_cross_line : true,
			m_candle_padding_top : 30.0, 
			m_candle_padding_bottom : 30.0, 
			m_vol_padding_top : 20.0, 
			m_vol_padding_bottom : 0.0,
			m_ind_padding_top : 20.0, 
			m_ind_padding_bottom : 20.0, 
			m_vscale_distance : 35.0,
			m_vscale_type : String::from("standard"), 
			m_indicator_colors : indicator_colors, 
			m_line_width:1.0,
			m_plots : plots, 
			m_select_plot_point : -1, 
			m_splot : splot, 
			m_start_move_plot : false,
			m_cross_stop_index : -1,
			m_cycle : String::from("minute"), 
			m_first_index_cache : -1,
			m_first_touch_index_cache : -1,
			m_first_touch_point_cache : FCPoint{x:0.0, y:0.0},
			m_last_index_cache : -1,
			m_second_touch_index_cache : -1,
			m_second_touch_point_cache : FCPoint{x:0.0, y:0.0},
			m_mouse_position : FCPoint{x:0.0, y:0.0},
			m_mouse_down_position : FCPoint{x:0.0, y:0.0},
			m_right_space:0.0,
			m_allema12 : allema12,
			m_allema26 : allema26,
			m_alldifarr : alldifarr,
			m_alldeaarr : alldeaarr,
			m_allmacdarr : allmacdarr,
			m_boll_up : boll_up,
			m_boll_down : boll_down,
			m_boll_mid : boll_mid,
			m_bias1 : bias1,
			m_bias2 : bias2,
			m_bias3 : bias3,
			m_kdj_k : kdj_k,
			m_kdj_d : kdj_d,
			m_kdj_j : kdj_j,
			m_rsi1 : rsi1,
			m_rsi2 : rsi2,
			m_rsi3 : rsi3,
			m_roc : roc,
			m_roc_ma : roc_ma,
			m_wr1 : wr1,
			m_wr2 : wr2,
			m_cci : cci,
			m_bbi : bbi,
			m_trix : trix,
			m_trix_ma : trix_ma,
			m_dma1 : dma1,
			m_dma2 : dma2,
			m_size : FCSize{cx:0.0, cy:0.0},
			m_ma5 : ma5,
			m_ma10 : ma10,
			m_ma20 : ma20,
			m_ma30 : ma30,
			m_ma120 : ma120,
			m_ma250 : ma250,
			m_select_shape : String::from(""),
			m_select_shape_ex : String::from("")
		}
	}
}

pub fn chart_grid_scale(min : f64, max : f64, y_len : f32, max_span : f32, min_span : f32, def_count : i32, grid_step:&mut f64, grid_digit:&mut i32){
	*grid_step = 0.0;
	*grid_digit = 0;
	let sub = max - min;
    let n_min_count = (y_len / max_span).ceil() as i32;
    let n_max_count = (y_len / min_span).floor() as i32 ;
    let mut n_count = def_count;
    let mut log_step = sub / (n_count as f64);
    let mut start : bool  = false;
    let mut divisor : f64 = 0.0;
    let mut i : i32 = 15;
	let mut n_temp : i32 = 0;
	n_count = n_min_count.max(n_count);
    n_count = n_max_count.min(n_count);
    n_count = n_count.max(1);
	while i>= -6{
		let b:f64 = 10.0;
		divisor = b.powf(i as f64);
		n_temp = (log_step / divisor).floor() as i32;
		if (start) {
            if (n_temp < 4) {
                if (*grid_digit > 0) {
                    *grid_digit = *grid_digit - 1;
                }
            } else if (n_temp >= 4 && n_temp <= 6) {
                n_temp = 5;
                *grid_step = *grid_step + (n_temp as f64) * divisor;
            } else {
                *grid_step = *grid_step + 10.0 * divisor;
                if (*grid_digit > 0) {
                    *grid_digit = *grid_digit - 1;
                }
            }
            break;
        } else if (n_temp > 0) {
            *grid_step = (n_temp as f64) * divisor + *grid_step;
            log_step = log_step - *grid_step;
            start = true;
        }
        i = i - 1;
	}
}

pub fn parallelogram(x1:f32, y1:f32, x2:f32, y2:f32, x3:f32, y3:f32, x4:&mut f32, y4:&mut f32){
	*x4 = x1 + x3 - x2;
    *y4 = y1 + y3 - y2;
}

pub fn avg_value(list:Vec<f64>)->f64{
	let len = list.len();
	let mut sum:f64 = 0.0;
	for num in list{
		sum = sum + num;
	}
	sum = sum / (len as f64);
	return sum;
}

pub fn max_value(list:Vec<f64>)->f64{
	let mut max:f64 = 0.0;
	let len = list.len();
	if(len > 0){
		max = list[0];
	}
	for num in list{
		if(max < num){
			max = num;
		}
	}
	return max;
}

pub fn min_value(list:Vec<f64>)->f64{
	let mut min:f64 = 0.0;
	let len = list.len();
	if(len > 0){
		min = list[0];
	}
	for num in list{
		if(min > num){
			min = num;
		}
	}
	return min;
}

pub fn ellipse_or(x1:f32, y1:f32, x2:f32, y2:f32, x3:f32, y3:f32, o_x:&mut f32, o_y:&mut f32, r:&mut f32){
	*o_x = ((y3 - y1) * (y2 * y2 - y1 * y1 + x2 * x2 - x1 * x1) + (y2 - y1) * (y1 * y1 - y3 * y3 + x1 * x1 - x3 * x3))
        / (2.0 * (x2 - x1) * (y3 - y1) - 2.0 * (x3 - x1) * (y2 - y1));
    *o_y = ((x3 - x1) * (x2 * x2 - x1 * x1 + y2 * y2 - y1 * y1) + (x2 - x1) * (x1 * x1 - x3 * x3 + y1 * y1 - y3 * y3))
        / (2.0 * (y2 - y1) * (x3 - x1) - 2.0 * (y3 - y1) * (x2 - x1));
    *r = ((x1 - *o_x) * (x1 - *o_x) + (y1 - *o_y) * (y1 - *o_y)).sqrt();
}

pub fn line_xy(x1:f32, y1:f32, x2:f32, y2:f32, o_x:f32, o_y:f32, k:&mut f32, b:&mut f32){
	if ((x1 - o_x) != (x2 - o_x)) {
        *k = ((y2 - o_y) - (y1 - o_y)) / ((x2 - o_x) - (x1 - o_x));
        *b = (y1 - o_y) - *k * (x1 - o_x);
    }
}

pub fn ellipse_has_point(x:f32, y: f32, o_x:f32, o_y:f32, a:f32, b:f32)->bool{
	let mut new_x = x;
	let mut new_y = y;
	new_x = x - o_x;
    new_y = y - o_y;
    if (a == 0.0 && b == 0.0 && new_x == 0.0 && new_y == 0.0) {
        return true;
    }
    if (a == 0.0) {
        if (new_x == 0.0 && new_y >= -b && new_y <= b) {
            return false;
        }
    }
    if (b == 0.0) {
        if (y == 0.0 && new_x >= -a && new_x <= a) {
            return true;
        }
    }
    if ((new_x * new_x) / (a * a) + (new_y * new_y) / (b * b) >= 0.8 && (new_x * new_x) / (a * a) + (new_y * new_y) / (b * b) <= 1.2) {
        return true;
    }
    return false;
}

pub fn rectangle_xywh(x1:f32, y1:f32, x2:f32, y2:f32, x:&mut f32, y:&mut f32, w:&mut f32, h:&mut f32){
	if(x1 < x2){
		*x = x1;
	}else{
		*x = x2;
	}
	if(y1 < y2){
		*y = y1;
	}else{
		*y = y2;
	}
    *w = (x1 - x2).abs();
    *h = (y1 - y2).abs();
    if (*w <= 0.0) {
        *w = 4.0;
    }
    if (*h <= 0.0) {
        *h = 4.0;
    }
}

pub fn select_line(mp:FCPoint, x1:f32, y1:f32, x2:f32, y2:f32)->bool{
	let mut k:f32 = 0.0;
	let mut b:f32 = 0.0;
	let f32_x = mp.x as f32;
	let f32_y = mp.y as f32;
	line_xy(x1, y1, x2, y2, 0.0, 0.0, &mut k, &mut b);
	if (!(k == 0.0 && b == 0.0)) {
        if (f32_y / (f32_x * k + b) >= 0.9 && f32_y / (f32_x * k + b) <= 1.1) {
            return true;
        }
    } else {
        if (f32_x >= x1 - 5.0 && f32_x <= x1 + 5.0) {
            return true;
        }
    }
    return false;
}

pub fn select_ray(mp:FCPoint, x1:f32, y1:f32, x2:f32, y2:f32)->bool{
	let mut k:f32 = 0.0;
	let mut b:f32 = 0.0;
	let f32_x = mp.x as f32;
	let f32_y = mp.y as f32;
	line_xy(x1, y1, x2, y2, 0.0, 0.0, &mut k, &mut b);
	if (!(k == 0.0 && b == 0.0)) {
        if (f32_y / (f32_x * k + b) >= 0.9 && f32_y / (f32_x * k + b) <= 1.1) {
            if (x1 >= x2) {
                if (f32_x > x1 + 5.0) {
					return false;
				}
            } else if (x1 < x2) {
                if (f32_x < x1 - 5.0){
					return false;
				}
            }
            return true;
        }
    } else {
        if (f32_x>= x1 - 5.0 && f32_x <= x1 + 5.0){
            if (y1 >= y2) {
                if (f32_y <= y1 - 5.0) {
                    return true;
                }
            } else {
                if (f32_y >= y1 - 5.0) {
                    return true;
                }
            }
        }
    }
    return false;
}

pub fn select_segment(mp:FCPoint, x1:f32, y1:f32, x2:f32, y2:f32)->bool{
	let mut k:f32 = 0.0;
	let mut b:f32 = 0.0;
	let f32_x = mp.x as f32;
	let f32_y = mp.y as f32;
	line_xy(x1, y1, x2, y2, 0.0, 0.0, &mut k, &mut b);
	let mut small_x:f32 = 0.0;
	let mut small_y:f32 = 0.0;
	let mut big_x:f32 = 0.0;
	let mut big_y:f32 = 0.0;
	if(x1 <= x2){
		small_x = x1;
	}else{
		small_x = x2;
	}
	if(y1 <= y2){
		small_y = y1;
	}else{
		small_y = y2;
	}
	if(x1 > x2){
		big_x = x1;
	}else{
		big_x = x2;
	}
	if(y1 > y2){
		big_y = y1;
	}else{
		big_y = y2;
	}
    if (f32_x >= small_x - 2.0 && f32_x <= big_x + 2.0 && f32_y >= small_y - 2.0 && f32_y <= big_y + 2.0) {
        if (k != 0.0 || b != 0.0) {
            if (f32_y / (f32_x * k + b) >= 0.9 && f32_y / (f32_x * k + b) <= 1.1) {
                return true;
            }
        } else {
            if (f32_x >= x1 - 5.0 && f32_x <= x1 + 5.0){
                return true;
            }
        }
    }
    return false;
}

pub fn linear_regression_equation(list:Vec<f64>, rk:&mut f32, rb:&mut f32){
	let mut sum_x:f64 = 0.0;
    let mut sum_y:f64 = 0.0;
    let mut sum_up:f64 = 0.0;
    let mut sum_down:f64 = 0.0;
    let mut x_avg:f64 = 0.0;
    let mut y_avg:f64 = 0.0;
    let length = list.len();
    if(length > 1){
		for i in 0..length{
			sum_x = sum_x + (i as f64) + 1.0;
            sum_y = sum_y + list[i];
		}
        x_avg = sum_x / (length as f64);
        y_avg = sum_y / (length as f64);
		for i in 0..length{
			sum_up = sum_up + ((i as f64) + 1.0 - x_avg) * (list[i] - y_avg);
            sum_down = sum_down + ((i as f64) + 1.0 - x_avg) * ((i as f64)+ 1.0 - x_avg);
		}
        *rk = (sum_up / sum_down) as f32;
        *rb = (y_avg - (*rk as f64) * x_avg) as f32;
    }
}

pub fn standard_deviation_sum(list:Vec<f64>, avg_value:f64, param:f64)->f64{
	let target_value:f64 = list[list.len() - 1];
	let mut sum = (target_value - avg_value) * (target_value - avg_value);
	for i in 0..(list.len() - 1){
		let ileft = list[i];
        sum = sum + (ileft - avg_value) * (ileft - avg_value);
	}
	return sum;
}

pub fn fibonacci_value(index:i32)->i32{
	if (index < 1){
        return 0;
    }
    else{
        let mut vlist: Vec<i32> = Vec::new();
		for i in 0..index{
			vlist.push(0);
		}
        let mut result:i32 = 0;
		for i in 0..index{
			if(i == 0 || i == 1){
				vlist[i as usize] = 1;
			}else{
				vlist[i as usize] = vlist[(i - 1)  as usize] + vlist[(i - 2) as usize];
			}
		}
        result = vlist[(index - 1) as usize];
        return result;
    }
}

pub fn get_max_visible_count(chart:&mut FCChart, h_scale_pixel:f32, pure_h:f32)->i32{
	let mut count = ((pure_h - h_scale_pixel) / h_scale_pixel) as i32;
    if(count < 0){
        count = 0;
    }
    return count;
}

pub fn get_candle_div_height(chart:&mut FCChart)->f32{
	let height = chart.m_view.m_size.cy - chart.m_hscale_height;
	if(height > 0.0){
		return height * (chart.m_candle_div_percent as f32);
	}else{
		return 0.0;
	}
}

pub fn get_vol_div_height(chart:&mut FCChart)->f32{
	let height = chart.m_view.m_size.cy - chart.m_hscale_height;
	if(height > 0.0){
		return height * (chart.m_vol_div_percent as f32);
	}else{
		return 0.0;
	}
}

pub fn get_ind_div_height(chart:&mut FCChart)->f32{
	let height = chart.m_view.m_size.cy - chart.m_hscale_height;
	if(height > 0.0){
		return height * (chart.m_ind_div_percent as f32);
	}else{
		return 0.0;
	}
}

pub fn get_chart_workarea_width(chart:&mut FCChart)->f32{
	return chart.m_view.m_size.cx - chart.m_left_vscale_width - chart.m_right_vscale_width - chart.m_right_space;
}

pub fn get_chart_x(chart:&mut FCChart, index:i32)->f32{
	return chart.m_left_vscale_width + ((index - chart.m_first_visible_index) as f32) * chart.m_hscale_pixel + chart.m_hscale_pixel;
}

pub fn get_chart_index(chart:&mut FCChart, mp:FCPoint)->i32{
	let data_len = chart.m_data.len() as i32;
	if(data_len == 0){
		return -1;
	}
	if(mp.x <= 0.0) {
		return 0;
	}
	let width = chart.m_view.m_size.cx - chart.m_left_vscale_width - chart.m_right_vscale_width;
	let int_x = mp.x - chart.m_left_vscale_width - chart.m_hscale_pixel;
	let mut index = (chart.m_first_visible_index as f32 + int_x / chart.m_hscale_pixel) as i32;
	if((int_x as i32) % (chart.m_hscale_pixel as i32) != 0){
		index = index + 1;
	}
	if(index < 0){
		 index = 0;
	}else if(index > data_len - 1){
		 index = data_len - 1;
	}	
	return index;
}

pub fn get_chart_index_by_date(chart:&mut FCChart,date:f64)->i32{
	let data_len = chart.m_data.len() as i32;
	let mut index : i32 = -1;
	let mut pos : i32 = 0;
	let vecter_iterator = chart.m_data.iter();
	for data in vecter_iterator{
		if(data.m_date == date){
			index = pos;
			break;
		}
		pos = pos + 1;
	}
	return index;
}

pub fn get_chart_date_by_index(chart:&mut FCChart, index:i32)->f64{
	let mut date : f64 = 0.0;
	let data_len = chart.m_data.len() as i32;
    if(index >= 0 && index < data_len){
        date = chart.m_data[index as usize].m_date;
    }
    return date;
}

pub fn check_chart_last_visible_index(chart:&mut FCChart){
	let data_len = chart.m_data.len() as i32;
    if (chart.m_last_visible_index > data_len - 1) {
        chart.m_last_visible_index = data_len - 1;
    }
    if (data_len > 0) {
        chart.m_last_visible_key = chart.m_data[chart.m_last_visible_index as usize].m_date;
        if (chart.m_last_visible_index == data_len - 1) {
            chart.m_last_record_is_visible = true;
        } else {
            chart.m_last_record_is_visible = false;
        }
    } else {
        chart.m_last_visible_key = 0.0;
        chart.m_last_record_is_visible = true;
    }
}

pub fn set_chart_visible_index(chart:&mut FCChart, first_visible_index : i32, last_visible_index : i32){
    let xscale_pixel = get_chart_workarea_width(chart) / ((last_visible_index - first_visible_index + 1) as f32);
    if (xscale_pixel < 1000000.0) {
        chart.m_first_visible_index = first_visible_index;
        chart.m_last_visible_index = last_visible_index;
	let data_len = chart.m_data.len() as i32;
        if (last_visible_index != data_len - 1) {
            chart.m_last_record_is_visible = false;
        } else {
            chart.m_last_record_is_visible = true;
        }
        chart.m_hscale_pixel = xscale_pixel;
        check_chart_last_visible_index(chart);
    }
}

pub fn reset_chart_visible_record(chart:&mut FCChart){
    let rows_count = chart.m_data.len() as i32;
    let working_area_width = get_chart_workarea_width(chart);
    if (chart.m_auto_fill_hscale) {
        if (working_area_width > 0.0 && rows_count > 0) {
            chart.m_hscale_pixel = working_area_width / (rows_count as f32);
            chart.m_first_visible_index = 0;
            chart.m_last_visible_index = rows_count - 1;
        }
    } else {
        let max_visible_record = get_max_visible_count(chart, chart.m_hscale_pixel, working_area_width);
        if (rows_count == 0) {
            chart.m_first_visible_index = -1;
            chart.m_last_visible_index = -1;
        } else {
            if (rows_count < max_visible_record) {
                chart.m_last_visible_index = rows_count - 1;
                chart.m_first_visible_index = 0;
            }
            else {
                if (chart.m_first_visible_index != -1 && chart.m_last_visible_index != -1 && !chart.m_last_record_is_visible) {
                    let index = get_chart_index_by_date(chart, chart.m_last_visible_key);
                    if (index != -1) {
                        chart.m_last_visible_index = index;
                    }
                    chart.m_first_visible_index = chart.m_last_visible_index - max_visible_record + 1;
                    if (chart.m_first_visible_index < 0) {
                        chart.m_first_visible_index = 0;
                        chart.m_last_visible_index = chart.m_first_visible_index + max_visible_record;
                        check_chart_last_visible_index(chart);
                    }
                } else {
                    chart.m_last_visible_index = rows_count - 1;
                    chart.m_first_visible_index = chart.m_last_visible_index - max_visible_record + 1;
                    if (chart.m_first_visible_index > chart.m_last_visible_index) {
                        chart.m_first_visible_index = chart.m_last_visible_index;
                    }
                }
            }
        }
    }
}

pub fn get_candle_range(chart:&mut FCChart, plot:&mut FCPlot, n_high:&mut f64, n_low:&mut f64){
    let mut bindex = get_chart_index_by_date(chart, plot.m_key1);
    let mut eindex = get_chart_index_by_date(chart, plot.m_key2);
    let mut temp_bindex : i32 = 0;
	let mut temp_eindex : i32 = 0;
	if(bindex > eindex){
		temp_bindex = eindex;
		temp_eindex = bindex;
	}else{
		temp_bindex = bindex;
		temp_eindex = eindex;
	}
    bindex = temp_bindex;
    eindex = temp_eindex;
	let mut high_list: Vec<f64> = Vec::new();
	let mut low_list: Vec<f64> = Vec::new();
	for i in bindex..(eindex + 1){
		high_list.push(chart.m_data[i as usize].m_high);
        low_list.push(chart.m_data[i as usize].m_low);
	}
    *n_high = max_value(high_list);
    *n_low = min_value(low_list);
}

pub fn get_chart_y(chart:&mut FCChart, div_index:i32, value:f64)->f32{
    if(div_index == 0){
        if(chart.m_candle_max > chart.m_candle_min){
            let mut c_value = value;
            let mut c_max = chart.m_candle_max;
            let mut c_min = chart.m_candle_min;
            if(chart.m_vscale_type != "standard"){
                if (c_value > 0.0) {
                    c_value = c_value.log10();
                } else if (c_value < 0.0) {
                    c_value = -c_value.abs().log10();
                }
                if (c_max > 0.0) {
                    c_max = c_max.log10();
                } else if (c_max < 0.0) {
                    c_max = -c_max.abs().log10();
                }
                if (c_min > 0.0) {
                    c_min = c_min.log10();
                } else if (c_min < 0.0) {
                    c_min = -c_min.abs().log10();
                }
            }
            let rate = (c_value - c_min) / (c_max - c_min);
            let div_height = get_candle_div_height(chart);
            return div_height - chart.m_candle_padding_bottom - (div_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom) * (rate as f32);
        }else{
            return 0.0;
        }
    }else if(div_index == 1){
        if(chart.m_vol_max > chart.m_vol_min){
            let rate = (value - chart.m_vol_min) / (chart.m_vol_max - chart.m_vol_min);
            let candle_height = get_candle_div_height(chart);
            let vol_height = get_vol_div_height(chart);
            return candle_height + vol_height - chart.m_vol_padding_bottom - (vol_height - chart.m_vol_padding_top - chart.m_vol_padding_bottom) * (rate as f32);
        }else{
            return 0.0;
        }
    }else if(div_index == 2){
        if(chart.m_ind_max > chart.m_ind_min){
            let rate = (value - chart.m_ind_min) / (chart.m_ind_max - chart.m_ind_min);
            let candle_height = get_candle_div_height(chart);
            let vol_height = get_vol_div_height(chart);
            let ind_height = get_ind_div_height(chart);
            return candle_height + vol_height + ind_height - chart.m_ind_padding_bottom - (ind_height - chart.m_ind_padding_top - chart.m_ind_padding_bottom) * (rate as f32);
        }else{
            return 0.0;
        }
    }
    return 0.0;
}

pub fn get_chart_value(chart:&mut FCChart, point:FCPoint)->f64{
    let candle_height = get_candle_div_height(chart);
    let vol_height = get_vol_div_height(chart);
    let ind_height = get_ind_div_height(chart);
    if(point.y <= candle_height){
        let rate = (candle_height - chart.m_candle_padding_bottom - point.y) / (candle_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom);
        let mut c_min = chart.m_candle_min;
        let mut c_max = chart.m_candle_max;
        if(chart.m_vscale_type != "standard"){
            if (c_max > 0.0) {
                c_max = c_max.log10();
            } else if (c_max < 0.0) {
                c_max = -c_max.abs().log10();
            }
            if (c_min > 0.0) {
                c_min = c_min.log10();
            } else if (c_min < 0.0) {
                c_min = -c_min.abs().log10();
            }
        }
        let result = c_min + (c_max - c_min) * (rate as f64);;
        if(chart.m_vscale_type != "standard"){
            let b:f64 = 10.0;
            return b.powf(result as f64);
        }else{
            return result;
        }
    }
    else if(point.y > candle_height && point.y <= candle_height + vol_height){
        let rate = (vol_height - chart.m_vol_padding_bottom - (point.y - candle_height)) / (vol_height - chart.m_vol_padding_top - chart.m_vol_padding_bottom);
        return chart.m_vol_min + (chart.m_vol_max - chart.m_vol_min) * (rate as f64);
    }else if(point.y > candle_height + vol_height && point.y <= candle_height + vol_height + ind_height){
        let rate = (ind_height - chart.m_ind_padding_bottom - (point.y - candle_height - vol_height)) / (ind_height - chart.m_ind_padding_top - chart.m_ind_padding_bottom);
        return chart.m_ind_min + (chart.m_ind_max - chart.m_ind_min) * (rate as f64);
    }
    return 0.0;
}

pub fn select_lines(chart:&mut FCChart, mp:FCPoint, div_index:i32, datas:Vec<f64>, cur_index:i32)->bool{
	let top_y = get_chart_y(chart, div_index, datas[cur_index as usize]);
    if (chart.m_hscale_pixel <= 1.0) {
        if(mp.y >= top_y - 8.0 && mp.y <= top_y + 8.0) {
            return true;
        }
    } else {
        let index = cur_index;
        let scale_x = get_chart_x(chart, index);
        let mut judge_top = 0.0;
        let mut judge_scale_x = scale_x;
        if (mp.x >= scale_x) {
            let left_index = cur_index + 1;
            if (cur_index < chart.m_last_visible_index) {
                let right_value = datas[left_index as usize];
                judge_top = get_chart_y(chart, div_index, right_value);
            }
            else {
                judge_top = top_y;
            }
        }
        else {
            judge_scale_x = scale_x - chart.m_hscale_pixel;
            let right_index = cur_index - 1;
            if (cur_index > 0) {
                let left_value = datas[right_index as usize];
                judge_top = get_chart_y(chart, div_index, left_value);
            }else {
                judge_top = top_y;
            }
        }
        let line_width : f32 = 4.0;
        let mut judge_x : f32 = 0.0;
        let mut judge_y : f32 = 0.0;
        let mut judge_w : f32 = 0.0;
        let mut judge_h : f32 = 0.0;
        if (judge_top >= top_y) {
            judge_x = judge_scale_x;
            judge_y = top_y - 2.0 - line_width;
            judge_w = chart.m_hscale_pixel;
            if(judge_top - top_y + line_width < 4.0){
				judge_h = 4.0;
            }else{
				judge_h = judge_top - top_y + 4.0 + line_width;
            }
        }
        else {
            judge_x = judge_scale_x;
            judge_y = judge_top - 2.0 - line_width / 2.0;
            judge_w = chart.m_hscale_pixel;
            if(top_y - judge_top + line_width < 4.0){
				judge_h = 4.0;
            }else{
				judge_h = top_y - judge_top + 4.0 + line_width;
            }
        }
       
        if (mp.x >= judge_x && mp.x <= judge_x + judge_w && mp.y >= judge_y && mp.y <= judge_y + judge_h) {
           
            return true;
        }
    }
    return false;
}

pub fn select_shape(chart:&mut FCChart, mp:FCPoint){
	chart.m_select_shape = "".to_string();
    chart.m_select_shape_ex = "".to_string();
    let candle_height = get_candle_div_height(chart);
    let vol_height = get_vol_div_height(chart);
    let ind_height = get_ind_div_height(chart);
    let index = get_chart_index(chart, mp.clone());
    if (mp.y >= candle_height + vol_height && mp.y <= candle_height + vol_height + ind_height) {
        if (chart.m_show_indicator == "MACD") {
            let macd_y = get_chart_y(chart, 2, chart.m_allmacdarr[index as usize]);
            let zero_y = get_chart_y(chart, 2, 0.0);
            if (select_lines(chart, mp.clone(), 2, chart.m_allmacdarr.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "MACD".to_string();
            }
            if (select_lines(chart, mp.clone(), 2, chart.m_alldifarr.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "DIF".to_string();
            }
            else if (select_lines(chart, mp.clone(), 2, chart.m_alldeaarr.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "DEA".to_string();
            }
        } else if (chart.m_show_indicator == "KDJ") {
            if (select_lines(chart, mp.clone(), 2, chart.m_kdj_k.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "K".to_string();
            }
            else if (select_lines(chart, mp.clone(), 2, chart.m_kdj_d.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "D".to_string();
            } else if (select_lines(chart, mp.clone(), 2, chart.m_kdj_j.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "J".to_string();
            }
        } else if (chart.m_show_indicator == "RSI") {
            if (select_lines(chart, mp.clone(), 2, chart.m_rsi1.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "6".to_string();
            }
            else if (select_lines(chart, mp.clone(), 2, chart.m_rsi2.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "12".to_string();
            } else if (select_lines(chart, mp.clone(), 2, chart.m_rsi3.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "24".to_string();
            }
        }
        else if (chart.m_show_indicator == "BIAS") {
            if (select_lines(chart, mp.clone(), 2, chart.m_bias1.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "1".to_string();
            }
            else if (select_lines(chart, mp.clone(), 2, chart.m_bias2.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "2".to_string();
            } else if (select_lines(chart, mp.clone(), 2, chart.m_bias3.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "3".to_string();
            }
        }
        else if (chart.m_show_indicator == "ROC") {
            if (select_lines(chart, mp.clone(), 2, chart.m_roc.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "ROC".to_string();
            }
            else if (select_lines(chart, mp.clone(), 2, chart.m_roc_ma.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "ROCMA".to_string();
            }
        } else if (chart.m_show_indicator == "WR") {
            if (select_lines(chart, mp.clone(), 2, chart.m_wr1.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "1".to_string();
            }
            else if (select_lines(chart, mp.clone(), 2, chart.m_wr2.clone(), index)) {
                chart.m_select_shape = "WR".to_string();
                chart.m_select_shape_ex = "2".to_string();
            }
        } else if (chart.m_show_indicator == "CCI") {
            if (select_lines(chart, mp.clone(), 2, chart.m_cci.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
            }
        } else if (chart.m_show_indicator == "BBI") {
            if (select_lines(chart, mp.clone(), 2, chart.m_bbi.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
            }
        } else if (chart.m_show_indicator == "TRIX") {
            if (select_lines(chart, mp.clone(), 2, chart.m_trix.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "TRIX".to_string();
            }
            else if (select_lines(chart, mp.clone(), 2, chart.m_trix_ma.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "TRIXMA".to_string();
            }
        } else if (chart.m_show_indicator == "DMA") {
            if (select_lines(chart, mp.clone(), 2, chart.m_dma1.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "DIF".to_string();
            }
            else if (select_lines(chart, mp.clone(), 2, chart.m_dma2.clone(), index)) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "DIFMA".to_string();
            }
        }
    }
    else if (mp.y >= candle_height && mp.y <= candle_height + vol_height) {
        let vol_y = get_chart_y(chart, 1, chart.m_data[index as usize].m_volume);
        let zero_y = get_chart_y(chart, 1, 0.0); 
        if (mp.y >= vol_y.min(zero_y) && mp.y <= vol_y.max(zero_y)) {
            chart.m_select_shape = "VOL".to_string();
        }
    }
    else if (mp.y >= 0.0 && mp.y <= candle_height) {
        let is_trend:bool = false;
        if (!is_trend) {
            if (chart.m_main_indicator == "BOLL") {
                if (select_lines(chart, mp.clone(), 0, chart.m_boll_mid.clone(), index)) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "MID".to_string();
                }
                else if (select_lines(chart, mp.clone(), 0, chart.m_boll_up.clone(), index)) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "UP".to_string();
                } else if (select_lines(chart, mp.clone(), 0, chart.m_boll_down.clone(), index)) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "DOWN".to_string();
                }
            } else if (chart.m_main_indicator == "MA") {
                if (select_lines(chart, mp.clone(), 0, chart.m_ma5.clone(), index)) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "5".to_string();
                }
                else if (select_lines(chart, mp.clone(), 0, chart.m_ma10.clone(), index)) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "10".to_string();
                }
                else if (select_lines(chart, mp.clone(), 0, chart.m_ma20.clone(), index)) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "20".to_string();
                }
                else if (select_lines(chart, mp.clone(), 0, chart.m_ma30.clone(), index)) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "30".to_string();
                }
                else if (select_lines(chart, mp.clone(), 0, chart.m_ma120.clone(), index)) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "120".to_string();
                }
                else if (select_lines(chart, mp.clone(), 0, chart.m_ma250.clone(), index)) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "250".to_string();
                }
            }
        }
        if (chart.m_select_shape == "") {
            let high_y = get_chart_y(chart, 0, chart.m_data[index as usize].m_high);
            let low_y = get_chart_y(chart, 0, chart.m_data[index as usize].m_low);
			if (mp.y >= low_y.min(high_y) && mp.y <= low_y.max(high_y)) {
                chart.m_select_shape = "CANDLE".to_string();
            }
        }
    }
}

pub fn clear_data_arr(chart:&mut FCChart){
	let allema12 = Vec::new();
	let allema26 = Vec::new();
	let alldifarr = Vec::new();
	let alldeaarr = Vec::new();
	let allmacdarr = Vec::new();
	let boll_up = Vec::new();
	let boll_down = Vec::new();
	let boll_mid = Vec::new();
	let bias1 = Vec::new();
	let bias2 = Vec::new();
	let bias3 = Vec::new();
	let kdj_k = Vec::new();
	let kdj_d = Vec::new();
	let kdj_j = Vec::new();
	let rsi1 = Vec::new();
	let rsi2 = Vec::new();
	let rsi3 = Vec::new();
	let roc = Vec::new();
	let roc_ma = Vec::new();
	let wr1 = Vec::new();
	let wr2 = Vec::new();
	let cci = Vec::new();
	let bbi = Vec::new();
	let trix = Vec::new();
	let trix_ma = Vec::new();
	let dma1 = Vec::new();
	let dma2 = Vec::new();
	let ma5 = Vec::new();
	let ma10 = Vec::new();
	let ma20 = Vec::new();
	let ma30 = Vec::new();
	let ma120 = Vec::new();
	let ma250 = Vec::new();
	chart.m_allema12 = allema12;
	chart.m_allema26 = allema26;
	chart.m_alldifarr = alldifarr;
	chart.m_alldeaarr = alldeaarr;
	chart.m_allmacdarr = allmacdarr;
	chart.m_boll_up = boll_up;
	chart.m_boll_down = boll_down;
	chart.m_boll_mid = boll_mid;
	chart.m_bias1 = bias1;
	chart.m_bias2 = bias2;
	chart.m_bias3 = bias3;
	chart.m_kdj_k = kdj_k;
	chart.m_kdj_d = kdj_d;
	chart.m_kdj_j = kdj_j;
	chart.m_rsi1 = rsi1;
	chart.m_rsi2 = rsi2;
	chart.m_rsi3 = rsi3;
	chart.m_roc = roc;
	chart.m_roc_ma = roc_ma;
	chart.m_wr1 = wr1;
	chart.m_wr2 = wr2;
	chart.m_cci = cci;
	chart.m_bbi = bbi;
	chart.m_trix = trix;
	chart.m_trix_ma = trix_ma;
	chart.m_dma1 = dma1;
	chart.m_dma2 = dma2;
	chart.m_ma5 = ma5;
	chart.m_ma10 = ma10;
	chart.m_ma20 = ma20;
	chart.m_ma30 = ma30;
	chart.m_ma120 = ma120;
	chart.m_ma250 = ma250;
}

pub fn calc_chart_indicator(chart:&mut FCChart){
	clear_data_arr(chart);
	let mut close_arr = Vec::new();
	let mut high_arr = Vec::new();
	let mut low_arr = Vec::new();
	let data_len = chart.m_data.len() as i32;
	for i in 0..data_len{
        let close = chart.m_data[i as usize].m_close;
        let high = chart.m_data[i as usize].m_high;
        let low = chart.m_data[i as usize].m_low;
        close_arr.push(close);
        high_arr.push(high);
        low_arr.push(low);
    }
    if (chart.m_main_indicator == "BOLL") {
		get_boll_data(close_arr.clone(), &mut chart.m_boll_up, &mut chart.m_boll_mid, &mut chart.m_boll_down);
    } else if (chart.m_main_indicator == "MA") {
		chart.m_ma5 = ma_value(close_arr.clone(), 5);
		chart.m_ma10 = ma_value(close_arr.clone(), 10);
		chart.m_ma20 = ma_value(close_arr.clone(), 20);
		chart.m_ma30 = ma_value(close_arr.clone(), 30);
		chart.m_ma120 = ma_value(close_arr.clone(), 120);
		chart.m_ma250 = ma_value(close_arr.clone(), 250);
    }
    if (chart.m_show_indicator == "BIAS") {
		get_bias_data(close_arr.clone(), &mut chart.m_bias1, &mut chart.m_bias2, &mut chart.m_bias3);
	}
	else if(chart.m_show_indicator == "DMA"){
		get_dma_data(close_arr.clone(), &mut chart.m_dma1, &mut chart.m_dma2);
	}
	else if(chart.m_show_indicator == "BBI"){
		get_bbi_data(close_arr.clone(), &mut chart.m_bbi);
	}
	else if(chart.m_show_indicator == "RSI"){
		get_rsi_data(close_arr.clone(), &mut chart.m_rsi1, &mut chart.m_rsi2, &mut chart.m_rsi3);
	}
	else if(chart.m_show_indicator == "ROC"){
		get_roc_data(close_arr.clone(), &mut chart.m_roc, &mut chart.m_roc_ma);
	}
	else if(chart.m_show_indicator == "TRIX"){
		get_trix_data(close_arr.clone(), &mut chart.m_trix, &mut chart.m_trix_ma);
	}
	else if(chart.m_show_indicator == "KDJ"){
		get_kdj_data(high_arr.clone(), low_arr.clone(), close_arr.clone(), &mut chart.m_kdj_k, &mut chart.m_kdj_d, &mut chart.m_kdj_j);
	}
	else if(chart.m_show_indicator == "WR"){
		get_wr_data(high_arr.clone(), low_arr.clone(), close_arr.clone(), &mut chart.m_wr1, &mut chart.m_wr2);
	}
	else if(chart.m_show_indicator == "CCI"){
		get_cci_data(high_arr.clone(), low_arr.clone(), close_arr.clone(), &mut chart.m_cci);
	}
	else if(chart.m_show_indicator == "MACD"){
		chart.m_allema12.push(close_arr[0]);
        chart.m_allema26.push(close_arr[0]);
	    chart.m_alldeaarr.push(0.0);
	    for i in 1..close_arr.len(){
			chart.m_allema12.push(get_ema(12, close_arr[i], chart.m_allema12[i - 1]));
            chart.m_allema26.push(get_ema(26, close_arr[i], chart.m_allema26[i - 1]));
	    }
        chart.m_alldifarr = get_dif(chart.m_allema12.clone(), chart.m_allema26.clone());
        for i in 1..chart.m_alldifarr.len(){  
            chart.m_alldeaarr.push(chart.m_alldeaarr[i - 1] * 8.0 / 10.0 + chart.m_alldifarr[i] * 2.0 / 10.0);
	    }
        chart.m_allmacdarr = get_macd(chart.m_alldifarr.clone(), chart.m_alldeaarr.clone());
	}
}

pub fn calculate_chart_max_min(chart:&mut FCChart){
    let data_len = chart.m_data.len() as i32;
    chart.m_candle_max = 0.0;
    chart.m_candle_min = 0.0;
    chart.m_vol_max = 0.0;
    chart.m_vol_min = 0.0;
    chart.m_ind_min = 0.0;
    chart.m_ind_min = 0.0;
    let mut is_trend : bool = false;
    if(chart.m_cycle == "trend"){
        is_trend = true;
    }
    let mut first_open : f64 = 0.0;
    if(data_len > 0) {
        for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
            let ui = i as usize;
            if(i == chart.m_first_visible_index){
                if(is_trend){
                    chart.m_candle_max = chart.m_data[ui].m_close;
                    chart.m_candle_min = chart.m_data[ui].m_close;  
                    first_open = chart.m_data[ui].m_close;
                }else{
                    chart.m_candle_max = chart.m_data[ui].m_high;
                    chart.m_candle_min = chart.m_data[ui].m_low;
                }
                chart.m_vol_max = chart.m_data[ui].m_volume;
                if(chart.m_show_indicator == "MACD"){
                    chart.m_ind_max = chart.m_alldifarr[ui];
                    chart.m_ind_min = chart.m_alldifarr[ui];
                }
                else if(chart.m_show_indicator == "KDJ"){
                    chart.m_ind_max = chart.m_kdj_k[ui];
                    chart.m_ind_min = chart.m_kdj_k[ui];
                }
                else if(chart.m_show_indicator == "RSI"){
                    chart.m_ind_max = chart.m_rsi1[ui];
                    chart.m_ind_min = chart.m_rsi1[ui];
                }
                else if(chart.m_show_indicator == "BIAS"){
                    chart.m_ind_max = chart.m_bias1[ui];
                    chart.m_ind_min = chart.m_bias1[ui];
                }
                 else if(chart.m_show_indicator == "ROC"){
                    chart.m_ind_max = chart.m_roc[ui];
                    chart.m_ind_min = chart.m_roc[ui];
                }
                 else if(chart.m_show_indicator == "BOLL"){
                    chart.m_ind_max = chart.m_boll_mid[ui];
                    chart.m_ind_min = chart.m_boll_mid[ui];
                }
                else if(chart.m_show_indicator == "WR"){
                    chart.m_ind_max = chart.m_wr1[ui];
                    chart.m_ind_min = chart.m_wr1[ui];
                }else if(chart.m_show_indicator == "CCI"){
                    chart.m_ind_max = chart.m_cci[ui];
                    chart.m_ind_min = chart.m_cci[ui];
                }else if(chart.m_show_indicator == "BBI"){
                    chart.m_ind_max = chart.m_bbi[ui];
                    chart.m_ind_min = chart.m_bbi[ui];
                }
                else if(chart.m_show_indicator == "TRIX"){
                    chart.m_ind_max = chart.m_trix[ui];
                    chart.m_ind_min = chart.m_trix[ui];
                }
                else if(chart.m_show_indicator == "DMA"){
                    chart.m_ind_max = chart.m_dma1[ui];
                    chart.m_ind_min = chart.m_dma1[ui];
                }
               
            }else{
                if(is_trend){
                    if(chart.m_candle_max < chart.m_data[ui].m_close){
                        chart.m_candle_max = chart.m_data[ui].m_close;
                    }
                    if(chart.m_candle_min > chart.m_data[ui].m_close){
                        chart.m_candle_min = chart.m_data[ui].m_close;
                    }
                }else{
                    if(chart.m_candle_max < chart.m_data[ui].m_high){
                        chart.m_candle_max = chart.m_data[ui].m_high;
                    }
                    if(chart.m_candle_min > chart.m_data[ui].m_low){
                        chart.m_candle_min = chart.m_data[ui].m_low;
                    }
                }
                if(chart.m_vol_max < chart.m_data[ui].m_volume){
                    chart.m_vol_max = chart.m_data[ui].m_volume;
                }   
				if(chart.m_show_indicator == "MACD"){
					if (chart.m_ind_max < chart.m_alldifarr[ui]){
						chart.m_ind_max = chart.m_alldifarr[ui];
					}
					if (chart.m_ind_max < chart.m_alldeaarr[ui]){
						chart.m_ind_max = chart.m_alldeaarr[ui];
					}
					if (chart.m_ind_max < chart.m_allmacdarr[ui]){
						chart.m_ind_max = chart.m_allmacdarr[ui];
					}
					if (chart.m_ind_min > chart.m_alldifarr[ui]){
						chart.m_ind_min = chart.m_alldifarr[ui];
					}
					if (chart.m_ind_min > chart.m_alldeaarr[ui]){
						chart.m_ind_min = chart.m_alldeaarr[ui];
					}
					if (chart.m_ind_min > chart.m_allmacdarr[ui]){
						chart.m_ind_min = chart.m_allmacdarr[ui];
					}
				}else if(chart.m_show_indicator == "KDJ"){
					if (chart.m_ind_max < chart.m_kdj_k[ui]){
						chart.m_ind_max = chart.m_kdj_k[ui];
					}
					if (chart.m_ind_max < chart.m_kdj_d[ui]){
						chart.m_ind_max = chart.m_kdj_d[ui];
					}
					if (chart.m_ind_max < chart.m_kdj_j[ui]){
						chart.m_ind_max = chart.m_kdj_j[ui];
					}
					if (chart.m_ind_min > chart.m_kdj_k[ui]){
						chart.m_ind_min = chart.m_kdj_k[ui];
					}
					if (chart.m_ind_min > chart.m_kdj_d[ui]){
						chart.m_ind_min = chart.m_kdj_d[ui];
					}
					if (chart.m_ind_min > chart.m_kdj_j[ui]){
						chart.m_ind_min = chart.m_kdj_j[ui];
					}
				}else if(chart.m_show_indicator == "RSI"){
					if (chart.m_ind_max < chart.m_rsi1[ui]){
						chart.m_ind_max = chart.m_rsi1[ui];
					}
					if (chart.m_ind_max < chart.m_rsi2[ui]){
						 chart.m_ind_max = chart.m_rsi2[ui];
					}
					if (chart.m_ind_max < chart.m_rsi3[ui]){
						chart.m_ind_max = chart.m_rsi3[ui];
					}
					if (chart.m_ind_min > chart.m_rsi1[ui]){
						chart.m_ind_min = chart.m_rsi1[ui];
					}
					if (chart.m_ind_min > chart.m_rsi2[ui]){
						chart.m_ind_min = chart.m_rsi2[ui];
					}
					if (chart.m_ind_min > chart.m_rsi3[ui]){
						chart.m_ind_min = chart.m_rsi3[ui];
					}
				}else if(chart.m_show_indicator == "BIAS"){
					if (chart.m_ind_max < chart.m_bias1[ui]){
						chart.m_ind_max = chart.m_bias1[ui];
					}
					if (chart.m_ind_max < chart.m_bias2[ui]){
						chart.m_ind_max = chart.m_bias2[ui];
					}
					if (chart.m_ind_max < chart.m_bias3[ui]){
						chart.m_ind_max = chart.m_bias3[ui];
					}
					if (chart.m_ind_min > chart.m_bias1[ui]){
						chart.m_ind_min = chart.m_bias1[ui];
					}
					if (chart.m_ind_min > chart.m_bias2[ui]){
						chart.m_ind_min = chart.m_bias2[ui];
					}
					if (chart.m_ind_min > chart.m_bias3[ui]){
						chart.m_ind_min = chart.m_bias3[ui];
					}
				}else if(chart.m_show_indicator == "ROC"){
					if (chart.m_ind_max < chart. m_roc[ui]){
						chart.m_ind_max = chart.m_roc[ui];
					}
					if (chart.m_ind_max < chart.m_roc_ma[ui]){
						chart.m_ind_max = chart.m_roc_ma[ui];
					}
					if (chart.m_ind_min > chart.m_roc[ui]){
						chart.m_ind_min = chart.m_roc[ui];
					}
					if (chart.m_ind_min > chart.m_roc_ma[ui]){
						chart.m_ind_min = chart.m_roc_ma[ui];
					}
				}else if(chart.m_show_indicator == "BOLL"){
					if (chart.m_ind_max < chart.m_boll_mid[ui]){
						chart.m_ind_max = chart.m_boll_mid[ui];
					}
					if (chart.m_ind_max < chart.m_boll_up[ui]){
						chart.m_ind_max = chart.m_boll_up[ui];
					}
					if (chart.m_ind_max < chart.m_boll_down[ui]){
						chart.m_ind_max = chart.m_boll_down[ui];
					}
					if (chart.m_ind_min > chart.m_boll_mid[ui]){
						chart.m_ind_min = chart.m_boll_mid[ui];
					}
					if (chart.m_ind_min > chart.m_boll_up[ui]){
						chart.m_ind_min = chart.m_boll_up[ui];
					}
					if (chart.m_ind_min > chart.m_boll_down[ui]){
						chart.m_ind_min = chart.m_boll_down[ui];
					}
				}
				else if(chart.m_show_indicator == "WR"){
					if (chart.m_ind_max < chart.m_wr1[ui]){
						chart.m_ind_max = chart.m_wr1[ui];
					}
					if (chart.m_ind_max < chart.m_wr2[ui]){
						chart.m_ind_max = chart.m_wr2[ui];
					}
					if (chart.m_ind_min > chart.m_wr1[ui]){
						chart.m_ind_min = chart.m_wr1[ui];
					}
					if (chart.m_ind_min > chart.m_wr2[ui]){
						chart.m_ind_min = chart.m_wr2[ui];
					}
				}else if(chart.m_show_indicator == "CCI"){
					if (chart.m_ind_max < chart.m_cci[ui]){
						chart.m_ind_max = chart.m_cci[ui];
					}
					if (chart.m_ind_min > chart.m_cci[ui]){
						chart.m_ind_min = chart.m_cci[ui];
					}
				}else if(chart.m_show_indicator == "BBI"){
					if (chart.m_ind_max < chart.m_bbi[ui]){
						chart.m_ind_max = chart.m_bbi[ui];
					}
					if (chart.m_ind_min > chart.m_bbi[ui]){
						chart.m_ind_min = chart.m_bbi[ui];
					}
				}else if(chart.m_show_indicator == "TRIX"){
					if (chart.m_ind_max < chart.m_trix[ui]){
						chart.m_ind_max = chart.m_trix[ui];
					}
					if (chart.m_ind_max < chart.m_trix_ma[ui]){
						chart.m_ind_max = chart.m_trix_ma[ui];
					}
					if (chart.m_ind_min > chart.m_trix[ui]){
						chart.m_ind_min = chart.m_trix[ui];
					}
					if (chart.m_ind_min > chart.m_trix_ma[ui]){
						chart.m_ind_min = chart.m_trix_ma[ui];
					}
				}else if(chart.m_show_indicator == "DMA"){
					if (chart.m_ind_max < chart.m_dma1[ui]){
						chart.m_ind_max = chart.m_dma1[ui];
					}
					if (chart.m_ind_max < chart.m_dma2[ui]){
						chart.m_ind_max = chart.m_dma2[ui];
					}
					if (chart.m_ind_min > chart.m_dma1[ui]){
						chart.m_ind_min = chart.m_dma1[ui];
					}
					if (chart.m_ind_min > chart.m_dma2[ui]){
						chart.m_ind_min = chart.m_dma2[ui];
					}
				}
            }
        }
    }
    if(is_trend){
        let mut sub_max : f64 = 0.0;
        let f_value : f64 = (chart.m_candle_max - first_open).abs();
        let s_value : f64 = (chart.m_candle_min - first_open).abs();
        if(f_value > s_value){
            sub_max = f_value;
        }else{
            sub_max = s_value;
        }
        chart.m_candle_max = first_open + sub_max;
        chart.m_candle_min = first_open - sub_max;
    }
}

pub fn mouse_move_chart(chart:&mut FCChart, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
    let mp_x = first_point.x;
    let mp_y = first_point.y;   
    let mp_x1 = second_point.x;
    let mp_y2 = second_point.y;
    let mp = FCPoint{x:mp_x, y:mp_y};
    chart.m_cross_stop_index = get_chart_index(chart, mp);
    let data_len = chart.m_data.len() as i32;
	if(first_touch && chart.m_splot.m_id > 0){
	    let new_index = get_chart_index(chart, FCPoint{x:mp_x, y:mp_y});
	    if(new_index >= 0 && new_index < data_len){
	        let new_date = get_chart_date_by_index(chart, new_index);
	        let new_value = get_chart_value(chart, FCPoint{x:mp_x, y:mp_y});
            if (chart.m_select_plot_point == 0){
	            chart.m_splot.m_key1 = new_date;
                chart.m_splot.m_value1 = new_value;
            } else if (chart.m_select_plot_point == 1){
                chart.m_splot.m_key2 = new_date;
                chart.m_splot.m_value2 = new_value;
            } else if (chart.m_select_plot_point == 2){
                chart.m_splot.m_key3 = new_date;
                chart.m_splot.m_value3 = new_value;
	        }
            else if (chart.m_start_move_plot){
	            let bvalue = get_chart_value(chart, FCPoint{x:chart.m_mouse_down_position.x, y:chart.m_mouse_down_position.y});
	            let bindex = get_chart_index(chart, FCPoint{x:chart.m_mouse_down_position.x, y:chart.m_mouse_down_position.y});
                if (chart.m_splot.m_key1 > 0.0){
                    chart.m_splot.m_value1 = chart.m_splot.m_start_value1 + (new_value - bvalue);
                    let start_index1 = get_chart_index_by_date(chart, chart.m_splot.m_start_key1);
                    let mut new_index1 = start_index1 + (new_index - bindex);
                    if(new_index1 < 0){
                        new_index1 = 0;
                    }
                    else if(new_index1 > data_len - 1){
                        new_index1 = data_len - 1;
                    }
                    chart.m_splot.m_key1 = get_chart_date_by_index(chart, new_index1);
                }
                if (chart.m_splot.m_key2 > 0.0){
                    chart.m_splot.m_value2 = chart.m_splot.m_start_value2 + (new_value - bvalue);
                    let start_index2 = get_chart_index_by_date(chart, chart.m_splot.m_start_key2);
                    let mut new_index2 = start_index2 + (new_index - bindex);
                    if(new_index2 < 0){
                        new_index2 = 0;
                    }
                    else if(new_index2 > data_len - 1){
                        new_index2 = data_len - 1;
                    }
                    chart.m_splot.m_key2 = get_chart_date_by_index(chart, new_index2);
                }
                if (chart.m_splot.m_key3 > 0.0){
                    chart.m_splot.m_value3 = chart.m_splot.m_start_value3 + (new_value - bvalue);
                    let start_index3 = get_chart_index_by_date(chart, chart.m_splot.m_start_key3);
                    let mut new_index3 = start_index3 + (new_index - bindex);
                    if(new_index3 < 0){
                        new_index3 = 0;
                    }
                    else if(new_index3 > data_len - 1){
                        new_index3 = data_len - 1;
                    }
                    chart.m_splot.m_key3 = get_chart_date_by_index(chart, new_index3);
                }
            }
	    }
	    for i in 0..chart.m_plots.len(){
			let mut plot = chart.m_plots[i].clone();
			if(plot.m_id == chart.m_splot.m_id){
				chart.m_plots[i] = chart.m_splot.clone();
				break;
			}
		}
	    return;
	}
	if (first_touch && second_touch) {
        if (first_point.x > second_point.x) {
            chart.m_first_touch_point_cache.x = mp_x1;
            chart.m_first_touch_point_cache.y = mp_y2;
            chart.m_second_touch_point_cache.x = mp_x;
            chart.m_second_touch_point_cache.y = mp_y;
        } else {
            chart.m_first_touch_point_cache.x = mp_x;
            chart.m_first_touch_point_cache.y = mp_y;
            chart.m_second_touch_point_cache.x = mp_x1;
            chart.m_second_touch_point_cache.y = mp_y2;
        }
        if (chart.m_first_touch_index_cache == -1 || chart.m_second_touch_index_cache == -1) {
            chart.m_first_touch_index_cache = get_chart_index(chart, FCPoint{x:chart.m_first_touch_point_cache.x, y:chart.m_first_touch_point_cache.y});
            chart.m_second_touch_index_cache = get_chart_index(chart, FCPoint{x:chart.m_second_touch_point_cache.x, y:chart.m_second_touch_point_cache.y});
            chart.m_first_index_cache = chart.m_first_visible_index;
            chart.m_last_index_cache = chart.m_last_visible_index;
        }
    } else if (first_touch) {
        chart.m_second_touch_index_cache = -1;
        if (chart.m_first_touch_index_cache == -1) {
            chart.m_first_touch_point_cache.x = mp_x;
            chart.m_first_touch_point_cache.y = mp_y;
            chart.m_first_touch_index_cache = get_chart_index(chart, FCPoint{x:chart.m_first_touch_point_cache.x, y:chart.m_first_touch_point_cache.y});
            chart.m_first_index_cache = chart.m_first_visible_index;
            chart.m_last_index_cache = chart.m_last_visible_index;
        }
    }

    if (first_touch && second_touch) {
        if (chart.m_first_touch_index_cache != -1 && chart.m_second_touch_index_cache != -1) {
            let mut f_point = FCPoint{x:mp_x, y:mp_y};
            let mut s_point = FCPoint{x:mp_x1, y:mp_y2};
            if (first_point.x > second_point.x) {
                f_point.x = mp_x1;
                f_point.y = mp_y2;
                s_point.x = mp_x;
                s_point.y = mp_y;
            }
            let sub_x = (s_point.x - f_point.x).abs();
            let sub_index = (chart.m_second_touch_index_cache - chart.m_first_touch_index_cache).abs();
            if (sub_x > 0.0 && sub_index > 0) {
                let mut new_scale_pixel = sub_x / (sub_index as f32);
                if (new_scale_pixel >= 3.0) {
                    let int_scale_pixel = new_scale_pixel as i32;
                    new_scale_pixel = int_scale_pixel as f32;
                }
                if (new_scale_pixel != chart.m_hscale_pixel) {
                    let mut new_first_index = chart.m_first_touch_index_cache;
                    let mut this_x = f_point.x;
                    this_x = this_x - new_scale_pixel;
                    while (this_x > chart.m_left_vscale_width + new_scale_pixel) {
                        new_first_index = new_first_index - 1;
                        if (new_first_index < 0) {
                            new_first_index = 0;
                            break;
                        }
                        this_x = this_x - new_scale_pixel;
                    }

                    this_x = s_point.x;
                    let mut new_second_index = chart.m_second_touch_index_cache;
                    this_x = this_x + new_scale_pixel;
                    while (this_x < chart.m_view.m_size.cx - chart.m_right_vscale_width - new_scale_pixel) {
                        new_second_index = new_second_index + 1;
                        if (new_second_index > data_len) {
                            new_second_index = data_len - 1;
                            break;
                        }
                        this_x = this_x + new_scale_pixel;
                    }
                    set_chart_visible_index(chart, new_first_index, new_second_index);
                    let working_area_width = get_chart_workarea_width(chart);
                    let max_visible_record = get_max_visible_count(chart, chart.m_hscale_pixel, working_area_width);
                    while (max_visible_record < chart.m_last_visible_index - chart.m_first_visible_index + 1
                          && chart.m_last_visible_index > chart.m_first_visible_index) {
                        chart.m_last_visible_index = chart.m_last_visible_index - 1;
                    }
                    check_chart_last_visible_index(chart);
                    reset_chart_visible_record(chart);
                    calculate_chart_max_min(chart);
                }
            }
        }
    } else if (first_touch) {
        let mut sub_index = ((chart.m_first_touch_point_cache.x - first_point.x) / chart.m_hscale_pixel) as i32;
        if (chart.m_last_visible_index + sub_index > data_len - 1) {
            sub_index = data_len - 1 - chart.m_last_index_cache;
        } else if (chart.m_first_visible_index + sub_index < 0) {
            sub_index = chart.m_first_index_cache;
        }
        chart.m_first_visible_index = chart.m_first_index_cache + sub_index;
        chart.m_last_visible_index = chart.m_last_index_cache + sub_index;
        check_chart_last_visible_index(chart);
        reset_chart_visible_record(chart);
        calculate_chart_max_min(chart);
    }
}

pub fn get_lrband_range(chart:&mut FCChart, plot:&mut FCPlot, a:f32, b:f32, up_sub_value:&mut f64, down_sub_value:&mut f64){
    let mut bindex = get_chart_index_by_date(chart, plot.m_key1);
    let mut eindex = get_chart_index_by_date(chart, plot.m_key2);
    let mut temp_bindex : i32 = 0;
	let mut temp_eindex : i32 = 0;
	if(bindex > eindex){
		temp_bindex = eindex;
		temp_eindex = bindex;
	}else{
		temp_bindex = bindex;
		temp_eindex = eindex;
	}
    bindex = temp_bindex;
    eindex = temp_eindex;
	let mut high_list: Vec<f64> = Vec::new();
	let mut low_list: Vec<f64> = Vec::new();
	for i in bindex..(eindex + 1){
        let high = chart.m_data[i as usize].m_high;
        let low = chart.m_data[i as usize].m_low;
        let mid_value = (((i - bindex + 1) as f32) * a + b) as f64;
		high_list.push(high - mid_value);
        low_list.push(mid_value - low);
	}
    *up_sub_value = max_value(high_list);
    *down_sub_value = max_value(low_list);
}

pub fn zoom_out_chart(chart:&mut FCChart){
    if (!chart.m_auto_fill_hscale) {
        let mut h_scale_pixel = chart.m_hscale_pixel;
        let old_x = get_chart_x(chart, chart.m_cross_stop_index);
        let pure_h = get_chart_workarea_width(chart);
        let mut ori_max : i32 = -1;
        let mut max : i32 = -1;
        let mut deal : i32 = 0;
        let data_count = chart.m_data.len() as i32;
        let mut findex = chart.m_first_visible_index;
        let mut lindex = chart.m_last_visible_index;
        if (h_scale_pixel < 30.0) {
            ori_max = get_max_visible_count(chart, h_scale_pixel, pure_h);
            if (data_count < ori_max) {
                deal = 1;
            }
            if (h_scale_pixel > 3.0) {
                h_scale_pixel = h_scale_pixel + 1.0;
            } else {
                if (h_scale_pixel == 1.0) {
                    h_scale_pixel = 2.0;
                } else {
                    h_scale_pixel = h_scale_pixel * 1.5;
                    if (h_scale_pixel > 3.0) {
                        h_scale_pixel = (h_scale_pixel as i32) as f32;
                    }
                }
            }
            max = get_max_visible_count(chart, h_scale_pixel, pure_h);
            if (data_count >= max) {
                if (deal == 1) {
                    lindex = data_count - 1;
                }
                findex = lindex - max + 1;
                if (findex < 0) {
                    findex = 0;
                }
            }
        }
        chart.m_hscale_pixel = h_scale_pixel;
        chart.m_first_visible_index = findex;
        chart.m_last_visible_index = lindex;
        if (chart.m_show_cross_line){
            let mut new_x = get_chart_x(chart, chart.m_cross_stop_index);
            if (new_x > old_x) {
                while (chart.m_last_visible_index < data_count - 1){
                    chart.m_first_visible_index = chart.m_first_visible_index + 1;
                    chart.m_last_visible_index = chart.m_last_visible_index + 1;
                    new_x = get_chart_x(chart, chart.m_cross_stop_index);
                    if (new_x <= old_x){
                        break;
                    }
                }

            }
            else if (new_x < old_x){
                while (chart.m_first_visible_index > 0){
                    chart.m_first_visible_index = chart.m_first_visible_index - 1;
                    chart.m_last_visible_index = chart.m_last_visible_index - 1;
                    new_x = get_chart_x(chart, chart.m_cross_stop_index);
                    if (new_x >= old_x){
                        break;
                    }
                }
            }
        }
        check_chart_last_visible_index(chart);
        calculate_chart_max_min(chart);
    }
}

pub fn zoom_in_chart(chart:&mut FCChart){
     if (!chart.m_auto_fill_hscale) {
        let mut h_scale_pixel = chart.m_hscale_pixel;
        let old_x = get_chart_x(chart, chart.m_cross_stop_index);
        let pure_h = get_chart_workarea_width(chart);
        let mut max : i32 = -1;
        let data_count = chart.m_data.len() as i32;
        let mut findex = chart.m_first_visible_index;
        let mut lindex = chart.m_last_visible_index;
        if (h_scale_pixel > 3.0) {
            h_scale_pixel -= 1.0;
        } else {
            h_scale_pixel = h_scale_pixel * 2.0 / 3.0;
            if (h_scale_pixel > 3.0) {
                 h_scale_pixel = (h_scale_pixel as i32) as f32;
            }
        }
        max = get_max_visible_count(chart, h_scale_pixel, pure_h);
        if (max >= data_count) {
            if (h_scale_pixel < 1.0) {
                h_scale_pixel = pure_h / (max as f32);
            }
            findex = 0;
            lindex = data_count - 1;
        } else {
            findex = lindex - max + 1;
            if (findex < 0) {
                findex = 0;
            }
        }
        chart.m_hscale_pixel = h_scale_pixel;
        chart.m_first_visible_index = findex;
        chart.m_last_visible_index = lindex;
        if (chart.m_show_cross_line){
            let mut new_x = get_chart_x(chart, chart.m_cross_stop_index);
            if (new_x > old_x){
                while (chart.m_last_visible_index < data_count - 1){
                    chart.m_first_visible_index = chart.m_first_visible_index + 1;
                    chart.m_last_visible_index = chart.m_last_visible_index + 1;
                    new_x = get_chart_x(chart, chart.m_cross_stop_index);
                    if (new_x <= old_x)
                    {
                        break;
                    }
                }

            }
            else if (new_x < old_x){
                while (chart.m_first_visible_index > 0){
                    chart.m_first_visible_index = chart.m_first_visible_index - 1;
                    chart.m_last_visible_index = chart.m_last_visible_index - 1;
                    new_x = get_chart_x(chart, chart.m_cross_stop_index);
                    if (new_x >= old_x){
                        break;
                    }
                }
            }
        }
        check_chart_last_visible_index(chart);
        calculate_chart_max_min(chart);
    }
}

pub fn llv_value(ticks:Vec<f64>, days:i32)->Vec<f64>{
    let mut llv: Vec<f64> = Vec::new();
    let min = ticks[0];
    for i in 0..ticks.len(){
		let mut min : f64 = 0.0;
		if((i as i32) >= days) {
			min = ticks[i as usize];
            let mut j = i as i32;
            while(j > (i as i32) - days){
                if(min > ticks[j as usize]) {
					min = ticks[j as usize];
				}
                j = j - 1;
            }
			llv.push(min);
		} else {
			if(min > ticks[i as usize]) {
				min = ticks[i as usize];
			}
			llv.push(min);
		}
	}
    return llv;
}

pub fn hhv_value(ticks:Vec<f64>, days:i32)->Vec<f64>{
    let mut hhv: Vec<f64> = Vec::new();
	let max = ticks[0];
	for i in 0..ticks.len(){
		let mut max : f64 = 0.0;
		if((i as i32) >= days) {
			max = ticks[i as usize];
			let mut j = i as i32;
            while(j > (i as i32) - days){
                if(max < ticks[j as usize]) {
					max = ticks[j as usize];
				}
                j = j - 1;
            }
			hhv.push(max);
		} else {
			if(max > ticks[i as usize]) {
				max = ticks[i as usize];
			}
			hhv.push(max);
		}
	}
	return hhv;
}

pub fn ref_value(ticks:Vec<f64>, days:i32)->Vec<f64>{
    let mut ref_arr: Vec<f64> = Vec::new();
	let length = ticks.len();
    for i in 0..length{
		let mut ref_value : f64 = 0.0;
		if((i as i32) >= days) {
			ref_value = ticks[(i as i32 - days) as usize];
		} else {
			ref_value = ticks[0];
		}
		ref_arr.push(ref_value);
	}
	return ref_arr;
}

pub fn get_ema(n:i32, value:f64, last_ema:f64)->f64{
    return (value * 2.0 + last_ema * ((n - 1) as f64)) / ((n + 1) as f64);
}

pub fn get_macd(dif:Vec<f64>, dea:Vec<f64>)->Vec<f64>{
    let mut result: Vec<f64> = Vec::new();
    for i in 0..dif.len(){
			result.push((dif[i as usize] - dea[i as usize]) * 2.0);
    }
    return result;
}

pub fn get_dif(close12:Vec<f64>, close26:Vec<f64>)->Vec<f64>{
	let mut result : Vec<f64> = Vec::new();
	for i in 0..close12.len(){
		result.push(close12[i] - close26[i]);
	}
	return result;
}

pub fn get_boll_data(ticks:Vec<f64>, ups:&mut Vec<f64>, mas:&mut Vec<f64>, lows:&mut Vec<f64>){
    let ma_days : i32 = 20;
	let tick_begin = ma_days - 1;
	let mut ma_sum : f64= 0.0;
	let mut p : f64 = 0.0;
    for i in 0..ticks.len(){
		let c = ticks[i as usize];
        let mut ma : f64 = 0.0;
        let mut md : f64 = 0.0;
        let mut bstart : i32 = 0;
        let mut md_sum : f64 = 0.0;
		ma_sum = ma_sum + c;
        if((i as i32) >= tick_begin) {
			ma_sum = ma_sum - p;
			ma = ma_sum / (ma_days as f64);
			bstart = (i as i32) - tick_begin;
			p = ticks[bstart as usize];
			mas.push(ma);
			bstart = (i as i32) - tick_begin;
			p = ticks[bstart as usize];
			let mut values:Vec<f64> = Vec::new();
			for j in bstart..(bstart + ma_days){
				values.push(ticks[j as usize]);
			}
			md_sum = standard_deviation_sum(values.clone(), ma, 2.0);
			md = (md_sum / (ma_days as f64)).sqrt();
			ups.push(ma + 2.0 * md);
			lows.push(ma - 2.0 * md);
		} else {
			ma = ma_sum / ((i + 1) as f64);
			mas.push(ma);
			let mut values:Vec<f64> = Vec::new();
			for j in 0..(i + 1){
				values.push(ticks[j as usize]);
			}
			md_sum = standard_deviation_sum(values.clone(), ma, 2.0);
			md = (md_sum / (i + 1) as f64).sqrt();
			ups.push(ma + 2.0 * md);
			lows.push(ma - 2.0 * md);
		}
	}
}

pub fn get_max_high_and_min_low(high_arr:Vec<f64>, low_arr:Vec<f64>, max_high:&mut f64, min_low:&mut f64){
    for i in 0..low_arr.len(){
        let high = high_arr[i as usize];
		let low = low_arr[i as usize];
		if(high > *max_high) {
			*max_high = high;
		}
		if(low < *min_low) {
			*min_low = low;
		}
    }
}

pub fn get_kdj_data(high_arr:Vec<f64>, low_arr:Vec<f64>, close_arr:Vec<f64>, ks:&mut Vec<f64>, ds:&mut Vec<f64>, js:&mut Vec<f64>){
    let days : i32 = 9;
    let mut rsvs: Vec<f64> = Vec::new();
    let mut last_k : f64 = 0.0;
    let mut last_d : f64 = 0.0;
    let mut cur_k : f64 = 0.0;
    let mut cur_d : f64 = 0.0;
    for i in 0..high_arr.len(){
        let mut high_list: Vec<f64> = Vec::new();
        let mut low_list: Vec<f64> = Vec::new();
        let mut start_index : i32 = i as i32 - days;
        if(start_index < 0){
            start_index = 0;
        }
        for j in start_index..((i + 1) as i32){
            high_list.push(high_arr[j as usize]);
            low_list.push(low_arr[j as usize]);
        }
        let mut max : f64 = 0.0;
        let mut min : f64 = 0.0;
        let close = close_arr[i as usize];
        get_max_high_and_min_low(high_list.to_vec(), low_list.to_vec(), &mut max, &mut min);
        if(max == min) {
			rsvs.push(0.0);
		} else {
			rsvs.push((close - min) / (max - min) * 100.0);
		}
		if(i == 0) {
			last_k = rsvs[i as usize];
            last_d = rsvs[i as usize];
		}
		cur_k = 2.0 / 3.0 * last_k + 1.0 / 3.0 * rsvs[i as usize];
		ks.push(cur_k);
		last_k = cur_k;

		cur_d = 2.0 / 3.0 * last_d + 1.0 / 3.0 * cur_k;
		ds.push(cur_d);
		last_d = cur_d;

		js.push(3.0 * cur_k - 2.0 * cur_d);
    }
}

pub fn get_rsi_data(ticks:Vec<f64>, rsi1:&mut Vec<f64>, rsi2:&mut Vec<f64>, rsi3:&mut Vec<f64>){
    let n1 : i32 =  6;
    let n2 : i32 =  12;
    let n3 : i32 =  24;
    let mut last_close_px = ticks[0];
    let mut last_sm1: f64 = 0.0;
    let mut last_sa1 : f64 = 0.0;
    let mut last_sm2: f64 = 0.0;
    let mut last_sa2 : f64 = 0.0;
    let mut last_sm3: f64 = 0.0;
    let mut last_sa3 : f64 = 0.0;
    for i in 0..ticks.len(){
        let c = ticks[i as usize];
		let m = (c - last_close_px).max(0.0);
		let a = (c - last_close_px).abs();
        if(i == 0){
            last_sm1 = 0.0;
            last_sa1 = 0.0;
            rsi1.push(0.0);
        }else{
            last_sm1 = (m + ((n1 - 1) as f64) * last_sm1) / (n1 as f64);
		    last_sa1 = (a + ((n1 - 1) as f64) * last_sa1)/ (n1 as f64);
            if(last_sa1 != 0.0) {
                rsi1.push(last_sm1 / last_sa1 * 100.0);
			} else {
				rsi1.push(0.0);
			}
        }

         if(i == 0){
            last_sm2 = 0.0;
            last_sa2 = 0.0;
            rsi2.push(0.0);
        }else{
            last_sm2 = (m + ((n2 - 1) as f64) * last_sm2) / (n2 as f64);
		    last_sa2 = (a + ((n2 - 1) as f64) * last_sa2)/ (n2 as f64);
            if(last_sa2 != 0.0) {
                rsi2.push(last_sm2 / last_sa2 * 100.0);
			} else {
				rsi2.push(0.0);
			}
        }

         if(i == 0){
            last_sm3 = 0.0;
            last_sa3 = 0.0;
            rsi3.push(0.0);
        }else{
            last_sm3 = (m + ((n3 - 1) as f64) * last_sm3) / (n3 as f64);
		    last_sa3 = (a + ((n3 - 1) as f64) * last_sa3)/ (n3 as f64);
            if(last_sa3 != 0.0) {
                rsi3.push(last_sm3 / last_sa3 * 100.0);
			} else {
				rsi3.push(0.0);
			}
        }
        last_close_px =  c;
    }
}

pub fn ma_value(ticks:Vec<f64>, days:i32)->Vec<f64>{
    let mut ma_sum : f64 = 0.0;
	let mut mas: Vec<f64> = Vec::new();
	let mut last : f64 = 0.0;
	for i in 0..ticks.len(){
		let mut ma : f64 = 0.0;
		if((i as i32) >= days) {
			last = ticks[(i as i32 - days) as usize];
			ma_sum = ma_sum + ticks[i as usize] - last;
			ma = ma_sum / (days as f64);
		} else {
			ma_sum = ma_sum + ticks[i as usize];
			ma = ma_sum / ((i + 1) as f64);
		}
		mas.push(ma);
	}
	return mas;
}

pub fn get_roc_data(ticks:Vec<f64>, roc:&mut Vec<f64>, maroc:&mut Vec<f64>){
    let n : i32 = 12;
	let m : i32 = 6;
	for i in 0..ticks.len(){
		let mut curr_roc : f64 = 0.0;
		if((i as i32) >= n) {
			curr_roc = 100.0 * (ticks[i as usize] - ticks[(i as i32 - n) as usize]) / ticks[(i as i32 - n) as usize];
			roc.push(curr_roc);
		} else {
			curr_roc = 100.0 * (ticks[i as usize] - ticks[0]) / ticks[0];
			roc.push(curr_roc);
		}
	}
	let ma_result = ma_value(roc.to_vec(), m);
    for i in 0..ma_result.len(){
        maroc.push(ma_result[i as usize]);
    }
}

pub fn get_bias_data(ticks:Vec<f64>, bias1_arr:&mut Vec<f64>, bias2_arr:&mut Vec<f64>, bias3_arr:&mut Vec<f64>){
    let n1 : i32 = 6;
	let n2 : i32 = 12;
	let n3 : i32 = 24;
    let ma1 = ma_value(ticks.to_vec(), n1);
	let ma2 = ma_value(ticks.to_vec(), n2);
	let ma3 = ma_value(ticks.to_vec(), n3);
	for i in 0..ticks.len(){
		let mut b1 : f64 = 0.0;
        let mut b2 : f64 = 0.0;
        let mut b3 : f64 = 0.0;
        let ui = i as usize;
		b1 = (ticks[ui] - ma1[ui]) / ma1[ui] * 100.0;
		b2 = (ticks[ui] - ma2[ui]) / ma2[ui] * 100.0;
		b3 = (ticks[ui] - ma3[ui]) / ma3[ui] * 100.0;
		bias1_arr.push(b1);
		bias2_arr.push(b2);
		bias3_arr.push(b3);
	}
}

pub fn get_dma_data(ticks:Vec<f64>, dif_arr:&mut Vec<f64>, difma_arr:&mut Vec<f64>){
    let n1 : i32 = 10;
	let n2 : i32 = 50;
	let ma10 = ma_value(ticks.to_vec(), n1);
	let ma50 = ma_value(ticks.to_vec(), n2);
	for i in 0..ticks.len(){
		let dif = ma10[i as usize] - ma50[i as usize];
		dif_arr.push(dif);
	}
    let ma_result = ma_value(dif_arr.to_vec(), n1);
    for i in 0..ma_result.len(){
        difma_arr.push(ma_result[i as usize]);
    }
}

pub fn get_bbi_data(ticks:Vec<f64>, bbi_arr:&mut Vec<f64>){
    let ma3 = ma_value(ticks.to_vec(), 3);
	let ma6 = ma_value(ticks.to_vec(), 6);
	let ma12 = ma_value(ticks.to_vec(), 12);
	let ma24 = ma_value(ticks.to_vec(), 24);
	for i in 0..ticks.len(){
        let ui = i as usize;
		let bbi = (ma3[ui] + ma6[ui] + ma12[ui] + ma24[ui]) / 4.0;
		bbi_arr.push(bbi);
	}
}

pub fn get_wr_data(high_arr:Vec<f64>, low_arr:Vec<f64>, close_arr:Vec<f64>, wr1_arr:&mut Vec<f64>, wr2_arr:&mut Vec<f64>){
    let n1 : i32 = 5;
	let n2 : i32 = 10;
	let high_arr1 = hhv_value(high_arr.to_vec(), n1);
	let high_arr2 = hhv_value(high_arr.to_vec(), n2);
	let low_arr1 = llv_value(low_arr.to_vec(), n1);
	let low_arr2 = llv_value(low_arr.to_vec(), n2);
	for i in 0..close_arr.len(){
        let ui = i as usize;
		let high1 = high_arr1[ui];
		let low1 = low_arr1[ui];
		let high2 = high_arr2[ui];
		let low2 = low_arr2[ui];
		let close = close_arr[ui];
		let wr1 = 100.0 * (high1 - close) / (high1 - low1);
		let wr2 = 100.0 * (high2 - close) / (high2 - low2);
		wr1_arr.push(wr1);
		wr2_arr.push(wr2);
	}
}

pub fn get_cci_data(high_arr:Vec<f64>, low_arr:Vec<f64>, close_arr:Vec<f64>, cci_arr:&mut Vec<f64>){
    let n : i32 = 14;
    let mut tp_arr: Vec<f64> = Vec::new();
	for i in 0..close_arr.len(){
        let iu = i as usize;
		tp_arr.push((high_arr[iu] + low_arr[iu] + close_arr[iu]) / 3.0);
	}
	let ma_close = ma_value(close_arr.to_vec(), n);
    let mut md_arr: Vec<f64> = Vec::new();
	for i in 0..close_arr.len(){
		md_arr.push(ma_close[i as usize] - close_arr[i as usize]);
	}
	let ma_md = ma_value(md_arr.to_vec(), n);
	for i in 0..close_arr.len(){
        let iu = i as usize;
		let mut cci:f64 = 0.0;
		if(ma_md[iu] != 0.0){
			cci = (tp_arr[iu] - ma_close[iu]) / (ma_md[iu] * 0.015);
		}
		cci_arr.push(cci);
	}
}


pub fn get_trix_data(ticks:Vec<f64>, trix_arr:&mut Vec<f64>, matrix_arr:&mut Vec<f64>){
    let mut mtr_arr: Vec<f64> = Vec::new();
    let n : i32 = 12;
    let m : i32 = 9;
    let mut ema_arr1: Vec<f64> = Vec::new();
	ema_arr1.push(ticks[0]);
    for i in 1..ticks.len(){
		ema_arr1.push(get_ema(12, ticks[i as usize], ema_arr1[(i as i32 - 1) as usize]));
	}

    let mut ema_arr2: Vec<f64> = Vec::new();
	ema_arr2.push(ema_arr1[0]);
    for i in 1..ticks.len(){
		ema_arr2.push(get_ema(12, ema_arr1[i as usize], ema_arr2[(i as i32 - 1) as usize]));
	}

	mtr_arr.push(ema_arr2[0]);
	for i in 1..ticks.len(){
		mtr_arr.push(get_ema(12, ema_arr2[i as usize], mtr_arr[(i as i32 - 1) as usize]));
	}

	let ref_value = ref_value(mtr_arr.to_vec(), 1);
	for i in 0..ticks.len(){
        let iu = i as usize;
		let trix = 100.0 * (mtr_arr[iu] - ref_value[iu]) / ref_value[iu];
		trix_arr.push(trix);
	}
    let ma_result = ma_value(trix_arr.to_vec(), m);
    for i in 0..ma_result.len(){
        matrix_arr.push(ma_result[i as usize]);
    }
}

pub fn get_percent_params(y1:f32, y2:f32)->Vec<f32>{
    let mut y0 : f32 = 0.0;
    let mut y25 : f32 = 0.0;
    let mut y50 : f32 = 0.0;
    let mut y75 : f32 = 0.0;
    let mut y100 : f32 = 0.0;
    y0 = y1;
    if(y1 <= y2){
        y25 = y1 + (y2 - y1) / 4.0;
        y50 = y1 + (y2 - y1) / 2.0;
        y75 = y1 + (y2 - y1) * 3.0 / 4.0;
    }else{
        y25 = y2 + (y1 - y2) * 3.0 / 4.0;
        y50 = y2 + (y1 - y2) / 2.0;
        y75 = y2 + (y1 - y2) / 4.0;
    }
    y100 = y2;
    let mut list: Vec<f32> = Vec::new();
    list.push(y0);
    list.push(y25);
    list.push(y50);
    list.push(y75);
    list.push(y100);
    return list;
}

pub fn select_plot(chart:&mut FCChart, mp:FCPoint)->FCPlot{
    chart.m_start_move_plot = false;
    chart.m_select_plot_point = -1;
    let plot_point_size : f32 = 5.0;
    let mut splot:FCPlot = FCPlot::new();
    for i in 0..chart.m_plots.len(){
		let mut plot = chart.m_plots[i].clone();
        let mut m_index1 : i32 = 0;
        let mut m_index2 : i32 = 0;
        let mut m_index3 : i32 = 0;
        let mut mpx1 : f32 = 0.0;
        let mut mpy1 : f32 = 0.0;
        let mut mpx2 : f32 = 0.0;
        let mut mpy2 : f32 = 0.0;
        let mut mpx3 : f32 = 0.0;
        let mut mpy3 : f32 = 0.0;
        if(plot.m_key1 > 0.0){
            m_index1 = get_chart_index_by_date(chart, plot.m_key1);
            mpx1 = get_chart_x(chart, m_index1);
            mpy1 = get_chart_y(chart, 0, plot.m_value1);
            if(mp.x >= mpx1 - plot_point_size && mp.x <= mpx1 + plot_point_size && mp.y >= mpy1 - plot_point_size && mp.y <= mpy1 + plot_point_size){
                splot = plot.clone();
                chart.m_select_plot_point = 0;
                break;
            }
        }
        if(plot.m_key2 > 0.0){
            m_index2 = get_chart_index_by_date(chart, plot.m_key2);
            mpx2 = get_chart_x(chart, m_index2);
            mpy2 = get_chart_y(chart, 0, plot.m_value2);
            if(mp.x >= mpx2 - plot_point_size && mp.x <= mpx2 + plot_point_size && mp.y >= mpy2 - plot_point_size && mp.y <= mpy2 + plot_point_size){
                splot = plot.clone();
                chart.m_select_plot_point = 1;
                break;
            }
        }
        if(plot.m_key3 > 0.0){
            m_index3 = get_chart_index_by_date(chart, plot.m_key3);
            mpx3 = get_chart_x(chart, m_index3);
            mpy3 = get_chart_y(chart, 0, plot.m_value3);
            if(mp.x >= mpx3 - plot_point_size && mp.x <= mpx3 + plot_point_size && mp.y >= mpy3 - plot_point_size && mp.y <= mpy3 + plot_point_size){
                splot = plot.clone();
                chart.m_select_plot_point = 2;
                break;
            }
        }
        if(chart.m_select_plot_point == -1){
            if(plot.m_plot_type == "Line"){
                chart.m_start_move_plot = select_line(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
            }
            else if(plot.m_plot_type == "AngleLine"){
                chart.m_start_move_plot = select_line(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_line(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx3, mpy3);
                }
            }
            else if(plot.m_plot_type == "Parallel"){
                chart.m_start_move_plot = select_line(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if (!chart.m_start_move_plot){
                    let mut k : f32 = 0.0;
                    let mut b : f32 = 0.0;
                    line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
                    let new_b = mpy3 - k * mpx3;
                    if(mpx2 == mpx1){
                        if(mp.x >= mpx3 - plot_point_size && mp.x <= mpx3 + plot_point_size){
                            chart.m_start_move_plot = true;
                        }
                    }else{
                        let new_x1 = chart.m_left_vscale_width;
                        let new_y1 = new_x1 * k + new_b;
                        let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                        let new_y2 = new_x2 * k + new_b;
                        chart.m_start_move_plot = select_line(FCPoint{x:mp.x, y:mp.y}, new_x1, new_y1, new_x2, new_y2);
                    }
                }
            }
            else if(plot.m_plot_type == "LRLine"){
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
            }
            else if(plot.m_plot_type == "Segment"){
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
            }else if(plot.m_plot_type == "Ray"){
                chart.m_start_move_plot = select_ray(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
            }
            else if(plot.m_plot_type == "Triangle"){
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx2, mpy2, mpx3, mpy3);
                }
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx3, mpy3);
                }
            }
            else if(plot.m_plot_type == "Rect"){
                let s_x1 : f32 = mpx1.min(mpx2);
                let s_y1 : f32 = mpy1.min(mpy2);
                let s_x2 : f32 = mpx1.max(mpx2);
                let s_y2 : f32 = mpy1.max(mpy2);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x2, s_y1);
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x2, s_y1, s_x2, s_y2);
                }
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y2, s_x2, s_y2);
                }
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x1, s_y2);
                }
            }
            else if(plot.m_plot_type == "BoxLine"){
                let s_x1 : f32 = mpx1.min(mpx2);
                let s_y1 : f32 = mpy1.min(mpy2);
                let s_x2 : f32 = mpx1.max(mpx2);
                let s_y2 : f32 = mpy1.max(mpy2);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x2, s_y1);
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x2, s_y1, s_x2, s_y2);
                }
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y2, s_x2, s_y2);
                }
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x1, s_y2);
                }
            }
            else if(plot.m_plot_type == "TironeLevels"){
                let s_x1 : f32 = mpx1.min(mpx2);
                let s_y1 : f32 = mpy1.min(mpy2);
                let s_x2 : f32 = mpx1.max(mpx2);
                let s_y2 : f32 = mpy1.max(mpy2);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x2, s_y1);
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y2, s_x2, s_y2);
                }
            }
            else if(plot.m_plot_type == "GoldenRatio"){
                let s_x1 : f32 = mpx1.min(mpx2);
                let s_y1 : f32 = mpy1.min(mpy2);
                let s_x2 : f32 = mpx1.max(mpx2);
                let s_y2 : f32 = mpy1.max(mpy2);
                let mut ranges: Vec<f32> = Vec::new();
                ranges.push(0.0);
                ranges.push(0.236);
                ranges.push(0.382);
                ranges.push(0.5);
                ranges.push(0.618);
                ranges.push(0.809);
                ranges.push(1.0);
                ranges.push(1.382);
                ranges.push(1.618);
                ranges.push(2.0);
                ranges.push(2.382);
                ranges.push(2.618);
                let min_value = plot.m_value1.min(plot.m_value2);
                let max_value = plot.m_value1.max(plot.m_value2);
                for j in 0..ranges.len(){
                    let mut new_y : f32 = 0.0;
                    if(s_y1 <= s_y2){
                        new_y = s_y1 + (s_y2 - s_y1) * ranges[j as usize];
                    }else{
                        new_y = s_y2 + (s_y1 - s_y2) * (1.0 - ranges[j as usize]);
                    }
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, chart.m_left_vscale_width, new_y, chart.m_view.m_size.cx - chart.m_right_vscale_width, new_y);
                    if (chart.m_start_move_plot){
                        break;
                    }
                }
            }
            else if(plot.m_plot_type == "Cycle"){
                let r = ((mpx2 - mpx1) * (mpx2 - mpx1) + (mpy2 - mpy1) * (mpy2 - mpy1)).abs().sqrt();
                let round = (mp.x - mpx1) * (mp.x - mpx1) + (mp.y - mpy1) * (mp.y - mpy1);
                if (round / (r * r) >= 0.9 && round / (r * r) <= 1.1){
                    chart.m_start_move_plot = true;
                }
            }else if(plot.m_plot_type == "CircumCycle"){
                let mut o_x : f32 = 0.0;
                let mut o_y : f32 = 0.0;
                let mut r : f32 = 0.0;
                ellipse_or(mpx1, mpy1, mpx2, mpy2, mpx3, mpy3, &mut o_x, &mut o_y, &mut r);
                let round = (mp.x - o_x) * (mp.x - o_x) + (mp.y - o_y) * (mp.y - o_y);
                if (round / (r * r) >= 0.9 && round / (r * r) <= 1.1){
                    chart.m_start_move_plot = true;
                }
            }
            else if(plot.m_plot_type == "Ellipse"){
                let mut x1 : f32 = 0.0;
                let mut y1 : f32 = 0.0;
                let mut x2 : f32 = 0.0;
                let mut y2 : f32 = 0.0;
                if(mpx1 <= mpx2){
                    x1 = mpx2;
                    y1 = mpy2;
                    x2 = mpx1;
                    y2 = mpy1;
                }else{
                    x1 = mpx1;
                    y1 = mpy1;
                    x2 = mpx2;
                    y2 = mpy2;
                }
                let x = x1 - (x1 - x2);
                let mut y : f32 = 0.0;
                let width = (x1 - x2) * 2.0;
                let mut height : f32 = 0.0;
                if (y1 >= y2){
                    height = (y1 - y2) * 2.0;
                }
                else{
                    height = (y2 - y1) * 2.0;
                }
                y = y2 - height / 2.0;
                let a = width / 2.0;
                let b = height / 2.0;
                chart.m_start_move_plot = ellipse_has_point(mp.x, mp.y, x + (width / 2.0), y + (height / 2.0), a, b);
            }else if(plot.m_plot_type == "LRBand"){
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if (!chart.m_start_move_plot){
                    let mut list: Vec<f64> = Vec::new();
                    let min_index = m_index1.min(m_index2);
                    let max_index = m_index1.max(m_index2);
                    for j in min_index..(max_index + 1){
                        list.push(chart.m_data[j as usize].m_close);
                    }
                    let mut rk : f32 = 0.0;
                    let mut rb : f32 = 0.0;
                    linear_regression_equation(list, &mut rk, &mut rb);
                    let mut up_sub_value : f64 = 0.0;
                    let mut down_sub_value : f64 = 0.0;
                    get_lrband_range(chart, &mut plot, rk, rb, &mut up_sub_value, &mut down_sub_value);
                    mpy1 = get_chart_y(chart, 0, plot.m_value1 + up_sub_value);
                    mpy2 = get_chart_y(chart, 0, plot.m_value2 + up_sub_value);
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                    if (!chart.m_start_move_plot){
                        mpy1 = get_chart_y(chart, 0, plot.m_value1 - down_sub_value);
                        mpy2 = get_chart_y(chart, 0, plot.m_value2 - down_sub_value);
                        chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                    }
                }
            }else if(plot.m_plot_type == "LRChannel"){
                let mut k : f32 = 0.0;
                let mut b : f32 = 0.0;
                line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
                let right_x = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let mut right_y = right_x * k + b;
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, right_x, right_y);
                if (!chart.m_start_move_plot){
                    let mut list: Vec<f64> = Vec::new();
                    let min_index = m_index1.min(m_index2);
                    let max_index = m_index1.max(m_index2);
                    for j in min_index..(max_index + 1){
                        list.push(chart.m_data[j as usize].m_close);
                    }
                    let mut rk : f32 = 0.0;
                    let mut rb : f32 = 0.0;
                    linear_regression_equation(list, &mut rk, &mut rb);
                    let mut up_sub_value : f64 = 0.0;
                    let mut down_sub_value : f64 = 0.0;
                    get_lrband_range(chart, &mut plot, rk, rb, &mut up_sub_value, &mut down_sub_value);
                    mpy1 = get_chart_y(chart, 0, plot.m_value1 + up_sub_value);
                    mpy2 = get_chart_y(chart, 0, plot.m_value2 + up_sub_value);
                    line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
                    right_y = right_x * k + b;
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, right_x, right_y);
                    if (!chart.m_start_move_plot){
                        mpy1 = get_chart_y(chart, 0, plot.m_value1 - down_sub_value);
                        mpy2 = get_chart_y(chart, 0, plot.m_value2 - down_sub_value);
                        line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
                        right_y = right_x * k + b;
                        chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, right_x, right_y);
                    }
                }
            }else if(plot.m_plot_type == "ParalleGram"){
                let mut x4 : f32 = 0.0;
                let mut y4 : f32 = 0.0;
                parallelogram(mpx1, mpy1, mpx2, mpy2, mpx3, mpy3, &mut x4, &mut y4);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if (!chart.m_start_move_plot){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx2, mpy2, mpx3, mpy3);
                    if (!chart.m_start_move_plot){
                        chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx3, mpy3, x4, y4);
                        if (!chart.m_start_move_plot){
                            chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, x4, y4, mpx1, mpy1);
                        }
                    }
                }
            }
            else if(plot.m_plot_type == "SpeedResist"){
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if (!chart.m_start_move_plot){
                    if (mpx1 != mpx2 && mpy1 != mpy2){
                        let first_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) / 3.0};
                        let second_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) * 2.0 / 3.0};
                        let start_p = FCPoint{x:mpx1,y:mpy1};
                        let mut f_k : f32 = 0.0;
                        let mut f_b : f32 = 0.0;
                        let mut s_k : f32 = 0.0;
                        let mut s_b : f32 = 0.0;
                        line_xy(start_p.x, start_p.y, first_p.x, first_p.y, 0.0, 0.0, &mut f_k, &mut f_b);
                        line_xy(start_p.x, start_p.y, second_p.x, second_p.y, 0.0, 0.0, &mut s_k, &mut s_b);
                        let mut new_yf : f32 = 0.0;
                        let mut new_ys : f32 = 0.0;
                        let mut new_x : f32 = 0.0;
                        if (mpx2 > mpx1){
                            new_yf = f_k * (chart.m_view.m_size.cx - chart.m_right_vscale_width) + f_b;
                            new_ys = s_k * (chart.m_view.m_size.cx - chart.m_right_vscale_width) + s_b;
                            new_x = (chart.m_view.m_size.cx - chart.m_right_vscale_width);
                        }
                        else{
                            new_yf = f_b;
                            new_ys = s_b;
                            new_x = chart.m_left_vscale_width;
                        }
                        chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, start_p.x, start_p.y, new_x, new_yf);
                        if (!chart.m_start_move_plot){
                            chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, start_p.x, start_p.y, new_x, new_ys);
                        }
                    }
                }
            }else if(plot.m_plot_type == "FiboFanline"){
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if (!chart.m_start_move_plot){
                    if (mpx1 != mpx2 && mpy1 != mpy2){
                        let first_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) * 0.382};
                        let second_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) * 0.5};
                        let third_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) * 0.618};
                        let start_p = FCPoint{x:mpx1, y:mpy1};
                        let mut list_p: Vec<FCPoint> = Vec::new();
                        list_p.push(first_p.clone());
						list_p.push(second_p.clone());
						list_p.push(third_p.clone());
						let list_size = list_p.len();
                        for j in 0..list_size{
                            let mut k : f32 = 0.0;
                            let mut b : f32 = 0.0;
                            line_xy(start_p.x, start_p.y, list_p[j as usize].x, list_p[j as usize].y, 0.0, 0.0, &mut k, &mut b);
                            let mut new_x : f32 = 0.0;
                            let mut new_y : f32 = 0.0;
                            if (mpx2 > mpx1){
                                new_y = k * (chart.m_view.m_size.cx - chart.m_right_vscale_width) + b;
                                new_x = (chart.m_view.m_size.cx - chart.m_right_vscale_width);
                            }
                            else
                            {
                                new_y = b;
                                new_x = chart.m_left_vscale_width;
                            }
                            chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, start_p.x, start_p.y, new_x, new_y);
                            if (chart.m_start_move_plot){
                                break;
                            }
                        }
                    }
                }
            }
            else if(plot.m_plot_type == "FiboTimezone"){
                let mut f_value : i32 = 1;
                let aindex = m_index1;
                let mut pos : i32 = 1;
                let div_height = get_candle_div_height(chart);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, 0.0, mpx1, div_height);
                if (!chart.m_start_move_plot){
                    while (aindex + f_value <= chart.m_last_visible_index){
                        f_value = fibonacci_value(pos);
                        let new_index = aindex + f_value;
                        let new_x = get_chart_x(chart, new_index);
                        chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, new_x, 0.0, new_x, div_height);
                        if (chart.m_start_move_plot){
                            break;
                        }
                        pos = pos + 1;
                    }
                }
            }
            else if(plot.m_plot_type == "Percent"){
                let list = get_percent_params(mpy1, mpy2);
                for j in 0..list.len(){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, chart.m_left_vscale_width, list[j as usize], chart.m_view.m_size.cx - chart.m_right_vscale_width, list[j as usize]);
                    if (chart.m_start_move_plot){
                        break;
                    }
                }
            }
            if (chart.m_start_move_plot){
				plot.m_start_key1 = plot.m_key1;
                plot.m_start_value1 = plot.m_value1;
                plot.m_start_key2 = plot.m_key2;
                plot.m_start_value2 = plot.m_value2;
                plot.m_start_key3 = plot.m_key3;
                plot.m_start_value3 = plot.m_value3;
                splot = plot.clone();
                break;
            }
        }
        chart.m_plots[i] = plot.clone();
    }
    return splot.clone();
}

pub fn draw_chart_lines(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, chart:&mut FCChart, clip_rect:FCRect, div_index:i32, datas:Vec<f64>, color:String, selected:bool) {
    M_PAINT.lock().unwrap().begin_path(&context);
    let mut last_x : f32 = 0.0;
    let mut last_y : f32 = 0.0;
    let working_area_width = get_chart_workarea_width(chart);
    let max_visible_record = get_max_visible_count(chart, chart.m_hscale_pixel, working_area_width); 
    for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
        let x = get_chart_x(chart, i);
        let value = datas[i as usize];
        let y = get_chart_y(chart, div_index, value);
        if(i > chart.m_first_visible_index){
            M_PAINT.lock().unwrap().add_line(&context, last_x, last_y, x, y);
        }
        last_x = x;
        last_y = y;
        if (selected) {
            let mut kp_interval = max_visible_record / 30;
            if (kp_interval < 2) {
                kp_interval = 3;
            }
            if (i % kp_interval == 0) {
                M_PAINT.lock().unwrap().fill_rect(&context, color.clone(), x - 3.0, y - 3.0, x + 3.0, y + 3.0);
            }
        }
    }
    M_PAINT.lock().unwrap().draw_path(&context, color, chart.m_line_width, Vec::new());
	M_PAINT.lock().unwrap().close_path(&context);
}

pub fn draw_chart_stock(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, chart:&mut FCChart, clip_rect:FCRect){
    let data_len = chart.m_data.len() as i32;
    if(data_len > 0) {
		let candle_height = get_candle_div_height(chart);
        let ind_height = get_ind_div_height(chart);
        let mut c_width = (((chart.m_hscale_pixel - 3.0) / 2.0) as i32) as f32;
        let working_area_width = get_chart_workarea_width(chart);
        let max_visible_record = get_max_visible_count(chart, chart.m_hscale_pixel, working_area_width);
        if(c_width < 0.0){
            c_width = 0.0;
        }
        let mut is_trend : bool = false;
        if(chart.m_cycle == "trend"){
            is_trend = true;
        }
        if(is_trend){
            M_PAINT.lock().unwrap().begin_path(&context);
            let mut last_x : f32 = 0.0;
            let mut last_y : f32 = 0.0;
            for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
                let x = get_chart_x(chart, i);
                let close = chart.m_data[i as usize].m_close;
                let close_y = get_chart_y(chart, 0, close);
                if(i > chart.m_first_visible_index){
                    M_PAINT.lock().unwrap().add_line(&context, last_x, last_y, x, close_y)
                }
                last_x = x;
                last_y = close_y;
            }
            M_PAINT.lock().unwrap().draw_path(&context, chart.m_indicator_colors[7].clone(), chart.m_line_width, Vec::new());
		    M_PAINT.lock().unwrap().close_path(&context);
        }
        let mut has_min_tag : bool = false;
        let mut has_max_tag : bool = false;
        for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
            let x = get_chart_x(chart, i);
            let iu = i as usize;
            let open = chart.m_data[iu].m_open;
            let close = chart.m_data[iu].m_close;
            let high = chart.m_data[iu].m_high;
            let low = chart.m_data[iu].m_low;
            let volume = chart.m_data[iu].m_volume;
            let open_y = get_chart_y(chart, 0, open);
            let close_y = get_chart_y(chart, 0, close);
            let high_y = get_chart_y(chart, 0, high);
            let low_y = get_chart_y(chart, 0, low);
            let vol_y = get_chart_y(chart, 1, volume);
            let zero_y = get_chart_y(chart, 1, 0.0);
            if(close >= open){
                if(is_trend){
                    M_PAINT.lock().unwrap().draw_line(&context, chart.m_indicator_colors[6].clone(), chart.m_line_width, Vec::new(), x, vol_y, x, zero_y);
                }else{
                    M_PAINT.lock().unwrap().draw_line(&context, chart.m_up_color.clone(), chart.m_line_width, Vec::new(), x, high_y, x, low_y);
                    if(c_width > 0.0){
                        if(close == open){
                            M_PAINT.lock().unwrap().draw_line(&context, chart.m_up_color.clone(), chart.m_line_width, Vec::new(), x - c_width, close_y, x + c_width, close_y);
                        }
                        else{
                            M_PAINT.lock().unwrap().fill_rect(&context, chart.m_up_color.clone(), x - c_width, close_y, x + c_width, open_y);
                        }
                        M_PAINT.lock().unwrap().fill_rect(&context, chart.m_up_color.clone(), x - c_width, vol_y, x + c_width, zero_y);
         
                    }else
                    {
                        M_PAINT.lock().unwrap().draw_line(&context, chart.m_up_color.clone(), chart.m_line_width, Vec::new(), x - c_width, vol_y, x + c_width, zero_y);
                    }
                }
            }else{
                if(is_trend){
                    M_PAINT.lock().unwrap().draw_line(&context, chart.m_indicator_colors[6].clone(), chart.m_line_width, Vec::new(), x, vol_y, x, zero_y);
                }else{
                    M_PAINT.lock().unwrap().draw_line(&context, chart.m_down_color.clone(), chart.m_line_width, Vec::new(), x, high_y, x, low_y);
                    if(c_width > 0.0){
                        M_PAINT.lock().unwrap().fill_rect(&context, chart.m_down_color.clone(), x - c_width, open_y, x + c_width, close_y);
                        M_PAINT.lock().unwrap().fill_rect(&context, chart.m_down_color.clone(), x - c_width, vol_y, x + c_width, zero_y);
                    }else{
                        M_PAINT.lock().unwrap().draw_line(&context, chart.m_down_color.clone(), chart.m_line_width, Vec::new(), x - c_width, vol_y, x + c_width, zero_y);
                    }
                }
            }
            if (chart.m_select_shape == "CANDLE") {
                let mut kp_interval = max_visible_record / 30;
                if (kp_interval < 2) {
                    kp_interval = 3;
                }
                if (i % kp_interval == 0) {
                    if (is_trend) {
                    } else {
                        M_PAINT.lock().unwrap().fill_rect(&context, chart.m_indicator_colors[0].clone(), x - 3.0, close_y - 3.0, x + 3.0, close_y + 3.0);
                    }
                }
            } else if (chart.m_select_shape == "VOL") {
                let mut kp_interval = max_visible_record / 30;
                if (kp_interval < 2) {
                    kp_interval = 3;
                }
                if (i % kp_interval == 0) {
                    M_PAINT.lock().unwrap().fill_rect(&context, chart.m_indicator_colors[0].clone(), x - 3.0, vol_y - 3.0, x + 3.0, vol_y + 3.0);
                }
            }
            if(!is_trend){
                if(!has_max_tag){
                    if(high == chart.m_candle_max){
                        let tag = to_fixed(high, chart.m_candle_digit);
                        let t_size = M_PAINT.lock().unwrap().text_size(&context, tag.clone(), chart.m_font.clone());
                        M_PAINT.lock().unwrap().draw_text(&context, tag.clone(), chart.m_text_color.clone(), chart.m_font.clone(), x - t_size.cx / 2.0, high_y - t_size.cy / 2.0 - 2.0);
                        has_max_tag = true;
                    }
                }
                if(!has_min_tag){
                    if(low == chart.m_candle_min){
                        let tag = to_fixed(low, chart.m_candle_digit);
                        let t_size = M_PAINT.lock().unwrap().text_size(&context, tag.clone(), chart.m_font.clone());
                        M_PAINT.lock().unwrap().draw_text(&context, tag.clone(), chart.m_text_color.clone(), chart.m_font.clone(), x - t_size.cx / 2.0, low_y + 2.0 + t_size.cy / 2.0);
                        has_min_tag = true;
                    }
                }
            }
        }
        if(!is_trend){
			M_PAINT.lock().unwrap().save(&context);
            M_PAINT.lock().unwrap().set_clip(&context, chart.m_left_vscale_width, 20.0, chart.m_view.m_size.cx - chart.m_right_vscale_width, candle_height);
			if (chart.m_main_indicator == "BOLL") {
				if(chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "MID"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_mid.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_mid.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if(chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "UP"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_up.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_up.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if(chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "DOWM"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_down.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_down.clone(), chart.m_indicator_colors[2].clone(), false);
				}
			}else if (chart.m_main_indicator == "MA"){
				if(chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "5"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma5.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma5.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if(chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "10"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma10.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma10.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if(chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "20"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma20.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma20.clone(), chart.m_indicator_colors[2].clone(), false);
				}
				if(chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "30"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma30.clone(), chart.m_indicator_colors[3].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma30.clone(), chart.m_indicator_colors[3].clone(), false);
				}
				if(chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "120"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma120.clone(), chart.m_indicator_colors[4].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma120.clone(), chart.m_indicator_colors[4].clone(), false);
				}
				if(chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "250"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma250.clone(), chart.m_indicator_colors[5].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma250.clone(), chart.m_indicator_colors[5].clone(), false);
				}
			}
			M_PAINT.lock().unwrap().restore(&context);
        }
        if(ind_height > 0.0){
			if (chart.m_show_indicator == "MACD") {
				let zero_y = get_chart_y(chart, 2, 0.0);
				for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
					let x = get_chart_x(chart, i);
					let iu = i as usize;
					let macd = chart.m_allmacdarr[iu];
					let macd_y = get_chart_y(chart, 2, macd);
					if (macd_y < zero_y) {
                        M_PAINT.lock().unwrap().draw_line(&context, chart.m_indicator_colors[3].clone(), chart.m_line_width, Vec::new(), x, macd_y, x, zero_y);
                    } else {
                        M_PAINT.lock().unwrap().draw_line(&context, chart.m_indicator_colors[4].clone(), chart.m_line_width, Vec::new(), x, macd_y, x, zero_y);
                    }
                    if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "MACD"){
						 let mut kp_interval = max_visible_record / 30;
						if (kp_interval < 2) {
							kp_interval = 3;
						}
						if (i % kp_interval == 0) {
							M_PAINT.lock().unwrap().fill_rect(&context, chart.m_indicator_colors[0].clone(), x - 3.0, macd_y - 3.0, x + 3.0, macd_y + 3.0);
						}
                    }
				}
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "DIF"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_alldifarr.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_alldifarr.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "DEA"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_alldeaarr.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_alldeaarr.clone(), chart.m_indicator_colors[1].clone(), false);
				}
			} else if (chart.m_show_indicator == "KDJ") {
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "K"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_k.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_k.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "D"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_d.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_d.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "J"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_j.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_j.clone(), chart.m_indicator_colors[2].clone(), false);
				}
			} else if (chart.m_show_indicator == "RSI") {
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "6"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi1.clone(), chart.m_indicator_colors[5].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi1.clone(), chart.m_indicator_colors[5].clone(), false);
				}
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "12"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi2.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi2.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "24"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi3.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi3.clone(), chart.m_indicator_colors[2].clone(), false);
				}
			}
			else if (chart.m_show_indicator == "BIAS") {
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "1"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias1.clone(), chart.m_indicator_colors[5].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias1.clone(), chart.m_indicator_colors[5].clone(), false);
				}
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "2"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias2.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias2.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "3"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias3.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias3.clone(), chart.m_indicator_colors[2].clone(), false);
				}
			}
			else if (chart.m_show_indicator == "ROC") {
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "ROC"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_roc.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_roc.clone(), chart.m_indicator_colors[0].clone(), false);
				}
	        
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "ROCMA"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_roc_ma.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_roc_ma.clone(), chart.m_indicator_colors[1].clone(), false);
				}
			} else if (chart.m_show_indicator == "WR") {
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "1"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_wr1.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_wr1.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "2"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_wr2.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_wr2.clone(), chart.m_indicator_colors[1].clone(), false);
				}
			} else if (chart.m_show_indicator == "CCI") {
				if(chart.m_select_shape == chart.m_show_indicator){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_cci.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_cci.clone(), chart.m_indicator_colors[0].clone(), false);
				}
			} else if (chart.m_show_indicator == "BBI") {
				if(chart.m_select_shape == chart.m_show_indicator){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bbi.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bbi.clone(), chart.m_indicator_colors[0].clone(), false);
				}
			} else if (chart.m_show_indicator == "TRIX") {
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "TRIX"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_trix.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_trix.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "TRIXMA"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_trix_ma.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_trix_ma.clone(), chart.m_indicator_colors[1].clone(), false);
				}
	        
			} else if (chart.m_show_indicator == "DMA") {
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "DIF"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_dma1.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_dma1.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				
				if(chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "DIFMA"){
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_dma2.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_dma2.clone(), chart.m_indicator_colors[1].clone(), false);
				}
			}
		}
	}
}

pub fn draw_chart_plot(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, chart:&mut FCChart){
    let plot_point_size : f32 = 5.0;
    let div_height = get_candle_div_height(chart);
    M_PAINT.lock().unwrap().save(&context);
    let candle_height = get_candle_div_height(chart);
    M_PAINT.lock().unwrap().set_clip(&context, chart.m_left_vscale_width, 20.0, chart.m_view.m_size.cx - chart.m_right_vscale_width, candle_height);
    for i in 0..chart.m_plots.len(){
		let mut plot = chart.m_plots[i].clone();
        let mut m_index1 : i32 = 0;
        let mut m_index2 : i32 = 0;
        let mut m_index3 : i32 = 0;
        let mut mpx1 : f32 = 0.0;
        let mut mpy1 : f32 = 0.0;
        let mut mpx2 : f32 = 0.0;
        let mut mpy2 : f32 = 0.0;
        let mut mpx3 : f32 = 0.0;
        let mut mpy3 : f32 = 0.0; 
        let mut rk : f32 = 0.0;
        let mut rb : f32 = 0.0;
        let mut n_high : f64 = 0.0;
        let mut n_low : f64 = 0.0;

       if(plot.m_plot_type == "LRLine" || plot.m_plot_type == "LRChannel" || plot.m_plot_type == "LRBand"){
            let mut list: Vec<f64> = Vec::new();
            m_index1 = get_chart_index_by_date(chart, plot.m_key1);
            m_index2 = get_chart_index_by_date(chart, plot.m_key2);
            let min_index = m_index1.min(m_index2);
            let max_index = m_index1.max(m_index2);
            for j in min_index..(max_index + 1){
                list.push(chart.m_data[j as usize].m_close);
            }
            linear_regression_equation(list, &mut rk, &mut rb);
            plot.m_value1 = rb as f64;
            plot.m_value2 = (rk * ((max_index - min_index + 1) as f32) + rb) as f64;
        }
        else if(plot.m_plot_type == "BoxLine" || plot.m_plot_type == "TironeLevels"){
            get_candle_range(chart, &mut plot, &mut n_high, &mut n_low);
            m_index1 = get_chart_index_by_date(chart, plot.m_key1);
            m_index2 = get_chart_index_by_date(chart, plot.m_key2);
            plot.m_key1 = get_chart_date_by_index(chart, m_index1.min(m_index2));
            plot.m_key2 = get_chart_date_by_index(chart, m_index1.max(m_index2));
            plot.m_value1 = n_high;
            plot.m_value2 = n_low;
        } 
        if(plot.m_key1 > 0.0){
            m_index1 = get_chart_index_by_date(chart, plot.m_key1);
            mpx1 = get_chart_x(chart, m_index1);
            mpy1 = get_chart_y(chart, 0, plot.m_value1);
            if (chart.m_splot.m_id == plot.m_id)
            {
                M_PAINT.lock().unwrap().fill_ellipse(&context, plot.m_point_color.clone(), mpx1 - plot_point_size, mpy1 - plot_point_size, mpx1 + plot_point_size, mpy1 + plot_point_size);
            }
        }
        if(plot.m_key2 > 0.0){
            m_index2 = get_chart_index_by_date(chart, plot.m_key2);
            mpx2 = get_chart_x(chart, m_index2);
            mpy2 = get_chart_y(chart, 0, plot.m_value2);
            if (chart.m_splot.m_id == plot.m_id)
            {
                M_PAINT.lock().unwrap().fill_ellipse(&context, plot.m_point_color.clone(), mpx2 - plot_point_size, mpy2 - plot_point_size, mpx2 + plot_point_size, mpy2 + plot_point_size);
            }
        }
        if(plot.m_key3 > 0.0){
            m_index3 = get_chart_index_by_date(chart, plot.m_key3);
            mpx3 = get_chart_x(chart, m_index3);
            mpy3 = get_chart_y(chart, 0, plot.m_value3);
            if (chart.m_splot.m_id == plot.m_id)
            {
                M_PAINT.lock().unwrap().fill_ellipse(&context, plot.m_point_color.clone(), mpx3 - plot_point_size, mpy3 - plot_point_size, mpx3 + plot_point_size, mpy3 + plot_point_size);
            }
        }
        if(plot.m_plot_type == "Line"){
            let mut k : f32 = 0.0;
            let mut b : f32 = 0.0;
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            if(mpx2 == mpx1){
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
        }
        else if(plot.m_plot_type == "AngleLine"){
            let mut k : f32 = 0.0;
            let mut b : f32 = 0.0;
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            if(mpx2 == mpx1){
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
            line_xy(mpx1, mpy1, mpx3, mpy3, 0.0, 0.0, &mut k, &mut b);
            if(mpx3 == mpx1){
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
        }
         else if(plot.m_plot_type == "Parallel"){
            let mut k : f32 = 0.0;
            let mut b : f32 = 0.0;
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            if(mpx2 == mpx1){
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
            let new_b = mpy3 - k * mpx3;
            if(mpx2 == mpx1){
               M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx3, 0.0, mpx3, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + new_b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + new_b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
        }
        else if(plot.m_plot_type == "Percent"){
            let list = get_percent_params(mpy1, mpy2);
            let mut texts: Vec<String> = Vec::new();
            texts.push("0%".to_string());
            texts.push("25%".to_string());
            texts.push("50%".to_string());
            texts.push("75%".to_string());
            texts.push("100%".to_string());
            for j in 0..list.len(){
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), chart.m_left_vscale_width, list[j as usize], chart.m_view.m_size.cx - chart.m_right_vscale_width, list[j as usize]);
                let t_size = M_PAINT.lock().unwrap().text_size(&context, texts[j as usize].clone(), chart.m_font.clone());
                M_PAINT.lock().unwrap().draw_text(&context, texts[j as usize].clone(), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width + 5.0, list[j as usize] - t_size.cy / 2.0 - 2.0);
            }
        }
        else if(plot.m_plot_type == "FiboTimezone"){
            let mut f_value : i32 = 1;
            let aindex = m_index1;
            let mut pos : i32 = 1;
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            let t_size = M_PAINT.lock().unwrap().text_size(&context, "1".to_string(), chart.m_font.clone());
            M_PAINT.lock().unwrap().draw_text(&context, "1".to_string(), chart.m_text_color.clone(), chart.m_font.clone(), mpx1, div_height - t_size.cy / 2.0);
            while (aindex + f_value <= chart.m_last_visible_index){
                f_value = fibonacci_value(pos);
                let new_index = aindex + f_value;
                let new_x = get_chart_x(chart, new_index);
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x, 0.0, new_x, div_height);
                let t_size = M_PAINT.lock().unwrap().text_size(&context, f_value.to_string(), chart.m_font.clone());
                M_PAINT.lock().unwrap().draw_text(&context, f_value.to_string(), chart.m_text_color.clone(), chart.m_font.clone(), new_x, div_height - t_size.cy / 2.0);
                pos = pos + 1;
            }
        }
        else if(plot.m_plot_type == "SpeedResist"){
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            if (mpx1 != mpx2 && mpy1 != mpy2){
                let first_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) / 3.0};
                let second_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) * 2.0 / 3.0};
                let start_p = FCPoint{x:mpx1,y:mpy1};
                let mut f_k : f32 = 0.0;
                let mut f_b : f32 = 0.0;
                let mut s_k : f32 = 0.0;
                let mut s_b : f32 = 0.0;
                line_xy(start_p.x, start_p.y, first_p.x, first_p.y, 0.0, 0.0, &mut f_k, &mut f_b);
                line_xy(start_p.x, start_p.y, second_p.x, second_p.y, 0.0, 0.0, &mut s_k, &mut s_b);
                let new_yf : f32 = 0.0;
                let new_ys : f32 = 0.0;
                let new_x : f32 = 0.0;
                let mut new_yf : f32 = 0.0;
                let mut new_ys : f32 = 0.0;
                let mut new_x : f32 = 0.0;
                if (mpx2 > mpx1){
                    new_yf = f_k * (chart.m_view.m_size.cx - chart.m_right_vscale_width) + f_b;
                    new_ys = s_k * (chart.m_view.m_size.cx - chart.m_right_vscale_width) + s_b;
                    new_x = (chart.m_view.m_size.cx - chart.m_right_vscale_width);
                }
                else{
                    new_yf = f_b;
                    new_ys = s_b;
                    new_x = chart.m_left_vscale_width;
                }
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), start_p.x, start_p.y, new_x, new_yf);
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), start_p.x, start_p.y, new_x, new_ys);
            }
        }
        else if(plot.m_plot_type == "LRLine"){
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
        }
        else if(plot.m_plot_type == "LRBand"){
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            let mut up_sub_value : f64 = 0.0;
            let mut down_sub_value : f64 = 0.0;
            get_lrband_range(chart, &mut plot, rk, rb, &mut up_sub_value, &mut down_sub_value);
            
            mpy1 = get_chart_y(chart, 0, plot.m_value1 + up_sub_value);
            mpy2 = get_chart_y(chart, 0, plot.m_value2 + up_sub_value);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            
            mpy1 = get_chart_y(chart, 0, plot.m_value1 - down_sub_value);
            mpy2 = get_chart_y(chart, 0, plot.m_value2 - down_sub_value);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
        }
        else if(plot.m_plot_type == "LRChannel"){
			let mut up_sub_value : f64 = 0.0;
            let mut down_sub_value : f64 = 0.0;
            get_lrband_range(chart, &mut plot, rk, rb, &mut up_sub_value, &mut down_sub_value);
            let mut k : f32 = 0.0;
            let mut b : f32 = 0.0;
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            let mut right_x = chart.m_view.m_size.cx - chart.m_right_vscale_width;
            let mut right_y = right_x * k + b;
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, right_x, right_y);
            mpy1 = get_chart_y(chart, 0, plot.m_value1 + up_sub_value);
            mpy2 = get_chart_y(chart, 0, plot.m_value2 + up_sub_value);
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            right_y = right_x * k + b;
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, right_x, right_y);
            mpy1 = get_chart_y(chart, 0, plot.m_value1 - down_sub_value);
            mpy2 = get_chart_y(chart, 0, plot.m_value2 - down_sub_value);
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            right_y = right_x * k + b;
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, right_x, right_y);
        }
        else if(plot.m_plot_type == "FiboFanline"){
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            if (mpx1 != mpx2 && mpy1 != mpy2){
                let first_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) * 0.382};
                let second_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) * 0.5};
                let third_p = FCPoint{x:mpx2, y:mpy2 - (mpy2 - mpy1) * 0.618};
                let start_p = FCPoint{x:mpx1, y:mpy1};
                let mut list_p: Vec<FCPoint> = Vec::new();
                list_p.push(first_p.clone());
				list_p.push(second_p.clone());
				list_p.push(third_p.clone());
				let list_size = list_p.len();
                for j in 0..list_size{
                    let mut k : f32 = 0.0;
                    let mut b : f32 = 0.0;
                    line_xy(start_p.x, start_p.y, list_p[j as usize].x, list_p[j as usize].y, 0.0, 0.0, &mut k, &mut b);
                    let mut new_x : f32 = 0.0;
                    let mut new_y : f32 = 0.0;
                    if (mpx2 > mpx1){
                        new_y = k * (chart.m_view.m_size.cx - chart.m_right_vscale_width) + b;
                        new_x = (chart.m_view.m_size.cx - chart.m_right_vscale_width);
                    }
                    else
                    {
                        new_y = b;
                        new_x = chart.m_left_vscale_width;
                    }
                    M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), start_p.x, start_p.y, new_x, new_y);
                }
            }
        }
        else if(plot.m_plot_type == "Segment"){
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
        }else if(plot.m_plot_type == "Ray"){
            let mut k : f32 = 0.0;
            let mut b : f32 = 0.0;
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            if (k != 0.0 || b != 0.0) {
                let left_x = chart.m_left_vscale_width;
                let left_y = left_x * k + b;
                let right_x = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let right_y = right_x * k + b;
                if (mpx1 >= mpx2) {
                    M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), left_x, left_y, mpx1, mpy1);
                } else {
                    M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, right_x, right_y);
                }
            }
            else {
                if (mpy1 >= mpy2) {
                    M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx1, 0.0);
                } else {
                    M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx1, div_height);
                }
            }
        }else if(plot.m_plot_type == "Triangle"){
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx2, mpy2, mpx3, mpy3);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx3, mpy3);
        }else if(plot.m_plot_type == "Rect"){
            let mut s_x1 : f32 = mpx1.min(mpx2);
            let mut s_y1 : f32 = mpy1.min(mpy2);
            let mut s_x2 : f32 = mpx1.max(mpx2);
            let mut s_y2 : f32 = mpy1.max(mpy2);
            M_PAINT.lock().unwrap().draw_rect(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), s_x1, s_y1, s_x2, s_y2);
        }else if(plot.m_plot_type == "Cycle"){
            let r = ((mpx2 - mpx1) * (mpx2 - mpx1) + (mpy2 - mpy1) * (mpy2 - mpy1)).abs().sqrt();
            M_PAINT.lock().unwrap().draw_ellipse(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1 - r, mpy1 - r, mpx1 + r, mpy1 + r);
        }else if(plot.m_plot_type == "CircumCycle"){
            let mut o_x : f32 = 0.0;
            let mut o_y : f32 = 0.0;
            let mut r : f32 = 0.0;
            ellipse_or(mpx1, mpy1, mpx2, mpy2, mpx3, mpy3, &mut o_x, &mut o_y, &mut r);
            M_PAINT.lock().unwrap().draw_ellipse(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), o_x - r, o_y - r, o_x + r, o_y + r);
        }else if(plot.m_plot_type == "Ellipse"){
           let mut x1 : f32 = 0.0;
            let mut y1 : f32 = 0.0;
            let mut x2 : f32 = 0.0;
            let mut y2 : f32 = 0.0;
            if(mpx1 <= mpx2){
                x1 = mpx2;
                y1 = mpy2;
                x2 = mpx1;
                y2 = mpy1;
            }else{
                x1 = mpx1;
                y1 = mpy1;
                x2 = mpx2;
                y2 = mpy2;
            }
            let x = x1 - (x1 - x2);
            let mut y : f32 = 0.0;
            let width = (x1 - x2) * 2.0;
            let mut height : f32 = 0.0;
            if (y1 >= y2){
                height = (y1 - y2) * 2.0;
            }
            else{
                height = (y2 - y1) * 2.0;
            }
            y = y2 - height / 2.0;
            M_PAINT.lock().unwrap().draw_ellipse(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), x, y, x + width, y + height);
        }else if(plot.m_plot_type == "ParalleGram"){
            let mut x4 : f32 = 0.0;
            let mut y4 : f32 = 0.0;
            parallelogram(mpx1, mpy1, mpx2, mpy2, mpx3, mpy3, &mut x4, &mut y4);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx2, mpy2, mpx3, mpy3);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx3, mpy3, x4, y4);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), x4, y4, mpx1, mpy1);
        }else if(plot.m_plot_type == "BoxLine"){
            let s_x1 : f32 = mpx1.min(mpx2);
            let s_y1 : f32 = mpy1.min(mpy2);
            let s_x2 : f32 = mpx1.max(mpx2);
            let s_y2 : f32 = mpy1.max(mpy2);
            M_PAINT.lock().unwrap().draw_rect(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), s_x1, s_y1, s_x2, s_y2);
            let str = "COUNT:".to_string() + &(((m_index2 - m_index1).abs() as i32) + 1).to_string();
            let b_size = M_PAINT.lock().unwrap().text_size(&context, str.clone(), chart.m_font.clone());
            M_PAINT.lock().unwrap().draw_text(&context, str, chart.m_text_color.clone(), chart.m_font.clone(), s_x1 + 2.0, s_y1 + 2.0 + b_size.cy / 2.0);
            let mut close_list: Vec<f64> = Vec::new();
            for j  in m_index1..(m_index2 + 1){
                close_list.push(chart.m_data[j as usize].m_close);
            }
            let avg_close = avg_value(close_list);
            let close_y = get_chart_y(chart, 0, avg_close);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), s_x1, close_y, s_x2, close_y);
            let draw_avg = "AVG:".to_string() + &to_fixed(avg_close, chart.m_candle_digit);
            let t_size = M_PAINT.lock().unwrap().text_size(&context, draw_avg.clone(), chart.m_font.clone());
            M_PAINT.lock().unwrap().draw_text(&context, draw_avg.clone(), chart.m_text_color.clone(), chart.m_font.clone(), s_x1 + 2.0, close_y - t_size.cy / 2.0 - 2.0);
        }
        else if(plot.m_plot_type == "TironeLevels"){
            let s_x1 : f32 = mpx1.min(mpx2);
            let s_y1 : f32 = mpy1.min(mpy2);
            let s_x2 : f32 = mpx1.max(mpx2);
            let s_y2 : f32 = mpy1.max(mpy2);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), s_x1, s_y1, s_x2, s_y1);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), s_x1, s_y2, s_x2, s_y2);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), s_x1 + (s_x2 - s_x1) / 2.0, s_y1, s_x1 + (s_x2 - s_x1) / 2.0, s_y2);
            let t1 = n_high;
            let t2 = n_high - (n_high - n_low) / 3.0;
            let t3 = n_high - (n_high - n_low) / 2.0;
            let t4 = n_high - 2.0 * (n_high - n_low) / 3.0;
            let t5 = n_low;
            let mut tlist: Vec<f64> = Vec::new();
            tlist.push(t2);
            tlist.push(t3);
            tlist.push(t4);
            for j in 0..tlist.len(){
                let y = get_chart_y(chart, 0, tlist[j as usize]);
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), chart.m_left_vscale_width, y, chart.m_view.m_size.cx - chart.m_right_vscale_width, y);
                let str = to_fixed(tlist[j], chart.m_candle_digit);
                let t_size = M_PAINT.lock().unwrap().text_size(&context, str.clone(), chart.m_font.clone());
                M_PAINT.lock().unwrap().draw_text(&context, str.clone(), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width + 2.0, y - t_size.cy / 2.0 - 2.0);
            }
        }
        else if(plot.m_plot_type == "GoldenRatio"){
            let s_x1 : f32 = mpx1.min(mpx2);
            let s_y1 : f32 = mpy1.min(mpy2);
            let s_x2 : f32 = mpx1.max(mpx2);
            let s_y2 : f32 = mpy1.max(mpy2);
            let mut ranges: Vec<f32> = Vec::new();
            ranges.push(0.0);
            ranges.push(0.236);
            ranges.push(0.382);
            ranges.push(0.5);
            ranges.push(0.618);
            ranges.push(0.809);
            ranges.push(1.0);
            ranges.push(1.382);
            ranges.push(1.618);
            ranges.push(2.0);
            ranges.push(2.382);
            ranges.push(2.618);
            let min_value = plot.m_value1.min(plot.m_value2);
            let max_value = plot.m_value1.max(plot.m_value2);
            for j in 0..ranges.len(){
                let mut new_y : f32 = 0.0;
                if(s_y1 <= s_y2){
                    new_y = s_y1 + (s_y2 - s_y1) * ranges[j as usize];
                }else{
                    new_y =  s_y2 + (s_y1 - s_y2) * (1.0 - ranges[j as usize]);
                }
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), chart.m_left_vscale_width, new_y, chart.m_view.m_size.cx - chart.m_right_vscale_width, new_y);
                let value = get_chart_value(chart, FCPoint{x:0.0, y:new_y});
                let str = to_fixed(value, chart.m_candle_digit);
                let t_size = M_PAINT.lock().unwrap().text_size(&context, str.clone(), chart.m_font.clone());
                M_PAINT.lock().unwrap().draw_text(&context, str.clone(), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width + 2.0, new_y - t_size.cy / 2.0 - 2.0);
            }
        }
        chart.m_plots[i] = plot.clone();
    }
    M_PAINT.lock().unwrap().restore(&context);
}

pub fn draw_chart_scale(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, chart:&mut FCChart, clip_rect:FCRect){
    if(chart.m_left_vscale_width > 0.0){
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, 0.0, chart.m_left_vscale_width, chart.m_view.m_size.cy - chart.m_hscale_height);
    }
    if(chart.m_right_vscale_width > 0.0){
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_view.m_size.cx - chart.m_right_vscale_width, 0.0, chart.m_view.m_size.cx - chart.m_right_vscale_width, chart.m_view.m_size.cy - chart.m_hscale_height);
    }
    if(chart.m_hscale_height > 0.0){
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), 0.0, chart.m_view.m_size.cy - chart.m_hscale_height, chart.m_view.m_size.cx, chart.m_view.m_size.cy - chart.m_hscale_height);
    }
    let candle_div_height = get_candle_div_height(chart);
    let vol_div_height = get_vol_div_height(chart);
    let ind_div_height = get_ind_div_height(chart);
    if(vol_div_height > 0.0){
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, candle_div_height, chart.m_view.m_size.cx - chart.m_right_vscale_width, candle_div_height);
    }
    if(ind_div_height > 0.0){
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, candle_div_height + vol_div_height, chart.m_view.m_size.cx - chart.m_right_vscale_width, candle_div_height + vol_div_height);
    }
    let data_len = chart.m_data.len() as i32;
    if(data_len > 0) {
		let mut grid_step : f64 = 0.0;
		let mut grid_digit : i32 = 0;
		chart_grid_scale(chart.m_candle_min, chart.m_candle_max,  (candle_div_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom) / 2.0, chart.m_vscale_distance, chart.m_vscale_distance / 2.0, ((candle_div_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom) / chart.m_vscale_distance) as i32, &mut grid_step, &mut grid_digit);
		if(grid_step > 0.0){
			let mut draw_values: Vec<f64> = Vec::new();
			let mut is_trend : bool = false;
			if(chart.m_cycle == "trend"){
				is_trend = true;
			}
			let mut first_open : f64 = 0.0;
			if(is_trend){
				first_open = chart.m_data[chart.m_first_visible_index as usize].m_close;
				let mut sub_value = (chart.m_candle_max - chart.m_candle_min);
				let count = ((candle_div_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom) / chart.m_vscale_distance) as i32;
				if(count > 0){
					sub_value = sub_value / (count as f64);
				}
				let mut start = first_open;
				while(start < chart.m_candle_max){
					start = start + sub_value;
					if(start <= chart.m_candle_max){
						draw_values.push(start);
					}
				}
				start = first_open;
				while(start > chart.m_candle_min){
					start -= sub_value;
					if(start >= chart.m_candle_min){
						draw_values.push(start);
					}
				}
			}else{
				let mut start : f64 = 0.0;
				if (chart.m_candle_min >= 0.0) {
					while (start + grid_step < chart.m_candle_min) {
						start = start + grid_step;
					}
				} else {
					while (start - grid_step > chart.m_candle_min) {
						start = start - grid_step;
					}
				}

				while (start <= chart.m_candle_max) {
					if(start > chart.m_candle_min){
						draw_values.push(start);
					}
					start = start + grid_step;
				}
			}
			draw_values.push(first_open);
			for i in 0..draw_values.len(){
				let start = draw_values[i as usize];
				let haxis_y = get_chart_y(chart, 0, start);
				M_PAINT.lock().unwrap().draw_line(&context, chart.m_grid_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, haxis_y, chart.m_view.m_size.cx - chart.m_right_vscale_width, haxis_y);
				M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width - 8.0, haxis_y, chart.m_left_vscale_width, haxis_y);
				M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_view.m_size.cx - chart.m_right_vscale_width, haxis_y, chart.m_view.m_size.cx - chart.m_right_vscale_width + 8.0, haxis_y);
				let t_size =  M_PAINT.lock().unwrap().text_size(&context, to_fixed(start, chart.m_candle_digit), chart.m_font.clone());
				if(is_trend){
					let diff_range = ((start - first_open) / first_open * 100.0);
					let diff_range_str = to_fixed(diff_range, chart.m_candle_digit);
					if(diff_range >= 0.0){
						M_PAINT.lock().unwrap().draw_text(&context, diff_range_str.clone(), chart.m_up_color.clone(), chart.m_font.clone(), chart.m_view.m_size.cx - chart.m_right_vscale_width + 10.0, haxis_y as f32 );
					}else{
						M_PAINT.lock().unwrap().draw_text(&context, diff_range_str.clone(), chart.m_down_color.clone(), chart.m_font.clone(), chart.m_view.m_size.cx - chart.m_right_vscale_width + 10.0, haxis_y as f32);
					}
				}else{
					M_PAINT.lock().unwrap().draw_text(&context, to_fixed(start, chart.m_candle_digit), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_view.m_size.cx - chart.m_right_vscale_width + 10.0, haxis_y as f32);
				}
				M_PAINT.lock().unwrap().draw_text(&context, to_fixed(start, chart.m_candle_digit), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width - t_size.cx - 10.0, haxis_y as f32);
			}
		}
		chart_grid_scale(chart.m_vol_min, chart.m_vol_max, (vol_div_height - chart.m_vol_padding_top - chart.m_vol_padding_bottom) / 2.0, chart.m_vscale_distance, chart.m_vscale_distance / 2.0, ((vol_div_height - chart.m_vol_padding_top - chart.m_vol_padding_bottom) / chart.m_vscale_distance) as i32, &mut grid_step, &mut grid_digit);
		if(grid_step > 0.0){
			let mut start : f64 = 0.0;
			if (chart.m_vol_min >= 0.0) {
				while (start + grid_step < chart.m_vol_min) {
					start = start + grid_step;
				}
			} else {
				while (start - grid_step > chart.m_vol_min) {
					start = start - grid_step;
				}
			}
			while (start <= chart.m_vol_max) {
				if(start > chart.m_vol_min){
					let haxis_y = get_chart_y(chart, 1, start);
					M_PAINT.lock().unwrap().draw_line(&context, chart.m_grid_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, haxis_y, chart.m_view.m_size.cx - chart.m_right_vscale_width, haxis_y);
					M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width - 8.0, haxis_y, chart.m_left_vscale_width, haxis_y);
					M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_view.m_size.cx - chart.m_right_vscale_width, haxis_y, chart.m_view.m_size.cx - chart.m_right_vscale_width + 8.0, haxis_y);
					let t_size = M_PAINT.lock().unwrap().text_size(&context, to_fixed(start, chart.m_vol_digit), chart.m_font.clone());
					M_PAINT.lock().unwrap().draw_text(&context, to_fixed(start, chart.m_vol_digit), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_view.m_size.cx - chart.m_right_vscale_width + 10.0, haxis_y as f32);
					M_PAINT.lock().unwrap().draw_text(&context, to_fixed(start, chart.m_vol_digit), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width - t_size.cx - 10.0, haxis_y as f32);
				}
				start = start + grid_step;
			}
		}
		if(ind_div_height > 0.0){
			chart_grid_scale(chart.m_ind_min, chart.m_ind_max, (ind_div_height - chart.m_ind_padding_top - chart.m_ind_padding_bottom) / 2.0, chart.m_vscale_distance, chart.m_vscale_distance / 2.0, ((ind_div_height - chart.m_ind_padding_top - chart.m_ind_padding_bottom) / chart.m_vscale_distance) as i32, &mut grid_step, &mut grid_digit);
			if(grid_step > 0.0){
				let mut start : f64 = 0.0;
				if (chart.m_ind_min >= 0.0) {
					while (start + grid_step < chart.m_ind_min) {
						start = start + grid_step;
					}
				} else {
					while (start - grid_step > chart.m_ind_min) {
						start = start - grid_step;
					}
				}
				while (start <= chart.m_ind_max) {
					if(start > chart.m_ind_min){
						let haxis_y = get_chart_y(chart, 2, start);
						M_PAINT.lock().unwrap().draw_line(&context, chart.m_grid_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, haxis_y, chart.m_view.m_size.cx - chart.m_right_vscale_width, haxis_y);
						M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width - 8.0, haxis_y, chart.m_left_vscale_width, haxis_y);
						M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_view.m_size.cx - chart.m_right_vscale_width, haxis_y, chart.m_view.m_size.cx - chart.m_right_vscale_width + 8.0, haxis_y);
						let t_size = M_PAINT.lock().unwrap().text_size(&context, to_fixed(start, chart.m_ind_digit), chart.m_font.clone());
						M_PAINT.lock().unwrap().draw_text(&context, to_fixed(start, chart.m_ind_digit), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_view.m_size.cx - chart.m_right_vscale_width + 10.0, haxis_y as f32);
						M_PAINT.lock().unwrap().draw_text(&context, to_fixed(start, chart.m_ind_digit), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width - t_size.cx - 10.0, haxis_y as f32);
					}
					start = start + grid_step;
				}
			}  
		}
		if(chart.m_hscale_height > 0.0){
			let mut d_left = chart.m_left_vscale_width + 10.0;
			for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
				let x_text = chart.m_data[i as usize].m_date.to_string();
				let t_size = M_PAINT.lock().unwrap().text_size(&context, x_text.clone(), chart.m_font.clone());
				let x = get_chart_x(chart, i);
				let dx = x - t_size.cx / 2.0;
				if(dx > d_left && dx < chart.m_view.m_size.cx - chart.m_right_vscale_width - 10.0){
					M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), x, chart.m_view.m_size.cy - chart.m_hscale_height, x, chart.m_view.m_size.cy - chart.m_hscale_height + 8.0);
					M_PAINT.lock().unwrap().draw_text(&context, x_text.clone(), chart.m_text_color.clone(), chart.m_font.clone(), dx, chart.m_view.m_size.cy - chart.m_hscale_height + 8.0 + t_size.cy / 2.0);
					d_left = x + t_size.cx;
				}
			}
		}
    }
}

pub fn draw_chart_cross_line(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, chart:&mut FCChart, clip_rect:FCRect){
	let data_len = chart.m_data.len() as i32;
    if(data_len > 0) {
		let candle_div_height = get_candle_div_height(chart);
		let vol_div_height = get_vol_div_height(chart);
		let ind_div_height = get_ind_div_height(chart);
		let mut cross_line_index = chart.m_cross_stop_index;
		if (cross_line_index == -1) {
			cross_line_index = chart.m_last_visible_index;
		}
		if(vol_div_height > 0.0){
			let voltxt = "VOL ".to_string() + &to_fixed(chart.m_data[cross_line_index as usize].m_volume, chart.m_vol_digit);
			let vol_size = M_PAINT.lock().unwrap().text_size(&context, voltxt.clone(), chart.m_font.clone());
			M_PAINT.lock().unwrap().draw_text(&context, voltxt.clone(), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width + 5.0, candle_div_height + 5.0 + vol_size.cy / 2.0);
		}
		let mut is_trend : bool = false;
		if(!is_trend){
			let mut draw_titles = Vec::new();
			let mut draw_colors = Vec::new();
			if (chart.m_main_indicator == "MA") {
				draw_titles.push("MA5 ".to_string() + &to_fixed(chart.m_ma5[cross_line_index as usize], chart.m_candle_digit));
				draw_titles.push("MA10 ".to_string() + &to_fixed(chart.m_ma10[cross_line_index as usize], chart.m_candle_digit));
				draw_titles.push("MA20 ".to_string() + &to_fixed(chart.m_ma20[cross_line_index as usize], chart.m_candle_digit));
				draw_titles.push("MA30 ".to_string() + &to_fixed(chart.m_ma30[cross_line_index as usize], chart.m_candle_digit));
				draw_titles.push("MA120 ".to_string() + &to_fixed(chart.m_ma120[cross_line_index as usize], chart.m_candle_digit));
				draw_titles.push("MA250 ".to_string() + &to_fixed(chart.m_ma250[cross_line_index as usize], chart.m_candle_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[2].clone());
				draw_colors.push(chart.m_indicator_colors[5].clone());
				draw_colors.push(chart.m_indicator_colors[4].clone());
				draw_colors.push(chart.m_indicator_colors[3].clone());
			} else if (chart.m_main_indicator == "BOLL") {
				draw_titles.push("MID ".to_string() + &to_fixed(chart.m_boll_mid[cross_line_index as usize], chart.m_candle_digit));
				draw_titles.push("UP ".to_string() + &to_fixed(chart.m_boll_up[cross_line_index as usize], chart.m_candle_digit));
				draw_titles.push("LOW ".to_string() + &to_fixed(chart.m_boll_down[cross_line_index as usize], chart.m_candle_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[2].clone());
			}
			let mut i_left = chart.m_left_vscale_width + 5.0;
			for i in 0..draw_titles.len(){
				let t_size = M_PAINT.lock().unwrap().text_size(&context, draw_titles[i].clone(), chart.m_font.clone());
				M_PAINT.lock().unwrap().draw_text(&context, draw_titles[i].clone(), draw_colors[i].clone(), chart.m_font.clone(), i_left, 5.0 + t_size.cy / 2.0);
				i_left = i_left + t_size.cx + 5.0;
			}
		}
		if(ind_div_height > 0.0){
			let mut draw_titles = Vec::new();
			let mut draw_colors = Vec::new();
			if (chart.m_show_indicator == "MACD") {
				draw_titles.push("DIF ".to_string() + &to_fixed(chart.m_alldifarr[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("DEA ".to_string() + &to_fixed(chart.m_alldeaarr[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("MACD ".to_string() + &to_fixed(chart.m_allmacdarr[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[4].clone());
			}else if (chart.m_show_indicator == "KDJ") {
				draw_titles.push("K ".to_string() + &to_fixed(chart.m_kdj_k[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("D ".to_string() + &to_fixed(chart.m_kdj_d[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("J ".to_string() + &to_fixed(chart.m_kdj_j[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[2].clone());
			}else if (chart.m_show_indicator == "RSI") {
				draw_titles.push("RSI6 ".to_string() + &to_fixed(chart.m_rsi1[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("RSI12 ".to_string() + &to_fixed(chart.m_rsi2[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("RSI24 ".to_string() + &to_fixed(chart.m_rsi3[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[5].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[2].clone());
			}
			else if (chart.m_show_indicator == "BIAS") {
				draw_titles.push("BIAS6 ".to_string() + &to_fixed(chart.m_bias1[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("BIAS12 ".to_string() + &to_fixed(chart.m_bias2[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("BIAS24 ".to_string() + &to_fixed(chart.m_bias3[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[5].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[2].clone());
			}
			else if (chart.m_show_indicator == "ROC") {
				draw_titles.push("ROC ".to_string() + &to_fixed(chart.m_roc[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("ROCMA ".to_string() + &to_fixed(chart.m_roc_ma[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
			}else if (chart.m_show_indicator == "WR") {
				draw_titles.push("WR5 ".to_string() + &to_fixed(chart.m_wr1[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("WR10 ".to_string() + &to_fixed(chart.m_wr2[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
			}else if (chart.m_show_indicator == "CCI") {
				draw_titles.push("CCI ".to_string() + &to_fixed(chart.m_cci[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
			}else if (chart.m_show_indicator == "BBI") {
				draw_titles.push("BBI ".to_string() + &to_fixed(chart.m_bbi[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
			}else if (chart.m_show_indicator == "TRIX") {
				draw_titles.push("TRIX ".to_string() + &to_fixed(chart.m_trix[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("TRIXMA ".to_string() + &to_fixed(chart.m_trix_ma[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
			}else if (chart.m_show_indicator == "DMA") {
				draw_titles.push("MA10 ".to_string() + &to_fixed(chart.m_dma1[cross_line_index as usize], chart.m_ind_digit));
				draw_titles.push("MA50 ".to_string() + &to_fixed(chart.m_dma2[cross_line_index as usize], chart.m_ind_digit));
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
			}
			let mut i_left = chart.m_left_vscale_width + 5.0;
			for i in 0..draw_titles.len(){
				let t_size = M_PAINT.lock().unwrap().text_size(&context, draw_titles[i].clone(), chart.m_font.clone());
				M_PAINT.lock().unwrap().draw_text(&context, draw_titles[i].clone(), draw_colors[i].clone(), chart.m_font.clone(), i_left, candle_div_height + vol_div_height + 5.0 + t_size.cy / 2.0);
				i_left = i_left + t_size.cx + 5.0;
			}
		}
		if(chart.m_show_cross_line){
			let mut right_text = String::from("");
			if(chart.m_mouse_position.y < candle_div_height) {
				right_text = to_fixed(get_chart_value(chart, chart.m_mouse_position.clone()), chart.m_candle_digit);
			}
			else if(chart.m_mouse_position.y > candle_div_height && chart.m_mouse_position.y < candle_div_height + vol_div_height) {
				right_text = to_fixed(get_chart_value(chart, chart.m_mouse_position.clone()), chart.m_vol_digit);
			}else if(chart.m_mouse_position.y > candle_div_height + vol_div_height && chart.m_mouse_position.y < candle_div_height + vol_div_height + ind_div_height){
				right_text = to_fixed(get_chart_value(chart, chart.m_mouse_position.clone()), chart.m_ind_digit);
			}

			let mut draw_y = chart.m_mouse_position.y;
			if(draw_y > chart.m_view.m_size.cy - chart.m_hscale_height){
				draw_y = chart.m_view.m_size.cy - chart.m_hscale_height;
			}
			let t_size = M_PAINT.lock().unwrap().text_size(&context, right_text.clone(), chart.m_font.clone());
			if(chart.m_left_vscale_width > 0.0){
				M_PAINT.lock().unwrap().fill_rect(&context, chart.m_cross_tip_color.clone(), chart.m_left_vscale_width - t_size.cx, draw_y - t_size.cy / 2.0 - 4.0, chart.m_left_vscale_width, draw_y + t_size.cy / 2.0 + 3.0);
				M_PAINT.lock().unwrap().draw_text(&context, right_text.clone(), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width - t_size.cx, draw_y);
			}
			if(chart.m_right_vscale_width > 0.0){
				M_PAINT.lock().unwrap().fill_rect(&context, chart.m_cross_tip_color.clone(), chart.m_view.m_size.cx - chart.m_right_vscale_width, draw_y - t_size.cy / 2.0 - 4.0, chart.m_view.m_size.cx - chart.m_right_vscale_width + t_size.cx, draw_y + t_size.cy / 2.0 + 3.0);
				M_PAINT.lock().unwrap().draw_text(&context, right_text.clone(), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_view.m_size.cx - chart.m_right_vscale_width, draw_y);
			}
			let mut draw_x = chart.m_mouse_position.x;
			if(draw_x < chart.m_left_vscale_width){
				draw_x = chart.m_left_vscale_width;
			}
			if(draw_x > chart.m_view.m_size.cx - chart.m_right_vscale_width){
				draw_x = chart.m_view.m_size.cx - chart.m_right_vscale_width;
			}
			M_PAINT.lock().unwrap().draw_line(&context, chart.m_cross_line_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, draw_y, chart.m_view.m_size.cx - chart.m_right_vscale_width, draw_y);
			M_PAINT.lock().unwrap().draw_line(&context, chart.m_cross_line_color.clone(), chart.m_line_width, Vec::new(), draw_x, 0.0, draw_x, chart.m_view.m_size.cy - chart.m_hscale_height);
	        
			if (chart.m_cross_stop_index != -1){
				let x_text = chart.m_data[chart.m_cross_stop_index as usize].m_date.to_string();
				let x_size = M_PAINT.lock().unwrap().text_size(&context, x_text.clone(), chart.m_font.clone());
				M_PAINT.lock().unwrap().fill_rect(&context, chart.m_cross_tip_color.clone(), draw_x - x_size.cx / 2.0 - 2.0, candle_div_height + vol_div_height + ind_div_height, draw_x + x_size.cx / 2.0 + 2.0, candle_div_height + vol_div_height + ind_div_height + x_size.cy + 6.0);
				M_PAINT.lock().unwrap().draw_text(&context, x_text.clone(), chart.m_text_color.clone(), chart.m_font.clone(), draw_x - x_size.cx / 2.0, candle_div_height + vol_div_height + ind_div_height + 3.0 + x_size.cy / 2.0);
			}
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

pub fn draw_div_border(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, div:&mut FCView, clip_rect:FCRect){
	if(div.m_border_color != "none"){
        M_PAINT.lock().unwrap().draw_rect(&context, div.m_border_color.clone(), 1.0, Vec::new(), 0.0, 0.0, div.m_size.cx, div.m_size.cy);
    }
}

pub fn draw_div(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, div:&mut FCView, clip_rect:FCRect){
	if(div.m_back_color != "none"){
        M_PAINT.lock().unwrap().fill_rect(&context, div.m_back_color.clone(), 0.0, 0.0, div.m_size.cx, div.m_size.cy);
    }
}

pub fn get_div_content_width(div:&mut FCView)->f32{
	let mut c_width : f32 = 0.0;
	let sub_views = get_sub_views(div.clone());
	if(sub_views.len() > 0){
		for i in 0..sub_views.len(){
			let sub_view = &sub_views[i];
			if (sub_view.m_visible) {
			    if(c_width < sub_view.m_location.x + sub_view.m_size.cx){
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
	if(sub_views.len() > 0){
		for i in 0..sub_views.len(){
			let sub_view = &sub_views[i];
			if (sub_view.m_visible) {
			    if(c_height < sub_view.m_location.y + sub_view.m_size.cy){
			        c_height = sub_view.m_location.y + sub_view.m_size.cy;
			    }
		    }
		}
	}
	return c_height;
}

pub fn mouse_wheel_div(div:&mut FCView, delta:i32){
	let mut old_scroll_v = div.m_scroll_v;
    if (delta > 0) {
	    old_scroll_v = old_scroll_v - 10.0;
    } else if (delta < 0) {
	    old_scroll_v = old_scroll_v + 10.0;
    }
    let content_height = get_div_content_height(div);
    if (content_height < div.m_size.cy) {
        div.m_scroll_v = 0.0;
    } else {
        if (old_scroll_v < 0.0) {
	        old_scroll_v = 0.0;
	    } else if (old_scroll_v > content_height - div.m_size.cy) {
		    old_scroll_v = content_height - div.m_size.cy;
        }
        div.m_scroll_v = old_scroll_v;
    }
}

pub fn draw_div_scroll_bar(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, div:&mut FCView, clip_rect:FCRect){
	if (div.m_show_hscrollbar) {
		let content_width = get_div_content_width(div);
		if (content_width > div.m_size.cx) {
			let s_left = div.m_scroll_h / content_width * div.m_size.cx;
			let mut s_right = (div.m_scroll_h + div.m_size.cx) / content_width * div.m_size.cx;
			if (s_right - s_left < div.m_scroll_size) {
				s_right = s_left + div.m_scroll_size;
			}
			M_PAINT.lock().unwrap().fill_rect(&context, div.m_scroll_barcolor.clone(), s_left, div.m_size.cy - div.m_scroll_size, s_right, div.m_size.cy);
		}
	}
	if(div.m_show_vscrollbar){
	    let content_height = get_div_content_height(div);		
		if (content_height > div.m_size.cy) {
			let s_top = div.m_scroll_v / content_height * div.m_size.cy;
			let mut s_bottom  = s_top + (div.m_size.cy / content_height * div.m_size.cy);
			if (s_bottom  - s_top < div.m_scroll_size) {
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
	if (first_touch) {
		let mp = first_point.clone();
		if (div.m_show_hscrollbar || div.m_show_vscrollbar) {
			if (div.m_down_scroll_hbutton) {
				let content_width = get_div_content_width(div);
				let sub_x = (mp.x - div.m_start_point.x) / div.m_size.cx * content_width;
				let mut new_scrollh = div.m_start_scroll_h + sub_x;
				if (new_scrollh < 0.0) {
					new_scrollh = 0.0;
				} else if (new_scrollh > content_width - div.m_size.cx) {
					new_scrollh = content_width - div.m_size.cx;
				}
				div.m_scroll_h = new_scrollh;
				unsafe{
					M_CANCEL_CLICK = true;
				}
				return;

			} else if (div.m_down_scroll_vbutton) {
				let content_height = get_div_content_height(div);
				let sub_y = (mp.y - div.m_start_point.y) / div.m_size.cy * content_height;
				let mut new_scroll_v = div.m_start_scroll_v + sub_y;
				if (new_scroll_v < 0.0) {
					new_scroll_v = 0.0;
				} else if (new_scroll_v > content_height - div.m_size.cy) {
					new_scroll_v = content_height - div.m_size.cy;
				}
				div.m_scroll_v = new_scroll_v;
				unsafe{
					M_CANCEL_CLICK = true;
				}
				return;
			}
		}
		if (div.m_allow_drag_scroll) {
			let content_width = get_div_content_width(div);
			if (content_width > div.m_size.cx) {
				let sub_x = div.m_start_point.x - mp.x;
				let mut new_scrollh = div.m_start_scroll_h + sub_x;
				if (new_scrollh < 0.0) {
					new_scrollh = 0.0;
				} else if (new_scrollh > content_width - div.m_size.cx) {
					new_scrollh = content_width - div.m_size.cx;
				}
				div.m_scroll_h = new_scrollh;
				if(sub_x.abs() > 5.0){
					unsafe{
						M_CANCEL_CLICK = true;
				    }
				}
			}
			let content_height = get_div_content_height(div);
			if (content_height > div.m_size.cy) {
				let sub_y = div.m_start_point.y - mp.y;
				let mut new_scroll_v = div.m_start_scroll_v + sub_y;
				if (new_scroll_v < 0.0) {
					new_scroll_v = 0.0;
				} else if (new_scroll_v > content_height - div.m_size.cy) {
					new_scroll_v = content_height - div.m_size.cy;
				}
				div.m_scroll_v = new_scroll_v;
				if(sub_y.abs() > 5.0){
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
	if (div.m_show_hscrollbar) {
		let content_width = get_div_content_width(div);
		if (content_width > div.m_size.cx) {
		    let s_left = div.m_scroll_h / content_width * div.m_size.cx;
			let mut s_right = (div.m_scroll_h + div.m_size.cx) / content_width * div.m_size.cx;
			if (s_right - s_left < div.m_scroll_size) {
				s_right = s_left + div.m_scroll_size;
			}
			if (mp.x >= s_left && mp.x <= s_right && mp.y >= div.m_size.cy - div.m_scroll_size && mp.y <= div.m_size.cy) {
				div.m_down_scroll_hbutton = true;
				div.m_start_scroll_h = div.m_scroll_h;
				return;
			}
		}
	}
	if (div.m_show_vscrollbar) {
	    let content_height = get_div_content_height(div);
		if (content_height > div.m_size.cy) {
			let s_top = div.m_scroll_v / content_height * div.m_size.cy;
			let mut s_bottom  = (div.m_scroll_v + div.m_size.cy) / content_height * div.m_size.cy;
			if (s_bottom  - s_top < div.m_scroll_size) {
				s_bottom  = s_top + div.m_scroll_size;
			}
			if (mp.x >= div.m_size.cx - div.m_scroll_size && mp.x <= div.m_size.cx && mp.y >= s_top && mp.y <= s_bottom ) {
				div.m_down_scroll_vbutton = true;
				div.m_start_scroll_v = div.m_scroll_v;
				return;
			}
		}
	}
	if (div.m_allow_drag_scroll) {
		div.m_start_scroll_h = div.m_scroll_h;
		div.m_start_scroll_v = div.m_scroll_v;
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

pub fn reset_split_layout_div(split:&mut FCSplitLayoutDiv)->bool{
	let mut reset : bool = false;
    let mut split_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
    let width = split.m_view.m_size.cx;
    let height = split.m_view.m_size.cy;
    let mut f_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
    let mut s_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
    let mut splitter_size = FCSize{cx:0.0, cy:0.0};
    if(split.m_splitter.m_visible){
        splitter_size.cx = split.m_splitter.m_size.cx;
        splitter_size.cy = split.m_splitter.m_size.cy;
    }
    let layout_style = split.m_layout_style.clone();
    if(layout_style == "bottomtotop"){
        if (split.m_split_mode == "absolutesize" || split.m_old_size.cy == 0.0){
            split_rect.left = 0.0;
            split_rect.top = height - (split.m_old_size.cy - split.m_splitter.m_location.y);
            split_rect.right = width;
            split_rect.bottom = split_rect.top + splitter_size.cy;
        }
        else if (split.m_split_mode == "percentsize"){
            split_rect.left = 0.0;
            if(split.m_split_percent == -1.0){
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
    else if(layout_style == "lefttoright"){
        if (split.m_split_mode == "absolutesize" || split.m_old_size.cx == 0.0){
            split_rect.left = split.m_splitter.m_location.x;
            split_rect.top = 0.0;
            split_rect.right = split_rect.left + splitter_size.cx;
            split_rect.bottom = height;
        }
        else if (split.m_split_mode == "percentsize"){
            if(split.m_split_percent == -1.0){
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
    else if(layout_style == "righttoleft"){
        if (split.m_split_mode == "absolutesize" || split.m_old_size.cx == 0.0){
            split_rect.left = width - (split.m_old_size.cx - split.m_splitter.m_location.x);
            split_rect.top = 0.0;
            split_rect.right = split_rect.left + splitter_size.cx;
            split_rect.bottom = height;
        }
        else if (split.m_split_mode == "percentsize"){
            if(split.m_split_percent == -1.0){
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
    else if(layout_style == "toptobottom"){
        if (split.m_split_mode == "absolutesize" || split.m_old_size.cy == 0.0){
            split_rect.left = 0.0;
            split_rect.top = split.m_splitter.m_location.y;
            split_rect.right = width;
            split_rect.bottom = split_rect.top + splitter_size.cy;
        }
        else if (split.m_split_mode == "percentsize"){
            split_rect.left = 0.0;
            if(split.m_split_percent == -1.0){
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
    if(split.m_splitter.m_visible){
        let sp_rect = FCRect{left:split.m_splitter.m_location.x,  top:split.m_splitter.m_location.y, right:split.m_splitter.m_location.x + split.m_splitter.m_size.cx, bottom:split.m_splitter.m_location.y + split.m_splitter.m_size.cy};
        if (sp_rect.left != split_rect.left || sp_rect.top != split_rect.top || sp_rect.right != split_rect.right || sp_rect.bottom != split_rect.bottom){
            split.m_splitter.m_location = FCPoint{x:split_rect.left, y:split_rect.top};
            split.m_splitter.m_size = FCSize{cx:split_rect.right - split_rect.left, cy:split_rect.bottom - split_rect.top};
            M_VIEW_MAP.lock().unwrap().insert(split.m_splitter.m_id, split.m_splitter.clone());
            reset = true;
        }
    }
    let fc_rect = FCRect{left:split.m_first_view.m_location.x,  top:split.m_first_view.m_location.y, right:split.m_first_view.m_location.x + split.m_first_view.m_size.cx, bottom:split.m_first_view.m_location.y + split.m_first_view.m_size.cy};
    if (fc_rect.left != f_rect.left || fc_rect.top != f_rect.top || fc_rect.right != f_rect.right || fc_rect.bottom != f_rect.bottom){
        reset = true;
        split.m_first_view.m_location = FCPoint{x:f_rect.left, y:f_rect.top};
        split.m_first_view.m_size = FCSize{cx:f_rect.right - f_rect.left, cy:f_rect.bottom - f_rect.top};
        M_VIEW_MAP.lock().unwrap().insert(split.m_first_view.m_id, split.m_first_view.clone());
    }
    let sc_rect = FCRect{left:split.m_second_view.m_location.x,  top:split.m_second_view.m_location.y, right:split.m_second_view.m_location.x + split.m_second_view.m_size.cx, bottom:split.m_second_view.m_location.y + split.m_second_view.m_size.cy};
    if (sc_rect.left != s_rect.left || sc_rect.top != s_rect.top || sc_rect.right != s_rect.right || sc_rect.bottom != s_rect.bottom){
        reset = true;
        split.m_second_view.m_location = FCPoint{x:s_rect.left, y:s_rect.top};
        split.m_second_view.m_size = FCSize{cx:s_rect.right - s_rect.left, cy:s_rect.bottom - s_rect.top};
        M_VIEW_MAP.lock().unwrap().insert(split.m_second_view.m_id, split.m_second_view.clone());
    }
    split.m_old_size = FCSize{cx:width, cy:height};
    return reset;
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
        if(view.m_visible){
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
            if(layout.m_layout_style == "bottomtotop"){
                if (i == 0){
                    top = height - padding.top;
                }
                let mut l_width:f32 = 0.0;
                if (layout.m_auto_wrap){
                    l_width = size.cx;
                    let l_top = top - margin.top - c_height - margin.bottom;
                    if (l_top < padding.top){
                        if(v_pos != 0){
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
            }else if(layout.m_layout_style == "lefttoright"){
                let mut l_height:f32 = 0.0;
                if (layout.m_auto_wrap){
                    l_height = size.cy;
                    let l_right = left + margin.left + c_width + margin.right;
                    if (l_right > width){
                        left = padding.left;
                        if(v_pos != 0){
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
            }else if(layout.m_layout_style == "righttoleft"){
                if (i == 0){
                    left = width - padding.left;
                }
                let mut l_height:f32 = 0.0;
                if (layout.m_auto_wrap){
                    l_height = size.cy;
                    let l_left = left - margin.left - c_width - margin.right;
                    if (l_left < padding.left){
                        left = width - padding.left;
                        if(v_pos != 0){
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
            }else if(layout.m_layout_style == "toptobottom"){
                let mut l_width:f32 = 0.0;
                if (layout.m_auto_wrap){
                    l_width = size.cx;
                    let l_bottom = top + margin.top + c_height + margin.bottom;
                    if (l_bottom > height){
                        if(v_pos != 0){
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
            if (c_left != n_left || c_top != n_top || c_width != n_width || c_height != n_height){
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

pub fn get_grid_content_width(grid:&mut FCGrid)->f32{
	let mut c_width:f32 = 0.0;
	for i in 0..grid.m_columns.len(){
		let grid_column = (&grid.m_columns[i]).clone();
		if (grid_column.m_visible) {
			c_width = c_width + grid_column.m_width;
		}
	}
	return c_width;
}


pub fn get_grid_content_height(grid:&mut FCGrid)->f32{
	let mut c_height:f32 = 0.0;
	for i in 0..grid.m_rows.len(){
		let grid_row = (&grid.m_rows[i]).clone();
		if (grid_row.m_visible) {
			c_height = c_height + grid.m_row_height;
		}
	}
	return c_height;
}

pub fn draw_grid_scroll_bar(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, grid:&mut FCGrid, clip_rect:FCRect){
	if (grid.m_view.m_show_hscrollbar) {
		let content_width = get_grid_content_width(grid);
		if (content_width > grid.m_view.m_size.cx) {
			let s_left = grid.m_view.m_scroll_h / content_width * grid.m_view.m_size.cx;
			let mut s_right = (grid.m_view.m_scroll_h + grid.m_view.m_size.cx) / content_width * grid.m_view.m_size.cx;
			if (s_right - s_left < grid.m_view.m_scroll_size) {
				s_right = s_left + grid.m_view.m_scroll_size;
			}
			M_PAINT.lock().unwrap().fill_rect(&context, grid.m_view.m_scroll_barcolor.clone(), s_left, grid.m_view.m_size.cy - grid.m_view.m_scroll_size, s_right, grid.m_view.m_size.cy);
		}
	}
	if(grid.m_view.m_show_vscrollbar){
	    let content_height = get_grid_content_height(grid);
		if (content_height > grid.m_view.m_size.cy) {
			let s_top = grid.m_header_height + grid.m_view.m_scroll_v / content_height * (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size);
			let mut s_bottom  = s_top + ((grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size)) / content_height * (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size);
			if (s_bottom  - s_top < grid.m_view.m_scroll_size) {
				s_bottom  = s_top + grid.m_view.m_scroll_size;
			}
			M_PAINT.lock().unwrap().fill_rect(&context, grid.m_view.m_scroll_barcolor.clone(), grid.m_view.m_size.cx - grid.m_view.m_scroll_size, s_top, grid.m_view.m_size.cx, s_bottom );
		}
	}
}

pub fn draw_grid_cell(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, grid:&mut FCGrid, row:FCGridRow, column:FCGridColumn, cell:FCGridCell, left:f32, top:f32, right:f32, bottom:f32){
	if (cell.m_back_color != "none") {
		M_PAINT.lock().unwrap().fill_rect(&context, cell.m_back_color.clone(), left, top, right, bottom);
	}
	if(row.m_selected){
		if(grid.m_selected_row_color != "none"){
			M_PAINT.lock().unwrap().fill_rect(&context, grid.m_selected_row_color.clone(), left, top, right, bottom);
		}
	}
	if (cell.m_border_color != "none") {
		M_PAINT.lock().unwrap().draw_rect(&context, cell.m_border_color.clone(), 1.0, Vec::new(), left, top, right, bottom);
	}
	if (cell.m_value.len() > 0) {
		let t_size = M_PAINT.lock().unwrap().text_size(&context, cell.m_value.clone(), cell.m_font.clone());
		if (t_size.cx > column.m_width) {
			M_PAINT.lock().unwrap().draw_text_auto_ellipsis(&context, cell.m_value.clone(), cell.m_text_color.clone(), cell.m_font.clone(), left + 2.0, top + grid.m_row_height / 2.0, left + 2.0 + column.m_width, top + grid.m_row_height / 2.0);
		} else {
			M_PAINT.lock().unwrap().draw_text(&context, cell.m_value.clone(), cell.m_text_color.clone(), cell.m_font.clone(), left + 2.0, top + grid.m_row_height / 2.0);
		}
	}
}

pub fn draw_grid_column(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, grid:&mut FCGrid, column:FCGridColumn, left:f32, top:f32, right:f32, bottom:f32){
	let t_size = M_PAINT.lock().unwrap().text_size(&context, column.m_text.clone(), column.m_font.clone());
	if (column.m_back_color != "none") {
		M_PAINT.lock().unwrap().fill_rect(&context, column.m_back_color.clone(), left, top, right, bottom);
	}
	if (column.m_border_color != "none") {
		M_PAINT.lock().unwrap().draw_rect(&context, column.m_border_color.clone(), 1.0, Vec::new(), left, top, right, bottom);
	}
	M_PAINT.lock().unwrap().draw_text(&context, column.m_text.clone(), column.m_text_color.clone(), column.m_font.clone(), left + (column.m_width - t_size.cx) / 2.0, top + grid.m_header_height / 2.0);
}

pub fn draw_grid(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, grid:&mut FCGrid, clip_rect:FCRect){
	let mut c_left = -grid.m_view.m_scroll_h;
	let mut c_top = -grid.m_view.m_scroll_v + grid.m_header_height;
	let mut col_left = 0.0;
	for i in 0..grid.m_columns.len(){
		let mut grid_column = (&grid.m_columns[i]).clone();
		let col_rect= FCRect{left:col_left, top:0.0, right:col_left + grid_column.m_width, bottom:grid.m_header_height};
		grid_column.m_bounds = col_rect;
		grid_column.m_index = i as i32;
		col_left = col_left + grid_column.m_width;
		grid.m_columns[i] = grid_column;
	}
	for i in 0..grid.m_rows.len(){
		let row = (&grid.m_rows[i]).clone();
		if (row.m_visible) {
			let r_top = c_top;
			let r_bottom = c_top + grid.m_row_height;
			if (r_bottom >= 0.0 && c_top <= grid.m_view.m_size.cy) {
				for j in 0..row.m_cells.len(){
					let cell = (&row.m_cells[j]).clone();
					let grid_column = (&grid.m_columns[j]).clone();
					if (grid_column.m_visible) {
						if (!grid_column.m_frozen) {
							let mut cell_width = grid_column.m_width;
							let col_span = cell.m_col_span;
							if (col_span > 1) {
								for n in 1..col_span{
									let span_column = (&grid.m_columns[(grid_column.m_index + n) as usize]).clone();
									if (span_column.m_visible) {
										cell_width = cell_width + span_column.m_width;
									}
								}
							}
							let mut cell_height = grid.m_row_height;
							let row_span = cell.m_row_span;
							if (row_span > 1) {
								for n in 1..row_span{
									let span_row = (&grid.m_rows[i + n as usize]).clone();
									if (span_row.m_visible) {
										cell_height = cell_height + grid.m_row_height;
									}
								}
							}
							let c_rect = FCRect{left:grid_column.m_bounds.left - grid.m_view.m_scroll_h, top:r_top, right:grid_column.m_bounds.left + cell_width - grid.m_view.m_scroll_h, bottom:r_top + cell_height};
							if (c_rect.right >= 0.0 && c_rect.left < grid.m_view.m_size.cx) {
							    draw_grid_cell(&context, grid, row.clone(), grid_column.clone(), cell.clone(), c_rect.left, c_rect.top, c_rect.right, c_rect.bottom);
							}
						}
					}
				}
			}
			if (r_bottom >= 0.0 && c_top <= grid.m_view.m_size.cy) {
				for j in 0..row.m_cells.len(){
					let cell = (&row.m_cells[j]).clone();
					let grid_column = (&grid.m_columns[j]).clone();
					if (grid_column.m_visible) {
						if (grid_column.m_frozen) {
							let mut cell_width = grid_column.m_width;
							let col_span = cell.m_col_span;
							if (col_span > 1) {
								for n in 1..col_span{
									let span_column = (&grid.m_columns[(grid_column.m_index + n) as usize]).clone();
									if (span_column.m_visible) {
										cell_width = cell_width + span_column.m_width;
									}
								}
							}
							let mut cell_height = grid.m_row_height;
							let row_span = cell.m_row_span;
							if (row_span > 1) {
								for n in 1..row_span{
									let span_row = (&grid.m_rows[i + n as usize]).clone();
									if (span_row.m_visible) {
										cell_height = cell_height + grid.m_row_height;
									}
								}
							}
							let c_rect = FCRect{left:grid_column.m_bounds.left, top:r_top, right:grid_column.m_bounds.left + cell_width, bottom:r_top + cell_height};
							if (c_rect.right >= 0.0 && c_rect.left < grid.m_view.m_size.cx) {
							    draw_grid_cell(&context, grid, row.clone(), grid_column.clone(), cell.clone(), c_rect.left, c_rect.top, c_rect.right, c_rect.bottom);
							}
						}
					}
				}
			}
			if (c_top > grid.m_view.m_size.cy) {
				break;
            }
			c_top += grid.m_row_height;
		}
	}
	if (grid.m_header_height > 0.0) {
		for i in 0..grid.m_columns.len(){
			let grid_column = (&grid.m_columns[i]).clone();
			if (grid_column.m_visible) {
				if (!grid_column.m_frozen) {
				    draw_grid_column(&context, grid, grid_column.clone(), c_left, 0.0, c_left + grid_column.m_width, grid.m_header_height);
				}
				c_left = c_left + grid_column.m_width;
			}
		}
		c_left = 0.0;
		for i in 0..grid.m_columns.len(){
			let grid_column = (&grid.m_columns[i]).clone();
			if (grid_column.m_visible) {
				if (grid_column.m_frozen) {
				    draw_grid_column(&context, grid, grid_column.clone(), c_left, 0.0, c_left + grid_column.m_width, grid.m_header_height);
				}
				c_left = c_left + grid_column.m_width;
			}
		}
	}
}

pub fn mouse_move_grid(grid:&mut FCGrid, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	if (first_touch) {
		let mp = first_point.clone();
		if (grid.m_view.m_show_hscrollbar || grid.m_view.m_show_vscrollbar){
			if (grid.m_view.m_down_scroll_hbutton) {
				let content_width = get_grid_content_width(grid);
				let sub_x = (mp.x - grid.m_view.m_start_point.x) / grid.m_view.m_size.cx * content_width;
				let mut new_scrollh = grid.m_view.m_start_scroll_h + sub_x;
				if (new_scrollh < 0.0) {
					new_scrollh = 0.0;
				} else if (new_scrollh > content_width - grid.m_view.m_size.cx) {
					new_scrollh = content_width - grid.m_view.m_size.cx;
				}
				grid.m_view.m_scroll_h = new_scrollh;
				unsafe{
					M_CANCEL_CLICK = true;
				}
				return;

			} else if (grid.m_view.m_down_scroll_vbutton) {
				let content_height = get_grid_content_height(grid);
				let sub_y = (mp.y - grid.m_view.m_start_point.y) / (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size) * content_height;
				let mut new_scroll_v = grid.m_view.m_start_scroll_v + sub_y;
				if (new_scroll_v < 0.0) {
					new_scroll_v = 0.0;
				} else if (new_scroll_v > content_height - (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size)) {
					new_scroll_v = content_height - (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size);
				}
				grid.m_view.m_scroll_v = new_scroll_v;
				unsafe{
					M_CANCEL_CLICK = true;
				}
				return;
			}
		}
		if (grid.m_view.m_allow_drag_scroll) {
			let content_width = get_grid_content_width(grid);
			if (content_width > grid.m_view.m_size.cx) {
				let sub_x = grid.m_view.m_start_point.x - mp.x;
				let mut new_scrollh = grid.m_view.m_start_scroll_h + sub_x;
				if (new_scrollh < 0.0) {
					new_scrollh = 0.0;
				} else if (new_scrollh > content_width - grid.m_view.m_size.cx) {
					new_scrollh = content_width - grid.m_view.m_size.cx;
				}
				grid.m_view.m_scroll_h = new_scrollh;
				if(sub_x.abs() > 5.0){
				    unsafe{
						M_CANCEL_CLICK = true;
					}
				}
			}
			let content_height = get_grid_content_height(grid);
			if (content_height > grid.m_view.m_size.cy) {
				let sub_y = grid.m_view.m_start_point.y - mp.y;
				let mut new_scroll_v = grid.m_view.m_start_scroll_v + sub_y;
				if (new_scroll_v < 0.0) {
					new_scroll_v = 0.0;
				} else if (new_scroll_v > content_height - (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size)) {
					new_scroll_v = content_height - (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size);
				}
				grid.m_view.m_scroll_v = new_scroll_v;
				if(sub_y.abs() > 5.0){
				    unsafe{
						M_CANCEL_CLICK = true;
					}
				}
			}
		}
	}
}

pub fn mouse_down_grid(grid:&mut FCGrid, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	let mp = first_point.clone();
	grid.m_view.m_start_point = mp.clone();
	grid.m_view.m_down_scroll_hbutton = false;
	grid.m_view.m_down_scroll_vbutton = false;
	if (grid.m_view.m_show_hscrollbar){
		let content_width = get_grid_content_width(grid);
		if (content_width > grid.m_view.m_size.cx) {
		    let s_left = grid.m_view.m_scroll_h / content_width * grid.m_view.m_size.cx;
		    let mut s_right = (grid.m_view.m_scroll_h + grid.m_view.m_size.cx) / content_width * grid.m_view.m_size.cx;
		    if (s_right - s_left < grid.m_view.m_scroll_size) {
			    s_right = s_left + grid.m_view.m_scroll_size;
		    }
		    if (mp.x >= s_left && mp.x <= s_right && mp.y >= grid.m_view.m_size.cy - grid.m_view.m_scroll_size && mp.y <= grid.m_view.m_size.cy) {
			    grid.m_view.m_down_scroll_hbutton = true;
			    grid.m_view.m_start_scroll_h = grid.m_view.m_scroll_h;
			    return;
		    }
		}
	}
	if(grid.m_view.m_show_vscrollbar){
	    let content_height = get_grid_content_height(grid);
		if (content_height > grid.m_view.m_size.cy) {
			let s_top = grid.m_header_height + grid.m_view.m_scroll_v / content_height * (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size);
			let mut s_bottom  = (grid.m_view.m_scroll_v + (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size)) / content_height * (grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size);
			if (s_bottom  - s_top < grid.m_view.m_scroll_size) {
				s_bottom  = s_top + grid.m_view.m_scroll_size;
			}
			if (mp.x >= grid.m_view.m_size.cx - grid.m_view.m_scroll_size && mp.x <= grid.m_view.m_size.cx && mp.y >= s_top && mp.y <= s_bottom ) {
				grid.m_view.m_down_scroll_vbutton = true;
				grid.m_view.m_start_scroll_v = grid.m_view.m_scroll_v;
				return;
			}
		}
	}
	if (grid.m_view.m_allow_drag_scroll) {
		grid.m_view.m_start_scroll_h = grid.m_view.m_scroll_h;
		grid.m_view.m_start_scroll_v = grid.m_view.m_scroll_v;
	}
}

pub fn mouse_up_grid(grid:&mut FCGrid, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	grid.m_view.m_down_scroll_hbutton = false;
	grid.m_view.m_down_scroll_vbutton = false;
	unsafe{
		if(M_CANCEL_CLICK){
			return;
		}
	}
	
	let mut c_left = -grid.m_view.m_scroll_h;
	let mut c_top = -grid.m_view.m_scroll_v + grid.m_header_height;
	let mut col_left = 0.0;
	for i in 0..grid.m_columns.len(){
		let mut grid_column = (&grid.m_columns[i]).clone();
		let col_rect= FCRect{left:col_left, top:0.0, right:col_left + grid_column.m_width, bottom:grid.m_header_height};
		grid_column.m_bounds = col_rect;
		grid_column.m_index = i as i32;
		col_left = col_left + grid_column.m_width;
		grid.m_columns[i] = grid_column;
	}
	for i in 0..grid.m_rows.len(){
		let row = (&grid.m_rows[i]).clone();
		if (row.m_visible) {
			let r_top = c_top;
			let r_bottom = c_top + grid.m_row_height;
			if (r_bottom >= 0.0 && c_top <= grid.m_view.m_size.cy) {
				for j in 0..row.m_cells.len(){
					let cell = (&row.m_cells[j]).clone();
					let grid_column = (&grid.m_columns[j]).clone();
					if (grid_column.m_visible) {
						if (!grid_column.m_frozen) {
							let mut cell_width = grid_column.m_width;
							let col_span = cell.m_col_span;
							if (col_span > 1) {
								for n in 1..col_span{
									let span_column = (&grid.m_columns[(grid_column.m_index + n) as usize]).clone();
									if (span_column.m_visible) {
										cell_width = cell_width + span_column.m_width;
									}
								}
							}
							let mut cell_height = grid.m_row_height;
							let row_span = cell.m_row_span;
							if (row_span > 1) {
								for n in 1..row_span{
									let span_row = (&grid.m_rows[i + n as usize]).clone();
									if (span_row.m_visible) {
										cell_height = cell_height + grid.m_row_height;
									}
								}
							}
							
							let c_rect = FCRect{left:grid_column.m_bounds.left - grid.m_view.m_scroll_h, top:r_top, right:grid_column.m_bounds.left + cell_width - grid.m_view.m_scroll_h, bottom:r_top + cell_height};
							if (c_rect.right >= 0.0 && c_rect.left < grid.m_view.m_size.cx) {
							    if(first_point.x >= c_rect.left && first_point.x <= c_rect.right && first_point.y >= c_rect.top && first_point.y <= c_rect.bottom){
							        for r in 0..grid.m_rows.len(){
										let mut subRow = (&grid.m_rows[r]).clone();
										if(r == i){
											subRow.m_selected = true
										}else{
											subRow.m_selected = false
										}
										grid.m_rows[r] = subRow
									}
									return;
							    }
							}
						}
					}
				}
			}
			if (r_bottom >= 0.0 && c_top <= grid.m_view.m_size.cy) {
				for j in 0..row.m_cells.len(){
					let cell = (&row.m_cells[j]).clone();
					let grid_column = (&grid.m_columns[j]).clone();
					if (grid_column.m_visible) {
						if (grid_column.m_frozen) {
							let mut cell_width = grid_column.m_width;
							let col_span = cell.m_col_span;
							if (col_span > 1) {
								for n in 1..col_span{
									let span_column = (&grid.m_columns[(grid_column.m_index + n) as usize]).clone();
									if (span_column.m_visible) {
										cell_width = cell_width + span_column.m_width;
									}
								}
							}
							let mut cell_height = grid.m_row_height;
							let row_span = cell.m_row_span;
							if (row_span > 1) {
								for n in 1..row_span{
									let span_row = (&grid.m_rows[i + n as usize]).clone();
									if (span_row.m_visible) {
										cell_height = cell_height + grid.m_row_height;
									}
								}
							}
							
							let c_rect = FCRect{left:grid_column.m_bounds.left, top:r_top, right:grid_column.m_bounds.left + cell_width, bottom:r_top + cell_height};
							if (c_rect.right >= 0.0 && c_rect.left < grid.m_view.m_size.cx) {
							    if(first_point.x >= c_rect.left && first_point.x <= c_rect.right && first_point.y >= c_rect.top && first_point.y <= c_rect.bottom){
							        for r in 0..grid.m_rows.len(){
										let mut subRow = (&grid.m_rows[r]).clone();
										if(r == i){
											subRow.m_selected = true
										}else{
											subRow.m_selected = false
										}
										grid.m_rows[r] = subRow
									}
									return;
							    }
							}
						}
					}
				}
			}
			if (c_top > grid.m_view.m_size.cy) {
				break;
            }
			c_top = c_top + grid.m_row_height;
		}
	}
	if (grid.m_header_height > 0.0) {
		for i in 0..grid.m_columns.len(){
			let grid_column = (&grid.m_columns[i]).clone();
			if (grid_column.m_visible) {
				if (!grid_column.m_frozen) {
					if(first_point.x >= c_left && first_point.x <= c_left + grid_column.m_width){
				        return;
				    }
				}
				c_left = c_left + grid_column.m_width;
			}
		}
		c_left = 0.0;
		for i in 0..grid.m_columns.len(){
			let grid_column = (&grid.m_columns[i]).clone();
			if (grid_column.m_visible) {
				if (grid_column.m_frozen) {
					if(first_point.x >= c_left && first_point.x <= c_left + grid_column.m_width){
				        return;
				    }
				}
				c_left = c_left + grid_column.m_width;
			}
		}
	}
}

pub fn mouse_wheel_grid(grid:&mut FCGrid, delta:i32){
	let mut old_scroll_v = grid.m_view.m_scroll_v;
    if (delta > 0) {
	    old_scroll_v = old_scroll_v - grid.m_row_height;
    } else if (delta < 0) {
	    old_scroll_v = old_scroll_v + grid.m_row_height;
    }
    let content_height = get_grid_content_height(grid);
    if (content_height < grid.m_view.m_size.cy) {
        grid.m_view.m_scroll_v = 0.0;
    } else {
        if (old_scroll_v < 0.0) {
	        old_scroll_v = 0.0;
	    } else if (old_scroll_v > content_height - grid.m_view.m_size.cy + grid.m_header_height + grid.m_view.m_scroll_size) {
		    old_scroll_v = content_height - grid.m_view.m_size.cy + grid.m_header_height + grid.m_view.m_scroll_size;
        }
        grid.m_view.m_scroll_v = old_scroll_v;
    }
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

pub fn on_paint(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, clip_rect:FCRect){
	if(view.m_type == "chart"){
		M_PAINT.lock().unwrap().fill_rect(&context, view.m_back_color.clone(), 0.0, 0.0, view.m_size.cx, view.m_size.cy);
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				reset_chart_visible_record(&mut *v);
				check_chart_last_visible_index(&mut *v);
				calculate_chart_max_min(&mut *v);
				draw_chart_scale(&context, &mut *v, clip_rect.clone());
				draw_chart_stock(&context, &mut *v, clip_rect.clone());
				draw_chart_plot(&context, &mut *v);
				draw_chart_cross_line(&context, &mut *v, clip_rect.clone());
				break;
			}
		}
	}else if(view.m_type == "grid"){
		M_PAINT.lock().unwrap().fill_rect(&context, view.m_back_color.clone(), 0.0, 0.0, view.m_size.cx, view.m_size.cy);
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				draw_grid(&context, &mut *v, clip_rect.clone());
				break;
			}
		}		
	}
	else if(view.m_type == "radiobutton"){
		for (id, v) in M_RADIO_BUTTON_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				draw_radio_button(&context, &mut *v, clip_rect.clone());
				break;
			}
		}		
	}
	else if(view.m_type == "checkbox"){
		for (id, v) in M_CHECK_BOX_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				draw_check_box(&context, &mut *v, clip_rect.clone());
				break;
			}
		}	
	}
	else if(view.m_type == "tab" || view.m_type == "tabpage" || view.m_type == "layout"){
		let mut cview = view.clone();
		draw_div(&context, &mut cview, clip_rect.clone());
	}
	else if(view.m_type == "label"){
		if(view.m_text_color != "none"){
			M_PAINT.lock().unwrap().draw_text(&context, view.m_text.clone(), view.m_text_color.clone(), view.m_font.clone(), 1.0, view.m_size.cy / 2.0 + 1.0);
		}
	}
	else{
		let mut cview = view.clone();
		draw_button(&context, &mut cview, clip_rect.clone());
	}
}

pub fn on_paint_border(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, clip_rect:FCRect){
	if(view.m_type == "grid"){
		M_PAINT.lock().unwrap().draw_rect(&context, view.m_border_color.clone(), 1.0, Vec::new(), 0.0, 0.0, view.m_size.cx, view.m_size.cy);
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				draw_grid_scroll_bar(&context, &mut *v, clip_rect.clone());
				break;
			}
		}
		
	}else if(view.m_type == "div" || view.m_type == "layout"){
		let mut div = view.clone();
		draw_div_border(&context, &mut div, clip_rect.clone());
		draw_div_scroll_bar(&context, &mut div, clip_rect.clone());
	}else if(view.m_type == "tab" || view.m_type == "tabpage"){
		let mut cview = view.clone();
		draw_div_border(&context, &mut cview, clip_rect.clone());
	}
}

pub fn on_mouse_down(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, mp:FCPoint, buttons:i32, clicks:i32, delta:i32){
	if(view.m_type == "chart"){
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if(buttons == 1){
			first_touch = true;
		}
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				(*v).m_mouse_position = first_point.clone();
				(*v).m_mouse_down_position = first_point.clone();
				(*v).m_cross_stop_index = get_chart_index(&mut *v, first_point.clone());
				unsafe{
					if(M_ADDING_PLOT != -1){
						if (first_point.y < get_candle_div_height(&mut *v)){
							let touch_index = get_chart_index(&mut *v, first_point.clone());
							if (touch_index >= (*v).m_first_visible_index && touch_index <= (*v).m_last_visible_index){
								let mut plots:Vec<String> = Vec::new();
								plots.push("Line".to_string());
								plots.push("Segment".to_string());
								plots.push("Ray".to_string());
								plots.push("Triangle".to_string());
								plots.push("Rect".to_string());
								plots.push("Cycle".to_string());
								plots.push("CircumCycle".to_string());
								plots.push("Ellipse".to_string());
								plots.push("AngleLine".to_string());
								plots.push("ParalleGram".to_string());
								plots.push("SpeedResist".to_string());
								plots.push("FiboFanline".to_string());
								plots.push("FiboTimezone".to_string());
								plots.push("Percent".to_string());
								plots.push("BoxLine".to_string());
								plots.push("TironeLevels".to_string());
								plots.push("Parallel".to_string());
								plots.push("GoldenRatio".to_string());
								plots.push("LRLine".to_string());
								plots.push("LRChannel".to_string());
								plots.push("LRBand".to_string());
								
								let str_plot = plots[M_ADDING_PLOT as usize].clone();
								if(str_plot == "FiboTimezone"){
									let f_index = touch_index;
									let f_date = get_chart_date_by_index(&mut *v, f_index);
									let y = get_chart_value(&mut *v, first_point.clone());
									let mut new_plot:FCPlot = FCPlot::new();
									new_plot.m_id = create_new_id();
									if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
										new_plot.m_line_color = "rgb(0,0,0)".to_string();
										new_plot.m_point_color = "rgba(0,0,0,0.5)".to_string();
									}
									new_plot.m_key1 = f_date;
									new_plot.m_value1 = y;
									new_plot.m_plot_type = str_plot;
									(*v).m_plots.push(new_plot);
									(*v).m_splot = select_plot(&mut *v, first_point.clone());
								}
								else if (str_plot == "Triangle" || str_plot == "CircumCycle" || str_plot == "ParalleGram" || str_plot == "AngleLine" || str_plot == "Parallel" || str_plot == "SymmetricTriangle"){
									let e_index = touch_index;
									let b_index = e_index - 5;
									if (b_index >= 0) {
										let f_date = get_chart_date_by_index(&mut *v, b_index);
										let s_date = get_chart_date_by_index(&mut *v, e_index);
										let y = get_chart_value(&mut *v, first_point.clone());
										let mut new_plot:FCPlot = FCPlot::new();
										new_plot.m_id = create_new_id();
										if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
											new_plot.m_line_color = "rgb(0,0,0)".to_string();
											new_plot.m_point_color = "rgba(0,0,0,0.5)".to_string();
										}
										new_plot.m_key1 = f_date;
										new_plot.m_value1 = y;
										new_plot.m_key2 = s_date;
										new_plot.m_value2 = y;
										new_plot.m_key3 = s_date;
										new_plot.m_value3 = (*v).m_candle_min + ((*v).m_candle_max - (*v).m_candle_min) / 2.0;
										new_plot.m_plot_type = str_plot;
										(*v).m_plots.push(new_plot);
										(*v).m_splot = select_plot(&mut *v, first_point.clone());
									}
								}else{
									let e_index = touch_index;
									let b_index = e_index - 5;
									if (b_index >= 0) {
										let f_date = get_chart_date_by_index(&mut *v, b_index);
										let s_date = get_chart_date_by_index(&mut *v, e_index);
										let y = get_chart_value(&mut *v, first_point.clone());
										let mut new_plot:FCPlot = FCPlot::new();
										new_plot.m_id = create_new_id();
										if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
											new_plot.m_line_color = "rgb(0,0,0)".to_string();
											new_plot.m_point_color = "rgba(0,0,0,0.5)".to_string();
										}
										new_plot.m_key1 = f_date;
										new_plot.m_value1 = y;
										new_plot.m_key2 = s_date;
										new_plot.m_value2 = y;
										new_plot.m_plot_type = str_plot;
										(*v).m_plots.push(new_plot);
										(*v).m_splot = select_plot(&mut *v, first_point.clone());
									}
								}
							}
						}
						M_ADDING_PLOT = -1;
					}else{
						(*v).m_splot = select_plot(&mut *v, mp.clone());
						if((*v).m_splot.m_id <= 0){
							select_shape(&mut *v, mp.clone());
						}
					}
				}
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }
	else if(view.m_type == "grid"){
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if(buttons == 1){
			first_touch = true;
		}
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				mouse_down_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}
	else if(view.m_type == "div" || view.m_type == "layout"){
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if(buttons == 1){
			first_touch = true;
		}
		let mut div = view.clone();
		mouse_down_div(&mut div, first_touch, second_touch, first_point, second_point);
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
	}
}

pub fn on_mouse_move(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, mp:FCPoint, buttons:i32, clicks:i32, delta:i32){
	if(view.m_type == "chart"){
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if(buttons == 1){
			first_touch = true;
		}
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				(*v).m_mouse_position = mp.clone();
				(*v).m_cross_stop_index = get_chart_index(&mut *v, mp.clone());
				mouse_move_chart(&mut (*v), first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if(view.m_type == "grid"){
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if(buttons == 1){
			first_touch = true;
		}
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				mouse_move_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		if(buttons == 1){
			invalidate_view(context, view.clone());
		}
	}
	else if(view.m_type == "div" || view.m_type == "layout"){
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if(buttons == 1){
			first_touch = true;
		}
		let mut div = view.clone();
		mouse_move_div(&mut div, first_touch, second_touch, first_point, second_point);
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		if(buttons == 1){
			invalidate_view(context, view.clone());
		}
	}
}

pub fn on_mouse_up(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, mp:FCPoint, buttons:i32, clicks:i32, delta:i32){
	if(view.m_type == "chart"){
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				(*v).m_first_touch_index_cache = -1;
				(*v).m_second_touch_index_cache = -1;
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if(view.m_type == "grid"){
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if(buttons == 1){
			first_touch = true;
		}
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				mouse_up_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}else if(view.m_type == "div" || view.m_type == "layout"){
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if(buttons == 1){
			first_touch = true;
		}
		let mut div = view.clone();
		mouse_up_div(&mut div, first_touch, second_touch, first_point, second_point);
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
	}else{
		invalidate_view(context, view.clone());
	}
}

pub fn on_click(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, mp:FCPoint, buttons:i32, clicks:i32, delta:i32){
	if(view.m_type == "plot"){
		unsafe{
			M_ADDING_PLOT = view.m_name.parse::<i32>().unwrap();
		}
	}
	else if(view.m_type == "indicator"){
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			(*v).m_view = view.clone();
			if (view.m_text == "BOLL" || view.m_text == "MA") {
				(*v).m_main_indicator = view.m_text.clone();
			} else {
				(*v).m_show_indicator = view.m_text.clone();
			}
			calc_chart_indicator(&mut *v);
			calculate_chart_max_min(&mut *v);
			M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
		}
		invalidate(context);
	}
	else if(view.m_type == "checkbox"){
		for (id, v) in M_CHECK_BOX_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				click_check_box(&mut *v, mp.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}else if(view.m_type == "radiobutton"){
		for (id, v) in M_RADIO_BUTTON_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				click_radio_button(&mut *v, mp.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}
	else if(view.m_type == "headerbutton"){
		let mut is_this_tab:bool = false;
		for (id, v) in M_TAB_MAP.lock().unwrap().iter_mut(){
			for i in 0..(*v).m_tab_pages.len(){
				let tp = (*v).m_tab_pages[i].clone();
				if(tp.m_header_button.m_id == view.m_id){
					is_this_tab = true;
					break;
				}
			}
			if(is_this_tab){
				for j in 0..(*v).m_tab_pages.len(){
					let mut tp = (*v).m_tab_pages[j].clone();
					if(tp.m_header_button.m_id == view.m_id){
						tp.m_view.m_visible = true;
					}else{
						tp.m_view.m_visible = false;
					}
					M_VIEW_MAP.lock().unwrap().insert(tp.m_view.m_id, tp.m_view.clone());
					(*v).m_tab_pages[j] = tp;
				}
				update_tab_layout(&mut *v);
				invalidate_view(context, (*v).m_view.clone());
				break;
			}
		}
	}
}

pub fn on_mouse_wheel(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, mp:FCPoint, buttons:i32, clicks:i32, delta:i32){
	if(view.m_type == "chart"){
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				if(delta > 0){
					zoom_out_chart(&mut *v);
				}else if(delta < 0){
					zoom_in_chart(&mut *v);
				}
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if(view.m_type == "grid"){
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				mouse_wheel_grid(&mut *v, delta);
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }
    else if(view.m_type == "div" || view.m_type == "layout"){
		let mut div = view.clone();
		mouse_wheel_div(&mut div, delta);
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
    }
}

pub fn on_touch_start(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	if(view.m_type == "chart"){
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				(*v).m_mouse_position = first_point.clone();
				(*v).m_mouse_down_position = first_point.clone();
				(*v).m_cross_stop_index = get_chart_index(&mut *v, first_point.clone());
				unsafe{
					if(M_ADDING_PLOT != -1){
						if (first_point.y < get_candle_div_height(&mut *v)){
							let touch_index = get_chart_index(&mut *v, first_point.clone());
							if (touch_index >= (*v).m_first_visible_index && touch_index <= (*v).m_last_visible_index){
								let mut plots:Vec<String> = Vec::new();
								plots.push("Line".to_string());
								plots.push("Segment".to_string());
								plots.push("Ray".to_string());
								plots.push("Triangle".to_string());
								plots.push("Rect".to_string());
								plots.push("Cycle".to_string());
								plots.push("CircumCycle".to_string());
								plots.push("Ellipse".to_string());
								plots.push("AngleLine".to_string());
								plots.push("ParalleGram".to_string());
								plots.push("SpeedResist".to_string());
								plots.push("FiboFanline".to_string());
								plots.push("FiboTimezone".to_string());
								plots.push("Percent".to_string());
								plots.push("BoxLine".to_string());
								plots.push("TironeLevels".to_string());
								plots.push("Parallel".to_string());
								plots.push("GoldenRatio".to_string());
								plots.push("LRLine".to_string());
								plots.push("LRChannel".to_string());
								plots.push("LRBand".to_string());
								
								let str_plot = plots[M_ADDING_PLOT as usize].clone();
								if(str_plot == "FiboTimezone"){
									let f_index = touch_index;
									let f_date = get_chart_date_by_index(&mut *v, f_index);
									let y = get_chart_value(&mut *v, first_point.clone());
									let mut new_plot:FCPlot = FCPlot::new();
									new_plot.m_id = create_new_id();
									if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
										new_plot.m_line_color = "rgb(0,0,0)".to_string();
										new_plot.m_point_color = "rgba(0,0,0,0.5)".to_string();
									}
									new_plot.m_key1 = f_date;
									new_plot.m_value1 = y;
									new_plot.m_plot_type = str_plot;
									(*v).m_plots.push(new_plot);
									(*v).m_splot = select_plot(&mut *v, first_point.clone());
								}
								else if (str_plot == "Triangle" || str_plot == "CircumCycle" || str_plot == "ParalleGram" || str_plot == "AngleLine" || str_plot == "Parallel" || str_plot == "SymmetricTriangle"){
									let e_index = touch_index;
									let b_index = e_index - 5;
									if (b_index >= 0) {
										let f_date = get_chart_date_by_index(&mut *v, b_index);
										let s_date = get_chart_date_by_index(&mut *v, e_index);
										let y = get_chart_value(&mut *v, first_point.clone());
										let mut new_plot:FCPlot = FCPlot::new();
										new_plot.m_id = create_new_id();
										if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
											new_plot.m_line_color = "rgb(0,0,0)".to_string();
											new_plot.m_point_color = "rgba(0,0,0,0.5)".to_string();
										}
										new_plot.m_key1 = f_date;
										new_plot.m_value1 = y;
										new_plot.m_key2 = s_date;
										new_plot.m_value2 = y;
										new_plot.m_key3 = s_date;
										new_plot.m_value3 = (*v).m_candle_min + ((*v).m_candle_max - (*v).m_candle_min) / 2.0;
										new_plot.m_plot_type = str_plot;
										(*v).m_plots.push(new_plot);
										(*v).m_splot = select_plot(&mut *v, first_point.clone());
									}
								}else{
									let e_index = touch_index;
									let b_index = e_index - 5;
									if (b_index >= 0) {
										let f_date = get_chart_date_by_index(&mut *v, b_index);
										let s_date = get_chart_date_by_index(&mut *v, e_index);
										let y = get_chart_value(&mut *v, first_point.clone());
										let mut new_plot:FCPlot = FCPlot::new();
										new_plot.m_id = create_new_id();
										if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
											new_plot.m_line_color = "rgb(0,0,0)".to_string();
											new_plot.m_point_color = "rgba(0,0,0,0.5)".to_string();
										}
										new_plot.m_key1 = f_date;
										new_plot.m_value1 = y;
										new_plot.m_key2 = s_date;
										new_plot.m_value2 = y;
										new_plot.m_plot_type = str_plot;
										(*v).m_plots.push(new_plot);
										(*v).m_splot = select_plot(&mut *v, first_point.clone());
									}
								}
							}
						}
						M_ADDING_PLOT = -1;
					}else{
						(*v).m_splot = select_plot(&mut *v, first_point.clone());
						if((*v).m_splot.m_id <= 0){
							select_shape(&mut *v, first_point.clone());
						}
					}
				}
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }
	else if(view.m_type == "grid"){
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				mouse_down_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}
	else if(view.m_type == "div" || view.m_type == "layout"){
		let mut div = view.clone();
		mouse_down_div(&mut div, first_touch, second_touch, first_point.clone(), second_point.clone());
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
	}
}

pub fn on_touch_move(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	if(view.m_type == "chart"){
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				(*v).m_mouse_position = first_point.clone();
				(*v).m_cross_stop_index = get_chart_index(&mut *v, first_point.clone());
				mouse_move_chart(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if(view.m_type == "grid"){
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				mouse_move_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}
	else if(view.m_type == "div" || view.m_type == "layout"){
		let mut div = view.clone();
		mouse_move_div(&mut div, first_touch, second_touch, first_point.clone(), second_point.clone());
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
	}
}

pub fn on_touch_end(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	if(view.m_type == "chart"){
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				(*v).m_first_touch_index_cache = -1;
				(*v).m_second_touch_index_cache = -1;
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if(view.m_type == "grid"){;
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if(view.m_id == *id){
				(*v).m_view = view.clone();
				mouse_up_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}else if(view.m_type == "div" || view.m_type == "layout"){
		let mut div = view.clone();
		mouse_up_div(&mut div, first_touch, second_touch, first_point.clone(), second_point.clone());
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
	}else{
		invalidate_view(context, view.clone());
	}
}

pub fn render_views(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, views:Vec<FCView>, rect:FCRect, has_rect:bool){
	let views_size = views.len();
	 for i in 0..views_size{
		let view = &views[i];
		let v_id = view.m_id;
		if(!has_rect){
			let sub_views = get_sub_views(view.clone());
			let sub_views_size = sub_views.len();
		    if(sub_views_size > 0) {
		        if(sub_views_size > 0) {
			        render_views(context, sub_views, rect.clone(), false);
		        }
	        }
	        let mut change_view = M_VIEW_MAP.lock().unwrap()[&v_id].clone();
		    change_view.m_has_clip = false;
		    M_VIEW_MAP.lock().unwrap().insert(v_id, change_view);
		    continue;
		}
		if(!view.m_top_most && is_paint_visible(view.clone())) {
			let clx = client_x(view.clone());
			let cly = client_y(view.clone());
			let draw_rect = FCRect{left:0.0, top:0.0, right:view.m_size.cx, bottom:view.m_size.cy};
			let clip_rect = FCRect{left:clx, top:cly, right:clx + view.m_size.cx, bottom:cly + view.m_size.cy};
			let mut dest_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
			if(get_intersect_rect(&mut dest_rect, rect.clone(), clip_rect.clone()) > 0){
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
				if(sub_views_size > 0) {
					render_views(context, sub_views, dest_rect.clone(), true);
				}
			    M_PAINT.lock().unwrap().set_offset(context, clx, cly);
			    on_paint_border(context, view.clone(), draw_rect.clone());
			    M_PAINT.lock().unwrap().restore(context);
			}else{
				let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if(sub_views_size > 0) {
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
		if(!has_rect){
		    continue;
		}
		if(view.m_top_most && is_paint_visible(view.clone())) {
			let clx = client_x(view.clone());
			let cly = client_y(view.clone());
			let draw_rect = FCRect{left:0.0, top:0.0, right:view.m_size.cx, bottom:view.m_size.cy};
			let clip_rect = FCRect{left:clx, top:cly, right:clx + view.m_size.cx, bottom:cly + view.m_size.cy};
			let mut dest_rect = FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0};
			if(get_intersect_rect(&mut dest_rect, rect.clone(), clip_rect.clone()) > 0){
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
				if(sub_views_size > 0) {
					render_views(context, sub_views, dest_rect.clone(), true);
				}
			    
			    M_PAINT.lock().unwrap().set_offset(context, clx, cly);
			    on_paint_border(context, view.clone(), draw_rect.clone());
			    M_PAINT.lock().unwrap().restore(context);
			}else{
				let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if(sub_views_size > 0) {
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
		if(view.m_visible && view.m_top_most) {
			if(contains_point(view.clone(), mp.clone())) {
			    if(view.m_show_hscrollbar && view.m_scroll_size > 0.0){
			        let clx = client_x(view.clone());
	                if(mp.x >= clx + view.m_size.cx - view.m_scroll_size){
	                    return view.clone();
	                }
			    }
			    if(view.m_show_vscrollbar && view.m_scroll_size > 0.0){
			        let cly = client_y(view.clone());
	                if(mp.y >= cly + view.m_size.cy - view.m_scroll_size){
	                    return view.clone();
	                }
			    }
			    let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if(sub_views_size > 0) {
					let sub_view = find_view(mp.clone(), sub_views);
					if(sub_view.m_id != -1) {
						return sub_view.clone();
					}
				}
				return view.clone();
			}
		}
	}
	for i in 0..views_size{
		let view = &views[views_size - i - 1];
		if(view.m_visible && !view.m_top_most) {
			if(contains_point(view.clone(), mp.clone())) {
			    if(view.m_show_hscrollbar && view.m_scroll_size > 0.0){
			        let clx = client_x(view.clone());
	                if(mp.x >= clx + view.m_size.cx - view.m_scroll_size){
	                    return view.clone();
	                }
			    }
			    if(view.m_show_vscrollbar && view.m_scroll_size > 0.0){
			        let cly = client_y(view.clone());
	                if(mp.y >= cly + view.m_size.cy - view.m_scroll_size){
	                    return view.clone();
	                }
			    }
			    let sub_views = get_sub_views(view.clone());
				let sub_views_size = sub_views.len();
				if(sub_views_size > 0) {
					let sub_view = find_view(mp.clone(), sub_views);
					if(sub_view.m_id != -1) {
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
		if(is_paint_visible(view.clone())){
			let top_views2 = get_top_views();
			let cl_x = client_x(view.clone());
	        let cl_y = client_y(view.clone());
	        let draw_rect = FCRect{left:cl_x, top:cl_y, right:cl_x + view.m_size.cx, bottom:cl_y + view.m_size.cy};
			render_views(&context, top_views2, draw_rect, true);
		}
	}
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    //let colors = vec!["#F4908E", "#F2F097", "#88B0DC", "#F7B5D1", "#53C4AF", "#FDE38C"];
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    document.body().unwrap().append_child(&canvas)?;

    canvas.set_width(900);
    canvas.set_height(800);
    canvas.style().set_property("border", "solid")?;
    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let context = Rc::new(context);
    let pressed = Rc::new(Cell::new(false));

    { mouse_down(&context, &pressed, &canvas); }
    { mouse_move(&context, &pressed, &canvas); }
    { mouse_up(&context, &pressed, &canvas); }
    { mouse_wheel(&context, &pressed, &canvas); }
    { touch_start(&context, &pressed, &canvas); }
    { touch_move(&context, &pressed, &canvas); }
    { touch_end(&context, &pressed, &canvas); }

    // Create divs for color picker
    /*for c in colors {
        let div = document
            .create_element("div")?
            .dyn_into::<web_sys::HtmlElement>()?;
        div.set_class_name("color");
        {
            click(&context, &div, c.clone());
        }
        
        div.style().set_property("background-color", c);
        let div = div.dyn_into::<web_sys::Node>()?;
        document.body().unwrap().append_child(&div)?;
    }*/
    
    let rustMode:i32 = 0;
    if(rustMode == 0){
		let mut chart_view:FCView = FCView::new();
		chart_view.m_location = FCPoint{x:0.0, y:0.0};
		chart_view.m_size = FCSize{cx:900.0, cy:500.0};
		chart_view.m_back_color = "rgb(0,0,0)".to_string();
		chart_view.m_border_color = "rgb(100,100,100)".to_string();
		chart_view.m_type = "chart".to_string();
		chart_view.m_id = add_view(chart_view.clone());
		
		let mut grid_view:FCView = FCView::new();
		grid_view.m_location = FCPoint{x:0.0, y:500.0};
		grid_view.m_size = FCSize{cx:500.0, cy:300.0};
		grid_view.m_back_color = "rgb(0,0,0)".to_string();
		grid_view.m_border_color = "rgb(100,100,100)".to_string();
		grid_view.m_type = "grid".to_string();
		grid_view.m_show_vscrollbar = true;
		grid_view.m_show_hscrollbar = true;
		grid_view.m_allow_drag_scroll = true;
		grid_view.m_id = add_view(grid_view.clone());
		
		let mut grid:FCGrid = FCGrid::new();
		grid.m_view = grid_view.clone();
		let mut grid_column1:FCGridColumn = FCGridColumn::new();
		grid_column1.m_text = String::from("date");
		grid_column1.m_back_color = String::from("rgb(0,0,0)");
		grid_column1.m_text_color = String::from("rgb(255,255,255)");
		grid_column1.m_border_color =  String::from("rgb(150,150,150)");
		grid_column1.m_frozen = true;
		grid.m_columns.push(grid_column1);
		
		let mut grid_column2:FCGridColumn = FCGridColumn::new();
		grid_column2.m_text = String::from("open");
		grid_column2.m_back_color = String::from("rgb(0,0,0)");
		grid_column2.m_text_color = String::from("rgb(255,255,255)");
		grid_column2.m_border_color =  String::from("rgb(150,150,150)");
		grid_column2.m_frozen = true;
		grid.m_columns.push(grid_column2);
		
		let mut grid_column3:FCGridColumn = FCGridColumn::new();
		grid_column3.m_text = String::from("high");
		grid_column3.m_back_color = String::from("rgb(0,0,0)");
		grid_column3.m_text_color = String::from("rgb(255,255,255)");
		grid_column3.m_border_color =  String::from("rgb(150,150,150)");
		grid.m_columns.push(grid_column3);
		
		let mut grid_column4:FCGridColumn = FCGridColumn::new();
		grid_column4.m_text = String::from("low");
		grid_column4.m_back_color = String::from("rgb(0,0,0)");
		grid_column4.m_text_color = String::from("rgb(255,255,255)");
		grid_column4.m_border_color =  String::from("rgb(150,150,150)");
		grid.m_columns.push(grid_column4);
		
		let mut grid_column5:FCGridColumn = FCGridColumn::new();
		grid_column5.m_text = String::from("close");
		grid_column5.m_back_color = String::from("rgb(0,0,0)");
		grid_column5.m_text_color = String::from("rgb(255,255,255)");
		grid_column5.m_border_color =  String::from("rgb(150,150,150)");
		grid.m_columns.push(grid_column5);
		
		let mut grid_column6:FCGridColumn = FCGridColumn::new();
		grid_column6.m_text = String::from("volume");
		grid_column6.m_back_color = String::from("rgb(0,0,0)");
		grid_column6.m_text_color = String::from("rgb(255,255,255)");
		grid_column6.m_border_color =  String::from("rgb(150,150,150)");
		grid.m_columns.push(grid_column6);
		
		let mut div_view:FCView = FCView::new();
		div_view.m_location = FCPoint{x:500.0, y:500.0};
		div_view.m_size = FCSize{cx:200.0, cy:150.0};
		div_view.m_back_color = "rgb(0,0,0)".to_string();
		div_view.m_border_color = "rgb(100,100,100)".to_string();
		div_view.m_type = "div".to_string();
		div_view.m_show_vscrollbar = true;
		div_view.m_show_hscrollbar = true;
		div_view.m_allow_drag_scroll = true;
		div_view.m_id = add_view(div_view.clone());
		let mut plots:Vec<String> = Vec::new();
        plots.push("Line".to_string());
        plots.push("Segment".to_string());
        plots.push("Ray".to_string());
        plots.push("Triangle".to_string());
        plots.push("Rect".to_string());
        plots.push("Cycle".to_string());
        plots.push("CircumCycle".to_string());
        plots.push("Ellipse".to_string());
        plots.push("AngleLine".to_string());
        plots.push("ParalleGram".to_string());
        plots.push("SpeedResist".to_string());
        plots.push("FiboFanline".to_string());
        plots.push("FiboTimezone".to_string());
        plots.push("Percent".to_string());
        plots.push("BoxLine".to_string());
        plots.push("TironeLevels".to_string());
        plots.push("Parallel".to_string());
        plots.push("GoldenRatio".to_string());
        plots.push("LRLine".to_string());
		plots.push("LRChannel".to_string());
		plots.push("LRBand".to_string());
		for i in 0..plots.len(){
			let mut sub_view:FCView = FCView::new();
			sub_view.m_location = FCPoint{x:50.0 * (i as f32), y:50.0 * (i as f32)};
			sub_view.m_size = FCSize{cx:100.0, cy:20.0};
			sub_view.m_back_color = "rgb(0,0,0)".to_string();
			sub_view.m_border_color = "rgb(100,100,100)".to_string();
			sub_view.m_text_color = "rgb(255,255,255)".to_string();
			sub_view.m_text = plots[i].clone();
			sub_view.m_name = i.to_string();
			sub_view.m_type = "plot".to_string();
			sub_view.m_allow_drag = true;
			add_view_to_parent(sub_view.clone(), div_view.clone());
		}
		
		let mut split_view:FCView = FCView::new();
		split_view.m_location = FCPoint{x:500.0, y:650.0};
		split_view.m_size = FCSize{cx:200.0, cy:150.0};
		split_view.m_back_color = "rgb(0,0,0)".to_string();
		split_view.m_border_color = "rgb(100,100,100)".to_string();
		split_view.m_type = "split".to_string();
		split_view.m_id = add_view(split_view.clone());
		
		let mut first_view:FCView = FCView::new();
		first_view.m_back_color = "rgb(255,0,0)".to_string();
		first_view.m_id = add_view_to_parent(first_view.clone(), split_view.clone());
		
		let mut second_view:FCView = FCView::new();
		second_view.m_back_color = "rgb(0,255,0)".to_string();
		second_view.m_id = add_view_to_parent(second_view.clone(), split_view.clone());
		
		let mut splitter:FCView = FCView::new();
		splitter.m_back_color = "rgb(0,0,255)".to_string();
		splitter.m_location = FCPoint{x:100.0, y:0.0};
		splitter.m_size = FCSize{cx:5.0, cy:200.0};
		splitter.m_id = add_view_to_parent(splitter.clone(), split_view.clone());
		
		let mut split:FCSplitLayoutDiv = FCSplitLayoutDiv::new();
		split.m_first_view = first_view.clone();
		split.m_second_view = second_view.clone();
		split.m_splitter = splitter.clone();
		split.m_old_size = FCSize{cx:200.0, cy:150.0};
		split.m_view = split_view.clone();
		reset_split_layout_div(&mut split);
		M_SPLIT_MAP.lock().unwrap().insert(split_view.m_id, split.clone());
		
		let mut layout_view:FCView = FCView::new();
		layout_view.m_location = FCPoint{x:700.0, y:500.0};
		layout_view.m_size = FCSize{cx:200.0, cy:150.0};
		layout_view.m_back_color = "rgb(0,0,0)".to_string();
		layout_view.m_border_color = "rgb(100,100,100)".to_string();
		layout_view.m_type = "layout".to_string();
		layout_view.m_show_vscrollbar = true;
		layout_view.m_show_hscrollbar = true;
		layout_view.m_allow_drag_scroll = true;
		layout_view.m_id = add_view(layout_view.clone());
		let mut indicators : Vec<String> = Vec::new();
        indicators.push("MA".to_string());
        indicators.push("BOLL".to_string());
        indicators.push("MACD".to_string());
        indicators.push("KDJ".to_string());
        indicators.push("BIAS".to_string());
        indicators.push("ROC".to_string());
        indicators.push("WR".to_string());
        indicators.push("DMA".to_string());
        indicators.push("RSI".to_string());
        indicators.push("BBI".to_string());
        indicators.push("CCI".to_string());
        indicators.push("TRIX".to_string());
		for i in 0..indicators.len(){
			let mut sub_view2:FCView = FCView::new();
			sub_view2.m_size = FCSize{cx:100.0, cy:30.0};
			sub_view2.m_back_color = "rgb(0,0,0)".to_string();
			sub_view2.m_border_color = "rgb(100,100,100)".to_string();
			sub_view2.m_text_color = "rgb(255,255,255)".to_string();
			sub_view2.m_text = indicators[i].to_string();
			sub_view2.m_name = "indicator".to_string();
			sub_view2.m_type = "indicator".to_string();
			sub_view2.m_allow_drag = true;
			sub_view2.m_margin = FCPadding{left:10.0, top:10.0, right:10.0, bottom:10.0};
			add_view_to_parent(sub_view2.clone(), layout_view.clone());
		}
		let mut layout_div:FCLayoutDiv = FCLayoutDiv::new();
		
		layout_div.m_view = layout_view.clone();
		layout_div.m_layout_style = "toptobottom".to_string();
		reset_layout_div(&mut layout_div);
		M_LAYOUT_MAP.lock().unwrap().insert(layout_view.m_id, layout_div.clone());
		
		let mut tab_view:FCView = FCView::new();
		tab_view.m_location = FCPoint{x:700.0, y:650.0};
		tab_view.m_size = FCSize{cx:200.0, cy:150.0};
		tab_view.m_back_color = "rgb(0,0,0)".to_string();
		tab_view.m_border_color = "rgb(100,100,100)".to_string();
		tab_view.m_type = "layout".to_string();
		tab_view.m_id = add_view(tab_view.clone());
		
		let mut tab:FCTabView = FCTabView::new();
		for i in 0..4{
			let mut tab_page_view:FCView = FCView::new();
			tab_page_view.m_size = FCSize{cx:100.0, cy:20.0};
			if(i == 0){
				tab_page_view.m_back_color = "rgb(255,0,255)".to_string();
			}else if(i == 1){
				tab_page_view.m_back_color = "rgb(0,255,0)".to_string();
			}else if(i == 2){
				tab_page_view.m_back_color = "rgb(0,0,255)".to_string();
			}else if(i == 3){
				tab_page_view.m_back_color = "rgb(255,0,0)".to_string();
			}
			tab_page_view.m_border_color = "rgb(100,100,100)".to_string();
			tab_page_view.m_text_color = "rgb(255,255,255)".to_string();
			tab_page_view.m_type = "tabpage".to_string();
			if(i != 0){
				tab_page_view.m_visible = false;
			}
			tab_page_view.m_id = add_view_to_parent(tab_page_view.clone(), tab_view.clone());
			
			let mut header_button_view:FCView = FCView::new();
			header_button_view.m_size = FCSize{cx:100.0, cy:20.0};
			header_button_view.m_back_color = "rgb(0,0,0)".to_string();
			header_button_view.m_border_color = "rgb(100,100,100)".to_string();
			header_button_view.m_text_color = "rgb(255,255,255)".to_string();
			header_button_view.m_type = "headerbutton".to_string();
			header_button_view.m_text = "Page".to_string() + &(i.to_string());
			header_button_view.m_id = add_view_to_parent(header_button_view.clone(), tab_view.clone());
			let mut tab_page:FCTabPage = FCTabPage::new();
			tab_page.m_view = tab_page_view.clone();
			tab_page.m_header_button = header_button_view.clone();
			tab.m_tab_pages.push(tab_page);
		}
		tab.m_view = tab_view.clone();
		update_tab_layout(&mut tab);
		M_TAB_MAP.lock().unwrap().insert(tab_view.m_id, tab.clone());
		
		let mut security_datas:Vec<SecurityData> = Vec::new();
		let mut s_data1:SecurityData = SecurityData::new();
		s_data1.m_volume = 78469.756458;
		s_data1.m_close = 15684.25;
		s_data1.m_high = 15965.01;
		s_data1.m_low = 15272.69;
		s_data1.m_open = 15297.22;
		s_data1.m_date = 1605052800.0;
		security_datas.push(s_data1);
		let mut s_data2:SecurityData = SecurityData::new();
		s_data2.m_volume = 102196.366592;
		s_data2.m_close = 16291.87;
		s_data2.m_high = 16340.71;
		s_data2.m_low = 15440.65;
		s_data2.m_open = 15684.26;
		s_data2.m_date = 1605139200.0;
		security_datas.push(s_data2);
		let mut s_data3:SecurityData = SecurityData::new();
		s_data3.m_volume = 75691.891014;
		s_data3.m_close = 16320.71;
		s_data3.m_high = 16480.01;
		s_data3.m_low = 15952.36;
		s_data3.m_open = 16291.86;
		s_data3.m_date = 1605225600.0;
		security_datas.push(s_data3);
		let mut s_data4:SecurityData = SecurityData::new();
		s_data4.m_volume = 59116.357179;
		s_data4.m_close = 16070.46;
		s_data4.m_high = 16327.0;
		s_data4.m_low = 15670.01;
		s_data4.m_open = 16320.05;
		s_data4.m_date = 1605312000.0;
		security_datas.push(s_data4);
		let mut s_data5:SecurityData = SecurityData::new();
		s_data5.m_volume = 43596.851513;
		s_data5.m_close = 15957.01;
		s_data5.m_high = 16180.01;
		s_data5.m_low = 15774.73;
		s_data5.m_open = 16069.57;
		s_data5.m_date = 1605398400.0;
		security_datas.push(s_data5);
		let mut s_data6:SecurityData = SecurityData::new();
		s_data6.m_volume = 81300.685924;
		s_data6.m_close = 16713.58;
		s_data6.m_high = 16880.01;
		s_data6.m_low = 15864.01;
		s_data6.m_open = 15957.01;
		s_data6.m_date = 1605484800.0;
		security_datas.push(s_data6);
		let mut s_data7:SecurityData = SecurityData::new();
		s_data7.m_volume = 115192.02613;
		s_data7.m_close = 17659.39;
		s_data7.m_high = 17858.83;
		s_data7.m_low = 16538.01;
		s_data7.m_open = 16713.09;
		s_data7.m_date = 1605571200.0;
		security_datas.push(s_data7);
		let mut s_data8:SecurityData = SecurityData::new();
		s_data8.m_volume = 141984.264415;
		s_data8.m_close = 17776.13;
		s_data8.m_high = 18476.94;
		s_data8.m_low = 17222.01;
		s_data8.m_open = 17659.39;
		s_data8.m_date = 1605657600.0;
		security_datas.push(s_data8);
		let mut s_data9:SecurityData = SecurityData::new();
		s_data9.m_volume = 93009.571008;
		s_data9.m_close = 17802.83;
		s_data9.m_high = 18179.81;
		s_data9.m_low = 17335.66;
		s_data9.m_open = 17777.76;
		s_data9.m_date = 1605744000.0;
		security_datas.push(s_data9);
		let mut s_data10:SecurityData = SecurityData::new();
		s_data10.m_volume = 88423.028489;
		s_data10.m_close = 18655.68;
		s_data10.m_high = 18815.23;
		s_data10.m_low = 17740.05;
		s_data10.m_open = 17802.82;
		s_data10.m_date = 1605830400.0;
		security_datas.push(s_data10);
		let mut s_data11:SecurityData = SecurityData::new();
		s_data11.m_volume = 71308.137138;
		s_data11.m_close = 18703.81;
		s_data11.m_high = 18965.91;
		s_data11.m_low = 18308.59;
		s_data11.m_open = 18655.67;
		s_data11.m_date = 1605916800.0;
		security_datas.push(s_data11);
		let mut s_data12:SecurityData = SecurityData::new();
		s_data12.m_volume = 74492.806006;
		s_data12.m_close = 18414.61;
		s_data12.m_high = 18750.01;
		s_data12.m_low = 17699.8;
		s_data12.m_open = 18703.81;
		s_data12.m_date = 1606003200.0;
		security_datas.push(s_data12);
		let mut s_data13:SecurityData = SecurityData::new();
		s_data13.m_volume = 74179.661546;
		s_data13.m_close = 18368.01;
		s_data13.m_high = 18766.01;
		s_data13.m_low = 18000.01;
		s_data13.m_open = 18413.89;
		s_data13.m_date = 1606089600.0;
		security_datas.push(s_data13);
		let mut s_data14:SecurityData = SecurityData::new();
		s_data14.m_volume = 70126.693681;
		s_data14.m_close = 19160.02;
		s_data14.m_high = 19410.0;
		s_data14.m_low = 18207.99;
		s_data14.m_open = 18368.02;
		s_data14.m_date = 1606176000.0;
		security_datas.push(s_data14);
		let mut s_data15:SecurityData = SecurityData::new();
		s_data15.m_volume = 70760.56353;
		s_data15.m_close = 18719.12;
		s_data15.m_high = 19444.01;
		s_data15.m_low = 18570.01;
		s_data15.m_open = 19160.01;
		s_data15.m_date = 1606262400.0;
		security_datas.push(s_data15);
		let mut s_data16:SecurityData = SecurityData::new();
		s_data16.m_volume = 102420.321403;
		s_data16.m_close = 17149.48;
		s_data16.m_high = 18915.04;
		s_data16.m_low = 16188.01;
		s_data16.m_open = 18718.84;
		s_data16.m_date = 1606348800.0;
		security_datas.push(s_data16);
		let mut s_data17:SecurityData = SecurityData::new();
		s_data17.m_volume = 84036.578341;
		s_data17.m_close = 17139.53;
		s_data17.m_high = 17457.63;
		s_data17.m_low = 16438.09;
		s_data17.m_open = 17149.48;
		s_data17.m_date = 1606435200.0;
		security_datas.push(s_data17);
		let mut s_data18:SecurityData = SecurityData::new();
		s_data18.m_volume = 60835.734083;
		s_data18.m_close = 17719.86;
		s_data18.m_high = 17880.5;
		s_data18.m_low = 16865.57;
		s_data18.m_open = 17139.54;
		s_data18.m_date = 1606521600.0;
		security_datas.push(s_data18);
		let mut s_data19:SecurityData = SecurityData::new();
		s_data19.m_volume = 55329.026303;
		s_data19.m_close = 18185.0;
		s_data19.m_high = 18360.06;
		s_data19.m_low = 17517.01;
		s_data19.m_open = 17719.85;
		s_data19.m_date = 1606608000.0;
		security_datas.push(s_data19);
		let mut s_data20:SecurityData = SecurityData::new();
		s_data20.m_volume = 96227.350495;
		s_data20.m_close = 19695.88;
		s_data20.m_high = 19777.04;
		s_data20.m_low = 18185.0;
		s_data20.m_open = 18185.01;
		s_data20.m_date = 1606694400.0;
		security_datas.push(s_data20);
		let mut s_data21:SecurityData = SecurityData::new();
		s_data21.m_volume = 100331.141203;
		s_data21.m_close = 18764.97;
		s_data21.m_high = 19888.01;
		s_data21.m_low = 18350.0;
		s_data21.m_open = 19695.88;
		s_data21.m_date = 1606780800.0;
		security_datas.push(s_data21);
		let mut s_data22:SecurityData = SecurityData::new();
		s_data22.m_volume = 75848.875728;
		s_data22.m_close = 19204.1;
		s_data22.m_high = 19342.01;
		s_data22.m_low = 18330.01;
		s_data22.m_open = 18764.97;
		s_data22.m_date = 1606867200.0;
		security_datas.push(s_data22);
		let mut s_data23:SecurityData = SecurityData::new();
		s_data23.m_volume = 64987.682732;
		s_data23.m_close = 19421.91;
		s_data23.m_high = 19598.01;
		s_data23.m_low = 18867.21;
		s_data23.m_open = 19204.09;
		s_data23.m_date = 1606953600.0;
		security_datas.push(s_data23);
		let mut s_data24:SecurityData = SecurityData::new();
		s_data24.m_volume = 65481.220724;
		s_data24.m_close = 18650.53;
		s_data24.m_high = 19527.01;
		s_data24.m_low = 18565.32;
		s_data24.m_open = 19422.35;
		s_data24.m_date = 1607040000.0;
		security_datas.push(s_data24);
		let mut s_data25:SecurityData = SecurityData::new();
		s_data25.m_volume = 42922.758573;
		s_data25.m_close = 19147.67;
		s_data25.m_high = 19177.01;
		s_data25.m_low = 18500.01;
		s_data25.m_open = 18650.52;
		s_data25.m_date = 1607126400.0;
		security_datas.push(s_data25);
		let mut s_data26:SecurityData = SecurityData::new();
		s_data26.m_volume = 37043.101861;
		s_data26.m_close = 19359.41;
		s_data26.m_high = 19420.01;
		s_data26.m_low = 18857.01;
		s_data26.m_open = 19147.67;
		s_data26.m_date = 1607212800.0;
		security_datas.push(s_data26);
		let mut s_data27:SecurityData = SecurityData::new();
		s_data27.m_volume = 40833.661907;
		s_data27.m_close = 19166.91;
		s_data27.m_high = 19420.92;
		s_data27.m_low = 18902.89;
		s_data27.m_open = 19358.68;
		s_data27.m_date = 1607299200.0;
		security_datas.push(s_data27);
		let mut s_data28:SecurityData = SecurityData::new();
		s_data28.m_volume = 56080.515386;
		s_data28.m_close = 18324.12;
		s_data28.m_high = 19294.85;
		s_data28.m_low = 18200.01;
		s_data28.m_open = 19166.91;
		s_data28.m_date = 1607385600.0;
		security_datas.push(s_data28);
		let mut s_data29:SecurityData = SecurityData::new();
		s_data29.m_volume = 75498.213856;
		s_data29.m_close = 18541.29;
		s_data29.m_high = 18639.58;
		s_data29.m_low = 17650.01;
		s_data29.m_open = 18324.12;
		s_data29.m_date = 1607472000.0;
		security_datas.push(s_data29);
		let mut s_data30:SecurityData = SecurityData::new();
		s_data30.m_volume = 52890.685094;
		s_data30.m_close = 18254.64;
		s_data30.m_high = 18557.33;
		s_data30.m_low = 17911.13;
		s_data30.m_open = 18541.3;
		s_data30.m_date = 1607558400.0;
		security_datas.push(s_data30);
		let mut s_data31:SecurityData = SecurityData::new();
		s_data31.m_volume = 72610.734259;
		s_data31.m_close = 18036.54;
		s_data31.m_high = 18292.74;
		s_data31.m_low = 17572.34;
		s_data31.m_open = 18254.82;
		s_data31.m_date = 1607644800.0;
		security_datas.push(s_data31);
		let mut s_data32:SecurityData = SecurityData::new();
		s_data32.m_volume = 47592.761572;
		s_data32.m_close = 18808.7;
		s_data32.m_high = 18948.67;
		s_data32.m_low = 18020.71;
		s_data32.m_open = 18036.54;
		s_data32.m_date = 1607731200.0;
		security_datas.push(s_data32);
		let mut s_data33:SecurityData = SecurityData::new();
		s_data33.m_volume = 56560.831744;
		s_data33.m_close = 19175.0;
		s_data33.m_high = 19411.01;
		s_data33.m_low = 18711.13;
		s_data33.m_open = 18808.7;
		s_data33.m_date = 1607817600.0;
		security_datas.push(s_data33);
		let mut s_data34:SecurityData = SecurityData::new();
		s_data34.m_volume = 47257.211294;
		s_data34.m_close = 19273.15;
		s_data34.m_high = 19349.01;
		s_data34.m_low = 19000.01;
		s_data34.m_open = 19175.0;
		s_data34.m_date = 1607904000.0;
		security_datas.push(s_data34);
		let mut s_data35:SecurityData = SecurityData::new();
		s_data35.m_volume = 61792.483421;
		s_data35.m_close = 19426.44;
		s_data35.m_high = 19570.01;
		s_data35.m_low = 19050.01;
		s_data35.m_open = 19273.7;
		s_data35.m_date = 1607990400.0;
		security_datas.push(s_data35);
		let mut s_data36:SecurityData = SecurityData::new();
		s_data36.m_volume = 97061.040854;
		s_data36.m_close = 21335.53;
		s_data36.m_high = 21560.01;
		s_data36.m_low = 19278.61;
		s_data36.m_open = 19426.44;
		s_data36.m_date = 1608076800.0;
		security_datas.push(s_data36);
		let mut s_data37:SecurityData = SecurityData::new();
		s_data37.m_volume = 148769.444419;
		s_data37.m_close = 22808.99;
		s_data37.m_high = 23699.71;
		s_data37.m_low = 21233.01;
		s_data37.m_open = 21335.53;
		s_data37.m_date = 1608163200.0;
		security_datas.push(s_data37);
		let mut s_data38:SecurityData = SecurityData::new();
		s_data38.m_volume = 79646.144315;
		s_data38.m_close = 23107.4;
		s_data38.m_high = 23285.19;
		s_data38.m_low = 22350.01;
		s_data38.m_open = 22797.16;
		s_data38.m_date = 1608249600.0;
		security_datas.push(s_data38);
		let mut s_data39:SecurityData = SecurityData::new();
		s_data39.m_volume = 82430.712232;
		s_data39.m_close = 23821.62;
		s_data39.m_high = 24171.48;
		s_data39.m_low = 22750.01;
		s_data39.m_open = 23107.4;
		s_data39.m_date = 1608336000.0;
		security_datas.push(s_data39);
		let mut s_data40:SecurityData = SecurityData::new();
		s_data40.m_volume = 74797.201958;
		s_data40.m_close = 23455.53;
		s_data40.m_high = 24295.01;
		s_data40.m_low = 23060.01;
		s_data40.m_open = 23821.61;
		s_data40.m_date = 1608422400.0;
		security_datas.push(s_data40);
		let mut s_data41:SecurityData = SecurityData::new();
		s_data41.m_volume = 83424.441428;
		s_data41.m_close = 22719.72;
		s_data41.m_high = 24102.78;
		s_data41.m_low = 21815.01;
		s_data41.m_open = 23455.55;
		s_data41.m_date = 1608508800.0;
		security_datas.push(s_data41);
		let mut s_data42:SecurityData = SecurityData::new();
		s_data42.m_volume = 87033.13616;
		s_data42.m_close = 23810.8;
		s_data42.m_high = 23837.11;
		s_data42.m_low = 22353.41;
		s_data42.m_open = 22719.89;
		s_data42.m_date = 1608595200.0;
		security_datas.push(s_data42);
		let mut s_data43:SecurityData = SecurityData::new();
		s_data43.m_volume = 112830.021287;
		s_data43.m_close = 23232.77;
		s_data43.m_high = 24100.01;
		s_data43.m_low = 22810.01;
		s_data43.m_open = 23810.8;
		s_data43.m_date = 1608681600.0;
		security_datas.push(s_data43);
		let mut s_data44:SecurityData = SecurityData::new();
		s_data44.m_volume = 69013.844252;
		s_data44.m_close = 23729.21;
		s_data44.m_high = 23794.44;
		s_data44.m_low = 22703.43;
		s_data44.m_open = 23232.4;
		s_data44.m_date = 1608768000.0;
		security_datas.push(s_data44);
		let mut s_data45:SecurityData = SecurityData::new();
		s_data45.m_volume = 78024.251646;
		s_data45.m_close = 24712.48;
		s_data45.m_high = 24789.87;
		s_data45.m_low = 23434.74;
		s_data45.m_open = 23729.0;
		s_data45.m_date = 1608854400.0;
		security_datas.push(s_data45);
		let mut s_data46:SecurityData = SecurityData::new();
		s_data46.m_volume = 97806.523386;
		s_data46.m_close = 26493.4;
		s_data46.m_high = 26867.04;
		s_data46.m_low = 24500.01;
		s_data46.m_open = 24712.48;
		s_data46.m_date = 1608940800.0;
		security_datas.push(s_data46);
		let mut s_data47:SecurityData = SecurityData::new();
		s_data47.m_volume = 145880.8092;
		s_data47.m_close = 26281.67;
		s_data47.m_high = 28422.01;
		s_data47.m_low = 25700.01;
		s_data47.m_open = 26493.41;
		s_data47.m_date = 1609027200.0;
		security_datas.push(s_data47);
		let mut s_data48:SecurityData = SecurityData::new();
		s_data48.m_volume = 79408.740817;
		s_data48.m_close = 27079.42;
		s_data48.m_high = 27500.01;
		s_data48.m_low = 26101.01;
		s_data48.m_open = 26281.55;
		s_data48.m_date = 1609113600.0;
		security_datas.push(s_data48);
		let mut s_data49:SecurityData = SecurityData::new();
		s_data49.m_volume = 69255.341092;
		s_data49.m_close = 27385.01;
		s_data49.m_high = 27410.01;
		s_data49.m_low = 25880.01;
		s_data49.m_open = 27079.43;
		s_data49.m_date = 1609200000.0;
		security_datas.push(s_data49);
		let mut s_data50:SecurityData = SecurityData::new();
		s_data50.m_volume = 95356.067826;
		s_data50.m_close = 28875.55;
		s_data50.m_high = 28996.01;
		s_data50.m_low = 27320.01;
		s_data50.m_open = 27385.01;
		s_data50.m_date = 1609286400.0;
		security_datas.push(s_data50);
		let mut s_data51:SecurityData = SecurityData::new();
		s_data51.m_volume = 75491.419522;
		s_data51.m_close = 28923.64;
		s_data51.m_high = 29300.01;
		s_data51.m_low = 27850.01;
		s_data51.m_open = 28875.56;
		s_data51.m_date = 1609372800.0;
		security_datas.push(s_data51);
		let mut s_data52:SecurityData = SecurityData::new();
		s_data52.m_volume = 54134.113624;
		s_data52.m_close = 29331.7;
		s_data52.m_high = 29600.01;
		s_data52.m_low = 28624.58;
		s_data52.m_open = 28923.64;
		s_data52.m_date = 1609459200.0;
		security_datas.push(s_data52);
		let mut s_data53:SecurityData = SecurityData::new();
		s_data53.m_volume = 126100.088124;
		s_data53.m_close = 32178.34;
		s_data53.m_high = 33300.01;
		s_data53.m_low = 28946.54;
		s_data53.m_open = 29331.71;
		s_data53.m_date = 1609545600.0;
		security_datas.push(s_data53);
		let mut s_data54:SecurityData = SecurityData::new();
		s_data54.m_volume = 110771.806254;
		s_data54.m_close = 33000.06;
		s_data54.m_high = 34778.12;
		s_data54.m_low = 31963.0;
		s_data54.m_open = 32176.46;
		s_data54.m_date = 1609632000.0;
		security_datas.push(s_data54);
		let mut s_data55:SecurityData = SecurityData::new();
		s_data55.m_volume = 121030.155465;
		s_data55.m_close = 31988.72;
		s_data55.m_high = 33600.01;
		s_data55.m_low = 28130.01;
		s_data55.m_open = 33000.06;
		s_data55.m_date = 1609718400.0;
		security_datas.push(s_data55);
		let mut s_data56:SecurityData = SecurityData::new();
		s_data56.m_volume = 111890.475678;
		s_data56.m_close = 33949.54;
		s_data56.m_high = 34360.01;
		s_data56.m_low = 29900.01;
		s_data56.m_open = 31989.76;
		s_data56.m_date = 1609804800.0;
		security_datas.push(s_data56);
		let mut s_data57:SecurityData = SecurityData::new();
		s_data57.m_volume = 116093.037717;
		s_data57.m_close = 36769.37;
		s_data57.m_high = 36939.22;
		s_data57.m_low = 33288.01;
		s_data57.m_open = 33949.54;
		s_data57.m_date = 1609891200.0;
		security_datas.push(s_data57);
		let mut s_data58:SecurityData = SecurityData::new();
		s_data58.m_volume = 121506.449096;
		s_data58.m_close = 39432.29;
		s_data58.m_high = 40365.01;
		s_data58.m_low = 36300.01;
		s_data58.m_open = 36769.37;
		s_data58.m_date = 1609977600.0;
		security_datas.push(s_data58);
		let mut s_data59:SecurityData = SecurityData::new();
		s_data59.m_volume = 138625.042444;
		s_data59.m_close = 40582.82;
		s_data59.m_high = 41950.01;
		s_data59.m_low = 36500.01;
		s_data59.m_open = 39432.49;
		s_data59.m_date = 1610064000.0;
		security_datas.push(s_data59);
		let mut s_data60:SecurityData = SecurityData::new();
		s_data60.m_volume = 75785.989675;
		s_data60.m_close = 40088.23;
		s_data60.m_high = 41380.01;
		s_data60.m_low = 38720.01;
		s_data60.m_open = 40586.97;
		s_data60.m_date = 1610150400.0;
		security_datas.push(s_data60);
		let mut s_data61:SecurityData = SecurityData::new();
		s_data61.m_volume = 112638.990321;
		s_data61.m_close = 38150.03;
		s_data61.m_high = 41350.01;
		s_data61.m_low = 35111.12;
		s_data61.m_open = 40088.23;
		s_data61.m_date = 1610236800.0;
		security_datas.push(s_data61);
		let mut s_data62:SecurityData = SecurityData::new();
		s_data62.m_volume = 231175.583454;
		s_data62.m_close = 35404.48;
		s_data62.m_high = 38264.75;
		s_data62.m_low = 30420.01;
		s_data62.m_open = 38150.03;
		s_data62.m_date = 1610323200.0;
		security_datas.push(s_data62);
		let mut s_data63:SecurityData = SecurityData::new();
		s_data63.m_volume = 133948.161996;
		s_data63.m_close = 34051.25;
		s_data63.m_high = 36628.01;
		s_data63.m_low = 32531.01;
		s_data63.m_open = 35410.38;
		s_data63.m_date = 1610409600.0;
		security_datas.push(s_data63);
		let mut s_data64:SecurityData = SecurityData::new();
		s_data64.m_volume = 124477.924938;
		s_data64.m_close = 37371.39;
		s_data64.m_high = 37850.01;
		s_data64.m_low = 32380.01;
		s_data64.m_open = 34049.16;
		s_data64.m_date = 1610496000.0;
		security_datas.push(s_data64);
		let mut s_data65:SecurityData = SecurityData::new();
		s_data65.m_volume = 102950.399421;
		s_data65.m_close = 39144.51;
		s_data65.m_high = 40100.01;
		s_data65.m_low = 36701.24;
		s_data65.m_open = 37371.39;
		s_data65.m_date = 1610582400.0;
		security_datas.push(s_data65);
		let mut s_data66:SecurityData = SecurityData::new();
		s_data66.m_volume = 111365.804668;
		s_data66.m_close = 36742.23;
		s_data66.m_high = 39747.77;
		s_data66.m_low = 34538.89;
		s_data66.m_open = 39145.22;
		s_data66.m_date = 1610668800.0;
		security_datas.push(s_data66);
		let mut s_data67:SecurityData = SecurityData::new();
		s_data67.m_volume = 86348.441508;
		s_data67.m_close = 35994.99;
		s_data67.m_high = 37950.01;
		s_data67.m_low = 35357.81;
		s_data67.m_open = 36737.44;
		s_data67.m_date = 1610755200.0;
		security_datas.push(s_data67);
		let mut s_data68:SecurityData = SecurityData::new();
		s_data68.m_volume = 80157.737384;
		s_data68.m_close = 35828.62;
		s_data68.m_high = 36852.51;
		s_data68.m_low = 33850.01;
		s_data68.m_open = 35994.99;
		s_data68.m_date = 1610841600.0;
		security_datas.push(s_data68);
		let mut s_data69:SecurityData = SecurityData::new();
		s_data69.m_volume = 70693.90404;
		s_data69.m_close = 36631.28;
		s_data69.m_high = 37469.84;
		s_data69.m_low = 34800.01;
		s_data69.m_open = 35825.0;
		s_data69.m_date = 1610928000.0;
		security_datas.push(s_data69);
		let mut s_data70:SecurityData = SecurityData::new();
		s_data70.m_volume = 79596.541309;
		s_data70.m_close = 35891.5;
		s_data70.m_high = 37850.01;
		s_data70.m_low = 35844.07;
		s_data70.m_open = 36622.47;
		s_data70.m_date = 1611014400.0;
		security_datas.push(s_data70);
		let mut s_data71:SecurityData = SecurityData::new();
		s_data71.m_volume = 89368.432918;
		s_data71.m_close = 35468.24;
		s_data71.m_high = 36415.32;
		s_data71.m_low = 33400.01;
		s_data71.m_open = 35901.95;
		s_data71.m_date = 1611100800.0;
		security_datas.push(s_data71);
		let mut s_data72:SecurityData = SecurityData::new();
		s_data72.m_volume = 134548.811336;
		s_data72.m_close = 30850.14;
		s_data72.m_high = 35600.01;
		s_data72.m_low = 30071.01;
		s_data72.m_open = 35468.24;
		s_data72.m_date = 1611187200.0;
		security_datas.push(s_data72);
		let mut s_data73:SecurityData = SecurityData::new();
		s_data73.m_volume = 138345.853436;
		s_data73.m_close = 32945.18;
		s_data73.m_high = 33826.54;
		s_data73.m_low = 28850.01;
		s_data73.m_open = 30852.0;
		s_data73.m_date = 1611273600.0;
		security_datas.push(s_data73);
		let mut s_data74:SecurityData = SecurityData::new();
		s_data74.m_volume = 64595.297675;
		s_data74.m_close = 32078.01;
		s_data74.m_high = 33456.01;
		s_data74.m_low = 31390.17;
		s_data74.m_open = 32950.01;
		s_data74.m_date = 1611360000.0;
		security_datas.push(s_data74);
		let mut s_data75:SecurityData = SecurityData::new();
		s_data75.m_volume = 57978.047966;
		s_data75.m_close = 32259.91;
		s_data75.m_high = 33071.01;
		s_data75.m_low = 30900.01;
		s_data75.m_open = 32078.01;
		s_data75.m_date = 1611446400.0;
		security_datas.push(s_data75);
		let mut s_data76:SecurityData = SecurityData::new();
		s_data76.m_volume = 88499.236921;
		s_data76.m_close = 32254.21;
		s_data76.m_high = 34875.01;
		s_data76.m_low = 31910.01;
		s_data76.m_open = 32259.46;
		s_data76.m_date = 1611532800.0;
		security_datas.push(s_data76);
		let mut s_data77:SecurityData = SecurityData::new();
		s_data77.m_volume = 84866.207055;
		s_data77.m_close = 32467.78;
		s_data77.m_high = 32921.89;
		s_data77.m_low = 30837.38;
		s_data77.m_open = 32254.2;
		s_data77.m_date = 1611619200.0;
		security_datas.push(s_data77);
		let mut s_data78:SecurityData = SecurityData::new();
		s_data78.m_volume = 95911.971711;
		s_data78.m_close = 30366.16;
		s_data78.m_high = 32557.3;
		s_data78.m_low = 29241.73;
		s_data78.m_open = 32464.02;
		s_data78.m_date = 1611705600.0;
		security_datas.push(s_data78);
		let mut s_data79:SecurityData = SecurityData::new();
		s_data79.m_volume = 92621.155617;
		s_data79.m_close = 33364.87;
		s_data79.m_high = 33783.99;
		s_data79.m_low = 29842.11;
		s_data79.m_open = 30362.2;
		s_data79.m_date = 1611792000.0;
		security_datas.push(s_data79);
		let mut s_data80:SecurityData = SecurityData::new();
		s_data80.m_volume = 193388.622446;
		s_data80.m_close = 34252.21;
		s_data80.m_high = 38531.91;
		s_data80.m_low = 31915.41;
		s_data80.m_open = 33368.19;
		s_data80.m_date = 1611878400.0;
		security_datas.push(s_data80);
		let mut s_data81:SecurityData = SecurityData::new();
		s_data81.m_volume = 82674.758249;
		s_data81.m_close = 34262.89;
		s_data81.m_high = 34933.01;
		s_data81.m_low = 32825.01;
		s_data81.m_open = 34246.29;
		s_data81.m_date = 1611964800.0;
		security_datas.push(s_data81);
		let mut s_data82:SecurityData = SecurityData::new();
		s_data82.m_volume = 66269.928016;
		s_data82.m_close = 33092.99;
		s_data82.m_high = 34342.7;
		s_data82.m_low = 32171.68;
		s_data82.m_open = 34262.9;
		s_data82.m_date = 1612051200.0;
		security_datas.push(s_data82);
		let mut s_data83:SecurityData = SecurityData::new();
		s_data83.m_volume = 80154.567802;
		s_data83.m_close = 33526.38;
		s_data83.m_high = 34717.28;
		s_data83.m_low = 32296.17;
		s_data83.m_open = 33089.45;
		s_data83.m_date = 1612137600.0;
		security_datas.push(s_data83);
		let mut s_data84:SecurityData = SecurityData::new();
		s_data84.m_volume = 78056.66988;
		s_data84.m_close = 35466.25;
		s_data84.m_high = 35984.34;
		s_data84.m_low = 33418.01;
		s_data84.m_open = 33517.1;
		s_data84.m_date = 1612224000.0;
		security_datas.push(s_data84);
		let mut s_data85:SecurityData = SecurityData::new();
		s_data85.m_volume = 80784.343663;
		s_data85.m_close = 37618.88;
		s_data85.m_high = 37662.64;
		s_data85.m_low = 35362.39;
		s_data85.m_open = 35472.72;
		s_data85.m_date = 1612310400.0;
		security_datas.push(s_data85);
		let mut s_data86:SecurityData = SecurityData::new();
		s_data86.m_volume = 89024.320756;
		s_data86.m_close = 36936.67;
		s_data86.m_high = 38708.28;
		s_data86.m_low = 36161.96;
		s_data86.m_open = 37620.27;
		s_data86.m_date = 1612396800.0;
		security_datas.push(s_data86);
		let mut s_data87:SecurityData = SecurityData::new();
		s_data87.m_volume = 65910.531514;
		s_data87.m_close = 38290.25;
		s_data87.m_high = 38310.13;
		s_data87.m_low = 36570.01;
		s_data87.m_open = 36936.66;
		s_data87.m_date = 1612483200.0;
		security_datas.push(s_data87);
		let mut s_data88:SecurityData = SecurityData::new();
		s_data88.m_volume = 94232.612846;
		s_data88.m_close = 39186.95;
		s_data88.m_high = 40955.52;
		s_data88.m_low = 38215.95;
		s_data88.m_open = 38289.33;
		s_data88.m_date = 1612569600.0;
		security_datas.push(s_data88);
		let mut s_data89:SecurityData = SecurityData::new();
		s_data89.m_volume = 81820.439177;
		s_data89.m_close = 38795.7;
		s_data89.m_high = 39700.01;
		s_data89.m_low = 37351.01;
		s_data89.m_open = 39181.02;
		s_data89.m_date = 1612656000.0;
		security_datas.push(s_data89);
		let mut s_data90:SecurityData = SecurityData::new();
		s_data90.m_volume = 121568.794672;
		s_data90.m_close = 46374.88;
		s_data90.m_high = 46794.46;
		s_data90.m_low = 37988.9;
		s_data90.m_open = 38795.7;
		s_data90.m_date = 1612742400.0;
		security_datas.push(s_data90);
		let mut s_data91:SecurityData = SecurityData::new();
		s_data91.m_volume = 114567.009318;
		s_data91.m_close = 46420.43;
		s_data91.m_high = 48142.2;
		s_data91.m_low = 44961.1;
		s_data91.m_open = 46374.87;
		s_data91.m_date = 1612828800.0;
		security_datas.push(s_data91);
		let mut s_data92:SecurityData = SecurityData::new();
		s_data92.m_volume = 78292.89657;
		s_data92.m_close = 44807.59;
		s_data92.m_high = 47310.01;
		s_data92.m_low = 43800.01;
		s_data92.m_open = 46420.43;
		s_data92.m_date = 1612915200.0;
		security_datas.push(s_data92);
		let mut s_data93:SecurityData = SecurityData::new();
		s_data93.m_volume = 81033.184776;
		s_data93.m_close = 47969.52;
		s_data93.m_high = 48678.91;
		s_data93.m_low = 43994.03;
		s_data93.m_open = 44807.59;
		s_data93.m_date = 1613001600.0;
		security_datas.push(s_data93);
		let mut s_data94:SecurityData = SecurityData::new();
		s_data94.m_volume = 81856.490636;
		s_data94.m_close = 47287.61;
		s_data94.m_high = 48985.81;
		s_data94.m_low = 46400.01;
		s_data94.m_open = 47968.67;
		s_data94.m_date = 1613088000.0;
		security_datas.push(s_data94);
		let mut s_data95:SecurityData = SecurityData::new();
		s_data95.m_volume = 58937.84066;
		s_data95.m_close = 47153.7;
		s_data95.m_high = 48150.01;
		s_data95.m_low = 46202.54;
		s_data95.m_open = 47298.16;
		s_data95.m_date = 1613174400.0;
		security_datas.push(s_data95);
		let mut s_data96:SecurityData = SecurityData::new();
		s_data96.m_volume = 70700.864117;
		s_data96.m_close = 48577.8;
		s_data96.m_high = 49707.44;
		s_data96.m_low = 47014.18;
		s_data96.m_open = 47156.79;
		s_data96.m_date = 1613260800.0;
		security_datas.push(s_data96);
		let mut s_data97:SecurityData = SecurityData::new();
		s_data97.m_volume = 63227.659425;
		s_data97.m_close = 47911.11;
		s_data97.m_high = 49010.93;
		s_data97.m_low = 46174.79;
		s_data97.m_open = 48580.48;
		s_data97.m_date = 1613347200.0;
		security_datas.push(s_data97);
		let mut s_data98:SecurityData = SecurityData::new();
		s_data98.m_volume = 78018.176005;
		s_data98.m_close = 49133.46;
		s_data98.m_high = 50080.22;
		s_data98.m_low = 47003.63;
		s_data98.m_open = 47911.11;
		s_data98.m_date = 1613433600.0;
		security_datas.push(s_data98);
		let mut s_data99:SecurityData = SecurityData::new();
		s_data99.m_volume = 84886.224046;
		s_data99.m_close = 52119.72;
		s_data99.m_high = 52618.75;
		s_data99.m_low = 48947.01;
		s_data99.m_open = 49133.46;
		s_data99.m_date = 1613520000.0;
		security_datas.push(s_data99);
		let mut s_data100:SecurityData = SecurityData::new();
		s_data100.m_volume = 58093.930825;
		s_data100.m_close = 51552.61;
		s_data100.m_high = 52530.01;
		s_data100.m_low = 50901.91;
		s_data100.m_open = 52117.68;
		s_data100.m_date = 1613606400.0;
		security_datas.push(s_data100);
		
		for i in 0..security_datas.len(){
			let s_data = (&security_datas[i]).clone();
			let mut grid_row:FCGridRow = FCGridRow::new();
			let mut cell1:FCGridCell = FCGridCell::new();
			cell1.m_value = s_data.m_date.to_string();
			cell1.m_back_color = String::from("rgb(0,0,0)");
			cell1.m_text_color = String::from("rgb(255,255,255)");
			cell1.m_border_color =  String::from("rgb(150,150,150)");
			grid_row.m_cells.push(cell1);
			
			let mut cell2:FCGridCell = FCGridCell::new();
			cell2.m_value = s_data.m_open.to_string();
			cell2.m_back_color = String::from("rgb(0,0,0)");
			cell2.m_text_color = String::from("rgb(255,0,0)");
			cell2.m_border_color =  String::from("rgb(150,150,150)");
			grid_row.m_cells.push(cell2);
			
			let mut cell3:FCGridCell = FCGridCell::new();
			cell3.m_value = s_data.m_high.to_string();
			cell3.m_back_color = String::from("rgb(0,0,0)");
			cell3.m_text_color = String::from("rgb(0,255,0)");
			cell3.m_border_color =  String::from("rgb(150,150,150)");
			grid_row.m_cells.push(cell3);
			
			let mut cell4:FCGridCell = FCGridCell::new();
			cell4.m_value = s_data.m_low.to_string();
			cell4.m_back_color = String::from("rgb(0,0,0)");
			cell4.m_text_color = String::from("rgb(255,255,0)");
			cell4.m_border_color =  String::from("rgb(150,150,150)");
			grid_row.m_cells.push(cell4);
			
			let mut cell5:FCGridCell = FCGridCell::new();
			cell5.m_value = s_data.m_close.to_string();
			cell5.m_back_color = String::from("rgb(0,0,0)");
			cell5.m_text_color = String::from("rgb(255,0,255)");
			cell5.m_border_color =  String::from("rgb(150,150,150)");
			grid_row.m_cells.push(cell5);
			
			let mut cell6:FCGridCell = FCGridCell::new();
			cell6.m_value = s_data.m_volume.to_string();
			cell6.m_back_color = String::from("rgb(0,0,0)");
			cell6.m_text_color = String::from("rgb(0,255,255)");
			cell6.m_border_color =  String::from("rgb(150,150,150)");
			grid_row.m_cells.push(cell6);
			
			grid.m_rows.push(grid_row);
		}
		M_GRID_MAP.lock().unwrap().insert(grid_view.m_id, grid.clone());
		
		let mut chart:FCChart = FCChart::new();
		chart.m_view = chart_view.clone();
		chart.m_data = security_datas;
		calc_chart_indicator(&mut chart);
		M_CHART_MAP.lock().unwrap().insert(chart_view.m_id, chart.clone());
	}else if(rustMode == 1){
		let xml = "".to_string();
		let dom_parse = DomParser::new();
		let s_type = SupportedType::TextXml;
		let xml_doc = dom_parse?.parse_from_string(&xml, s_type);
		let body = xml_doc?.get_elements_by_tag_name("body").item(0);
		let node_list = body.expect("REASON").children();
		let mut none_view = M_NONE_VIEW.lock().unwrap().clone();
		for i in 0..node_list.length(){
			let node = node_list.item(i).expect("REASON");
			let node = Rc::new(node);
			read_xml_node(&node, &mut none_view);
		}
		let top_views2 = get_top_views();
		for i in 0..top_views2.len(){
			let mut t_view = (&top_views2[i]).clone();
			if(t_view.m_dock == "fill"){
				unsafe{
					t_view.m_size = FCSize{cx:M_CANVAS_WIDTH, cy:M_CANVAS_HEIGHT};
				}
				M_VIEW_MAP.lock().unwrap().insert(t_view.m_id, t_view.clone());
			}
		}
		let top_views3 = get_top_views();
		update_views(top_views3);
	}
	invalidate(&context);
	//let utf_vec = vec![228,189,160,229,165,189];
	//let str_utf8 = String::from_utf8(utf_vec).unwrap();
	//log(&str_utf8);
    Ok(())
    //wasm-pack build --release --target web
}

pub fn read_xml_node(element:&std::rc::Rc<web_sys::Element>, parent:&mut FCView){
	let node_name = element.node_name().to_lowercase();
	let mut view = FCView::new();
	if(M_PAINT.lock().unwrap().m_default_ui_style == "dark"){
		view.m_back_color = "rgb(0,0,0)".to_string();
        view.m_border_color = "rgb(100,100,100)".to_string();
        view.m_text_color = "rgb(255,255,255)".to_string();
	}else if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
		view.m_back_color = "rgb(255,255,255)".to_string();
        view.m_border_color = "rgb(150,150,150)".to_string();
        view.m_text_color = "rgb(0,0,0)".to_string();
	}
	if(parent.m_id == -1){
		view.m_id = add_view(view.clone());
	}else{
		view.m_id = add_view_to_parent(view.clone(), parent.clone());
	}
	let node_value = element.node_value();
	let node_attributes = element.attributes();
	for i in 0..node_attributes.length(){
		let attribute = node_attributes.item(i).expect("REASON");
		let atr_name = attribute.name().to_lowercase();
		let atr_value = attribute.value();
		set_view_attribute(&mut view, atr_name.clone(), atr_value.clone());
		if(node_name == "div"){
			if(atr_name == "type"){
				if(atr_value == "splitlayout"){
					view.m_type = "split".to_string();
				}
				else if(atr_value == "layout"){
					view.m_type = "layout".to_string();
					view.m_show_vscrollbar = true;
					view.m_show_hscrollbar = true;
					view.m_allow_drag_scroll = true;
				}
				else if(atr_value == "tab"){
					view.m_type = "tabview".to_string();
				}
				else if(atr_value == "tabpage"){
					view.m_type = "tabpage".to_string();
				}
				else{
					view.m_type = "div".to_string();
					view.m_show_vscrollbar = true;
					view.m_show_hscrollbar = true;
					view.m_allow_drag_scroll = true;
				}
			}
		}else if(node_name == "input"){
			if(atr_name == "type"){
				if(atr_value == "radio"){
					view.m_type = "radiobutton".to_string();
				}else if(atr_value == "checkbox"){
					view.m_type = "checkbox".to_string();
				}else if(atr_value == "button"){
					view.m_type = "button".to_string();
				}
			}
		}
	}
	if(node_name == "chart"){
		view.m_type = "chart".to_string();
	}else if(node_name == "label"){
		view.m_type = "label".to_string();
	}else if(node_name == "table"){
		view.m_type = "grid".to_string();
		view.m_show_vscrollbar = true;
		view.m_show_hscrollbar = true;
		view.m_allow_drag_scroll = true;
	}
	M_VIEW_MAP.lock().unwrap().insert(view.m_id, view.clone());
	if(view.m_type == "split"){
		let mut split:FCSplitLayoutDiv = FCSplitLayoutDiv::new();
		let mut splitterposition : String = String::from("");
		for i in 0..node_attributes.length(){
			let attribute = node_attributes.item(i).expect("REASON");
			let atr_name = attribute.name().to_lowercase();
			let atr_value = attribute.value();
			set_split_attribute(&mut split, atr_name.clone(), atr_value.clone());
			if(atr_name == "datumsize"){
				let str_split:Vec<&str> = atr_value.split(",").collect();
				let cx : f32 = str_split[0].parse::<f32>().unwrap();
				let cy : f32 =  str_split[1].parse::<f32>().unwrap();
				view.m_size = FCSize{cx:cx, cy:cy};	
			}else if(atr_name == "splitterposition"){
				splitterposition = atr_value;
			}
		}
		M_VIEW_MAP.lock().unwrap().insert(view.m_id, view.clone());
		let mut splitter:FCView = FCView::new();
		if(M_PAINT.lock().unwrap().m_default_ui_style == "dark"){
			splitter.m_back_color = "rgb(100,100,100)".to_string();
		}else if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
			splitter.m_back_color = "rgb(150,150,150)".to_string();
		}
		
		let str_split:Vec<&str> = splitterposition.split(",").collect();
		if(str_split.len() >= 4){
			let left : f32 = str_split[0].parse::<f32>().unwrap();
			let top : f32 =  str_split[1].parse::<f32>().unwrap();
			let right : f32 = str_split[2].parse::<f32>().unwrap();
			let bottom : f32 =  str_split[3].parse::<f32>().unwrap();
			splitter.m_location = FCPoint{x:left, y:top};
			splitter.m_size = FCSize{cx:right - left + 1.0, cy:bottom - top + 1.0};
		}else{
			let s_position : f32 = str_split[0].parse::<f32>().unwrap();
			let s_size : f32 =  str_split[1].parse::<f32>().unwrap();
			if(split.m_layout_style == "lefttoright" || split.m_layout_style == "righttoleft"){
                splitter.m_location = FCPoint{x:s_position, y:0.0};
                splitter.m_size = FCSize{cx:s_size, cy:view.m_size.cy};
            }else{
                splitter.m_location = FCPoint{x:0.0, y:s_position};
                splitter.m_size = FCSize{cx:view.m_size.cx, cy:s_size};
            }
		}
		let child_elements = element.children();
		for i in 0..child_elements.length(){
			let sub_node = child_elements.item(i).expect("REASON");
			let sub_node = Rc::new(sub_node);
			read_xml_node(&sub_node, &mut view);
		}
		let sub_views = get_sub_views(view.clone());
		let first_view = (&sub_views[0]).clone();
		let second_view = (&sub_views[1]).clone();
		if(first_view.m_id >= second_view.m_id){
			split.m_first_view = second_view;
			split.m_second_view = first_view;
		}else{
			split.m_first_view = first_view;
			split.m_second_view = second_view;
		}
		splitter.m_id = add_view_to_parent(splitter.clone(), view.clone());
		split.m_splitter = splitter.clone();
		split.m_view = view.clone();
		split.m_old_size = view.m_size.clone();
		reset_split_layout_div(&mut split);
		M_SPLIT_MAP.lock().unwrap().insert(view.m_id, split.clone());
	}
	else if(view.m_type == "chart"){
		let mut chart:FCChart = FCChart::new();
		chart.m_view = view.clone();
		M_CHART_MAP.lock().unwrap().insert(view.m_id, chart.clone());
	}
	else if(view.m_type == "checkbox"){
		let mut check_box:FCCheckBox = FCCheckBox::new();
		for i in 0..node_attributes.length(){
			let attribute = node_attributes.item(i).expect("REASON");
			let atr_name = attribute.name().to_lowercase();
			let atr_value = attribute.value();
			set_check_box_attribute(&mut check_box, atr_name.clone(), atr_value.clone());
		}
		check_box.m_view = view.clone();
		M_CHECK_BOX_MAP.lock().unwrap().insert(view.m_id, check_box.clone());
	}
	else if(view.m_type == "radiobutton"){
		let mut radio_button:FCRadioButton = FCRadioButton::new();
		for i in 0..node_attributes.length(){
			let attribute = node_attributes.item(i).expect("REASON");
			let atr_name = attribute.name().to_lowercase();
			let atr_value = attribute.value();
			set_radio_button_attribute(&mut radio_button, atr_name.clone(), atr_value.clone());
		}
		radio_button.m_view = view.clone();
		M_RADIO_BUTTON_MAP.lock().unwrap().insert(view.m_id, radio_button.clone());
	}
	else if(view.m_type == "grid"){
		let mut grid:FCGrid = FCGrid::new();
		for i in 0..node_attributes.length(){
			let attribute = node_attributes.item(i).expect("REASON");
			let atr_name = attribute.name().to_lowercase();
			let atr_value = attribute.value();
			set_grid_attribute(&mut grid, atr_name.clone(), atr_value.clone());
		}
		grid.m_view = view.clone();
		let child_elements = element.children();
		for i in 0..child_elements.length(){
			let sub_node = child_elements.item(i).expect("REASON");
			let sub_node_name = sub_node.node_name().to_lowercase();
			if(sub_node_name == "tr"){
				let sun_elements = sub_node.children();
				for j in 0..sun_elements.length(){
					let sun_node = sun_elements.item(j).expect("REASON");
					let sun_node_name = sun_node.node_name().to_lowercase();
					if(sun_node_name == "th"){
						let mut grid_column:FCGridColumn = FCGridColumn::new();
						if(M_PAINT.lock().unwrap().m_default_ui_style == "dark"){
							grid_column.m_back_color = "rgb(0,0,0)".to_string();
							grid_column.m_border_color = "rgb(100,100,100)".to_string();
							grid_column.m_text_color = "rgb(255,255,255)".to_string();
						}else if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
							grid_column.m_back_color = "rgb(255,255,255)".to_string();
							grid_column.m_border_color = "rgb(150,150,150)".to_string();
							grid_column.m_text_color = "rgb(0,0,0)".to_string();
						}
						let sun_node_attributes = sun_node.attributes();
						for k in 0..sun_node_attributes.length(){
							let sun_attr = sun_node_attributes.item(k).expect("REASON");
							let sun_atr_name = sun_attr.name().to_lowercase();
							let sun_atr_value = sun_attr.value();
							if(sun_atr_name == "text"){
								grid_column.m_text = sun_atr_value;
								break;
							}
						}
						grid.m_columns.push(grid_column);
					}
				}
			}
		}
		M_GRID_MAP.lock().unwrap().insert(view.m_id, grid.clone());
	}
	else{
		if(view.m_type == "layout"){
			let mut layout:FCLayoutDiv = FCLayoutDiv::new();
			for i in 0..node_attributes.length(){
				let attribute = node_attributes.item(i).expect("REASON");
				let atr_name = attribute.name().to_lowercase();
				let atr_value = attribute.value();
				set_layout_attribute(&mut layout, atr_name.clone(), atr_value.clone());
			}
			layout.m_view = view.clone();
			M_LAYOUT_MAP.lock().unwrap().insert(view.m_id, layout.clone());
		}
		else if(view.m_type == "tabview"){
			let mut tab:FCTabView = FCTabView::new();
			tab.m_view = view.clone();
			M_TAB_MAP.lock().unwrap().insert(view.m_id, tab.clone());
		}else if(view.m_type == "tabpage"){
			let mut tab = M_TAB_MAP.lock().unwrap()[&parent.m_id].clone();
			let mut header_button_view:FCView = FCView::new();
			header_button_view.m_size = FCSize{cx:100.0, cy:20.0};
			header_button_view.m_type = "headerbutton".to_string();
			for i in 0..node_attributes.length(){
				let attribute = node_attributes.item(i).expect("REASON");
				let atr_name = attribute.name().to_lowercase();
				let atr_value = attribute.value();
				if(atr_name == "text"){
					header_button_view.m_text = atr_value;
				}else if(atr_name == "headersize"){
					let str_split:Vec<&str> = atr_value.split(",").collect();
					let cx : f32 = str_split[0].parse::<f32>().unwrap();
					let cy : f32 =  str_split[1].parse::<f32>().unwrap();
					header_button_view.m_size = FCSize{cx:cx, cy:cy};
				}
			}
			if(M_PAINT.lock().unwrap().m_default_ui_style == "dark"){
				header_button_view.m_back_color = "rgb(0,0,0)".to_string();
				header_button_view.m_border_color = "rgb(100,100,100)".to_string();
				header_button_view.m_text_color = "rgb(255,255,255)".to_string();
			}else if(M_PAINT.lock().unwrap().m_default_ui_style == "light"){
				header_button_view.m_back_color = "rgb(255,255,255)".to_string();
				header_button_view.m_border_color = "rgb(150,150,150)".to_string();
				header_button_view.m_text_color = "rgb(0,0,0)".to_string();
			}
			header_button_view.m_id = add_view_to_parent(header_button_view.clone(), parent.clone());
			if(tab.m_tab_pages.len() > 0){
				view.m_visible = false;
				M_VIEW_MAP.lock().unwrap().insert(view.m_id, view.clone());
			}
			let mut tab_page:FCTabPage = FCTabPage::new();
			tab_page.m_view = view.clone();
			tab_page.m_header_button = header_button_view.clone();
			tab.m_tab_pages.push(tab_page);
			M_TAB_MAP.lock().unwrap().insert(tab.m_view.m_id, tab.clone());
		}
		let child_elements = element.children();
		for i in 0..child_elements.length(){
			let sub_node = child_elements.item(i).expect("REASON");
			let sub_node = Rc::new(sub_node);
			read_xml_node(&sub_node, &mut view);
		}
	}
}

pub fn set_grid_attribute(grid:&mut FCGrid, name:String, value:String){
	if(name == "headerheight"){
		grid.m_header_height = value.parse::<f32>().unwrap();
	}
}

pub fn set_check_box_attribute(check_box:&mut FCCheckBox, name:String, value:String){
	if(name == "checked"){
		if(value == "true"){
			check_box.m_checked = true;
		}else{
			check_box.m_checked = false;
		}
	}else if(name == "buttonsize"){
		let str_split:Vec<&str> = value.split(",").collect();
		let cx : f32 = str_split[0].parse::<f32>().unwrap();
		let cy : f32 =  str_split[1].parse::<f32>().unwrap();
		check_box.m_button_size = FCSize{cx:cx, cy:cy};
	}
}

pub fn set_radio_button_attribute(radio_button:&mut FCRadioButton, name:String, value:String){
	if(name == "checked"){
		if(value == "true"){
			radio_button.m_checked = true;
		}else{
			radio_button.m_checked = false;
		}
	}else if(name == "buttonsize"){
		let str_split:Vec<&str> = value.split(",").collect();
		let cx : f32 = str_split[0].parse::<f32>().unwrap();
		let cy : f32 =  str_split[1].parse::<f32>().unwrap();
		radio_button.m_button_size = FCSize{cx:cx, cy:cy};
	}
}

pub fn set_split_attribute(split:&mut FCSplitLayoutDiv, name:String, value:String){
	if(name == "layoutstyle"){
		split.m_layout_style = value.to_lowercase();
	}else if(name == "splitmode"){
		split.m_split_mode = value;
	}
}

pub fn set_layout_attribute(layout:&mut FCLayoutDiv, name:String, value:String){
	if(name == "layoutstyle"){
		layout.m_layout_style = value.to_lowercase();
	}else if(name == "autowrap"){
		if(value == "true"){
			layout.m_auto_wrap = true;
		}else{
			layout.m_auto_wrap = false;
		}
	}
}

pub fn set_view_attribute(view:&mut FCView, name:String, value:String){
	if(name == "location"){
		let str_split:Vec<&str> = value.split(",").collect();
		let x : f32 = str_split[0].parse::<f32>().unwrap();
		let y : f32 =  str_split[1].parse::<f32>().unwrap();
		view.m_location = FCPoint{x:x, y:y};
	}
	else if(name == "size"){
		let str_split:Vec<&str> = value.split(",").collect();
		let cx : f32 = str_split[0].parse::<f32>().unwrap();
		let cy : f32 =  str_split[1].parse::<f32>().unwrap();
		view.m_size = FCSize{cx:cx, cy:cy};
	}else if(name == "text"){
		view.m_text = value;
	}else if(name == "backcolor"){ 
		if(value.find("rgb") == Some(0)){
			view.m_back_color = value;
		}else{
			view.m_back_color = "none".to_string();
		}
	}else if(name == "bordercolor"){
		if(value.find("rgb") == Some(0)){
			view.m_border_color = value;
		}else{
			view.m_border_color = "none".to_string();
		}
	}else if(name == "textcolor"){
		if(value.find("rgb") == Some(0)){
			view.m_text_color = value;
		}else{
			view.m_text_color = "none".to_string();
		}
	}else if(name == "dock"){
		view.m_dock = value.to_lowercase();
	}else if(name == "font"){
		view.m_font = value.replace("Default", "Arial");
	}else if(name == "name"){
		view.m_name = value;
	}else if(name == "showvscrollbar"){
		if(value == "true"){
			view.m_show_vscrollbar = true;
		}else{
			view.m_show_vscrollbar = false;
		}
	}else if(name == "showhscrollbar"){
		if(value == "true"){
			view.m_show_hscrollbar = true;
		}else{
			view.m_show_hscrollbar = false;
		}
	}else if(name == "visible"){
		if(value == "true"){
			view.m_visible = true;
		}else{
			view.m_visible = false;
		}
	}else if(name == "displayoffset"){
		if(value == "true"){
			view.m_display_offset = true;
		}else{
			view.m_display_offset = false;
		}
	}else if(name == "topmost"){
		if(value == "true"){
			view.m_top_most = true;
		}else{
			view.m_top_most = false;
		}
	}
}

pub fn update_views(views:Vec<FCView>){
	unsafe{
		let views_size = views.len();
		 for i in 0..views_size{
			let mut view = (&views[i]).clone();
			let mut find:bool = false;
			let mut p_id:i32 = 0;
			match M_PARENT_VIEW_MAP.lock().unwrap().get(&view.m_id) {
				Some(x) => {
					find = true;
					p_id = *x;	
				},
				None => {
					find = false;
				}
			}
			if(find && view.m_dock == "fill"){
				let parent = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
				if(parent.m_type != "split"){
					view.m_location = FCPoint{x:0.0, y:0.0};
					view.m_size = FCSize{cx:parent.m_size.cx, cy:parent.m_size.cy};
				}
			}
			M_VIEW_MAP.lock().unwrap().insert(view.m_id, view.clone());
			if(view.m_type == "split"){
				for (id, v) in M_SPLIT_MAP.lock().unwrap().iter_mut(){
					if(view.m_id == *id){
						(*v).m_view = view.clone();
						reset_split_layout_div(&mut *v);
					}
				}
            }else if(view.m_type == "tabview"){
				for (id, v) in M_TAB_MAP.lock().unwrap().iter_mut(){
					if(view.m_id == *id){
						(*v).m_view = view.clone();
						update_tab_layout(&mut *v);
					}
				}
            }else if(view.m_type == "layout"){
				for (id, v) in M_LAYOUT_MAP.lock().unwrap().iter_mut(){
					if(view.m_id == *id){
						(*v).m_view = view.clone();
						reset_layout_div(&mut *v);
					}
				}
            }
			let sub_views = get_sub_views(view.clone());
			let sub_views_size = sub_views.len();
			if(sub_views_size > 0) {
				update_views(sub_views);
			}
		 }
	 }
}