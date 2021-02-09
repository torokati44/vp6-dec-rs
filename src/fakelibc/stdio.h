#include <stdarg.h>
#include <stddef.h>

typedef int FILE;

#define SEEK_SET 0
#define SEEK_CUR 1
#define SEEK_END 2

#define stdin (int *)0
#define stdout (int *)1
#define stderr (int *)2

#define snprintf(STR, SIZE, FORMAT, ...) 0
int vsnprintf(char *s, size_t n, const char *format, va_list arg);
int fprintf(int *stream, const char *format, ... );
size_t fwrite(const void *ptr, size_t size, size_t nmemb, FILE *stream);
int fputs(const char *str, int *stream);