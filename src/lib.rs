use wasm_bindgen::prelude::*;
use usvg::TreeParsing;

// Import the `console.log` function from the Web API
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make it easier to call console.log
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// Global allocator setup removed - using default allocator

#[cfg_attr(not(test), wasm_bindgen(start))]
pub fn main() {
    console_error_panic_hook::set_once();
}

/// Convert SVG string to PNG bytes
#[wasm_bindgen]
pub fn convert_svg_to_png(svg_content: &str, width: Option<u32>, height: Option<u32>) -> Result<Vec<u8>, JsValue> {
    console_log!("Converting SVG to PNG...");
    
    // Set up default options
    let opt = usvg::Options::default();
    
    // Parse the SVG
    let tree = usvg::Tree::from_str(svg_content, &opt)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse SVG: {}", e)))?;
    
    // Get the SVG size
    let svg_size = tree.size;
    
    // Use provided dimensions or default to SVG dimensions
    let render_width = width.unwrap_or(svg_size.width() as u32);
    let render_height = height.unwrap_or(svg_size.height() as u32);
    
    // Create a pixmap (raster image)
    let mut pixmap = tiny_skia::Pixmap::new(render_width, render_height)
        .ok_or_else(|| JsValue::from_str("Failed to create pixmap"))?;
    
    // Calculate the transform to fit the SVG in the desired dimensions
    let scale_x = render_width as f32 / svg_size.width();
    let scale_y = render_height as f32 / svg_size.height();
    
    let transform = tiny_skia::Transform::from_scale(scale_x, scale_y);
    
    // Render the SVG to the pixmap
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    
    // Convert to PNG bytes
    let png_data = pixmap.encode_png()
        .map_err(|e| JsValue::from_str(&format!("Failed to encode PNG: {}", e)))?;
    
    console_log!("Successfully converted SVG to PNG");
    Ok(png_data)
}

/// Convert SVG string to PNG bytes with custom scale
#[wasm_bindgen]
pub fn convert_svg_to_png_with_scale(svg_content: &str, scale: f32) -> Result<Vec<u8>, JsValue> {
    console_log!("Converting SVG to PNG with scale: {}", scale);
    
    // Set up default options
    let opt = usvg::Options::default();
    
    // Parse the SVG
    let tree = usvg::Tree::from_str(svg_content, &opt)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse SVG: {}", e)))?;
    
    // Get the SVG size and scale it
    let svg_size = tree.size;
    let render_width = (svg_size.width() * scale) as u32;
    let render_height = (svg_size.height() * scale) as u32;
    
    // Create a pixmap (raster image)
    let mut pixmap = tiny_skia::Pixmap::new(render_width, render_height)
        .ok_or_else(|| JsValue::from_str("Failed to create pixmap"))?;
    
    // Create transform with scale
    let transform = tiny_skia::Transform::from_scale(scale, scale);
    
    // Render the SVG to the pixmap
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    
    // Convert to PNG bytes
    let png_data = pixmap.encode_png()
        .map_err(|e| JsValue::from_str(&format!("Failed to encode PNG: {}", e)))?;
    
    console_log!("Successfully converted SVG to PNG with scale");
    Ok(png_data)
}

/// Get SVG dimensions
#[wasm_bindgen]
pub fn get_svg_dimensions(svg_content: &str) -> Result<JsValue, JsValue> {
    let opt = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_content, &opt)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse SVG: {}", e)))?;
    
    let size = tree.size;
    
    let dimensions = js_sys::Object::new();
    js_sys::Reflect::set(&dimensions, &"width".into(), &(size.width() as f64).into())?;
    js_sys::Reflect::set(&dimensions, &"height".into(), &(size.height() as f64).into())?;
    
    Ok(dimensions.into())
}

/// Simple conversion function for the most common use case
#[wasm_bindgen]
pub fn svg_to_png(svg_content: &str) -> Result<Vec<u8>, JsValue> {
    convert_svg_to_png(svg_content, None, None)
}