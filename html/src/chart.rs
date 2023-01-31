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
    m_ind_max2 : f64,
    m_ind_min2 : f64, 
    m_cross_tip_color : String, 
    m_cross_line_color : String,
	m_font : String, 
    m_candle_digit : i32, 
    m_vol_digit : i32,
    m_ind_digit : i32, 
    m_ind_digit2 : i32, 
    m_last_record_is_visible : bool, 
    m_last_visible_key : f64, 
    m_auto_fill_hscale : bool, 
    m_candle_div_percent : f64,
    m_vol_div_percent : f64,
    m_ind_div_percent : f64, 
    m_ind_div_percent2 : f64, 
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
    m_ind_padding_top2 : f32, 
    m_ind_padding_bottom2 : f32, 
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
            m_ind_max2 : 0.0,
			m_ind_min2 : 0.0,
			m_cross_tip_color : String::from("rgb(50,50,50)"), 
			m_cross_line_color : String::from("rgb(100,100,100)"), 
			m_font : String::from("12px Arial"), 
			m_candle_digit : 2, 
			m_vol_digit : 0,
			m_ind_digit : 2, 
            m_ind_digit2 : 2, 
			m_last_record_is_visible : true, 
			m_last_visible_key : 0.0, 
			m_auto_fill_hscale : false, 
			m_candle_div_percent : 0.5,
			m_vol_div_percent : 0.2,
			m_ind_div_percent : 0.3, 
            m_ind_div_percent2 : 0.0, 
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
            m_ind_padding_top2 : 20.0, 
			m_ind_padding_bottom2 : 20.0, 
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
		if start {
            if n_temp < 4 {
                if *grid_digit > 0 {
                    *grid_digit = *grid_digit - 1;
                }
            } else if n_temp >= 4 && n_temp <= 6 {
                n_temp = 5;
                *grid_step = *grid_step + (n_temp as f64) * divisor;
            } else {
                *grid_step = *grid_step + 10.0 * divisor;
                if *grid_digit > 0 {
                    *grid_digit = *grid_digit - 1;
                }
            }
            break;
        } else if n_temp > 0 {
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
	if len > 0{
		max = list[0];
	}
	for num in list{
		if max < num{
			max = num;
		}
	}
	return max;
}

