//
// Created by Potato Yao on 2024/2/28.
//

#ifndef MACRO_H
#define MACRO_H

#include "statcode.h"

#define CHECK_INDEX_POSITIVE(index) if (index < 0) return ARGUMENT_NULL_ERROR
#define CHECK_INDEX_OUT_OF_BOUNDS(index, max) if (index < 0 || index > max) return INDEX_OUT_OF_BOUNDS_ERROR

#define CHECK_INDEX_MISMATCH(x, y) if (x != y) return INDEX_MISMATCH_ERROR

#define NOT_NULL_ALLOCATION_OR(size, op) \
    ({ \
        void *_ptr = malloc(size); \
        if (_ptr == NULL) { \
            op; \
            return ALLOCATION_FAILURE_ERROR; \
        } \
        _ptr; \
    })

#define NOT_NULL_ALLOCATION(size) NOT_NULL_ALLOCATION_OR(size, ;)

#define NO_ERROR_FUNC(func) \
    ({ \
        int _stat = func; \
        if (_stat != SUCCESS_CODE) { \
            return _stat; \
        } \
    })

#define CHECK_ARGUMENT_NULL(x) if (x == NULL) return ARGUMENT_NULL_ERROR

#endif //MACRO_H
