#include <stdio.h>
#include "include/flutter_gpu_texture_renderer/api.h"
int main() {
  printf("Hello, World!\n");
  int five = fltx_gpu_take_five();
  printf("fltx_gpu_take_five() returned %d\n", five);
  return 0;
}