pub fn min_value(list:Vec<f64>)->f64{
	let mut min:f64 = 0.0;
	let len = list.len();
	if len > 0{
		min = list[0];
	}
	for num in list{
		if min > num{
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
	if (x1 - o_x) != (x2 - o_x) {
        *k = ((y2 - o_y) - (y1 - o_y)) / ((x2 - o_x) - (x1 - o_x));
        *b = (y1 - o_y) - *k * (x1 - o_x);
    }
}

pub fn ellipse_has_point(x:f32, y: f32, o_x:f32, o_y:f32, a:f32, b:f32)->bool{
	let mut new_x = x;
	let mut new_y = y;
	new_x = x - o_x;
    new_y = y - o_y;
    if a == 0.0 && b == 0.0 && new_x == 0.0 && new_y == 0.0 {
        return true;
    }
    if a == 0.0 {
        if new_x == 0.0 && new_y >= -b && new_y <= b {
            return false;
        }
    }
    if b == 0.0 {
        if y == 0.0 && new_x >= -a && new_x <= a {
            return true;
        }
    }
    if (new_x * new_x) / (a * a) + (new_y * new_y) / (b * b) >= 0.8 && (new_x * new_x) / (a * a) + (new_y * new_y) / (b * b) <= 1.2 {
        return true;
    }
    return false;
}

pub fn rectangle_xywh(x1:f32, y1:f32, x2:f32, y2:f32, x:&mut f32, y:&mut f32, w:&mut f32, h:&mut f32){
	if x1 < x2{
		*x = x1;
	}else{
		*x = x2;
	}
	if y1 < y2{
		*y = y1;
	}else{
		*y = y2;
	}
    *w = (x1 - x2).abs();
    *h = (y1 - y2).abs();
    if *w <= 0.0 {
        *w = 4.0;
    }
    if *h <= 0.0 {
        *h = 4.0;
    }
}

pub fn select_line(mp:FCPoint, x1:f32, y1:f32, x2:f32, y2:f32)->bool{
	let mut k:f32 = 0.0;
	let mut b:f32 = 0.0;
	let f32_x = mp.x as f32;
	let f32_y = mp.y as f32;
	line_xy(x1, y1, x2, y2, 0.0, 0.0, &mut k, &mut b);
	if !(k == 0.0 && b == 0.0) {
        if f32_y / (f32_x * k + b) >= 0.9 && f32_y / (f32_x * k + b) <= 1.1 {
            return true;
        }
    } else {
        if f32_x >= x1 - 5.0 && f32_x <= x1 + 5.0 {
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
	if !(k == 0.0 && b == 0.0) {
        if f32_y / (f32_x * k + b) >= 0.9 && f32_y / (f32_x * k + b) <= 1.1 {
            if x1 >= x2{
                if f32_x > x1 + 5.0 {
					return false;
				}
            } else if x1 < x2 {
                if f32_x < x1 - 5.0{
					return false;
				}
            }
            return true;
        }
    } else {
        if f32_x>= x1 - 5.0 && f32_x <= x1 + 5.0{
            if y1 >= y2 {
                if f32_y <= y1 - 5.0 {
                    return true;
                }
            } else {
                if f32_y >= y1 - 5.0 {
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
	if x1 <= x2{
		small_x = x1;
	}else{
		small_x = x2;
	}
	if y1 <= y2{
		small_y = y1;
	}else{
		small_y = y2;
	}
	if x1 > x2{
		big_x = x1;
	}else{
		big_x = x2;
	}
	if y1 > y2{
		big_y = y1;
	}else{
		big_y = y2;
	}
    if f32_x >= small_x - 2.0 && f32_x <= big_x + 2.0 && f32_y >= small_y - 2.0 && f32_y <= big_y + 2.0 {
        if k != 0.0 || b != 0.0 {
            if f32_y / (f32_x * k + b) >= 0.9 && f32_y / (f32_x * k + b) <= 1.1 {
                return true;
            }
        } else {
            if f32_x >= x1 - 5.0 && f32_x <= x1 + 5.0{
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
    if length > 1{
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
	if index < 1{
        return 0;
    }
    else{
        let mut vlist: Vec<i32> = Vec::new();
		for i in 0..index{
			vlist.push(0);
		}
        let mut result:i32 = 0;
		for i in 0..index{
			if i == 0 || i == 1{
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
    if count < 0{
        count = 0;
    }
    return count;
}

pub fn get_candle_div_height(chart:&mut FCChart)->f32{
	let height = chart.m_view.m_size.cy - chart.m_hscale_height;
	if height > 0.0{
		return height * (chart.m_candle_div_percent as f32);
	}else{
		return 0.0;
	}
}

pub fn get_vol_div_height(chart:&mut FCChart)->f32{
	let height = chart.m_view.m_size.cy - chart.m_hscale_height;
	if height > 0.0{
		return height * (chart.m_vol_div_percent as f32);
	}else{
		return 0.0;
	}
}

pub fn get_ind_div_height(chart:&mut FCChart)->f32{
	let height = chart.m_view.m_size.cy - chart.m_hscale_height;
	if height > 0.0{
		return height * (chart.m_ind_div_percent as f32);
	}else{
		return 0.0;
	}
}

pub fn get_ind_div_height2(chart:&mut FCChart)->f32{
	let height = chart.m_view.m_size.cy - chart.m_hscale_height;
	if height > 0.0{
		return height * (chart.m_ind_div_percent2 as f32);
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
	if data_len == 0{
		return -1;
	}
	if mp.x <= 0.0 {
		return 0;
	}
	let width = chart.m_view.m_size.cx - chart.m_left_vscale_width - chart.m_right_vscale_width;
	let int_x = mp.x - chart.m_left_vscale_width - chart.m_hscale_pixel;
	let mut index = (chart.m_first_visible_index as f32 + int_x / chart.m_hscale_pixel) as i32;
	if (int_x as i32) % (chart.m_hscale_pixel as i32) != 0{
		index = index + 1;
	}
	if index < 0{
		 index = 0;
	}else if index > data_len - 1{
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
		if data.m_date == date{
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
    if index >= 0 && index < data_len{
        date = chart.m_data[index as usize].m_date;
    }
    return date;
}

pub fn check_chart_last_visible_index(chart:&mut FCChart){
	let data_len = chart.m_data.len() as i32;
    if chart.m_last_visible_index > data_len - 1 {
        chart.m_last_visible_index = data_len - 1;
    }
    if data_len > 0 {
        chart.m_last_visible_key = chart.m_data[chart.m_last_visible_index as usize].m_date;
        if chart.m_last_visible_index == data_len - 1 {
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
    if xscale_pixel < 1000000.0 {
        chart.m_first_visible_index = first_visible_index;
        chart.m_last_visible_index = last_visible_index;
	    let data_len = chart.m_data.len() as i32;
        if last_visible_index != data_len - 1{
                chart.m_last_record_is_visible = false;
        }else {
            chart.m_last_record_is_visible = true;
        }
        chart.m_hscale_pixel = xscale_pixel;
        check_chart_last_visible_index(chart);
    }
}

pub fn reset_chart_visible_record(chart:&mut FCChart){
    let rows_count = chart.m_data.len() as i32;
    let working_area_width = get_chart_workarea_width(chart);
    if chart.m_auto_fill_hscale {
        if working_area_width > 0.0 && rows_count > 0 {
            chart.m_hscale_pixel = working_area_width / (rows_count as f32);
            chart.m_first_visible_index = 0;
            chart.m_last_visible_index = rows_count - 1;
        }
    } else {
        let max_visible_record = get_max_visible_count(chart, chart.m_hscale_pixel, working_area_width);
        if rows_count == 0 {
            chart.m_first_visible_index = -1;
            chart.m_last_visible_index = -1;
        } else {
            if rows_count < max_visible_record {
                chart.m_last_visible_index = rows_count - 1;
                chart.m_first_visible_index = 0;
            }
            else {
                if chart.m_first_visible_index != -1 && chart.m_last_visible_index != -1 && !chart.m_last_record_is_visible{
                    let index = get_chart_index_by_date(chart, chart.m_last_visible_key);
                    if index != -1 {
                        chart.m_last_visible_index = index;
                    }
                    chart.m_first_visible_index = chart.m_last_visible_index - max_visible_record + 1;
                    if chart.m_first_visible_index < 0 {
                        chart.m_first_visible_index = 0;
                        chart.m_last_visible_index = chart.m_first_visible_index + max_visible_record;
                        check_chart_last_visible_index(chart);
                    }
                } else {
                    chart.m_last_visible_index = rows_count - 1;
                    chart.m_first_visible_index = chart.m_last_visible_index - max_visible_record + 1;
                    if chart.m_first_visible_index > chart.m_last_visible_index {
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
	if bindex > eindex{
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
    if div_index == 0{
        if chart.m_candle_max > chart.m_candle_min{
            let mut c_value = value;
            let mut c_max = chart.m_candle_max;
            let mut c_min = chart.m_candle_min;
            if chart.m_vscale_type != "standard"{
                if c_value > 0.0 {
                    c_value = c_value.log10();
                } else if c_value < 0.0 {
                    c_value = -c_value.abs().log10();
                }
                if c_max > 0.0 {
                    c_max = c_max.log10();
                } else if c_max < 0.0 {
                    c_max = -c_max.abs().log10();
                }
                if c_min > 0.0 {
                    c_min = c_min.log10();
                } else if c_min < 0.0 {
                    c_min = -c_min.abs().log10();
                }
            }
            let rate = (c_value - c_min) / (c_max - c_min);
            let div_height = get_candle_div_height(chart);
            return div_height - chart.m_candle_padding_bottom - (div_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom) * (rate as f32);
        }else{
            return 0.0;
        }
    }else if div_index == 1{
        if chart.m_vol_max > chart.m_vol_min{
            let rate = (value - chart.m_vol_min) / (chart.m_vol_max - chart.m_vol_min);
            let candle_height = get_candle_div_height(chart);
            let vol_height = get_vol_div_height(chart);
            return candle_height + vol_height - chart.m_vol_padding_bottom - (vol_height - chart.m_vol_padding_top - chart.m_vol_padding_bottom) * (rate as f32);
        }else{
            return 0.0;
        }
    }else if div_index == 2{
        if chart.m_ind_max > chart.m_ind_min{
            let rate = (value - chart.m_ind_min) / (chart.m_ind_max - chart.m_ind_min);
            let candle_height = get_candle_div_height(chart);
            let vol_height = get_vol_div_height(chart);
            let ind_height = get_ind_div_height(chart);
            return candle_height + vol_height + ind_height - chart.m_ind_padding_bottom - (ind_height - chart.m_ind_padding_top - chart.m_ind_padding_bottom) * (rate as f32);
        }else{
            return 0.0;
        }
    }
    else if div_index == 3{
        if chart.m_ind_max2 > chart.m_ind_min2{
            let rate = (value - chart.m_ind_min2) / (chart.m_ind_max2 - chart.m_ind_min2);
            let candle_height = get_candle_div_height(chart);
            let vol_height = get_vol_div_height(chart);
            let ind_height = get_ind_div_height(chart);
            let ind_height2 = get_ind_div_height2(chart);
            return candle_height + vol_height + ind_height + ind_height2 - chart.m_ind_padding_bottom2 - (ind_height2 - chart.m_ind_padding_top2 - chart.m_ind_padding_bottom2) * (rate as f32);
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
    let ind_height2 = get_ind_div_height2(chart);
    if point.y <= candle_height{
        let rate = (candle_height - chart.m_candle_padding_bottom - point.y) / (candle_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom);
        let mut c_min = chart.m_candle_min;
        let mut c_max = chart.m_candle_max;
        if chart.m_vscale_type != "standard"{
            if c_max > 0.0 {
                c_max = c_max.log10();
            } else if c_max < 0.0 {
                c_max = -c_max.abs().log10();
            }
            if c_min > 0.0 {
                c_min = c_min.log10();
            } else if c_min < 0.0 {
                c_min = -c_min.abs().log10();
            }
        }
        let result = c_min + (c_max - c_min) * (rate as f64);;
        if chart.m_vscale_type != "standard"{
            let b:f64 = 10.0;
            return b.powf(result as f64);
        }else{
            return result;
        }
    }
    else if point.y > candle_height && point.y <= candle_height + vol_height{
        let rate = (vol_height - chart.m_vol_padding_bottom - (point.y - candle_height)) / (vol_height - chart.m_vol_padding_top - chart.m_vol_padding_bottom);
        return chart.m_vol_min + (chart.m_vol_max - chart.m_vol_min) * (rate as f64);
    }else if point.y > candle_height + vol_height && point.y <= candle_height + vol_height + ind_height{
        let rate = (ind_height - chart.m_ind_padding_bottom - (point.y - candle_height - vol_height)) / (ind_height - chart.m_ind_padding_top - chart.m_ind_padding_bottom);
        return chart.m_ind_min + (chart.m_ind_max - chart.m_ind_min) * (rate as f64);
    }else if point.y > candle_height + vol_height + ind_height && point.y <= candle_height + vol_height + ind_height + ind_height2{
        let rate = (ind_height2 - chart.m_ind_padding_bottom2 - (point.y - candle_height - vol_height - ind_height)) / (ind_height2 - chart.m_ind_padding_top2 - chart.m_ind_padding_bottom2);
        return chart.m_ind_min2 + (chart.m_ind_max2 - chart.m_ind_min2) * (rate as f64);
    }
    return 0.0;
}

pub fn get_candle_div_value(chart:&mut FCChart, point:FCPoint)->f64{
    let candle_height = get_candle_div_height(chart);
    let rate = (candle_height - chart.m_candle_padding_bottom - point.y) / (candle_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom);
    let mut c_min = chart.m_candle_min;
    let mut c_max = chart.m_candle_max;
    if chart.m_vscale_type != "standard"{
        if c_max > 0.0{
            c_max = c_max.log10();
        } else if c_max < 0.0 {
            c_max = -c_max.abs().log10();
        }
        if c_min > 0.0 {
            c_min = c_min.log10();
        } else if c_min < 0.0 {
            c_min = -c_min.abs().log10();
        }
    }
    let result = c_min + (c_max - c_min) * (rate as f64);;
    if chart.m_vscale_type != "standard"{
        let b:f64 = 10.0;
        return b.powf(result as f64);
    }else{
        return result;
    }
}

pub fn select_lines(chart:&mut FCChart, mp:FCPoint, div_index:i32, datas:Vec<f64>, cur_index:i32)->bool{
    if(datas.len() > 0){
	    let top_y = get_chart_y(chart, div_index, datas[cur_index as usize]);
        if chart.m_hscale_pixel <= 1.0 {
            if mp.y >= top_y - 8.0 && mp.y <= top_y + 8.0 {
                return true;
            }
        } else {
            let index = cur_index;
            let scale_x = get_chart_x(chart, index);
            let mut judge_top = 0.0;
            let mut judge_scale_x = scale_x;
            if mp.x >= scale_x {
                let left_index = cur_index + 1;
                if cur_index < chart.m_last_visible_index {
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
                if cur_index > 0 {
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
            if judge_top >= top_y {
                judge_x = judge_scale_x;
                judge_y = top_y - 2.0 - line_width;
                judge_w = chart.m_hscale_pixel;
                if judge_top - top_y + line_width < 4.0{
				    judge_h = 4.0;
                }else{
				    judge_h = judge_top - top_y + 4.0 + line_width;
                }
            }
            else {
                judge_x = judge_scale_x;
                judge_y = judge_top - 2.0 - line_width / 2.0;
                judge_w = chart.m_hscale_pixel;
                if top_y - judge_top + line_width < 4.0{
				    judge_h = 4.0;
                }else{
				    judge_h = top_y - judge_top + 4.0 + line_width;
                }
            }
       
            if mp.x >= judge_x && mp.x <= judge_x + judge_w && mp.y >= judge_y && mp.y <= judge_y + judge_h {
           
                return true;
            }
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
    if mp.y >= candle_height + vol_height && mp.y <= candle_height + vol_height + ind_height {
        if chart.m_show_indicator == "MACD" {
            let macd_y = get_chart_y(chart, 2, chart.m_allmacdarr[index as usize]);
            let zero_y = get_chart_y(chart, 2, 0.0);
            if select_lines(chart, mp.clone(), 2, chart.m_allmacdarr.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "MACD".to_string();
            }
            if select_lines(chart, mp.clone(), 2, chart.m_alldifarr.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "DIF".to_string();
            }
            else if select_lines(chart, mp.clone(), 2, chart.m_alldeaarr.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "DEA".to_string();
            }
        } else if chart.m_show_indicator == "KDJ" {
            if select_lines(chart, mp.clone(), 2, chart.m_kdj_k.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "K".to_string();
            }
            else if select_lines(chart, mp.clone(), 2, chart.m_kdj_d.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "D".to_string();
            } else if select_lines(chart, mp.clone(), 2, chart.m_kdj_j.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "J".to_string();
            }
        } else if chart.m_show_indicator == "RSI" {
            if select_lines(chart, mp.clone(), 2, chart.m_rsi1.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "6".to_string();
            }
            else if select_lines(chart, mp.clone(), 2, chart.m_rsi2.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "12".to_string();
            } else if select_lines(chart, mp.clone(), 2, chart.m_rsi3.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "24".to_string();
            }
        }
        else if chart.m_show_indicator == "BIAS" {
            if select_lines(chart, mp.clone(), 2, chart.m_bias1.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "1".to_string();
            }
            else if select_lines(chart, mp.clone(), 2, chart.m_bias2.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "2".to_string();
            } else if select_lines(chart, mp.clone(), 2, chart.m_bias3.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "3".to_string();
            }
        }
        else if chart.m_show_indicator == "ROC" {
            if select_lines(chart, mp.clone(), 2, chart.m_roc.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "ROC".to_string();
            }
            else if select_lines(chart, mp.clone(), 2, chart.m_roc_ma.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "ROCMA".to_string();
            }
        } else if chart.m_show_indicator == "WR" {
            if select_lines(chart, mp.clone(), 2, chart.m_wr1.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "1".to_string();
            }
            else if select_lines(chart, mp.clone(), 2, chart.m_wr2.clone(), index) {
                chart.m_select_shape = "WR".to_string();
                chart.m_select_shape_ex = "2".to_string();
            }
        } else if chart.m_show_indicator == "CCI" {
            if select_lines(chart, mp.clone(), 2, chart.m_cci.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
            }
        } else if chart.m_show_indicator == "BBI" {
            if select_lines(chart, mp.clone(), 2, chart.m_bbi.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
            }
        } else if chart.m_show_indicator == "TRIX" {
            if select_lines(chart, mp.clone(), 2, chart.m_trix.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "TRIX".to_string();
            }
            else if select_lines(chart, mp.clone(), 2, chart.m_trix_ma.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "TRIXMA".to_string();
            }
        } else if chart.m_show_indicator == "DMA" {
            if select_lines(chart, mp.clone(), 2, chart.m_dma1.clone(), index) {
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "DIF".to_string();
            }
            else if select_lines(chart, mp.clone(), 2, chart.m_dma2.clone(), index){
                chart.m_select_shape = chart.m_show_indicator.clone();
                chart.m_select_shape_ex = "DIFMA".to_string();
            }
        }
    }
    else if mp.y >= candle_height && mp.y <= candle_height + vol_height {
        let vol_y = get_chart_y(chart, 1, chart.m_data[index as usize].m_volume);
        let zero_y = get_chart_y(chart, 1, 0.0); 
        if mp.y >= vol_y.min(zero_y) && mp.y <= vol_y.max(zero_y) {
            chart.m_select_shape = "VOL".to_string();
        }
    }
    else if mp.y >= 0.0 && mp.y <= candle_height {
        let is_trend:bool = false;
        if !is_trend {
            if chart.m_main_indicator == "BOLL" {
                if select_lines(chart, mp.clone(), 0, chart.m_boll_mid.clone(), index) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "MID".to_string();
                }
                else if select_lines(chart, mp.clone(), 0, chart.m_boll_up.clone(), index) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "UP".to_string();
                } else if select_lines(chart, mp.clone(), 0, chart.m_boll_down.clone(), index) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "DOWN".to_string();
                }
            } else if chart.m_main_indicator == "MA" {
                if select_lines(chart, mp.clone(), 0, chart.m_ma5.clone(), index) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "5".to_string();
                }
                else if select_lines(chart, mp.clone(), 0, chart.m_ma10.clone(), index) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "10".to_string();
                }
                else if select_lines(chart, mp.clone(), 0, chart.m_ma20.clone(), index) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "20".to_string();
                }
                else if select_lines(chart, mp.clone(), 0, chart.m_ma30.clone(), index) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "30".to_string();
                }
                else if select_lines(chart, mp.clone(), 0, chart.m_ma120.clone(), index) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "120".to_string();
                }
                else if select_lines(chart, mp.clone(), 0, chart.m_ma250.clone(), index) {
                    chart.m_select_shape = chart.m_main_indicator.clone();
                    chart.m_select_shape_ex = "250".to_string();
                }
            }
        }
        if chart.m_select_shape == "" {
            let high_y = get_chart_y(chart, 0, chart.m_data[index as usize].m_high);
            let low_y = get_chart_y(chart, 0, chart.m_data[index as usize].m_low);
			if mp.y >= low_y.min(high_y) && mp.y <= low_y.max(high_y) {
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
    if chart.m_main_indicator == "BOLL" {
		get_boll_data(close_arr.clone(), &mut chart.m_boll_up, &mut chart.m_boll_mid, &mut chart.m_boll_down);
    } else if chart.m_main_indicator == "MA" {
		chart.m_ma5 = ma_value(close_arr.clone(), 5);
		chart.m_ma10 = ma_value(close_arr.clone(), 10);
		chart.m_ma20 = ma_value(close_arr.clone(), 20);
		chart.m_ma30 = ma_value(close_arr.clone(), 30);
		chart.m_ma120 = ma_value(close_arr.clone(), 120);
		chart.m_ma250 = ma_value(close_arr.clone(), 250);
    }
    if chart.m_show_indicator == "BIAS" {
		get_bias_data(close_arr.clone(), &mut chart.m_bias1, &mut chart.m_bias2, &mut chart.m_bias3);
	}
	else if chart.m_show_indicator == "DMA"{
		get_dma_data(close_arr.clone(), &mut chart.m_dma1, &mut chart.m_dma2);
	}
	else if chart.m_show_indicator == "BBI"{
		get_bbi_data(close_arr.clone(), &mut chart.m_bbi);
	}
	else if chart.m_show_indicator == "RSI"{
		get_rsi_data(close_arr.clone(), &mut chart.m_rsi1, &mut chart.m_rsi2, &mut chart.m_rsi3);
	}
	else if chart.m_show_indicator == "ROC"{
		get_roc_data(close_arr.clone(), &mut chart.m_roc, &mut chart.m_roc_ma);
	}
	else if chart.m_show_indicator == "TRIX"{
		get_trix_data(close_arr.clone(), &mut chart.m_trix, &mut chart.m_trix_ma);
	}
	else if chart.m_show_indicator == "KDJ"{
		get_kdj_data(high_arr.clone(), low_arr.clone(), close_arr.clone(), &mut chart.m_kdj_k, &mut chart.m_kdj_d, &mut chart.m_kdj_j);
	}
	else if chart.m_show_indicator == "WR"{
		get_wr_data(high_arr.clone(), low_arr.clone(), close_arr.clone(), &mut chart.m_wr1, &mut chart.m_wr2);
	}
	else if chart.m_show_indicator == "CCI"{
		get_cci_data(high_arr.clone(), low_arr.clone(), close_arr.clone(), &mut chart.m_cci);
	}
	else if chart.m_show_indicator == "MACD"{
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
    if chart.m_cycle == "trend"{
        is_trend = true;
    }
    let mut first_open : f64 = 0.0;
    if data_len > 0 {
        for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
            let ui = i as usize;
            if i == chart.m_first_visible_index{
                if is_trend{
                    chart.m_candle_max = chart.m_data[ui].m_close;
                    chart.m_candle_min = chart.m_data[ui].m_close;  
                    first_open = chart.m_data[ui].m_close;
                }else{
                    chart.m_candle_max = chart.m_data[ui].m_high;
                    chart.m_candle_min = chart.m_data[ui].m_low;
                }
                chart.m_vol_max = chart.m_data[ui].m_volume;
                if chart.m_show_indicator == "MACD"{
                    chart.m_ind_max = chart.m_alldifarr[ui];
                    chart.m_ind_min = chart.m_alldifarr[ui];
                }
                else if chart.m_show_indicator == "KDJ"{
                    chart.m_ind_max = chart.m_kdj_k[ui];
                    chart.m_ind_min = chart.m_kdj_k[ui];
                }
                else if chart.m_show_indicator == "RSI"{
                    chart.m_ind_max = chart.m_rsi1[ui];
                    chart.m_ind_min = chart.m_rsi1[ui];
                }
                else if chart.m_show_indicator == "BIAS"{
                    chart.m_ind_max = chart.m_bias1[ui];
                    chart.m_ind_min = chart.m_bias1[ui];
                }
                 else if chart.m_show_indicator == "ROC"{
                    chart.m_ind_max = chart.m_roc[ui];
                    chart.m_ind_min = chart.m_roc[ui];
                }
                 else if chart.m_show_indicator == "BOLL"{
                    chart.m_ind_max = chart.m_boll_mid[ui];
                    chart.m_ind_min = chart.m_boll_mid[ui];
                }
                else if chart.m_show_indicator == "WR"{
                    chart.m_ind_max = chart.m_wr1[ui];
                    chart.m_ind_min = chart.m_wr1[ui];
                }else if chart.m_show_indicator == "CCI"{
                    chart.m_ind_max = chart.m_cci[ui];
                    chart.m_ind_min = chart.m_cci[ui];
                }else if chart.m_show_indicator == "BBI"{
                    chart.m_ind_max = chart.m_bbi[ui];
                    chart.m_ind_min = chart.m_bbi[ui];
                }
                else if chart.m_show_indicator == "TRIX"{
                    chart.m_ind_max = chart.m_trix[ui];
                    chart.m_ind_min = chart.m_trix[ui];
                }
                else if chart.m_show_indicator == "DMA"{
                    chart.m_ind_max = chart.m_dma1[ui];
                    chart.m_ind_min = chart.m_dma1[ui];
                }
               
            }else{
                if is_trend{
                    if chart.m_candle_max < chart.m_data[ui].m_close{
                        chart.m_candle_max = chart.m_data[ui].m_close;
                    }
                    if chart.m_candle_min > chart.m_data[ui].m_close{
                        chart.m_candle_min = chart.m_data[ui].m_close;
                    }
                }else{
                    if chart.m_candle_max < chart.m_data[ui].m_high{
                        chart.m_candle_max = chart.m_data[ui].m_high;
                    }
                    if chart.m_candle_min > chart.m_data[ui].m_low{
                        chart.m_candle_min = chart.m_data[ui].m_low;
                    }
                }
                if chart.m_vol_max < chart.m_data[ui].m_volume{
                    chart.m_vol_max = chart.m_data[ui].m_volume;
                }   
				if chart.m_show_indicator == "MACD"{
					if chart.m_ind_max < chart.m_alldifarr[ui]{
						chart.m_ind_max = chart.m_alldifarr[ui];
					}
					if chart.m_ind_max < chart.m_alldeaarr[ui]{
						chart.m_ind_max = chart.m_alldeaarr[ui];
					}
					if chart.m_ind_max < chart.m_allmacdarr[ui]{
						chart.m_ind_max = chart.m_allmacdarr[ui];
					}
					if chart.m_ind_min > chart.m_alldifarr[ui]{
						chart.m_ind_min = chart.m_alldifarr[ui];
					}
					if chart.m_ind_min > chart.m_alldeaarr[ui]{
						chart.m_ind_min = chart.m_alldeaarr[ui];
					}
					if chart.m_ind_min > chart.m_allmacdarr[ui]{
						chart.m_ind_min = chart.m_allmacdarr[ui];
					}
				}else if chart.m_show_indicator == "KDJ"{
					if chart.m_ind_max < chart.m_kdj_k[ui]{
						chart.m_ind_max = chart.m_kdj_k[ui];
					}
					if chart.m_ind_max < chart.m_kdj_d[ui]{
						chart.m_ind_max = chart.m_kdj_d[ui];
					}
					if chart.m_ind_max < chart.m_kdj_j[ui]{
						chart.m_ind_max = chart.m_kdj_j[ui];
					}
					if chart.m_ind_min > chart.m_kdj_k[ui]{
						chart.m_ind_min = chart.m_kdj_k[ui];
					}
					if chart.m_ind_min > chart.m_kdj_d[ui]{
						chart.m_ind_min = chart.m_kdj_d[ui];
					}
					if chart.m_ind_min > chart.m_kdj_j[ui]{
						chart.m_ind_min = chart.m_kdj_j[ui];
					}
				}else if chart.m_show_indicator == "RSI"{
					if chart.m_ind_max < chart.m_rsi1[ui]{
						chart.m_ind_max = chart.m_rsi1[ui];
					}
					if chart.m_ind_max < chart.m_rsi2[ui]{
						 chart.m_ind_max = chart.m_rsi2[ui];
					}
					if chart.m_ind_max < chart.m_rsi3[ui]{
						chart.m_ind_max = chart.m_rsi3[ui];
					}
					if chart.m_ind_min > chart.m_rsi1[ui]{
						chart.m_ind_min = chart.m_rsi1[ui];
					}
					if chart.m_ind_min > chart.m_rsi2[ui]{
						chart.m_ind_min = chart.m_rsi2[ui];
					}
					if chart.m_ind_min > chart.m_rsi3[ui]{
						chart.m_ind_min = chart.m_rsi3[ui];
					}
				}else if chart.m_show_indicator == "BIAS"{
					if chart.m_ind_max < chart.m_bias1[ui]{
						chart.m_ind_max = chart.m_bias1[ui];
					}
					if chart.m_ind_max < chart.m_bias2[ui]{
						chart.m_ind_max = chart.m_bias2[ui];
					}
					if chart.m_ind_max < chart.m_bias3[ui]{
						chart.m_ind_max = chart.m_bias3[ui];
					}
					if chart.m_ind_min > chart.m_bias1[ui]{
						chart.m_ind_min = chart.m_bias1[ui];
					}
					if chart.m_ind_min > chart.m_bias2[ui]{
						chart.m_ind_min = chart.m_bias2[ui];
					}
					if chart.m_ind_min > chart.m_bias3[ui]{
						chart.m_ind_min = chart.m_bias3[ui];
					}
				}else if chart.m_show_indicator == "ROC"{
					if chart.m_ind_max < chart. m_roc[ui]{
						chart.m_ind_max = chart.m_roc[ui];
					}
					if chart.m_ind_max < chart.m_roc_ma[ui]{
						chart.m_ind_max = chart.m_roc_ma[ui];
					}
					if chart.m_ind_min > chart.m_roc[ui]{
						chart.m_ind_min = chart.m_roc[ui];
					}
					if chart.m_ind_min > chart.m_roc_ma[ui]{
						chart.m_ind_min = chart.m_roc_ma[ui];
					}
				}else if chart.m_show_indicator == "BOLL"{
					if chart.m_ind_max < chart.m_boll_mid[ui]{
						chart.m_ind_max = chart.m_boll_mid[ui];
					}
					if chart.m_ind_max < chart.m_boll_up[ui]{
						chart.m_ind_max = chart.m_boll_up[ui];
					}
					if chart.m_ind_max < chart.m_boll_down[ui]{
						chart.m_ind_max = chart.m_boll_down[ui];
					}
					if chart.m_ind_min > chart.m_boll_mid[ui]{
						chart.m_ind_min = chart.m_boll_mid[ui];
					}
					if chart.m_ind_min > chart.m_boll_up[ui]{
						chart.m_ind_min = chart.m_boll_up[ui];
					}
					if chart.m_ind_min > chart.m_boll_down[ui]{
						chart.m_ind_min = chart.m_boll_down[ui];
					}
				}
				else if chart.m_show_indicator == "WR"{
					if chart.m_ind_max < chart.m_wr1[ui]{
						chart.m_ind_max = chart.m_wr1[ui];
					}
					if chart.m_ind_max < chart.m_wr2[ui]{
						chart.m_ind_max = chart.m_wr2[ui];
					}
					if chart.m_ind_min > chart.m_wr1[ui]{
						chart.m_ind_min = chart.m_wr1[ui];
					}
					if chart.m_ind_min > chart.m_wr2[ui]{
						chart.m_ind_min = chart.m_wr2[ui];
					}
				}else if chart.m_show_indicator == "CCI"{
					if chart.m_ind_max < chart.m_cci[ui]{
						chart.m_ind_max = chart.m_cci[ui];
					}
					if chart.m_ind_min > chart.m_cci[ui]{
						chart.m_ind_min = chart.m_cci[ui];
					}
				}else if chart.m_show_indicator == "BBI"{
					if chart.m_ind_max < chart.m_bbi[ui]{
						chart.m_ind_max = chart.m_bbi[ui];
					}
					if chart.m_ind_min > chart.m_bbi[ui]{
						chart.m_ind_min = chart.m_bbi[ui];
					}
				}else if chart.m_show_indicator == "TRIX"{
					if chart.m_ind_max < chart.m_trix[ui]{
						chart.m_ind_max = chart.m_trix[ui];
					}
					if chart.m_ind_max < chart.m_trix_ma[ui]{
						chart.m_ind_max = chart.m_trix_ma[ui];
					}
					if chart.m_ind_min > chart.m_trix[ui]{
						chart.m_ind_min = chart.m_trix[ui];
					}
					if chart.m_ind_min > chart.m_trix_ma[ui]{
						chart.m_ind_min = chart.m_trix_ma[ui];
					}
				}else if chart.m_show_indicator == "DMA"{
					if chart.m_ind_max < chart.m_dma1[ui]{
						chart.m_ind_max = chart.m_dma1[ui];
					}
					if chart.m_ind_max < chart.m_dma2[ui]{
						chart.m_ind_max = chart.m_dma2[ui];
					}
					if chart.m_ind_min > chart.m_dma1[ui]{
						chart.m_ind_min = chart.m_dma1[ui];
					}
					if chart.m_ind_min > chart.m_dma2[ui]{
						chart.m_ind_min = chart.m_dma2[ui];
					}
				}
            }
        }
    }
    if is_trend{
        let mut sub_max : f64 = 0.0;
        let f_value : f64 = (chart.m_candle_max - first_open).abs();
        let s_value : f64 = (chart.m_candle_min - first_open).abs();
        if f_value > s_value{
            sub_max = f_value;
        }else{
            sub_max = s_value;
        }
        chart.m_candle_max = first_open + sub_max;
        chart.m_candle_min = first_open - sub_max;
    }else{
        if (chart.m_candle_max == 0.0 && chart.m_candle_min == 0.0) {
            chart.m_candle_max = 1.0;
            chart.m_candle_min = -1.0;
        }
        if (chart.m_vol_max == 0.0 && chart.m_vol_min == 0.0) {
            chart.m_vol_max = 1.0;
            chart.m_vol_min = -1.0;
        }
        if (chart.m_ind_max == 0.0 && chart.m_ind_min == 0.0) {
            chart.m_ind_max = 1.0;
            chart.m_ind_min = -1.0;
        }
        if (chart.m_ind_max2 == 0.0 && chart.m_ind_min2 == 0.0) {
            chart.m_ind_max2 = 1.0;
            chart.m_ind_min2 = -1.0;
        }
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
	if first_touch && chart.m_splot.m_id > 0{
	    let new_index = get_chart_index(chart, FCPoint{x:mp_x, y:mp_y});
	    if new_index >= 0 && new_index < data_len{
	        let new_date = get_chart_date_by_index(chart, new_index);
	        let new_value = get_candle_div_value(chart, FCPoint{x:mp_x, y:mp_y});
            if chart.m_select_plot_point == 0{
	            chart.m_splot.m_key1 = new_date;
                chart.m_splot.m_value1 = new_value;
            } else if chart.m_select_plot_point == 1{
                chart.m_splot.m_key2 = new_date;
                chart.m_splot.m_value2 = new_value;
            } else if chart.m_select_plot_point == 2{
                chart.m_splot.m_key3 = new_date;
                chart.m_splot.m_value3 = new_value;
	        }
            else if chart.m_start_move_plot{
	            let bvalue = get_candle_div_value(chart, FCPoint{x:chart.m_mouse_down_position.x, y:chart.m_mouse_down_position.y});
	            let bindex = get_chart_index(chart, FCPoint{x:chart.m_mouse_down_position.x, y:chart.m_mouse_down_position.y});
                if chart.m_splot.m_key1 > 0.0{
                    chart.m_splot.m_value1 = chart.m_splot.m_start_value1 + (new_value - bvalue);
                    let start_index1 = get_chart_index_by_date(chart, chart.m_splot.m_start_key1);
                    let mut new_index1 = start_index1 + (new_index - bindex);
                    if new_index1 < 0{
                        new_index1 = 0;
                    }
                    else if new_index1 > data_len - 1{
                        new_index1 = data_len - 1;
                    }
                    chart.m_splot.m_key1 = get_chart_date_by_index(chart, new_index1);
                }
                if chart.m_splot.m_key2 > 0.0{
                    chart.m_splot.m_value2 = chart.m_splot.m_start_value2 + (new_value - bvalue);
                    let start_index2 = get_chart_index_by_date(chart, chart.m_splot.m_start_key2);
                    let mut new_index2 = start_index2 + (new_index - bindex);
                    if new_index2 < 0{
                        new_index2 = 0;
                    }
                    else if new_index2 > data_len - 1{
                        new_index2 = data_len - 1;
                    }
                    chart.m_splot.m_key2 = get_chart_date_by_index(chart, new_index2);
                }
                if chart.m_splot.m_key3 > 0.0{
                    chart.m_splot.m_value3 = chart.m_splot.m_start_value3 + (new_value - bvalue);
                    let start_index3 = get_chart_index_by_date(chart, chart.m_splot.m_start_key3);
                    let mut new_index3 = start_index3 + (new_index - bindex);
                    if new_index3 < 0{
                        new_index3 = 0;
                    }
                    else if new_index3 > data_len - 1{
                        new_index3 = data_len - 1;
                    }
                    chart.m_splot.m_key3 = get_chart_date_by_index(chart, new_index3);
                }
            }
	    }
	    for i in 0..chart.m_plots.len(){
			let mut plot = chart.m_plots[i].clone();
			if plot.m_id == chart.m_splot.m_id{
				chart.m_plots[i] = chart.m_splot.clone();
				break;
			}
		}
	    return;
	}
	if first_touch && second_touch {
        if first_point.x > second_point.x {
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
        if chart.m_first_touch_index_cache == -1 || chart.m_second_touch_index_cache == -1 {
            chart.m_first_touch_index_cache = get_chart_index(chart, FCPoint{x:chart.m_first_touch_point_cache.x, y:chart.m_first_touch_point_cache.y});
            chart.m_second_touch_index_cache = get_chart_index(chart, FCPoint{x:chart.m_second_touch_point_cache.x, y:chart.m_second_touch_point_cache.y});
            chart.m_first_index_cache = chart.m_first_visible_index;
            chart.m_last_index_cache = chart.m_last_visible_index;
        }
    } else if first_touch {
        chart.m_second_touch_index_cache = -1;
        if chart.m_first_touch_index_cache == -1 {
            chart.m_first_touch_point_cache.x = mp_x;
            chart.m_first_touch_point_cache.y = mp_y;
            chart.m_first_touch_index_cache = get_chart_index(chart, FCPoint{x:chart.m_first_touch_point_cache.x, y:chart.m_first_touch_point_cache.y});
            chart.m_first_index_cache = chart.m_first_visible_index;
            chart.m_last_index_cache = chart.m_last_visible_index;
        }
    }

    if first_touch && second_touch {
        if chart.m_first_touch_index_cache != -1 && chart.m_second_touch_index_cache != -1 {
            let mut f_point = FCPoint{x:mp_x, y:mp_y};
            let mut s_point = FCPoint{x:mp_x1, y:mp_y2};
            if first_point.x > second_point.x {
                f_point.x = mp_x1;
                f_point.y = mp_y2;
                s_point.x = mp_x;
                s_point.y = mp_y;
            }
            let sub_x = (s_point.x - f_point.x).abs();
            let sub_index = (chart.m_second_touch_index_cache - chart.m_first_touch_index_cache).abs();
            if sub_x > 0.0 && sub_index > 0{
                let mut new_scale_pixel = sub_x / (sub_index as f32);
                if new_scale_pixel >= 3.0 {
                    let int_scale_pixel = new_scale_pixel as i32;
                    new_scale_pixel = int_scale_pixel as f32;
                }
                if new_scale_pixel != chart.m_hscale_pixel {
                    let mut new_first_index = chart.m_first_touch_index_cache;
                    let mut this_x = f_point.x;
                    this_x = this_x - new_scale_pixel;
                    while this_x > chart.m_left_vscale_width + new_scale_pixel {
                        new_first_index = new_first_index - 1;
                        if new_first_index < 0 {
                            new_first_index = 0;
                            break;
                        }
                        this_x = this_x - new_scale_pixel;
                    }

                    this_x = s_point.x;
                    let mut new_second_index = chart.m_second_touch_index_cache;
                    this_x = this_x + new_scale_pixel;
                    while this_x < chart.m_view.m_size.cx - chart.m_right_vscale_width - new_scale_pixel {
                        new_second_index = new_second_index + 1;
                        if new_second_index > data_len {
                            new_second_index = data_len - 1;
                            break;
                        }
                        this_x = this_x + new_scale_pixel;
                    }
                    set_chart_visible_index(chart, new_first_index, new_second_index);
                    let working_area_width = get_chart_workarea_width(chart);
                    let max_visible_record = get_max_visible_count(chart, chart.m_hscale_pixel, working_area_width);
                    while max_visible_record < chart.m_last_visible_index - chart.m_first_visible_index + 1
                          && chart.m_last_visible_index > chart.m_first_visible_index {
                        chart.m_last_visible_index = chart.m_last_visible_index - 1;
                    }
                    check_chart_last_visible_index(chart);
                    reset_chart_visible_record(chart);
                    calculate_chart_max_min(chart);
                }
            }
        }
    } else if first_touch {
        let mut sub_index = ((chart.m_first_touch_point_cache.x - first_point.x) / chart.m_hscale_pixel) as i32;
        if chart.m_last_visible_index + sub_index > data_len - 1 {
            sub_index = data_len - 1 - chart.m_last_index_cache;
        } else if chart.m_first_visible_index + sub_index < 0 {
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
	if bindex > eindex{
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
    if !chart.m_auto_fill_hscale {
        let mut h_scale_pixel = chart.m_hscale_pixel;
        let old_x = get_chart_x(chart, chart.m_cross_stop_index);
        let pure_h = get_chart_workarea_width(chart);
        let mut ori_max : i32 = -1;
        let mut max : i32 = -1;
        let mut deal : i32 = 0;
        let data_count = chart.m_data.len() as i32;
        let mut findex = chart.m_first_visible_index;
        let mut lindex = chart.m_last_visible_index;
        if h_scale_pixel < 30.0 {
            ori_max = get_max_visible_count(chart, h_scale_pixel, pure_h);
            if data_count < ori_max {
                deal = 1;
            }
            if h_scale_pixel > 3.0 {
                h_scale_pixel = h_scale_pixel + 1.0;
            } else {
                if h_scale_pixel == 1.0 {
                    h_scale_pixel = 2.0;
                } else {
                    h_scale_pixel = h_scale_pixel * 1.5;
                    if (h_scale_pixel > 3.0) {
                        h_scale_pixel = (h_scale_pixel as i32) as f32;
                    }
                }
            }
            max = get_max_visible_count(chart, h_scale_pixel, pure_h);
            if data_count >= max {
                if deal == 1{
                    lindex = data_count - 1;
                }
                findex = lindex - max + 1;
                if findex < 0 {
                    findex = 0;
                }
            }
        }
        chart.m_hscale_pixel = h_scale_pixel;
        chart.m_first_visible_index = findex;
        chart.m_last_visible_index = lindex;
        if chart.m_show_cross_line{
            let mut new_x = get_chart_x(chart, chart.m_cross_stop_index);
            if new_x > old_x {
                while chart.m_last_visible_index < data_count - 1{
                    chart.m_first_visible_index = chart.m_first_visible_index + 1;
                    chart.m_last_visible_index = chart.m_last_visible_index + 1;
                    new_x = get_chart_x(chart, chart.m_cross_stop_index);
                    if new_x <= old_x{
                        break;
                    }
                }

            }
            else if new_x < old_x{
                while chart.m_first_visible_index > 0{
                    chart.m_first_visible_index = chart.m_first_visible_index - 1;
                    chart.m_last_visible_index = chart.m_last_visible_index - 1;
                    new_x = get_chart_x(chart, chart.m_cross_stop_index);
                    if new_x >= old_x{
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
     if !chart.m_auto_fill_hscale {
        let mut h_scale_pixel = chart.m_hscale_pixel;
        let old_x = get_chart_x(chart, chart.m_cross_stop_index);
        let pure_h = get_chart_workarea_width(chart);
        let mut max : i32 = -1;
        let data_count = chart.m_data.len() as i32;
        let mut findex = chart.m_first_visible_index;
        let mut lindex = chart.m_last_visible_index;
        if h_scale_pixel > 3.0 {
            h_scale_pixel -= 1.0;
        } else {
            h_scale_pixel = h_scale_pixel * 2.0 / 3.0;
            if h_scale_pixel > 3.0 {
                 h_scale_pixel = (h_scale_pixel as i32) as f32;
            }
        }
        max = get_max_visible_count(chart, h_scale_pixel, pure_h);
        if max >= data_count {
            if h_scale_pixel < 1.0 {
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
        if chart.m_show_cross_line{
            let mut new_x = get_chart_x(chart, chart.m_cross_stop_index);
            if new_x > old_x{
                while chart.m_last_visible_index < data_count - 1{
                    chart.m_first_visible_index = chart.m_first_visible_index + 1;
                    chart.m_last_visible_index = chart.m_last_visible_index + 1;
                    new_x = get_chart_x(chart, chart.m_cross_stop_index);
                    if new_x <= old_x
                    {
                        break;
                    }
                }

            }
            else if new_x < old_x{
                while chart.m_first_visible_index > 0{
                    chart.m_first_visible_index = chart.m_first_visible_index - 1;
                    chart.m_last_visible_index = chart.m_last_visible_index - 1;
                    new_x = get_chart_x(chart, chart.m_cross_stop_index);
                    if new_x >= old_x{
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
		if (i as i32) >= days {
			min = ticks[i as usize];
            let mut j = i as i32;
            while j > (i as i32) - days{
                if min > ticks[j as usize] {
					min = ticks[j as usize];
				}
                j = j - 1;
            }
			llv.push(min);
		} else {
			if min > ticks[i as usize] {
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
		if (i as i32) >= days{
			max = ticks[i as usize];
			let mut j = i as i32;
            while j > (i as i32) - days{
                if max < ticks[j as usize] {
					max = ticks[j as usize];
				}
                j = j - 1;
            }
			hhv.push(max);
		} else {
			if max > ticks[i as usize] {
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
		if (i as i32) >= days {
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
        if (i as i32) >= tick_begin {
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
		if high > *max_high {
			*max_high = high;
		}
		if low < *min_low {
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
        if start_index < 0{
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
        if max == min {
			rsvs.push(0.0);
		} else {
			rsvs.push((close - min) / (max - min) * 100.0);
		}
		if i == 0 {
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
        if i == 0{
            last_sm1 = 0.0;
            last_sa1 = 0.0;
            rsi1.push(0.0);
        }else{
            last_sm1 = (m + ((n1 - 1) as f64) * last_sm1) / (n1 as f64);
		    last_sa1 = (a + ((n1 - 1) as f64) * last_sa1)/ (n1 as f64);
            if last_sa1 != 0.0 {
                rsi1.push(last_sm1 / last_sa1 * 100.0);
			} else {
				rsi1.push(0.0);
			}
        }

         if i == 0{
            last_sm2 = 0.0;
            last_sa2 = 0.0;
            rsi2.push(0.0);
        }else{
            last_sm2 = (m + ((n2 - 1) as f64) * last_sm2) / (n2 as f64);
		    last_sa2 = (a + ((n2 - 1) as f64) * last_sa2)/ (n2 as f64);
            if last_sa2 != 0.0 {
                rsi2.push(last_sm2 / last_sa2 * 100.0);
			} else {
				rsi2.push(0.0);
			}
        }

         if i == 0{
            last_sm3 = 0.0;
            last_sa3 = 0.0;
            rsi3.push(0.0);
        }else{
            last_sm3 = (m + ((n3 - 1) as f64) * last_sm3) / (n3 as f64);
		    last_sa3 = (a + ((n3 - 1) as f64) * last_sa3)/ (n3 as f64);
            if last_sa3 != 0.0 {
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
		if (i as i32) >= days {
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
		if (i as i32) >= n {
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
		if ma_md[iu] != 0.0{
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
    if y1 <= y2{
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
        if plot.m_key1 > 0.0{
            m_index1 = get_chart_index_by_date(chart, plot.m_key1);
            mpx1 = get_chart_x(chart, m_index1);
            mpy1 = get_chart_y(chart, 0, plot.m_value1);
            if mp.x >= mpx1 - plot_point_size && mp.x <= mpx1 + plot_point_size && mp.y >= mpy1 - plot_point_size && mp.y <= mpy1 + plot_point_size{
                splot = plot.clone();
                chart.m_select_plot_point = 0;
                break;
            }
        }
        if plot.m_key2 > 0.0{
            m_index2 = get_chart_index_by_date(chart, plot.m_key2);
            mpx2 = get_chart_x(chart, m_index2);
            mpy2 = get_chart_y(chart, 0, plot.m_value2);
            if mp.x >= mpx2 - plot_point_size && mp.x <= mpx2 + plot_point_size && mp.y >= mpy2 - plot_point_size && mp.y <= mpy2 + plot_point_size{
                splot = plot.clone();
                chart.m_select_plot_point = 1;
                break;
            }
        }
        if plot.m_key3 > 0.0{
            m_index3 = get_chart_index_by_date(chart, plot.m_key3);
            mpx3 = get_chart_x(chart, m_index3);
            mpy3 = get_chart_y(chart, 0, plot.m_value3);
            if mp.x >= mpx3 - plot_point_size && mp.x <= mpx3 + plot_point_size && mp.y >= mpy3 - plot_point_size && mp.y <= mpy3 + plot_point_size{
                splot = plot.clone();
                chart.m_select_plot_point = 2;
                break;
            }
        }
        if chart.m_select_plot_point == -1{
            if plot.m_plot_type == "Line"{
                chart.m_start_move_plot = select_line(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
            }
            else if plot.m_plot_type == "AngleLine"{
                chart.m_start_move_plot = select_line(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_line(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx3, mpy3);
                }
            }
            else if plot.m_plot_type == "Parallel"{
                chart.m_start_move_plot = select_line(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if !chart.m_start_move_plot{
                    let mut k : f32 = 0.0;
                    let mut b : f32 = 0.0;
                    line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
                    let new_b = mpy3 - k * mpx3;
                    if mpx2 == mpx1{
                        if mp.x >= mpx3 - plot_point_size && mp.x <= mpx3 + plot_point_size{
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
            else if plot.m_plot_type == "LRLine"{
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
            }
            else if plot.m_plot_type == "Segment"{
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
            }else if plot.m_plot_type == "Ray"{
                chart.m_start_move_plot = select_ray(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
            }
            else if plot.m_plot_type == "Triangle"{
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx2, mpy2, mpx3, mpy3);
                }
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx3, mpy3);
                }
            }
            else if plot.m_plot_type == "Rect"{
                let s_x1 : f32 = mpx1.min(mpx2);
                let s_y1 : f32 = mpy1.min(mpy2);
                let s_x2 : f32 = mpx1.max(mpx2);
                let s_y2 : f32 = mpy1.max(mpy2);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x2, s_y1);
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x2, s_y1, s_x2, s_y2);
                }
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y2, s_x2, s_y2);
                }
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x1, s_y2);
                }
            }
            else if plot.m_plot_type == "BoxLine"{
                let s_x1 : f32 = mpx1.min(mpx2);
                let s_y1 : f32 = mpy1.min(mpy2);
                let s_x2 : f32 = mpx1.max(mpx2);
                let s_y2 : f32 = mpy1.max(mpy2);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x2, s_y1);
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x2, s_y1, s_x2, s_y2);
                }
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y2, s_x2, s_y2);
                }
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x1, s_y2);
                }
            }
            else if plot.m_plot_type == "TironeLevels"{
                let s_x1 : f32 = mpx1.min(mpx2);
                let s_y1 : f32 = mpy1.min(mpy2);
                let s_x2 : f32 = mpx1.max(mpx2);
                let s_y2 : f32 = mpy1.max(mpy2);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y1, s_x2, s_y1);
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, s_x1, s_y2, s_x2, s_y2);
                }
            }
            else if plot.m_plot_type == "GoldenRatio"{
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
                    if s_y1 <= s_y2{
                        new_y = s_y1 + (s_y2 - s_y1) * ranges[j as usize];
                    }else{
                        new_y = s_y2 + (s_y1 - s_y2) * (1.0 - ranges[j as usize]);
                    }
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, chart.m_left_vscale_width, new_y, chart.m_view.m_size.cx - chart.m_right_vscale_width, new_y);
                    if chart.m_start_move_plot{
                        break;
                    }
                }
            }
            else if plot.m_plot_type == "Cycle"{
                let r = ((mpx2 - mpx1) * (mpx2 - mpx1) + (mpy2 - mpy1) * (mpy2 - mpy1)).abs().sqrt();
                let round = (mp.x - mpx1) * (mp.x - mpx1) + (mp.y - mpy1) * (mp.y - mpy1);
                if round / (r * r) >= 0.9 && round / (r * r) <= 1.1{
                    chart.m_start_move_plot = true;
                }
            }else if plot.m_plot_type == "CircumCycle"{
                let mut o_x : f32 = 0.0;
                let mut o_y : f32 = 0.0;
                let mut r : f32 = 0.0;
                ellipse_or(mpx1, mpy1, mpx2, mpy2, mpx3, mpy3, &mut o_x, &mut o_y, &mut r);
                let round = (mp.x - o_x) * (mp.x - o_x) + (mp.y - o_y) * (mp.y - o_y);
                if round / (r * r) >= 0.9 && round / (r * r) <= 1.1{
                    chart.m_start_move_plot = true;
                }
            }
            else if plot.m_plot_type == "Ellipse"{
                let mut x1 : f32 = 0.0;
                let mut y1 : f32 = 0.0;
                let mut x2 : f32 = 0.0;
                let mut y2 : f32 = 0.0;
                if mpx1 <= mpx2{
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
                if y1 >= y2{
                    height = (y1 - y2) * 2.0;
                }
                else{
                    height = (y2 - y1) * 2.0;
                }
                y = y2 - height / 2.0;
                let a = width / 2.0;
                let b = height / 2.0;
                chart.m_start_move_plot = ellipse_has_point(mp.x, mp.y, x + (width / 2.0), y + (height / 2.0), a, b);
            }else if plot.m_plot_type == "LRBand"{
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if !chart.m_start_move_plot{
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
                    if !chart.m_start_move_plot{
                        mpy1 = get_chart_y(chart, 0, plot.m_value1 - down_sub_value);
                        mpy2 = get_chart_y(chart, 0, plot.m_value2 - down_sub_value);
                        chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                    }
                }
            }else if plot.m_plot_type == "LRChannel"{
                let mut k : f32 = 0.0;
                let mut b : f32 = 0.0;
                line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
                let right_x = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let mut right_y = right_x * k + b;
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, right_x, right_y);
                if !chart.m_start_move_plot{
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
                    if !chart.m_start_move_plot{
                        mpy1 = get_chart_y(chart, 0, plot.m_value1 - down_sub_value);
                        mpy2 = get_chart_y(chart, 0, plot.m_value2 - down_sub_value);
                        line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
                        right_y = right_x * k + b;
                        chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, right_x, right_y);
                    }
                }
            }else if plot.m_plot_type == "ParalleGram"{
                let mut x4 : f32 = 0.0;
                let mut y4 : f32 = 0.0;
                parallelogram(mpx1, mpy1, mpx2, mpy2, mpx3, mpy3, &mut x4, &mut y4);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if !chart.m_start_move_plot{
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx2, mpy2, mpx3, mpy3);
                    if !chart.m_start_move_plot{
                        chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx3, mpy3, x4, y4);
                        if !chart.m_start_move_plot{
                            chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, x4, y4, mpx1, mpy1);
                        }
                    }
                }
            }
            else if plot.m_plot_type == "SpeedResist"{
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if !chart.m_start_move_plot{
                    if mpx1 != mpx2 && mpy1 != mpy2{
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
                        if mpx2 > mpx1{
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
                        if !chart.m_start_move_plot{
                            chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, start_p.x, start_p.y, new_x, new_ys);
                        }
                    }
                }
            }else if plot.m_plot_type == "FiboFanline"{
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, mpy1, mpx2, mpy2);
                if !chart.m_start_move_plot{
                    if mpx1 != mpx2 && mpy1 != mpy2{
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
                            if mpx2 > mpx1{
                                new_y = k * (chart.m_view.m_size.cx - chart.m_right_vscale_width) + b;
                                new_x = (chart.m_view.m_size.cx - chart.m_right_vscale_width);
                            }
                            else
                            {
                                new_y = b;
                                new_x = chart.m_left_vscale_width;
                            }
                            chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, start_p.x, start_p.y, new_x, new_y);
                            if chart.m_start_move_plot{
                                break;
                            }
                        }
                    }
                }
            }
            else if plot.m_plot_type == "FiboTimezone"{
                let mut f_value : i32 = 1;
                let aindex = m_index1;
                let mut pos : i32 = 1;
                let div_height = get_candle_div_height(chart);
                chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, mpx1, 0.0, mpx1, div_height);
                if !chart.m_start_move_plot{
                    while aindex + f_value <= chart.m_last_visible_index{
                        f_value = fibonacci_value(pos);
                        let new_index = aindex + f_value;
                        let new_x = get_chart_x(chart, new_index);
                        chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, new_x, 0.0, new_x, div_height);
                        if chart.m_start_move_plot{
                            break;
                        }
                        pos = pos + 1;
                    }
                }
            }
            else if plot.m_plot_type == "Percent"{
                let list = get_percent_params(mpy1, mpy2);
                for j in 0..list.len(){
                    chart.m_start_move_plot = select_segment(FCPoint{x:mp.x, y:mp.y}, chart.m_left_vscale_width, list[j as usize], chart.m_view.m_size.cx - chart.m_right_vscale_width, list[j as usize]);
                    if chart.m_start_move_plot{
                        break;
                    }
                }
            }
            if chart.m_start_move_plot{
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
        if i > chart.m_first_visible_index{
            M_PAINT.lock().unwrap().add_line(&context, last_x, last_y, x, y);
        }
        last_x = x;
        last_y = y;
        if selected {
            let mut kp_interval = max_visible_record / 30;
            if kp_interval < 2 {
                kp_interval = 3;
            }
            if i % kp_interval == 0 {
                M_PAINT.lock().unwrap().fill_rect(&context, color.clone(), x - 3.0, y - 3.0, x + 3.0, y + 3.0);
            }
        }
    }
    M_PAINT.lock().unwrap().draw_path(&context, color, chart.m_line_width, Vec::new());
	M_PAINT.lock().unwrap().close_path(&context);
}

pub fn draw_chart_stock(context:&std::rc::Rc<web_sys::CanvasRenderingContext2d>, chart:&mut FCChart, clip_rect:FCRect){
    let data_len = chart.m_data.len() as i32;
    if data_len > 0 {
		let candle_height = get_candle_div_height(chart);
        let vol_height = get_vol_div_height(chart);
        let ind_height = get_ind_div_height(chart);
        let mut c_width = (((chart.m_hscale_pixel - 3.0) / 2.0) as i32) as f32;
        let working_area_width = get_chart_workarea_width(chart);
        let max_visible_record = get_max_visible_count(chart, chart.m_hscale_pixel, working_area_width);
        if c_width < 0.0{
            c_width = 0.0;
        }
        let mut is_trend : bool = false;
        if chart.m_cycle == "trend"{
            is_trend = true;
        }
        if is_trend{
            M_PAINT.lock().unwrap().begin_path(&context);
            let mut last_x : f32 = 0.0;
            let mut last_y : f32 = 0.0;
            for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
                let x = get_chart_x(chart, i);
                let close = chart.m_data[i as usize].m_close;
                let close_y = get_chart_y(chart, 0, close);
                if i > chart.m_first_visible_index{
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
            let open_y = get_chart_y(chart, 0, open);
            let close_y = get_chart_y(chart, 0, close);
            let high_y = get_chart_y(chart, 0, high);
            let low_y = get_chart_y(chart, 0, low);
            let mut vol_y = 0.0;
            let mut zero_y = 0.0;
            if vol_height > 0.0{
                let volume = chart.m_data[iu].m_volume;
                vol_y = get_chart_y(chart, 1, volume);
                zero_y = get_chart_y(chart, 1, 0.0);
            }
            if close >= open{
                if is_trend{
                    if vol_height > 0.0{
                        M_PAINT.lock().unwrap().draw_line(&context, chart.m_indicator_colors[6].clone(), chart.m_line_width, Vec::new(), x, vol_y, x, zero_y);
                    }
                }else{
                    M_PAINT.lock().unwrap().draw_line(&context, chart.m_up_color.clone(), chart.m_line_width, Vec::new(), x, high_y, x, low_y);
                    if c_width > 0.0{
                        if close == open{
                            M_PAINT.lock().unwrap().draw_line(&context, chart.m_up_color.clone(), chart.m_line_width, Vec::new(), x - c_width, close_y, x + c_width, close_y);
                        }
                        else{
                            M_PAINT.lock().unwrap().fill_rect(&context, chart.m_up_color.clone(), x - c_width, close_y, x + c_width, open_y);
                        }
                        if vol_height > 0.0{
                            M_PAINT.lock().unwrap().fill_rect(&context, chart.m_up_color.clone(), x - c_width, vol_y, x + c_width, zero_y);
                        }
         
                    }else
                    {
                        if vol_height > 0.0{
                            M_PAINT.lock().unwrap().draw_line(&context, chart.m_up_color.clone(), chart.m_line_width, Vec::new(), x - c_width, vol_y, x + c_width, zero_y);
                        }
                    }
                }
            }else{
                if is_trend{
                    if vol_height > 0.0{
                        M_PAINT.lock().unwrap().draw_line(&context, chart.m_indicator_colors[6].clone(), chart.m_line_width, Vec::new(), x, vol_y, x, zero_y);
                    }
                }else{
                    M_PAINT.lock().unwrap().draw_line(&context, chart.m_down_color.clone(), chart.m_line_width, Vec::new(), x, high_y, x, low_y);
                    if c_width > 0.0{
                        M_PAINT.lock().unwrap().fill_rect(&context, chart.m_down_color.clone(), x - c_width, open_y, x + c_width, close_y);
                        if vol_height > 0.0{
                            M_PAINT.lock().unwrap().fill_rect(&context, chart.m_down_color.clone(), x - c_width, vol_y, x + c_width, zero_y);
                        }
                    }else{
                        if vol_height > 0.0{
                            M_PAINT.lock().unwrap().draw_line(&context, chart.m_down_color.clone(), chart.m_line_width, Vec::new(), x - c_width, vol_y, x + c_width, zero_y);
                        }
                    }
                }
            }
            if chart.m_select_shape == "CANDLE" {
                let mut kp_interval = max_visible_record / 30;
                if kp_interval < 2 {
                    kp_interval = 3;
                }
                if i % kp_interval == 0{
                    if is_trend {
                    } else {
                        M_PAINT.lock().unwrap().fill_rect(&context, chart.m_indicator_colors[0].clone(), x - 3.0, close_y - 3.0, x + 3.0, close_y + 3.0);
                    }
                }
            } else if chart.m_select_shape == "VOL" {
                let mut kp_interval = max_visible_record / 30;
                if kp_interval < 2 {
                    kp_interval = 3;
                }
                if i % kp_interval == 0 {
                    M_PAINT.lock().unwrap().fill_rect(&context, chart.m_indicator_colors[0].clone(), x - 3.0, vol_y - 3.0, x + 3.0, vol_y + 3.0);
                }
            }
            if !is_trend{
                if !has_max_tag{
                    if high == chart.m_candle_max{
                        let tag = to_fixed(high, chart.m_candle_digit);
                        let t_size = M_PAINT.lock().unwrap().text_size(&context, tag.clone(), chart.m_font.clone());
                        M_PAINT.lock().unwrap().draw_text(&context, tag.clone(), chart.m_text_color.clone(), chart.m_font.clone(), x - t_size.cx / 2.0, high_y - t_size.cy / 2.0 - 2.0);
                        has_max_tag = true;
                    }
                }
                if !has_min_tag{
                    if low == chart.m_candle_min{
                        let tag = to_fixed(low, chart.m_candle_digit);
                        let t_size = M_PAINT.lock().unwrap().text_size(&context, tag.clone(), chart.m_font.clone());
                        M_PAINT.lock().unwrap().draw_text(&context, tag.clone(), chart.m_text_color.clone(), chart.m_font.clone(), x - t_size.cx / 2.0, low_y + 2.0 + t_size.cy / 2.0);
                        has_min_tag = true;
                    }
                }
            }
        }
        if !is_trend{
			M_PAINT.lock().unwrap().save(&context);
            M_PAINT.lock().unwrap().set_clip(&context, chart.m_left_vscale_width, 20.0, chart.m_view.m_size.cx - chart.m_right_vscale_width, candle_height);
			if chart.m_main_indicator == "BOLL" {
				if chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "MID"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_mid.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_mid.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "UP"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_up.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_up.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "DOWM"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_down.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_boll_down.clone(), chart.m_indicator_colors[2].clone(), false);
				}
			}else if chart.m_main_indicator == "MA"{
				if chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "5"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma5.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma5.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "10"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma10.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma10.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "20"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma20.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma20.clone(), chart.m_indicator_colors[2].clone(), false);
				}
				if chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "30"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma30.clone(), chart.m_indicator_colors[3].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma30.clone(), chart.m_indicator_colors[3].clone(), false);
				}
				if chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "120"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma120.clone(), chart.m_indicator_colors[4].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma120.clone(), chart.m_indicator_colors[4].clone(), false);
				}
				if chart.m_select_shape == chart.m_main_indicator && chart.m_select_shape_ex == "250"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma250.clone(), chart.m_indicator_colors[5].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 0, chart.m_ma250.clone(), chart.m_indicator_colors[5].clone(), false);
				}
			}
			M_PAINT.lock().unwrap().restore(&context);
        }
        if ind_height > 0.0{
			if chart.m_show_indicator == "MACD" {
				let zero_y = get_chart_y(chart, 2, 0.0);
				for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
					let x = get_chart_x(chart, i);
					let iu = i as usize;
					let macd = chart.m_allmacdarr[iu];
					let macd_y = get_chart_y(chart, 2, macd);
					if macd_y < zero_y {
                        M_PAINT.lock().unwrap().draw_line(&context, chart.m_indicator_colors[3].clone(), chart.m_line_width, Vec::new(), x, macd_y, x, zero_y);
                    } else {
                        M_PAINT.lock().unwrap().draw_line(&context, chart.m_indicator_colors[4].clone(), chart.m_line_width, Vec::new(), x, macd_y, x, zero_y);
                    }
                    if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "MACD"{
						 let mut kp_interval = max_visible_record / 30;
						if kp_interval < 2 {
							kp_interval = 3;
						}
						if i % kp_interval == 0 {
							M_PAINT.lock().unwrap().fill_rect(&context, chart.m_indicator_colors[0].clone(), x - 3.0, macd_y - 3.0, x + 3.0, macd_y + 3.0);
						}
                    }
				}
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "DIF"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_alldifarr.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_alldifarr.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "DEA"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_alldeaarr.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_alldeaarr.clone(), chart.m_indicator_colors[1].clone(), false);
				}
			} else if chart.m_show_indicator == "KDJ" {
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "K"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_k.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_k.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "D"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_d.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_d.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "J"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_j.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_kdj_j.clone(), chart.m_indicator_colors[2].clone(), false);
				}
			} else if chart.m_show_indicator == "RSI" {
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "6"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi1.clone(), chart.m_indicator_colors[5].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi1.clone(), chart.m_indicator_colors[5].clone(), false);
				}
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "12"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi2.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi2.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "24"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi3.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_rsi3.clone(), chart.m_indicator_colors[2].clone(), false);
				}
			}
			else if chart.m_show_indicator == "BIAS" {
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "1"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias1.clone(), chart.m_indicator_colors[5].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias1.clone(), chart.m_indicator_colors[5].clone(), false);
				}
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "2"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias2.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias2.clone(), chart.m_indicator_colors[1].clone(), false);
				}
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "3"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias3.clone(), chart.m_indicator_colors[2].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bias3.clone(), chart.m_indicator_colors[2].clone(), false);
				}
			}
			else if chart.m_show_indicator == "ROC" {
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "ROC"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_roc.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_roc.clone(), chart.m_indicator_colors[0].clone(), false);
				}
	        
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "ROCMA"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_roc_ma.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_roc_ma.clone(), chart.m_indicator_colors[1].clone(), false);
				}
			} else if chart.m_show_indicator == "WR" {
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "1"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_wr1.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_wr1.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "2"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_wr2.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_wr2.clone(), chart.m_indicator_colors[1].clone(), false);
				}
			} else if chart.m_show_indicator == "CCI"{
				if chart.m_select_shape == chart.m_show_indicator{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_cci.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_cci.clone(), chart.m_indicator_colors[0].clone(), false);
				}
			} else if chart.m_show_indicator == "BBI" {
				if chart.m_select_shape == chart.m_show_indicator{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bbi.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_bbi.clone(), chart.m_indicator_colors[0].clone(), false);
				}
			} else if chart.m_show_indicator == "TRIX" {
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "TRIX"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_trix.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_trix.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "TRIXMA"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_trix_ma.clone(), chart.m_indicator_colors[1].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_trix_ma.clone(), chart.m_indicator_colors[1].clone(), false);
				}
	        
			} else if chart.m_show_indicator == "DMA" {
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "DIF"{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_dma1.clone(), chart.m_indicator_colors[0].clone(), true);
				}else{
					draw_chart_lines(&context, chart, clip_rect.clone(), 2, chart.m_dma1.clone(), chart.m_indicator_colors[0].clone(), false);
				}
				
				if chart.m_select_shape == chart.m_show_indicator && chart.m_select_shape_ex == "DIFMA"{
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

       if plot.m_plot_type == "LRLine" || plot.m_plot_type == "LRChannel" || plot.m_plot_type == "LRBand"{
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
        else if plot.m_plot_type == "BoxLine" || plot.m_plot_type == "TironeLevels"{
            get_candle_range(chart, &mut plot, &mut n_high, &mut n_low);
            m_index1 = get_chart_index_by_date(chart, plot.m_key1);
            m_index2 = get_chart_index_by_date(chart, plot.m_key2);
            plot.m_key1 = get_chart_date_by_index(chart, m_index1.min(m_index2));
            plot.m_key2 = get_chart_date_by_index(chart, m_index1.max(m_index2));
            plot.m_value1 = n_high;
            plot.m_value2 = n_low;
        } 
        if plot.m_key1 > 0.0{
            m_index1 = get_chart_index_by_date(chart, plot.m_key1);
            mpx1 = get_chart_x(chart, m_index1);
            mpy1 = get_chart_y(chart, 0, plot.m_value1);
            if chart.m_splot.m_id == plot.m_id
            {
                M_PAINT.lock().unwrap().fill_ellipse(&context, plot.m_point_color.clone(), mpx1 - plot_point_size, mpy1 - plot_point_size, mpx1 + plot_point_size, mpy1 + plot_point_size);
            }
        }
        if plot.m_key2 > 0.0{
            m_index2 = get_chart_index_by_date(chart, plot.m_key2);
            mpx2 = get_chart_x(chart, m_index2);
            mpy2 = get_chart_y(chart, 0, plot.m_value2);
            if chart.m_splot.m_id == plot.m_id
            {
                M_PAINT.lock().unwrap().fill_ellipse(&context, plot.m_point_color.clone(), mpx2 - plot_point_size, mpy2 - plot_point_size, mpx2 + plot_point_size, mpy2 + plot_point_size);
            }
        }
        if plot.m_key3 > 0.0{
            m_index3 = get_chart_index_by_date(chart, plot.m_key3);
            mpx3 = get_chart_x(chart, m_index3);
            mpy3 = get_chart_y(chart, 0, plot.m_value3);
            if chart.m_splot.m_id == plot.m_id
            {
                M_PAINT.lock().unwrap().fill_ellipse(&context, plot.m_point_color.clone(), mpx3 - plot_point_size, mpy3 - plot_point_size, mpx3 + plot_point_size, mpy3 + plot_point_size);
            }
        }
        if plot.m_plot_type == "Line"{
            let mut k : f32 = 0.0;
            let mut b : f32 = 0.0;
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            if mpx2 == mpx1{
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
        }
        else if plot.m_plot_type == "AngleLine"{
            let mut k : f32 = 0.0;
            let mut b : f32 = 0.0;
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            if mpx2 == mpx1{
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
            line_xy(mpx1, mpy1, mpx3, mpy3, 0.0, 0.0, &mut k, &mut b);
            if mpx3 == mpx1{
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
        }
         else if plot.m_plot_type == "Parallel"{
            let mut k : f32 = 0.0;
            let mut b : f32 = 0.0;
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            if mpx2 == mpx1{
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
            let new_b = mpy3 - k * mpx3;
            if mpx2 == mpx1{
               M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx3, 0.0, mpx3, div_height);
            }else{
                let new_x1 = chart.m_left_vscale_width;
                let new_y1 = new_x1 * k + new_b;
                let new_x2 = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let new_y2 = new_x2 * k + new_b;
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x1, new_y1, new_x2, new_y2);
            }
        }
        else if plot.m_plot_type == "Percent"{
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
        else if plot.m_plot_type == "FiboTimezone"{
            let mut f_value : i32 = 1;
            let aindex = m_index1;
            let mut pos : i32 = 1;
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, 0.0, mpx1, div_height);
            let t_size = M_PAINT.lock().unwrap().text_size(&context, "1".to_string(), chart.m_font.clone());
            M_PAINT.lock().unwrap().draw_text(&context, "1".to_string(), chart.m_text_color.clone(), chart.m_font.clone(), mpx1, div_height - t_size.cy / 2.0);
            while aindex + f_value <= chart.m_last_visible_index{
                f_value = fibonacci_value(pos);
                let new_index = aindex + f_value;
                let new_x = get_chart_x(chart, new_index);
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), new_x, 0.0, new_x, div_height);
                let t_size = M_PAINT.lock().unwrap().text_size(&context, f_value.to_string(), chart.m_font.clone());
                M_PAINT.lock().unwrap().draw_text(&context, f_value.to_string(), chart.m_text_color.clone(), chart.m_font.clone(), new_x, div_height - t_size.cy / 2.0);
                pos = pos + 1;
            }
        }
        else if plot.m_plot_type == "SpeedResist"{
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            if mpx1 != mpx2 && mpy1 != mpy2{
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
                if mpx2 > mpx1{
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
        else if plot.m_plot_type == "LRLine"{
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
        }
        else if plot.m_plot_type == "LRBand"{
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
        else if plot.m_plot_type == "LRChannel"{
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
        else if plot.m_plot_type == "FiboFanline"{
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            if mpx1 != mpx2 && mpy1 != mpy2{
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
                    if mpx2 > mpx1{
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
        else if plot.m_plot_type == "Segment"{
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
        }else if plot.m_plot_type == "Ray"{
            let mut k : f32 = 0.0;
            let mut b : f32 = 0.0;
            line_xy(mpx1, mpy1, mpx2, mpy2, 0.0, 0.0, &mut k, &mut b);
            if k != 0.0 || b != 0.0 {
                let left_x = chart.m_left_vscale_width;
                let left_y = left_x * k + b;
                let right_x = chart.m_view.m_size.cx - chart.m_right_vscale_width;
                let right_y = right_x * k + b;
                if mpx1 >= mpx2 {
                    M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), left_x, left_y, mpx1, mpy1);
                } else {
                    M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, right_x, right_y);
                }
            }
            else {
                if mpy1 >= mpy2 {
                    M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx1, 0.0);
                } else {
                    M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx1, div_height);
                }
            }
        }else if plot.m_plot_type == "Triangle"{
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx2, mpy2, mpx3, mpy3);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx3, mpy3);
        }else if plot.m_plot_type == "Rect"{
            let mut s_x1 : f32 = mpx1.min(mpx2);
            let mut s_y1 : f32 = mpy1.min(mpy2);
            let mut s_x2 : f32 = mpx1.max(mpx2);
            let mut s_y2 : f32 = mpy1.max(mpy2);
            M_PAINT.lock().unwrap().draw_rect(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), s_x1, s_y1, s_x2, s_y2);
        }else if plot.m_plot_type == "Cycle"{
            let r = ((mpx2 - mpx1) * (mpx2 - mpx1) + (mpy2 - mpy1) * (mpy2 - mpy1)).abs().sqrt();
            M_PAINT.lock().unwrap().draw_ellipse(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1 - r, mpy1 - r, mpx1 + r, mpy1 + r);
        }else if plot.m_plot_type == "CircumCycle"{
            let mut o_x : f32 = 0.0;
            let mut o_y : f32 = 0.0;
            let mut r : f32 = 0.0;
            ellipse_or(mpx1, mpy1, mpx2, mpy2, mpx3, mpy3, &mut o_x, &mut o_y, &mut r);
            M_PAINT.lock().unwrap().draw_ellipse(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), o_x - r, o_y - r, o_x + r, o_y + r);
        }else if plot.m_plot_type == "Ellipse"{
           let mut x1 : f32 = 0.0;
            let mut y1 : f32 = 0.0;
            let mut x2 : f32 = 0.0;
            let mut y2 : f32 = 0.0;
            if mpx1 <= mpx2{
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
            if y1 >= y2{
                height = (y1 - y2) * 2.0;
            }
            else{
                height = (y2 - y1) * 2.0;
            }
            y = y2 - height / 2.0;
            M_PAINT.lock().unwrap().draw_ellipse(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), x, y, x + width, y + height);
        }else if plot.m_plot_type == "ParalleGram"{
            let mut x4 : f32 = 0.0;
            let mut y4 : f32 = 0.0;
            parallelogram(mpx1, mpy1, mpx2, mpy2, mpx3, mpy3, &mut x4, &mut y4);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx1, mpy1, mpx2, mpy2);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx2, mpy2, mpx3, mpy3);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), mpx3, mpy3, x4, y4);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), x4, y4, mpx1, mpy1);
        }else if plot.m_plot_type == "BoxLine"{
            let s_x1 : f32 = mpx1.min(mpx2);
            let s_y1 : f32 = mpy1.min(mpy2);
            let s_x2 : f32 = mpx1.max(mpx2);
            let s_y2 : f32 = mpy1.max(mpy2);
            M_PAINT.lock().unwrap().draw_rect(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), s_x1, s_y1, s_x2, s_y2);
            let str = "COUNT:".to_string() + &(((m_index2 - m_index1).abs() as i32) + 1).to_string();
            let b_size = M_PAINT.lock().unwrap().text_size(&context, str.clone(), chart.m_font.clone());
            M_PAINT.lock().unwrap().draw_text(&context, str, chart.m_text_color.clone(), chart.m_font.clone(), s_x1 + 2.0, s_y1 + 2.0 + b_size.cy / 2.0);
            let mut close_list: Vec<f64> = Vec::new();
            for j in m_index1..(m_index2 + 1){
                close_list.push(chart.m_data[j as usize].m_close);
            }
            let avg_close = avg_value(close_list);
            let close_y = get_chart_y(chart, 0, avg_close);
            M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), s_x1, close_y, s_x2, close_y);
            let draw_avg = "AVG:".to_string() + &to_fixed(avg_close, chart.m_candle_digit);
            let t_size = M_PAINT.lock().unwrap().text_size(&context, draw_avg.clone(), chart.m_font.clone());
            M_PAINT.lock().unwrap().draw_text(&context, draw_avg.clone(), chart.m_text_color.clone(), chart.m_font.clone(), s_x1 + 2.0, close_y - t_size.cy / 2.0 - 2.0);
        }
        else if plot.m_plot_type == "TironeLevels"{
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
        else if plot.m_plot_type == "GoldenRatio"{
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
                if s_y1 <= s_y2{
                    new_y = s_y1 + (s_y2 - s_y1) * ranges[j as usize];
                }else{
                    new_y =  s_y2 + (s_y1 - s_y2) * (1.0 - ranges[j as usize]);
                }
                M_PAINT.lock().unwrap().draw_line(&context, plot.m_line_color.clone(), plot.m_line_width, Vec::new(), chart.m_left_vscale_width, new_y, chart.m_view.m_size.cx - chart.m_right_vscale_width, new_y);
                let value = get_candle_div_value(chart, FCPoint{x:0.0, y:new_y});
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
    if chart.m_left_vscale_width > 0.0{
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, 0.0, chart.m_left_vscale_width, chart.m_view.m_size.cy - chart.m_hscale_height);
    }
    if chart.m_right_vscale_width > 0.0{
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_view.m_size.cx - chart.m_right_vscale_width, 0.0, chart.m_view.m_size.cx - chart.m_right_vscale_width, chart.m_view.m_size.cy - chart.m_hscale_height);
    }
    if chart.m_hscale_height > 0.0{
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), 0.0, chart.m_view.m_size.cy - chart.m_hscale_height, chart.m_view.m_size.cx, chart.m_view.m_size.cy - chart.m_hscale_height);
    }
    let candle_div_height = get_candle_div_height(chart);
    let vol_div_height = get_vol_div_height(chart);
    let ind_div_height = get_ind_div_height(chart);
    let ind_div_height2 = get_ind_div_height2(chart);
    if vol_div_height > 0.0{
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, candle_div_height, chart.m_view.m_size.cx - chart.m_right_vscale_width, candle_div_height);
    }
    if ind_div_height > 0.0{
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, candle_div_height + vol_div_height, chart.m_view.m_size.cx - chart.m_right_vscale_width, candle_div_height + vol_div_height);
    }
    if ind_div_height2 > 0.0{
        M_PAINT.lock().unwrap().draw_line(&context, chart.m_scale_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, candle_div_height + vol_div_height + ind_div_height, chart.m_view.m_size.cx - chart.m_right_vscale_width, candle_div_height + vol_div_height + ind_div_height);
    }
    let data_len = chart.m_data.len() as i32;
    if data_len > 0 {
		let mut grid_step : f64 = 0.0;
		let mut grid_digit : i32 = 0;
		chart_grid_scale(chart.m_candle_min, chart.m_candle_max,  (candle_div_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom) / 2.0, chart.m_vscale_distance, chart.m_vscale_distance / 2.0, ((candle_div_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom) / chart.m_vscale_distance) as i32, &mut grid_step, &mut grid_digit);
		if grid_step > 0.0{
			let mut draw_values: Vec<f64> = Vec::new();
			let mut is_trend : bool = false;
			if chart.m_cycle == "trend"{
				is_trend = true;
			}
			let mut first_open : f64 = 0.0;
			if is_trend{
				first_open = chart.m_data[chart.m_first_visible_index as usize].m_close;
				let mut sub_value = (chart.m_candle_max - chart.m_candle_min);
				let count = ((candle_div_height - chart.m_candle_padding_top - chart.m_candle_padding_bottom) / chart.m_vscale_distance) as i32;
				if count > 0{
					sub_value = sub_value / (count as f64);
				}
				let mut start = first_open;
				while start < chart.m_candle_max{
					start = start + sub_value;
					if start <= chart.m_candle_max{
						draw_values.push(start);
					}
				}
				start = first_open;
				while start > chart.m_candle_min{
					start -= sub_value;
					if start >= chart.m_candle_min{
						draw_values.push(start);
					}
				}
			}else{
				let mut start : f64 = 0.0;
				if chart.m_candle_min >= 0.0 {
					while start + grid_step < chart.m_candle_min {
						start = start + grid_step;
					}
				} else {
					while start - grid_step > chart.m_candle_min {
						start = start - grid_step;
					}
				}

				while start <= chart.m_candle_max {
					if start > chart.m_candle_min{
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
				if is_trend{
					let diff_range = ((start - first_open) / first_open * 100.0);
					let diff_range_str = to_fixed(diff_range, chart.m_candle_digit);
					if diff_range >= 0.0{
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
		if grid_step > 0.0{
			let mut start : f64 = 0.0;
			if chart.m_vol_min >= 0.0 {
				while start + grid_step < chart.m_vol_min {
					start = start + grid_step;
				}
			} else {
				while start - grid_step > chart.m_vol_min {
					start = start - grid_step;
				}
			}
			while start <= chart.m_vol_max {
				if start > chart.m_vol_min{
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
		if ind_div_height > 0.0{
			chart_grid_scale(chart.m_ind_min, chart.m_ind_max, (ind_div_height - chart.m_ind_padding_top - chart.m_ind_padding_bottom) / 2.0, chart.m_vscale_distance, chart.m_vscale_distance / 2.0, ((ind_div_height - chart.m_ind_padding_top - chart.m_ind_padding_bottom) / chart.m_vscale_distance) as i32, &mut grid_step, &mut grid_digit);
			if grid_step > 0.0{
				let mut start : f64 = 0.0;
				if chart.m_ind_min >= 0.0 {
					while start + grid_step < chart.m_ind_min {
						start = start + grid_step;
					}
				} else {
					while start - grid_step > chart.m_ind_min {
						start = start - grid_step;
					}
				}
				while start <= chart.m_ind_max {
					if start > chart.m_ind_min{
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
		if chart.m_hscale_height > 0.0{
			let mut d_left = chart.m_left_vscale_width + 10.0;
			for i in chart.m_first_visible_index..(chart.m_last_visible_index + 1){
				let x_text = chart.m_data[i as usize].m_date.to_string();
				let t_size = M_PAINT.lock().unwrap().text_size(&context, x_text.clone(), chart.m_font.clone());
				let x = get_chart_x(chart, i);
				let dx = x - t_size.cx / 2.0;
				if dx > d_left && dx < chart.m_view.m_size.cx - chart.m_right_vscale_width - 10.0{
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
    if data_len > 0 {
		let candle_div_height = get_candle_div_height(chart);
		let vol_div_height = get_vol_div_height(chart);
		let ind_div_height = get_ind_div_height(chart);
		let mut cross_line_index = chart.m_cross_stop_index;
		if cross_line_index == -1 {
			cross_line_index = chart.m_last_visible_index;
		}
		if vol_div_height > 0.0{
			let voltxt = "VOL ".to_string() + &to_fixed(chart.m_data[cross_line_index as usize].m_volume, chart.m_vol_digit);
			let vol_size = M_PAINT.lock().unwrap().text_size(&context, voltxt.clone(), chart.m_font.clone());
			M_PAINT.lock().unwrap().draw_text(&context, voltxt.clone(), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width + 5.0, candle_div_height + 5.0 + vol_size.cy / 2.0);
		}
		let mut is_trend : bool = false;
		if !is_trend{
			let mut draw_titles = Vec::new();
			let mut draw_colors = Vec::new();
			if chart.m_main_indicator == "MA" {
                if(chart.m_ma5.len() > 0){
				    draw_titles.push("MA5 ".to_string() + &to_fixed(chart.m_ma5[cross_line_index as usize], chart.m_candle_digit));
                }else{
                    draw_titles.push("MA5".to_string());
                }
                 if(chart.m_ma10.len() > 0){
				    draw_titles.push("MA10 ".to_string() + &to_fixed(chart.m_ma10[cross_line_index as usize], chart.m_candle_digit));
                 }else{
                     draw_titles.push("MA10".to_string());
                 }
                 if(chart.m_ma20.len() > 0){
				    draw_titles.push("MA20 ".to_string() + &to_fixed(chart.m_ma20[cross_line_index as usize], chart.m_candle_digit));
                 }else{
                     draw_titles.push("MA20".to_string());
                 }
                 if(chart.m_ma30.len() > 0){
				    draw_titles.push("MA30 ".to_string() + &to_fixed(chart.m_ma30[cross_line_index as usize], chart.m_candle_digit));
                 }else{
                     draw_titles.push("MA30".to_string());
                 }
                 if(chart.m_ma120.len() > 0){
				    draw_titles.push("MA120 ".to_string() + &to_fixed(chart.m_ma120[cross_line_index as usize], chart.m_candle_digit));
                 }else{
                     draw_titles.push("MA120".to_string());
                 }
                 if(chart.m_ma250.len() > 0){
				    draw_titles.push("MA250 ".to_string() + &to_fixed(chart.m_ma250[cross_line_index as usize], chart.m_candle_digit));
                 }else{
                     draw_titles.push("MA250".to_string());
                 }
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[2].clone());
				draw_colors.push(chart.m_indicator_colors[5].clone());
				draw_colors.push(chart.m_indicator_colors[4].clone());
				draw_colors.push(chart.m_indicator_colors[3].clone());
			} else if chart.m_main_indicator == "BOLL" {
                if(chart.m_boll_mid.len() > 0){
				    draw_titles.push("MID ".to_string() + &to_fixed(chart.m_boll_mid[cross_line_index as usize], chart.m_candle_digit));
                }else{
                    draw_titles.push("MID".to_string());
                }
                if(chart.m_boll_up.len() > 0){
				    draw_titles.push("UP ".to_string() + &to_fixed(chart.m_boll_up[cross_line_index as usize], chart.m_candle_digit));
                }else{
                    draw_titles.push("UP".to_string());
                }
                if(chart.m_boll_down.len() > 0){
				    draw_titles.push("LOW ".to_string() + &to_fixed(chart.m_boll_down[cross_line_index as usize], chart.m_candle_digit));
                }else{
                    draw_titles.push("LOW".to_string());
                }
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
		if ind_div_height > 0.0{
			let mut draw_titles = Vec::new();
			let mut draw_colors = Vec::new();
			if chart.m_show_indicator == "MACD" {
                if(chart.m_alldifarr.len() > 0){
				    draw_titles.push("DIF ".to_string() + &to_fixed(chart.m_alldifarr[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("DIF".to_string());
                }
                if(chart.m_alldeaarr.len() > 0){
				    draw_titles.push("DEA ".to_string() + &to_fixed(chart.m_alldeaarr[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("DEA".to_string());
                }
                if(chart.m_allmacdarr.len() > 0){
				    draw_titles.push("MACD ".to_string() + &to_fixed(chart.m_allmacdarr[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("MACD".to_string());
                }
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[4].clone());
			}else if chart.m_show_indicator == "KDJ" {
                if(chart.m_kdj_k.len() > 0){
				    draw_titles.push("K ".to_string() + &to_fixed(chart.m_kdj_k[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("K".to_string());
                }
                if(chart.m_kdj_d.len() > 0){
				    draw_titles.push("D ".to_string() + &to_fixed(chart.m_kdj_d[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("D".to_string());
                }
                if(chart.m_kdj_j.len() > 0){
				    draw_titles.push("J ".to_string() + &to_fixed(chart.m_kdj_j[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("J".to_string());
                }
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[2].clone());
			}else if chart.m_show_indicator == "RSI" {
                if(chart.m_rsi1.len() > 0){
				    draw_titles.push("RSI6 ".to_string() + &to_fixed(chart.m_rsi1[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("RSI6".to_string());
                }
                if(chart.m_rsi2.len() > 0){
				    draw_titles.push("RSI12 ".to_string() + &to_fixed(chart.m_rsi2[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("RSI12".to_string());
                }
                if(chart.m_rsi3.len() > 0){
				    draw_titles.push("RSI24 ".to_string() + &to_fixed(chart.m_rsi3[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("RSI24".to_string());
                }
				draw_colors.push(chart.m_indicator_colors[5].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[2].clone());
			}
			else if chart.m_show_indicator == "BIAS" {
                if(chart.m_bias1.len() > 0){
				    draw_titles.push("BIAS6 ".to_string() + &to_fixed(chart.m_bias1[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("BIAS6".to_string());
                }
                if(chart.m_bias2.len() > 0){
				    draw_titles.push("BIAS12 ".to_string() + &to_fixed(chart.m_bias2[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("BIAS12".to_string());
                }
                if(chart.m_bias3.len() > 0){
				    draw_titles.push("BIAS24 ".to_string() + &to_fixed(chart.m_bias3[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("BIAS24".to_string());
                }
				draw_colors.push(chart.m_indicator_colors[5].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
				draw_colors.push(chart.m_indicator_colors[2].clone());
			}
			else if chart.m_show_indicator == "ROC" {
                if(chart.m_roc.len() > 0){
				    draw_titles.push("ROC ".to_string() + &to_fixed(chart.m_roc[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("ROC".to_string());
                }
                if(chart.m_roc_ma.len() > 0){
				    draw_titles.push("ROCMA ".to_string() + &to_fixed(chart.m_roc_ma[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("ROCMA".to_string());
                }
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
			}else if chart.m_show_indicator == "WR" {
                if(chart.m_wr1.len() > 0){
				    draw_titles.push("WR5 ".to_string() + &to_fixed(chart.m_wr1[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("WR5".to_string());
                }
                if(chart.m_wr2.len() > 0){
				    draw_titles.push("WR10 ".to_string() + &to_fixed(chart.m_wr2[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("WR10".to_string());
                }
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
			}else if chart.m_show_indicator == "CCI" {
                if(chart.m_cci.len() > 0){
				    draw_titles.push("CCI ".to_string() + &to_fixed(chart.m_cci[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("CCI".to_string());
                }
				draw_colors.push(chart.m_indicator_colors[0].clone());
			}else if chart.m_show_indicator == "BBI" {
                if(chart.m_bbi.len() > 0){
				    draw_titles.push("BBI ".to_string() + &to_fixed(chart.m_bbi[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("BBI".to_string());
                }
				draw_colors.push(chart.m_indicator_colors[0].clone());
			}else if chart.m_show_indicator == "TRIX" {
                if(chart.m_trix.len() > 0){
				    draw_titles.push("TRIX ".to_string() + &to_fixed(chart.m_trix[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("TRIX".to_string());
                }
                if(chart.m_trix_ma.len() > 0){
				    draw_titles.push("TRIXMA ".to_string() + &to_fixed(chart.m_trix_ma[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("TRIXMA".to_string());
                }
				draw_colors.push(chart.m_indicator_colors[0].clone());
				draw_colors.push(chart.m_indicator_colors[1].clone());
			}else if chart.m_show_indicator == "DMA" {
                if(chart.m_dma1.len() > 0){
				    draw_titles.push("MA10 ".to_string() + &to_fixed(chart.m_dma1[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("MA10".to_string());
                }
                if(chart.m_dma2.len() > 0){
				    draw_titles.push("MA50 ".to_string() + &to_fixed(chart.m_dma2[cross_line_index as usize], chart.m_ind_digit));
                }else{
                    draw_titles.push("MA50".to_string());
                }
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
		if chart.m_show_cross_line{
			let mut right_text = String::from("");
			if chart.m_mouse_position.y < candle_div_height {
				right_text = to_fixed(get_chart_value(chart, chart.m_mouse_position.clone()), chart.m_candle_digit);
			}
			else if chart.m_mouse_position.y > candle_div_height && chart.m_mouse_position.y < candle_div_height + vol_div_height {
				right_text = to_fixed(get_chart_value(chart, chart.m_mouse_position.clone()), chart.m_vol_digit);
			}else if chart.m_mouse_position.y > candle_div_height + vol_div_height && chart.m_mouse_position.y < candle_div_height + vol_div_height + ind_div_height{
				right_text = to_fixed(get_chart_value(chart, chart.m_mouse_position.clone()), chart.m_ind_digit);
			}

			let mut draw_y = chart.m_mouse_position.y;
			if draw_y > chart.m_view.m_size.cy - chart.m_hscale_height{
				draw_y = chart.m_view.m_size.cy - chart.m_hscale_height;
			}
			let t_size = M_PAINT.lock().unwrap().text_size(&context, right_text.clone(), chart.m_font.clone());
			if chart.m_left_vscale_width > 0.0{
				M_PAINT.lock().unwrap().fill_rect(&context, chart.m_cross_tip_color.clone(), chart.m_left_vscale_width - t_size.cx, draw_y - t_size.cy / 2.0 - 4.0, chart.m_left_vscale_width, draw_y + t_size.cy / 2.0 + 3.0);
				M_PAINT.lock().unwrap().draw_text(&context, right_text.clone(), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_left_vscale_width - t_size.cx, draw_y);
			}
			if chart.m_right_vscale_width > 0.0{
				M_PAINT.lock().unwrap().fill_rect(&context, chart.m_cross_tip_color.clone(), chart.m_view.m_size.cx - chart.m_right_vscale_width, draw_y - t_size.cy / 2.0 - 4.0, chart.m_view.m_size.cx - chart.m_right_vscale_width + t_size.cx, draw_y + t_size.cy / 2.0 + 3.0);
				M_PAINT.lock().unwrap().draw_text(&context, right_text.clone(), chart.m_text_color.clone(), chart.m_font.clone(), chart.m_view.m_size.cx - chart.m_right_vscale_width, draw_y);
			}
			let mut draw_x = chart.m_mouse_position.x;
			if draw_x < chart.m_left_vscale_width{
				draw_x = chart.m_left_vscale_width;
			}
			if draw_x > chart.m_view.m_size.cx - chart.m_right_vscale_width{
				draw_x = chart.m_view.m_size.cx - chart.m_right_vscale_width;
			}
			M_PAINT.lock().unwrap().draw_line(&context, chart.m_cross_line_color.clone(), chart.m_line_width, Vec::new(), chart.m_left_vscale_width, draw_y, chart.m_view.m_size.cx - chart.m_right_vscale_width, draw_y);
			M_PAINT.lock().unwrap().draw_line(&context, chart.m_cross_line_color.clone(), chart.m_line_width, Vec::new(), draw_x, 0.0, draw_x, chart.m_view.m_size.cy - chart.m_hscale_height);
	        
			if chart.m_cross_stop_index != -1{
				let x_text = chart.m_data[chart.m_cross_stop_index as usize].m_date.to_string();
				let x_size = M_PAINT.lock().unwrap().text_size(&context, x_text.clone(), chart.m_font.clone());
				M_PAINT.lock().unwrap().fill_rect(&context, chart.m_cross_tip_color.clone(), draw_x - x_size.cx / 2.0 - 2.0, candle_div_height + vol_div_height + ind_div_height, draw_x + x_size.cx / 2.0 + 2.0, candle_div_height + vol_div_height + ind_div_height + x_size.cy + 6.0);
				M_PAINT.lock().unwrap().draw_text(&context, x_text.clone(), chart.m_text_color.clone(), chart.m_font.clone(), draw_x - x_size.cx / 2.0, candle_div_height + vol_div_height + ind_div_height + 3.0 + x_size.cy / 2.0);
			}
		 }
     }
}