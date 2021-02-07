#include <stdarg.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>

void abort(void) {
}

size_t fwrite(const void *ptr, size_t size, size_t nmemb, int *stream) {
    return nmemb;
}

int fputs(const char *s, int *stream) {
    return 1;
}

size_t strspn(const char *cs, const char *ct) {
  size_t n;
  const char* p;
  for(n=0; *cs; cs++, n++) {
    for(p=ct; *p && *p != *cs; p++)
      ;
    if (!*p)
      break;
  }
  return n;
}

int vsnprintf(char *s, size_t n, const char *format, va_list arg) {
    return 1;
}

int fprintf(int *stream, const char *format, ...) {
    return 1;
}
