#include <stdarg.h>
#include <string.h>
#include <stdio.h>
#include <stdlib.h>
#include <time.h>

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

int errno;



void* bsearch( const void *key, const void *ptr, size_t count, size_t size,
               int (*comp)(const void*, const void*) )
{
  return ptr;
}


size_t strcspn( const char *dest, const char *src ) {
  return 0;
}
const char * strchr ( const char * str, int character ) {
  return str;
}
double fabs(double x) {
  return x > 0.0 ? x : -x;
}

long long strtoll( const char *restrict str, char **restrict str_end, int base ) {
  return 0;
}
int strerror_r(int errnum, char *strerrbuf, size_t  buflen) {
  return 0;
}
char * strcpy ( char * destination, const char * source ) {
  return destination;
}
int sscanf(const char *str, const char *format, ...) {
  return 1;
}
unsigned long int strtoul (const char* str, char** endptr, int base) {
  return 0;
}
double strtod (const char* str, char** endptr) {
  return 0.0;
}
int open(const char *path, int oflag, ... ) {
  return -1;
}

int read(int fildes, void *buf, size_t nbyte) {
  return -1;
}
int close(int fildes) {
  return -1;
}

clock_t clock(void) {
  return 1;
}
double frexp (double x, int* exp) {
  return x;
}

struct tm *gmtime(const time_t *timer) {
  return 0;
}

struct tm * localtime (const time_t * timer) {
  return 0;
}

time_t mktime(struct tm *timeptr) {
  return 0;
}