#include <stdlib.h>
#include <string.h>
#include <stdio.h>
#include <stdint.h>

#include "./util.h"


#include <stdio.h>

static int num_malloc_calls = 0;
static int num_calloc_calls = 0;
static int num_free_calls = 0;
static int countdown_to_fail = 0;     // 0 = off

typedef struct MemBlock MemBlock;
struct MemBlock {
  void* ptr_;
  size_t size_;
  MemBlock* next_;
};

static MemBlock* all_blocks = NULL;
static size_t total_mem = 0;
static size_t total_mem_allocated = 0;
static size_t high_water_mark = 0;
static size_t mem_limit = 0;

static int exit_registered = 0;

#define Increment(v) do {} while (0)
#define AddMem(p, s) do {} while (0)
#define SubMem(p)    do {} while (0)

// Returns 0 in case of overflow of nmemb * size.
static int CheckSizeArgumentsOverflow(uint64_t nmemb, size_t size) {
  const uint64_t total_size = nmemb * size;
  if (nmemb == 0) return 1;
  if ((uint64_t)size > WEBP_MAX_ALLOCABLE_MEMORY / nmemb) return 0;
  if (total_size != (size_t)total_size) return 0;
#if defined(PRINT_MEM_INFO) && defined(MALLOC_FAIL_AT)
  if (countdown_to_fail > 0 && --countdown_to_fail == 0) {
    return 0;    // fake fail!
  }
#endif
#if defined(MALLOC_LIMIT)
  if (mem_limit > 0) {
    const uint64_t new_total_mem = (uint64_t)total_mem + total_size;
    if (new_total_mem != (size_t)new_total_mem ||
        new_total_mem > mem_limit) {
      return 0;   // fake fail!
    }
  }
#endif

  return 1;
}

void *WebPSafeMalloc(uint64_t nmemb, size_t size)
{
    void *ptr;
    Increment(&num_malloc_calls);
    if (!CheckSizeArgumentsOverflow(nmemb, size))
        return NULL;
    assert(nmemb * size > 0);
    ptr = malloc((size_t)(nmemb * size));
    AddMem(ptr, (size_t)(nmemb * size));
    return ptr;
}
