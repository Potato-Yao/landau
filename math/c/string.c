//
// Created by Potato Yao on 2024/2/25.
//

#include "string.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int string_resize(String *s, int new);

int string_init(String **s) {
    String *str = malloc(sizeof(String));
    if (str == NULL) return 1;

    str->size = 0;
    str->capacity = 4;
    str->data = malloc(str->capacity * sizeof(char));
    if (str->data == NULL) {
        free(str);
        return 1;
    }
    str->data[0] = '\0';
    *s = str;

    return 0;
}

static int string_resize(String *s, const int new) {
    if (new < s->capacity) {
        return -1;
    }

    // todo realloc as multiple of 4 byte
    char *new_data = realloc(s->data, new * sizeof(char));
    if (new_data == NULL) {
        return 1;
    }

    s->capacity = new;
    s->data = new_data;

    return 0;
}

int string_append(String *s, const char *add) {
    const int len = strlen(add);
    if (s->size + len + 1 >= s->capacity) {
        const int stat = string_resize(s, s->size + len + 1);
        if (stat != 0) return stat;
    }
    strncat(s->data, add, s->capacity - s->size - 1);
    s->size += len;

    return 0;
}

int string_destroy(String *s) {
    free(s->data);
    s->data = NULL;
    free(s);

    return 0;
}

char *double_to_str(const double value) {
    char *str;
    const int size = snprintf(NULL, 0, "%.*f", CAST_PRECISION, value) + 1;
    str = malloc(size * sizeof(char));

    snprintf(str, size, "%.*f", CAST_PRECISION, value);
    return str;
}
