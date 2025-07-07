# **svg2png-wasm-rs**

> **Warning**: This project is a work-in-progress and is **not** yet ready for production use. The functionality is not fully implemented, and breaking changes may occur in future versions. Please use with caution.

A Rust-based WebAssembly (WASM) library that converts SVG (Scalable Vector Graphics) to PNG (Portable Network Graphics). This library is designed for use in serverless environments and is optimized for both client-side (browser) and backend (serverless function) usage.

## **Features**

- **SVG to PNG Conversion**: Converts SVG files to high-quality PNG images with customizable dimensions and scaling
- **Rust + WASM**: Powered by Rust and compiled to WebAssembly, ensuring high performance and minimal overhead
- **Serverless Support**: Easily deployable in serverless environments like Vercel, AWS Lambda, Cloudflare Workers, and more
- **Browser and Backend Compatibility**: Can be used both in the browser via WASM or in backend APIs (e.g., Node.js or serverless functions)
- **Lightweight**: Optimized for fast execution with minimal resources, ideal for use in performance-critical applications
- **Multiple Output Formats**: Supports web, bundler, and Node.js targets

## **Installation**

### **NPM Package**
```bash
npm install svg2png-wasm-rs
```

**Repository**: https://www.npmjs.com/package/svg2png-wasm-rs

### **Crates.io**
```bash
cargo add svg2png-wasm-rs
```

**Repository**: https://github.com/don-hicks/svg2png-wasm-rs

## **Usage**

### **Frontend (Browser) Usage**

Import and use the library in your JavaScript code:

```javascript
import init, { convert_svg_to_png, convert_svg_to_png_with_scale, get_svg_dimensions, svg_to_png } from 'svg2png-wasm-rs';

// Initialize the WASM module
await init();

const svgContent = `<svg xmlns="http://www.w3.org/2000/svg" width="200" height="200" viewBox="0 0 200 200">
    <circle cx="100" cy="100" r="80" fill="blue" stroke="black" stroke-width="2"/>
    <text x="100" y="110" text-anchor="middle" fill="white" font-size="20">Hello SVG!</text>
</svg>`;

// Simple conversion (uses original SVG dimensions)
const pngBytes = svg_to_png(svgContent);

// Conversion with custom dimensions
const pngBytesCustom = convert_svg_to_png(svgContent, 400, 300);

// Conversion with scaling
const pngBytesScaled = convert_svg_to_png_with_scale(svgContent, 2.0);

// Get SVG dimensions
const dimensions = get_svg_dimensions(svgContent);
console.log(`SVG dimensions: ${dimensions.width}x${dimensions.height}`);

// Display the PNG
const blob = new Blob([pngBytes], { type: 'image/png' });
const url = URL.createObjectURL(blob);
document.getElementById('output').src = url;
```

### **Backend (Node.js/Serverless) Usage**

For Node.js or serverless functions, use the Node.js target build:

```javascript
import { convert_svg_to_png, convert_svg_to_png_with_scale, get_svg_dimensions } from 'svg2png-wasm-rs';

// SvelteKit API route example
export async function POST({ request }) {
    try {
        const { svg, width, height, scale } = await request.json();
        
        let pngBytes;
        
        if (scale && scale !== 1.0) {
            pngBytes = convert_svg_to_png_with_scale(svg, scale);
        } else {
            pngBytes = convert_svg_to_png(svg, width || null, height || null);
        }

        return new Response(pngBytes, {
            headers: {
                'Content-Type': 'image/png',
                'Cache-Control': 'public, max-age=31536000',
            },
        });
    } catch (error) {
        console.error('Conversion failed:', error);
        return new Response(`Conversion failed: ${error.message}`, { status: 500 });
    }
}
```

### **Available Functions**

- `svg_to_png(svg_content: string): Uint8Array` - Simple conversion using original SVG dimensions
- `convert_svg_to_png(svg_content: string, width?: number, height?: number): Uint8Array` - Convert with optional custom dimensions  
- `convert_svg_to_png_with_scale(svg_content: string, scale: number): Uint8Array` - Convert with scaling factor
- `get_svg_dimensions(svg_content: string): {width: number, height: number}` - Get SVG dimensions

## **Development**

### **Prerequisites**

- **Rust** (v1.70+): Install via [rust-lang.org](https://www.rust-lang.org/)
- **wasm-pack**: Install via:
  ```bash
  cargo install wasm-pack
  ```

### **Building**

1. Clone the repository:
   ```bash
   git clone https://github.com/don-hicks/svg2png-wasm-rs.git
   cd svg2png-wasm-rs
   ```

2. Build the project for different targets:
   ```bash
   # Build for web (ES modules)
   wasm-pack build --target web --out-dir pkg --release
   
   # Build for bundler (webpack, rollup, etc.)
   wasm-pack build --target bundler --out-dir pkg-bundler --release
   
   # Build for Node.js
   wasm-pack build --target nodejs --out-dir pkg-node --release
   
   # Or build all targets using the build script
   ./build.sh
   ```

3. The WASM packages will be generated in:
   - `pkg/` (web target)
   - `pkg-bundler/` (bundler target)  
   - `pkg-node/` (Node.js target)

### **Testing**

Run the test suite:
```bash
wasm-pack test --headless --chrome
```

Test with the example:
```bash
npm run build
npm run serve
# Visit http://localhost:8000/examples/
```

### **Examples**

The project includes examples in the `examples/` directory:

- `examples/index.html` - Complete browser example with file upload and conversion
- `examples/sveltekit-example.js` - SvelteKit API route for serverless deployment

## **Repository**

**GitHub**: https://github.com/don-hicks/svg2png-wasm-rs

## **Contributing**

We welcome contributions to **svg2png-wasm-rs**! To get started:

1. Fork the repository
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/svg2png-wasm-rs.git
   ```
3. Make your changes and ensure tests pass
4. Push changes to your fork
5. Open a **Pull Request** from your fork to the main repository

### **Coding Guidelines**

- Follow the existing code style
- Ensure that any new functionality is tested thoroughly
- Update documentation for any API changes
- Run `cargo fmt` and `cargo clippy` before submitting

## **License**

This project is licensed under the MIT OR Apache-2.0 License - see the [LICENSE](LICENSE) file for details.

## **Dependencies**

- **resvg** - SVG rendering library
- **tiny-skia** - 2D graphics library
- **usvg** - SVG parser
- **wasm-bindgen** - WebAssembly bindings

## **Acknowledgments**

- **Rust** for its speed and safety
- **WebAssembly** for bringing Rust to the web and serverless environments
- **The resvg ecosystem** for excellent SVG rendering capabilities
- **The open-source community** for contributing ideas, tools, and libraries that make projects like this possible
