#ifndef WEBP_UTILS_UTILS_H_
#define WEBP_UTILS_UTILS_H_

#ifndef WEBP_EXTERN
// This explicitly marks library functions and allows for changing the
// signature for e.g., Windows DLL builds.
#if defined(__GNUC__) && __GNUC__ >= 4
#define WEBP_EXTERN extern __attribute__((visibility("default")))
#else
#define WEBP_EXTERN extern
#endif /* __GNUC__ >= 4 */
#endif /* WEBP_EXTERN */

#ifdef HAVE_CONFIG_H
#include "webp/config.h"
#endif

#include <assert.h>
#include <limits.h>

#ifdef __cplusplus
extern "C" {
#endif

//------------------------------------------------------------------------------
// Memory allocation

// This is the maximum memory amount that libwebp will ever try to allocate.
#ifndef WEBP_MAX_ALLOCABLE_MEMORY
#if SIZE_MAX > (1ULL << 34)
#define WEBP_MAX_ALLOCABLE_MEMORY (1ULL << 34)
#else
// For 32-bit targets keep this below INT_MAX to avoid valgrind warnings.
#define WEBP_MAX_ALLOCABLE_MEMORY ((1ULL << 31) - (1 << 16))
#endif
#endif  // WEBP_MAX_ALLOCABLE_MEMORY   

#define WEBP_ALIGN_CST 31
#define WEBP_ALIGN(PTR) (((uintptr_t)(PTR) + WEBP_ALIGN_CST) & ~WEBP_ALIGN_CST)

#ifdef __cplusplus
} // extern "C"
#endif

#endif // WEBP_UTILS_UTILS_H_