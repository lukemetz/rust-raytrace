
pub fn power_heuristic(n_f : i32, f_pdf : f32, n_g : i32 , g_pdf : f32) -> f32 {
  let f = (n_f as f32) * f_pdf;
  let g = (n_g as f32) * g_pdf;
  (f*f) / (f*f + g*g)
}
