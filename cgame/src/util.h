#include <stdio.h>
#include <stdlib.h>

#define expect(x)                                                              \
  do {                                                                         \
    if (!(x)) {                                                                \
      fprintf(stderr, "Fatal error: %s:%d: assertion '%s' failed\n", __FILE__, \
              __LINE__, #x);                                                   \
      abort();                                                                 \
    }                                                                          \
  } while (0)
