#include <metal_stdlib>
using namespace metal;

// Twice the signed area of the triangle `a`, `b`, `p`
static float edge(float2 a, float2 b, float2 p) {
    return (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
}

// Draws a triangle with a red top, green bottom left
// and blue bottom right corner, one thread per pixel
//
// Pixels outside the triangle aren't written to
kernel void triangle(
    texture2d<ushort, access::write> out [[texture(0)]],
    uint2 gid [[thread_position_in_grid]]
) {
    uint width = out.get_width();
    uint height = out.get_height();

    if (gid.x >= width || gid.y >= height) {
        return;
    }

    float2 v0 = float2(width / 2, height / 16); // Top
    float2 v1 = float2(width / 16, height - height / 16); // Bottom left
    float2 v2 = float2(width - width / 16, height - height / 16); // Bottom right

    float area = edge(v0, v1, v2);

    // Samples the centre of the pixel
    float2 p = float2(gid) + 0.5;

    // How much of each corner's colour this pixel gets
    float w0 = edge(v1, v2, p) / area;
    float w1 = edge(v2, v0, p) / area;
    float w2 = edge(v0, v1, p) / area;

    if (w0 < 0.0 || w1 < 0.0 || w2 < 0.0) {
        return;
    }

    out.write(ushort4(
        ushort(w0 * 65535.0),
        ushort(w1 * 65535.0),
        ushort(w2 * 65535.0),
        65535
    ), gid);
}
