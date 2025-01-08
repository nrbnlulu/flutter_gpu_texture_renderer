#include "include/flutter_gpu_texture_renderer/api.h"



int fltx_gpu_take_five()
{
  return 5;
}

Foo* fltx_gpu_foo_new()
{
  Foo* foo = (struct Foo*)malloc(sizeof(struct Foo));
  foo->a = 22;
  foo->b = 2;
  return foo;
}