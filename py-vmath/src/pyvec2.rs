use pyo3::prelude::*;
use vmath::Vec2;

#[pyclass]
pub struct PyVec2 {
	pub vec2: Vec2<f64> // Since generic types aren't allowed
}

impl From<Vec2<f64>> for PyVec2 {
	fn from(vec2: Vec2<f64>) -> Self {
		PyVec2 { vec2 }
	}
}

#[pymethods]
impl PyVec2 {
	#[new]
	fn new(x: f64, y: f64) -> Self {
		let vec2 = Vec2::new(x, y);
		PyVec2 { vec2 }
	}
	fn as_str(&self) -> PyResult<String> {
		Ok(format!("{:?}", self.vec2))
	}

	fn to_tuple(&self) -> PyResult<(f64, f64)> {
		Ok((self.vec2.x, self.vec2.y))
	}
}
