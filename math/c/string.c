//
// Created by Potato Yao on 2024/2/25.
//

#include "string.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "macro.h"

static int string_resize(String *s, int new);

int string_init(String **s) {
    String *str = NOT_NULL_ALLOCATION(sizeof(String));

    str->size = 0;
    str->capacity = 4;
    str->data = NOT_NULL_ALLOCATION_OR(str->capacity * sizeof(char), free(str));
    str->data[0] = '\0';
    *s = str;

    return SUCCESS_CODE;
}

static int string_resize(String *s, const int new) {
    if (new < s->capacity) {
        return INDEX_OUT_OF_BOUNDS_ERROR;
    }

    // realloc to the smallest multiple of eight that greater than 'new'
    // in order to reduce calls to realloc
    const int re = (new + 7) / 8 * 8;
    char *new_data = realloc(s->data, re * sizeof(char));
    if (new_data == NULL) return ALLOCATION_FAILURE_ERROR;

    s->capacity = new;
    s->data = new_data;

    return SUCCESS_CODE;
}

int string_append(String *s, const char *add) {
    const int len = strlen(add);
    if (s->size + len + 1 >= s->capacity) {
        const int stat = string_resize(s, s->size + len + 1);
        if (stat != 0) return stat;
    }
    strncat(s->data, add, s->capacity - s->size - 1);
    s->size += len;

    return SUCCESS_CODE;
}

int string_destroy(String *s) {
    free(s->data);
    s->data = NULL;
    free(s);

    return SUCCESS_CODE;
}

char *double_to_str(const double value) {
    const int size = snprintf(NULL, 0, "%.*f", CAST_PRECISION, value) + 1;
    char *str = malloc(size * sizeof(char));

    snprintf(str, size, "%.*f", CAST_PRECISION, value);
    return str;
}
