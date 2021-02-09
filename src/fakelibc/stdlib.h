#include <stddef.h>

typedef long long ssize_t;

#define getenv(NAME) 0

long int strtol(const char *str, char **endptr, int base);

long long int strtoll (const char* str, char** endptr, int base);
void qsort(void *, size_t, size_t, int (*)(const void *, const void *));
void abort(void);

double strtod(const char *str, char **endptr);