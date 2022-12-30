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
include!("facecat.rs");
include!("grid.rs");
include!("div.rs");
include!("tab.rs");
include!("layout.rs");
include!("split.rs");
include!("btn.rs");
include!("chart.rs");
include!("input.rs");

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
extern {
	#[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub fn on_paint(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, clip_rect:FCRect){
	if view.m_type == "chart"{
		M_PAINT.lock().unwrap().fill_rect(&context, view.m_back_color.clone(), 0.0, 0.0, view.m_size.cx, view.m_size.cy);
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
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
	}else if view.m_type == "grid"{
		M_PAINT.lock().unwrap().fill_rect(&context, view.m_back_color.clone(), 0.0, 0.0, view.m_size.cx, view.m_size.cy);
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				draw_grid(&context, &mut *v, clip_rect.clone());
				break;
			}
		}		
	}
	else if view.m_type == "radiobutton"{
		for (id, v) in M_RADIO_BUTTON_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				draw_radio_button(&context, &mut *v, clip_rect.clone());
				break;
			}
		}		
	}
	else if view.m_type == "checkbox"{
		for (id, v) in M_CHECK_BOX_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				draw_check_box(&context, &mut *v, clip_rect.clone());
				break;
			}
		}	
	}
	else if view.m_type == "tab" || view.m_type == "tabpage" || view.m_type == "layout" || view.m_type == "div"{
		let mut cview = view.clone();
		draw_div(&context, &mut cview, clip_rect.clone());
	}
	else if view.m_type == "label"{
		if view.m_text_color != "none"{
			M_PAINT.lock().unwrap().draw_text(&context, view.m_text.clone(), view.m_text_color.clone(), view.m_font.clone(), 1.0, view.m_size.cy / 2.0 + 1.0);
		}
	}
	else{
		let mut cview = view.clone();
		draw_button(&context, &mut cview, clip_rect.clone());
	}
}

pub fn on_paint_border(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, clip_rect:FCRect){
	if view.m_type == "grid"{
		M_PAINT.lock().unwrap().draw_rect(&context, view.m_border_color.clone(), 1.0, Vec::new(), 0.0, 0.0, view.m_size.cx, view.m_size.cy);
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				draw_grid_scroll_bar(&context, &mut *v, clip_rect.clone());
				break;
			}
		}
		
	}else if view.m_type == "div" || view.m_type == "layout"{
		let mut div = view.clone();
		draw_div_border(&context, &mut div, clip_rect.clone());
		draw_div_scroll_bar(&context, &mut div, clip_rect.clone());
	}else if view.m_type == "tab" || view.m_type == "tabpage"{
		let mut cview = view.clone();
		draw_div_border(&context, &mut cview, clip_rect.clone());
	}
}

