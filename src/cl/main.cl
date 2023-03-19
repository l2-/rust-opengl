__kernel void main() {
  // const int width = 800; const int height = 600;
  // const int x = get_global_id( 0 ), y = get_global_id( 1 );
  // if (x >= 0 && x < width && y >= 0 && y < height)
  // {
  //     write_imagei(outimg, (int2)(x, y), (int4)(127, 0, 0, 255));
  // }

  // write_only image2d_t outimg
}

__kernel void fill_vbo(write_only image2d_t outimg) {
  int id = get_global_id(0);
//   vbo[id] = (id % 6) / 3 + (id % 2) * (id / 6);
//   vbo[id] /= 3;
}