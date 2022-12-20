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