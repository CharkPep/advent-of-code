#include <ctype.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char *read_input(const char *filename, int *n) {
  FILE *file = fopen(filename, "rb");
  if (!file) {
    perror("Failed to open file");
    return NULL;
  }

  if (fseek(file, 0, SEEK_END) != 0) {
    perror("Failed to seek file");
    fclose(file);
    return NULL;
  }

  long file_size = ftell(file);
  if (file_size < 0) {
    perror("Failed to tell file size");
    fclose(file);
    return NULL;
  }

  rewind(file);

  char *buffer = (char *)malloc(file_size + 1);
  if (!buffer) {
    perror("Failed to allocate buffer");
    fclose(file);
    return NULL;
  }

  size_t read_size = fread(buffer, 1, file_size, file);
  if (read_size != file_size) {
    perror("Failed to read file content");
    free(buffer);
    fclose(file);
    return NULL;
  }

  if (n) {
    *n = read_size;
  }

  fclose(file);
  return buffer;
}

typedef struct {
  int p;
  char *b;
  int size;
} Parser;

typedef struct {
  int a;
  int b;
} Pair;

Parser new_parser(char *buf, int n) {
  Parser p = {0, buf, n};
  return p;
}

void p_rewind(Parser *self, uint n) {
  if (self->p - n >= 0) {
    self->p -= n;
  }
}
char *peek(Parser *self) {
  if (self->p + 1 > self->size) {
    return NULL;
  }

  return &self->b[self->p + 1];
}
char *next(Parser *self) {
  if (self->p + 1 > self->size) {
    return NULL;
  }

  self->p += 1;
  return &self->b[self->p];
}
char current(Parser *self) { return self->b[self->p]; }
bool assert_eq(Parser *self, char c) { return current(self) == c; }
int parse_int(Parser *self) {
  int a = 0;
  while (isdigit(current(self))) {
    a *= 10;
    a += current(self) - '0';
    next(self);
  }

  return a;
}
void parse_parentheses(Parser *self, Pair *p) {
  if (!assert_eq(self, '(')) {
    return;
  }
  next(self);
  int a = parse_int(self);
  if (!a) {
    p->a = -1;
    return;
  }
  if (!assert_eq(self, ',')) {
    return;
  }
  next(self);
  int b = parse_int(self);
  if (!b) {
    p->b = -1;
    return;
  }
  if (!assert_eq(self, ')')) {
    return;
  }
  next(self);

  p->a = a;
  p->b = b;
}
bool assert_seq(Parser *self, const char *seq) {
  for (int i = 0; i < strlen(seq); i++) {
    if (!assert_eq(self, seq[i])) {
      return false;
    }
    next(self);
  }

  return true;
}

int parse(Parser *self) {
  int mult = 0;
  bool d = true;
  for (int i = 0; i < self->size && current(self); i++) {
    switch (current(self)) {
    case 'm':
      if (!d || !assert_seq(self, "mul")) {
        next(self);
        continue;
      }
      Pair pair = {0, 0};
      parse_parentheses(self, &pair);
      if (pair.a > 0 && pair.b > 0) {
        mult += pair.a * pair.b;
      }
      continue;
    case 'd':
      if (assert_seq(self, "do()")) {
        d = true;
        continue;
      }

      while (current(self) != 'd') {
        p_rewind(self, 1);
      }

      if (assert_seq(self, "don't()")) {
        d = false;
      }

      continue;
    default:
      next(self);
      continue;
    }
  }

  return mult;
}

int main() {
  int n = 0;
  char *input = read_input("input.txt", &n);
  if (!input) {
    return 1;
  }

  Parser p = {0, input, n};
  printf("%d", parse(&p));
  return 0;
}