pub fn on_mouse_down(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, mp:FCPoint, buttons:i32, clicks:i32, delta:i32){
	if view.m_type == "chart"{
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if buttons == 1{
			first_touch = true;
		}
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				(*v).m_mouse_position = first_point.clone();
				(*v).m_mouse_down_position = first_point.clone();
				(*v).m_cross_stop_index = get_chart_index(&mut *v, first_point.clone());
				unsafe{
					if M_ADDING_PLOT != -1{
						if first_point.y < get_candle_div_height(&mut *v){
							let touch_index = get_chart_index(&mut *v, first_point.clone());
							if touch_index >= (*v).m_first_visible_index && touch_index <= (*v).m_last_visible_index{
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
								if str_plot == "FiboTimezone"{
									let f_index = touch_index;
									let f_date = get_chart_date_by_index(&mut *v, f_index);
									let y = get_candle_div_value(&mut *v, first_point.clone());
									let mut new_plot:FCPlot = FCPlot::new();
									new_plot.m_id = create_new_id();
									if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
										new_plot.m_line_color = "rgb(0,0,0)".to_string();
										new_plot.m_point_color = "rgba(0,0,0,0.5)".to_string();
									}
									new_plot.m_key1 = f_date;
									new_plot.m_value1 = y;
									new_plot.m_plot_type = str_plot;
									(*v).m_plots.push(new_plot);
									(*v).m_splot = select_plot(&mut *v, first_point.clone());
								}
								else if str_plot == "Triangle" || str_plot == "CircumCycle" || str_plot == "ParalleGram" || str_plot == "AngleLine" || str_plot == "Parallel" || str_plot == "SymmetricTriangle"{
									let e_index = touch_index;
									let b_index = e_index - 5;
									if b_index >= 0 {
										let f_date = get_chart_date_by_index(&mut *v, b_index);
										let s_date = get_chart_date_by_index(&mut *v, e_index);
										let y = get_candle_div_value(&mut *v, first_point.clone());
										let mut new_plot:FCPlot = FCPlot::new();
										new_plot.m_id = create_new_id();
										if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
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
									if b_index >= 0 {
										let f_date = get_chart_date_by_index(&mut *v, b_index);
										let s_date = get_chart_date_by_index(&mut *v, e_index);
										let y = get_candle_div_value(&mut *v, first_point.clone());
										let mut new_plot:FCPlot = FCPlot::new();
										new_plot.m_id = create_new_id();
										if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
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
						if (*v).m_splot.m_id <= 0{
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
	else if view.m_type == "grid"{
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if buttons == 1{
			first_touch = true;
		}
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				mouse_down_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}
	else if view.m_type == "div" || view.m_type == "layout"{
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if buttons == 1{
			first_touch = true;
		}
		let mut div = view.clone();
		mouse_down_div(&mut div, first_touch, second_touch, first_point, second_point);
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
	}
}

pub fn on_mouse_move(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, mp:FCPoint, buttons:i32, clicks:i32, delta:i32){
	if view.m_type == "chart"{
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if buttons == 1{
			first_touch = true;
		}
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				(*v).m_mouse_position = mp.clone();
				(*v).m_cross_stop_index = get_chart_index(&mut *v, mp.clone());
				mouse_move_chart(&mut (*v), first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if view.m_type == "grid"{
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if buttons == 1{
			first_touch = true;
		}
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				mouse_move_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		if buttons == 1{
			invalidate_view(context, view.clone());
		}
	}
	else if view.m_type == "div" || view.m_type == "layout"{
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if buttons == 1{
			first_touch = true;
		}
		let mut div = view.clone();
		mouse_move_div(&mut div, first_touch, second_touch, first_point, second_point);
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		if buttons == 1{
			invalidate_view(context, view.clone());
		}
	}
}

pub fn on_mouse_up(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, mp:FCPoint, buttons:i32, clicks:i32, delta:i32){
	if view.m_type == "chart"{
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				(*v).m_first_touch_index_cache = -1;
				(*v).m_second_touch_index_cache = -1;
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if view.m_type == "grid"{
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if buttons == 1{
			first_touch = true;
		}
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				mouse_up_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}else if view.m_type == "div" || view.m_type == "layout"{
		let mut first_touch:bool = false;
		let second_touch:bool = false;
		let first_point = mp.clone();
		let second_point = mp.clone();
		if buttons == 1 {
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
	if view.m_type == "plot"{
		unsafe{
			M_ADDING_PLOT = view.m_name.parse::<i32>().unwrap();
		}
	}
	else if view.m_type == "indicator"{
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			(*v).m_view = view.clone();
			if view.m_text == "BOLL" || view.m_text == "MA" {
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
	else if view.m_type == "checkbox"{
		for (id, v) in M_CHECK_BOX_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				click_check_box(&mut *v, mp.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}else if view.m_type == "radiobutton"{
		for (id, v) in M_RADIO_BUTTON_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				click_radio_button(&mut *v, mp.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}
	else if view.m_type == "headerbutton"{
		let mut is_this_tab:bool = false;
		for (id, v) in M_TAB_MAP.lock().unwrap().iter_mut(){
			for i in 0..(*v).m_tab_pages.len(){
				let tp = (*v).m_tab_pages[i].clone();
				if tp.m_header_button.m_id == view.m_id{
					is_this_tab = true;
					break;
				}
			}
			if is_this_tab{
				for j in 0..(*v).m_tab_pages.len(){
					let mut tp = (*v).m_tab_pages[j].clone();
					if tp.m_header_button.m_id == view.m_id{
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
	if view.m_type == "chart"{
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				if delta > 0{
					zoom_out_chart(&mut *v);
				}else if delta < 0{
					zoom_in_chart(&mut *v);
				}
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if view.m_type == "grid"{
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				mouse_wheel_grid(&mut *v, delta);
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }
    else if view.m_type == "div" || view.m_type == "layout"{
		let mut div = view.clone();
		mouse_wheel_div(&mut div, delta);
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
    }
}

pub fn on_touch_start(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	if view.m_type == "chart"{
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				(*v).m_mouse_position = first_point.clone();
				(*v).m_mouse_down_position = first_point.clone();
				(*v).m_cross_stop_index = get_chart_index(&mut *v, first_point.clone());
				unsafe{
					if M_ADDING_PLOT != -1{
						if first_point.y < get_candle_div_height(&mut *v){
							let touch_index = get_chart_index(&mut *v, first_point.clone());
							if touch_index >= (*v).m_first_visible_index && touch_index <= (*v).m_last_visible_index{
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
								if str_plot == "FiboTimezone"{
									let f_index = touch_index;
									let f_date = get_chart_date_by_index(&mut *v, f_index);
									let y = get_candle_div_value(&mut *v, first_point.clone());
									let mut new_plot:FCPlot = FCPlot::new();
									new_plot.m_id = create_new_id();
									if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
										new_plot.m_line_color = "rgb(0,0,0)".to_string();
										new_plot.m_point_color = "rgba(0,0,0,0.5)".to_string();
									}
									new_plot.m_key1 = f_date;
									new_plot.m_value1 = y;
									new_plot.m_plot_type = str_plot;
									(*v).m_plots.push(new_plot);
									(*v).m_splot = select_plot(&mut *v, first_point.clone());
								}
								else if str_plot == "Triangle" || str_plot == "CircumCycle" || str_plot == "ParalleGram" || str_plot == "AngleLine" || str_plot == "Parallel" || str_plot == "SymmetricTriangle"{
									let e_index = touch_index;
									let b_index = e_index - 5;
									if b_index >= 0 {
										let f_date = get_chart_date_by_index(&mut *v, b_index);
										let s_date = get_chart_date_by_index(&mut *v, e_index);
										let y = get_candle_div_value(&mut *v, first_point.clone());
										let mut new_plot:FCPlot = FCPlot::new();
										new_plot.m_id = create_new_id();
										if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
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
									if b_index >= 0{
										let f_date = get_chart_date_by_index(&mut *v, b_index);
										let s_date = get_chart_date_by_index(&mut *v, e_index);
										let y = get_candle_div_value(&mut *v, first_point.clone());
										let mut new_plot:FCPlot = FCPlot::new();
										new_plot.m_id = create_new_id();
										if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
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
						if (*v).m_splot.m_id <= 0{
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
	else if view.m_type == "grid"{
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				mouse_down_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}
	else if view.m_type == "div" || view.m_type == "layout"{
		let mut div = view.clone();
		mouse_down_div(&mut div, first_touch, second_touch, first_point.clone(), second_point.clone());
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
	}
}

pub fn on_touch_move(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	if view.m_type == "chart"{
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				(*v).m_mouse_position = first_point.clone();
				(*v).m_cross_stop_index = get_chart_index(&mut *v, first_point.clone());
				mouse_move_chart(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if view.m_type == "grid"{
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				mouse_move_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}
	else if view.m_type == "div" || view.m_type == "layout"{
		let mut div = view.clone();
		mouse_move_div(&mut div, first_touch, second_touch, first_point.clone(), second_point.clone());
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
	}
}

pub fn on_touch_end(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, view:FCView, first_touch:bool, second_touch:bool, first_point:FCPoint, second_point:FCPoint){
	if view.m_type == "chart"{
		for (id, v) in M_CHART_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				(*v).m_first_touch_index_cache = -1;
				(*v).m_second_touch_index_cache = -1;
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
    }else if view.m_type == "grid"{;
		for (id, v) in M_GRID_MAP.lock().unwrap().iter_mut(){
			if view.m_id == *id{
				(*v).m_view = view.clone();
				mouse_up_grid(&mut *v, first_touch, second_touch, first_point.clone(), second_point.clone());
				M_VIEW_MAP.lock().unwrap().insert((*v).m_view.m_id, (*v).m_view.clone());
				break;
			}
		}
		invalidate_view(context, view.clone());
	}else if view.m_type == "div" || view.m_type == "layout"{
		let mut div = view.clone();
		mouse_up_div(&mut div, first_touch, second_touch, first_point.clone(), second_point.clone());
		M_VIEW_MAP.lock().unwrap().insert(div.m_id, div.clone());
		invalidate_view(context, view.clone());
	}else{
		invalidate_view(context, view.clone());
	}
}

pub fn read_xml_node(element:&std::rc::Rc<web_sys::Element>, parent:&mut FCView){
	let node_name = element.node_name().to_lowercase();
	let mut view = FCView::new();
	if M_PAINT.lock().unwrap().m_default_ui_style == "dark"{
		view.m_back_color = "rgb(0,0,0)".to_string();
        view.m_border_color = "rgb(100,100,100)".to_string();
        view.m_text_color = "rgb(255,255,255)".to_string();
	}else if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
		view.m_back_color = "rgb(255,255,255)".to_string();
        view.m_border_color = "rgb(150,150,150)".to_string();
        view.m_text_color = "rgb(0,0,0)".to_string();
	}
	if parent.m_id == -1{
		view.m_id = add_view(view.clone());
	}else{
		view.m_id = add_view_to_parent(view.clone(), parent.clone());
	}
	let node_value = element.node_value();
	let node_attributes = element.attributes();
	let mut isDiv:bool = true;
	for i in 0..node_attributes.length(){
		let attribute = node_attributes.item(i).expect("REASON");
		let atr_name = attribute.name().to_lowercase();
		let atr_value = attribute.value();
		set_view_attribute(&mut view, atr_name.clone(), atr_value.clone());
		if node_name == "div" || node_name == "view"{
			if atr_name == "type"{
				isDiv = false;
				if atr_value == "splitlayout"{
					view.m_type = "split".to_string();
				}
				else if atr_value == "layout"{
					view.m_type = "layout".to_string();
					view.m_show_vscrollbar = true;
					view.m_show_hscrollbar = true;
					view.m_allow_drag_scroll = true;
				}
				else if atr_value == "tab"{
					view.m_type = "tabview".to_string();
				}
				else if atr_value == "tabpage"{
					view.m_type = "tabpage".to_string();
				}
				else if atr_value == "radio"{
					view.m_type = "radiobutton".to_string();
				}else if atr_value == "checkbox"{
					view.m_type = "checkbox".to_string();
				}else if atr_value == "button"{
					view.m_type = "button".to_string();
				}
				else{
					view.m_type = "div".to_string();
					view.m_show_vscrollbar = true;
					view.m_show_hscrollbar = true;
					view.m_allow_drag_scroll = true;
				}
			}
		}else if node_name == "input"{
			if atr_name == "type"{
				if atr_value == "radio"{
					view.m_type = "radiobutton".to_string();
				}else if atr_value == "checkbox"{
					view.m_type = "checkbox".to_string();
				}else if atr_value == "button"{
					view.m_type = "button".to_string();
				}
			}
		}
	}
	if isDiv {
		view.m_type = "div".to_string();
		view.m_show_vscrollbar = true;
		view.m_show_hscrollbar = true;
		view.m_allow_drag_scroll = true;
	}
	if node_name == "chart"{
		view.m_type = "chart".to_string();
	}else if node_name == "label"{
		view.m_type = "label".to_string();
	}else if node_name == "table"{
		view.m_type = "grid".to_string();
		view.m_show_vscrollbar = true;
		view.m_show_hscrollbar = true;
		view.m_allow_drag_scroll = true;
	}
	M_VIEW_MAP.lock().unwrap().insert(view.m_id, view.clone());
	if view.m_type == "split"{
		let mut split:FCSplitLayoutDiv = FCSplitLayoutDiv::new();
		let mut splitterposition : String = String::from("");
		let mut can_drag_splitter : bool = false;
		for i in 0..node_attributes.length(){
			let attribute = node_attributes.item(i).expect("REASON");
			let atr_name = attribute.name().to_lowercase();
			let atr_value = attribute.value();
			set_split_attribute(&mut split, atr_name.clone(), atr_value.clone());
			if atr_name == "datumsize"{
				let str_split:Vec<&str> = atr_value.split(",").collect();
				let cx : f32 = str_split[0].parse::<f32>().unwrap();
				let cy : f32 =  str_split[1].parse::<f32>().unwrap();
				view.m_size = FCSize{cx:cx, cy:cy};	
			}else if atr_name == "splitterposition"{
				splitterposition = atr_value;
			}else if atr_name == "candragsplitter"{
				if atr_value == "true"{
					can_drag_splitter = true;
				}
			}
		}
		M_VIEW_MAP.lock().unwrap().insert(view.m_id, view.clone());
		let mut splitter:FCView = FCView::new();
		if M_PAINT.lock().unwrap().m_default_ui_style == "dark"{
			splitter.m_back_color = "rgb(100,100,100)".to_string();
		}else if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
			splitter.m_back_color = "rgb(150,150,150)".to_string();
		}
		splitter.m_allow_drag = can_drag_splitter;
		let str_split:Vec<&str> = splitterposition.split(",").collect();
		if str_split.len() >= 4{
			let left : f32 = str_split[0].parse::<f32>().unwrap();
			let top : f32 =  str_split[1].parse::<f32>().unwrap();
			let right : f32 = str_split[2].parse::<f32>().unwrap();
			let bottom : f32 =  str_split[3].parse::<f32>().unwrap();
			splitter.m_location = FCPoint{x:left, y:top};
			splitter.m_size = FCSize{cx:right - left + 1.0, cy:bottom - top + 1.0};
		}else{
			let s_position : f32 = str_split[0].parse::<f32>().unwrap();
			let s_size : f32 =  str_split[1].parse::<f32>().unwrap();
			if split.m_layout_style == "lefttoright" || split.m_layout_style == "righttoleft"{
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
		if first_view.m_id >= second_view.m_id{
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
	else if view.m_type == "chart"{
		let mut chart:FCChart = FCChart::new();
		chart.m_view = view.clone();
		M_CHART_MAP.lock().unwrap().insert(view.m_id, chart.clone());
	}
	else if view.m_type == "checkbox"{
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
	else if view.m_type == "radiobutton"{
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
	else if view.m_type == "grid"{
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
			if sub_node_name == "tr"{
				let sun_elements = sub_node.children();
				for j in 0..sun_elements.length(){
					let sun_node = sun_elements.item(j).expect("REASON");
					let sun_node_name = sun_node.node_name().to_lowercase();
					if sun_node_name == "th"{
						let mut grid_column:FCGridColumn = FCGridColumn::new();
						if M_PAINT.lock().unwrap().m_default_ui_style == "dark"{
							grid_column.m_back_color = "rgb(0,0,0)".to_string();
							grid_column.m_border_color = "rgb(100,100,100)".to_string();
							grid_column.m_text_color = "rgb(255,255,255)".to_string();
						}else if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
							grid_column.m_back_color = "rgb(255,255,255)".to_string();
							grid_column.m_border_color = "rgb(150,150,150)".to_string();
							grid_column.m_text_color = "rgb(0,0,0)".to_string();
						}
						let sun_node_attributes = sun_node.attributes();
						for k in 0..sun_node_attributes.length(){
							let sun_attr = sun_node_attributes.item(k).expect("REASON");
							let sun_atr_name = sun_attr.name().to_lowercase();
							let sun_atr_value = sun_attr.value();
							if sun_atr_name == "text"{
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
		if view.m_type == "layout"{
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
		else if view.m_type == "tabview"{
			let mut tab:FCTabView = FCTabView::new();
			tab.m_view = view.clone();
			M_TAB_MAP.lock().unwrap().insert(view.m_id, tab.clone());
		}else if view.m_type == "tabpage"{
			let mut tab = M_TAB_MAP.lock().unwrap()[&parent.m_id].clone();
			let mut header_button_view:FCView = FCView::new();
			header_button_view.m_size = FCSize{cx:100.0, cy:20.0};
			header_button_view.m_type = "headerbutton".to_string();
			for i in 0..node_attributes.length(){
				let attribute = node_attributes.item(i).expect("REASON");
				let atr_name = attribute.name().to_lowercase();
				let atr_value = attribute.value();
				if atr_name == "text"{
					header_button_view.m_text = atr_value;
				}else if atr_name == "headersize"{
					let str_split:Vec<&str> = atr_value.split(",").collect();
					let cx : f32 = str_split[0].parse::<f32>().unwrap();
					let cy : f32 =  str_split[1].parse::<f32>().unwrap();
					header_button_view.m_size = FCSize{cx:cx, cy:cy};
				}
			}
			if M_PAINT.lock().unwrap().m_default_ui_style == "dark"{
				header_button_view.m_back_color = "rgb(0,0,0)".to_string();
				header_button_view.m_border_color = "rgb(100,100,100)".to_string();
				header_button_view.m_text_color = "rgb(255,255,255)".to_string();
			}else if M_PAINT.lock().unwrap().m_default_ui_style == "light"{
				header_button_view.m_back_color = "rgb(255,255,255)".to_string();
				header_button_view.m_border_color = "rgb(150,150,150)".to_string();
				header_button_view.m_text_color = "rgb(0,0,0)".to_string();
			}
			header_button_view.m_id = add_view_to_parent(header_button_view.clone(), parent.clone());
			if tab.m_tab_pages.len() > 0{
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
	if name == "headerheight"{
		grid.m_header_height = value.parse::<f32>().unwrap();
	}
}

pub fn set_check_box_attribute(check_box:&mut FCCheckBox, name:String, value:String){
	if name == "checked"{
		if value == "true"{
			check_box.m_checked = true;
		}else{
			check_box.m_checked = false;
		}
	}else if name == "buttonsize"{
		let str_split:Vec<&str> = value.split(",").collect();
		let cx : f32 = str_split[0].parse::<f32>().unwrap();
		let cy : f32 =  str_split[1].parse::<f32>().unwrap();
		check_box.m_button_size = FCSize{cx:cx, cy:cy};
	}
}

pub fn set_radio_button_attribute(radio_button:&mut FCRadioButton, name:String, value:String){
	if name == "checked"{
		if value == "true"{
			radio_button.m_checked = true;
		}else{
			radio_button.m_checked = false;
		}
	}else if name == "buttonsize"{
		let str_split:Vec<&str> = value.split(",").collect();
		let cx : f32 = str_split[0].parse::<f32>().unwrap();
		let cy : f32 =  str_split[1].parse::<f32>().unwrap();
		radio_button.m_button_size = FCSize{cx:cx, cy:cy};
	}
}

pub fn set_split_attribute(split:&mut FCSplitLayoutDiv, name:String, value:String){
	if name == "layoutstyle"{
		split.m_layout_style = value.to_lowercase();
	}else if name == "splitmode"{
		split.m_split_mode = value;
	}
}

pub fn set_layout_attribute(layout:&mut FCLayoutDiv, name:String, value:String){
	if name == "layoutstyle"{
		layout.m_layout_style = value.to_lowercase();
	}else if name == "autowrap"{
		if value == "true"{
			layout.m_auto_wrap = true;
		}else{
			layout.m_auto_wrap = false;
		}
	}
}

pub fn set_view_attribute(view:&mut FCView, name:String, value:String){
	if name == "location"{
		let str_split:Vec<&str> = value.split(",").collect();
		let x : f32 = str_split[0].parse::<f32>().unwrap();
		let y : f32 =  str_split[1].parse::<f32>().unwrap();
		view.m_location = FCPoint{x:x, y:y};
	}
	else if name == "size"{
		let str_split:Vec<&str> = value.split(",").collect();
		let cx : f32 = str_split[0].parse::<f32>().unwrap();
		let cy : f32 =  str_split[1].parse::<f32>().unwrap();
		view.m_size = FCSize{cx:cx, cy:cy};
	}else if name == "text"{
		view.m_text = value;
	}else if name == "backcolor"{ 
		if value.find("rgb") == Some(0){
			view.m_back_color = value;
		}else{
			view.m_back_color = "none".to_string();
		}
	}else if name == "bordercolor"{
		if value.find("rgb") == Some(0){
			view.m_border_color = value;
		}else{
			view.m_border_color = "none".to_string();
		}
	}else if name == "textcolor"{
		if value.find("rgb") == Some(0){
			view.m_text_color = value;
		}else{
			view.m_text_color = "none".to_string();
		}
	}else if name == "dock"{
		view.m_dock = value.to_lowercase();
	}else if name == "font"{
		view.m_font = value.replace("Default", "Arial");
	}else if name == "name"{
		view.m_name = value;
	}else if name == "showvscrollbar"{
		if value.to_lowercase() == "true"{
			view.m_show_vscrollbar = true;
		}else{
			view.m_show_vscrollbar = false;
		}
	}else if name == "showhscrollbar"{
		if value.to_lowercase() == "true"{
			view.m_show_hscrollbar = true;
		}else{
			view.m_show_hscrollbar = false;
		}
	}else if name == "visible"{
		if value.to_lowercase() == "true"{
			view.m_visible = true;
		}else{
			view.m_visible = false;
		}
	}else if name == "displayoffset"{
		if value.to_lowercase() == "true"{
			view.m_display_offset = true;
		}else{
			view.m_display_offset = false;
		}
	}else if name == "topmost"{
		if value.to_lowercase() == "true"{
			view.m_top_most = true;
		}else{
			view.m_top_most = false;
		}
	}else if name == "allowdrag"{
		if value.to_lowercase() == "true"{
			view.m_allow_drag = true;
		}else{
			view.m_allow_drag = false;
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
			if find && view.m_dock == "fill"{
				let parent = M_VIEW_MAP.lock().unwrap()[&p_id].clone();
				if parent.m_type != "split"{
					view.m_location = FCPoint{x:0.0, y:0.0};
					view.m_size = FCSize{cx:parent.m_size.cx, cy:parent.m_size.cy};
				}
			}
			M_VIEW_MAP.lock().unwrap().insert(view.m_id, view.clone());
			if view.m_type == "split"{
				for (id, v) in M_SPLIT_MAP.lock().unwrap().iter_mut(){
					if view.m_id == *id{
						(*v).m_view = view.clone();
						reset_split_layout_div(&mut *v);
					}
				}
            }else if view.m_type == "tabview"{
				for (id, v) in M_TAB_MAP.lock().unwrap().iter_mut(){
					if view.m_id == *id{
						(*v).m_view = view.clone();
						update_tab_layout(&mut *v);
					}
				}
            }else if view.m_type == "layout"{
				for (id, v) in M_LAYOUT_MAP.lock().unwrap().iter_mut(){
					if view.m_id == *id{
						(*v).m_view = view.clone();
						reset_layout_div(&mut *v);
					}
				}
            }
			let sub_views = get_sub_views(view.clone());
			let sub_views_size = sub_views.len();
			if sub_views_size > 0 {
				update_views(sub_views);
			}
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
    if rustMode == 0{
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
			if i == 0{
				tab_page_view.m_back_color = "rgb(255,0,255)".to_string();
			}else if i == 1{
				tab_page_view.m_back_color = "rgb(0,255,0)".to_string();
			}else if i == 2{
				tab_page_view.m_back_color = "rgb(0,0,255)".to_string();
			}else if i == 3{
				tab_page_view.m_back_color = "rgb(255,0,0)".to_string();
			}
			tab_page_view.m_border_color = "rgb(100,100,100)".to_string();
			tab_page_view.m_text_color = "rgb(255,255,255)".to_string();
			tab_page_view.m_type = "tabpage".to_string();
			if i != 0{
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
	}else if rustMode == 1{
		let xml = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\r\n<html xmlns=\"facecat\">\r\n  <head>\r\n</head>\r\n  <body>\r\n    <div type=\"splitlayout\" layoutstyle=\"lefttoright\" bordercolor=\"none\" dock=\"fill\" size=\"400,400\" candragsplitter=\"true\" splittervisible=\"true\" splitterposition=\"200,5\">\r\n      <div name=\"div1\" backcolor=\"rgb(216,112,173)\"/>\r\n      <div name=\"div2\" backcolor=\"rgb(75,137,220)\"/>\r\n    </div>\r\n  </body>\r\n</html>\r\n".to_string();
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
			if t_view.m_dock == "fill"{
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
