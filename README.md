
# **svg2png-wasm-rs**

> **Warning**: This project is a work-in-progress and is **not** yet ready for production use. The functionality is not fully implemented, and breaking changes may occur in future versions. Please use with caution.

A Rust-based WebAssembly (WASM) library that converts SVG (Scalable Vector Graphics) to PNG (Portable Network Graphics). This library is designed for use in serverless environments and is optimized for both client-side (browser) and backend (serverless function) usage. 

## **Features**

- **SVG to PNG Conversion**: Converts SVG files to high-quality PNG images.
- **Rust + WASM**: Powered by Rust and compiled to WebAssembly, ensuring high performance and minimal overhead.
- **Serverless Support**: Easily deployable in serverless environments like Vercel, AWS Lambda, and more.
- **Browser and Backend Compatibility**: Can be used both in the browser via WASM or in backend APIs (e.g., Node.js or serverless functions).
- **Lightweight**: Optimized for fast execution with minimal resources, ideal for use in performance-critical applications.

## **Installation**

### **For Frontend (Browser) Usage**:
Once the WASM binary is built and published, you can import it directly in your frontend project (JavaScript/TypeScript):

```bash
npm install svg2png-wasm-rs
```

Import and use the library in your JavaScript code:

```javascript
import { convertSvgToPng } from 'svg2png-wasm-rs';

const svg = `<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100">...</svg>`;
const pngData = convertSvgToPng(svg);

// Handle the PNG data (e.g., display it on the page)
document.getElementById('output').src = URL.createObjectURL(pngData);
```

### **For Backend (Serverless/Node.js) Usage**:
Use the WASM module in a Node.js backend or serverless function:

```bash
npm install svg2png-wasm-rs
```

```javascript
const { convertSvgToPng } = require('svg2png-wasm-rs');

async function handler(req, res) {
  const svg = req.body.svg;
  const pngData = await convertSvgToPng(svg);

  res.setHeader('Content-Type', 'image/png');
  res.send(pngData);
}
```

## **Development**

To build the WASM library locally, you'll need **Rust** and **wasm-pack** installed.

### **Prerequisites**:

- **Rust** (v1.54+): Install via [rust-lang.org](https://www.rust-lang.org/).
- **wasm-pack**: Install via:
  ```bash
  cargo install wasm-pack
  ```

### **Building**:

1. Clone the repository:

   ```bash
   git clone https://github.com/don-hicks/svg2png-wasm-rs.git
   cd svg2png-wasm-rs
   ```

2. Build the project:

   ```bash
   wasm-pack build --target web
   ```

3. After building, you will find the **WASM package** in the `pkg/` directory, which can be imported into your web project.

### **Testing**:
Make sure to test your functions using a local environment for both frontend and backend cases. For frontend testing, you can use a local server to load and test the WASM functionality.

---

## **Contributing**

We welcome contributions to **svg2png-wasm-rs**! To get started:

1. Fork the repository.
2. Clone your fork locally:
   ```bash
   git clone https://github.com/YOUR-USERNAME/svg2png-wasm-rs.git
   ```
3. Make your changes and push them to your fork.
4. Open a **Pull Request** from your fork to the main repository.

### **Coding Guidelines**:
- Follow the existing code style.
- Ensure that any new functionality is tested thoroughly.
- Ensure your changes don’t break existing functionality.

---

## **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## **Acknowledgments**

- **Rust** for its speed and safety.
- **WASM** for bringing Rust to the web and serverless environments.
- **The open-source community** for contributing ideas, tools, and libraries that make projects like this possible.
# svg2png-wasm-rs
