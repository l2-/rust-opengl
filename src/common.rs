pub type float = f32;
pub type float2 = (f32, f32);
pub type float3 = (f32, f32, f32);
pub type float4 = (f32, f32, f32, f32);

pub type int = i32;
pub type int2 = (i32, i32);
pub type int3 = (i32, i32, i32);
pub type int4 = (i32, i32, i32, i32);

pub type Vertex = float3;

trait Math<T, H> {
    fn cross(a: T, b: T) -> H;
}

impl Math<int2, int> for int2 {
    fn cross((x1, y1): int2, (x2, y2): int2) -> int {
        return x1 * y2 - y1 * x2;
    }
}