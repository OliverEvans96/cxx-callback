#include <stdio.h>

#include "cxx-callback/include/callback.hpp"

void callme(rust::Fn<int(int)> cb) {
  int x = 3;
  printf("Hello from C++: %d\n", x);
  int ans = cb(x);
  printf("Goodbye from C++: %d\n", ans);
}
