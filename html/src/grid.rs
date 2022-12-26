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
	m_bounds:FCRect,
	m_allowSort:bool
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
			m_sort:String::from("none"),
			m_visible:true,
			m_index:-1,
			m_bounds:FCRect{left:0.0, top:0.0, right:0.0, bottom:0.0},
			m_allowSort:true
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
		if (content_width > grid.m_view.m_size.cx - grid.m_view.m_scroll_size) {
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
		if (content_height > grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size) {
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
			if (content_width > grid.m_view.m_size.cx - grid.m_view.m_scroll_size) {
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
			if (content_height > grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size) {
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
		if (content_width > grid.m_view.m_size.cx - grid.m_view.m_scroll_size) {
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
		if (content_height > grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size) {
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
	if (grid.m_header_height > 0.0 && first_point.y <= grid.m_header_height) {
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
    if (content_height < grid.m_view.m_size.cy - grid.m_header_height - grid.m_view.m_scroll_size) {
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