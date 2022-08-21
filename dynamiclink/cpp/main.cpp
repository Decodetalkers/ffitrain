#include <fromrust.h>
#include <stdio.h>

int main(int argc, char *argv[]) {
  int a = add(10, 20);
  printf("%d\n", a);
  foo();
  return 0;
}
