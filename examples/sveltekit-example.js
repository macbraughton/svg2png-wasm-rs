// Example SvelteKit API route for Cloudflare Workers
// Place this in your SvelteKit project at: src/routes/api/svg-to-png/+server.js

import { convert_svg_to_png, convert_svg_to_png_with_scale } from 'svg2png-wasm-rs';
import { json, error } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function POST({ request }) {
    try {
        const body = await request.json();
        const { svg, width, height, scale } = body;

        if (!svg) {
            throw error(400, 'SVG content is required');
        }

        let pngBytes;

        if (scale && scale !== 1.0) {
            pngBytes = convert_svg_to_png_with_scale(svg, scale);
        } else {
            pngBytes = convert_svg_to_png(svg, width || null, height || null);
        }

        // Return the PNG as a response
        return new Response(pngBytes, {
            headers: {
                'Content-Type': 'image/png',
                'Cache-Control': 'public, max-age=31536000', // Cache for 1 year
            },
        });
    } catch (err) {
        console.error('SVG to PNG conversion error:', err);
        throw error(500, `Conversion failed: ${err.message}`);
    }
}

/** @type {import('./$types').RequestHandler} */
export async function GET({ url }) {
    // Example: GET /api/svg-to-png?svg=<encoded-svg>&width=400&height=300
    const svg = url.searchParams.get('svg');
    const width = url.searchParams.get('width');
    const height = url.searchParams.get('height');
    const scale = url.searchParams.get('scale');

    if (!svg) {
        throw error(400, 'SVG parameter is required');
    }

    try {
        // URL decode the SVG
        const svgContent = decodeURIComponent(svg);
        
        let pngBytes;

        if (scale && parseFloat(scale) !== 1.0) {
            pngBytes = convert_svg_to_png_with_scale(svgContent, parseFloat(scale));
        } else {
            pngBytes = convert_svg_to_png(
                svgContent,
                width ? parseInt(width) : null,
                height ? parseInt(height) : null
            );
        }

        return new Response(pngBytes, {
            headers: {
                'Content-Type': 'image/png',
                'Cache-Control': 'public, max-age=31536000',
            },
        });
    } catch (err) {
        console.error('SVG to PNG conversion error:', err);
        throw error(500, `Conversion failed: ${err.message}`);
    }
}

// Example usage from frontend:
/*
// POST request with JSON body
const response = await fetch('/api/svg-to-png', {
    method: 'POST',
    headers: {
        'Content-Type': 'application/json',
    },
    body: JSON.stringify({
        svg: '<svg xmlns="http://www.w3.org/2000/svg">...</svg>',
        width: 400,
        height: 300
    })
});

const pngBlob = await response.blob();
const imgUrl = URL.createObjectURL(pngBlob);

// Or GET request with URL parameters
const svg = encodeURIComponent('<svg xmlns="http://www.w3.org/2000/svg">...</svg>');
const imgUrl = `/api/svg-to-png?svg=${svg}&width=400&height=300`;
*/