//
// Created by Potato Yao on 2024/2/25.
//

#ifndef STRING_H
#define STRING_H

#define CAST_PRECISION 4

typedef struct {
    char *data;
    int size;
    int capacity;
} String;

int string_init(String **s);

int string_append(String *s, const char *add);

int string_destroy(String *s);

char *double_to_str(double value);

#endif //STRING_H
