#include <stdlib.h>
#include <string.h>
#include <stdio.h>

static int num_malloc_calls = 0;
static int num_calloc_calls = 0;
static int num_free_calls = 0;
static int countdown_to_fail = 0; // 0 = off

typedef struct MemBlock MemBlock;
struct MemBlock
{
    void *ptr_;
    size_t size_;
    MemBlock *next_;
};

static MemBlock *all_blocks = NULL;
static size_t total_mem = 0;
static size_t total_mem_allocated = 0;
static size_t high_water_mark = 0;
static size_t mem_limit = 0;

static int exit_registered = 0;

static void PrintMemInfo(void)
{
    fprintf(stderr, "\nMEMORY INFO:\n");
    fprintf(stderr, "num calls to: malloc = %4d\n", num_malloc_calls);
    fprintf(stderr, "              calloc = %4d\n", num_calloc_calls);
    fprintf(stderr, "              free   = %4d\n", num_free_calls);
    fprintf(stderr, "total_mem: %u\n", (uint32_t)total_mem);
    fprintf(stderr, "total_mem allocated: %u\n", (uint32_t)total_mem_allocated);
    fprintf(stderr, "high-water mark: %u\n", (uint32_t)high_water_mark);
    while (all_blocks != NULL)
    {
        MemBlock *b = all_blocks;
        all_blocks = b->next_;
        free(b);
    }
}

static void Increment(int *const v)
{
    if (!exit_registered)
    {
#if defined(MALLOC_FAIL_AT)
        {
            const char *const malloc_fail_at_str = getenv("MALLOC_FAIL_AT");
            if (malloc_fail_at_str != NULL)
            {
                countdown_to_fail = atoi(malloc_fail_at_str);
            }
        }
#endif
#if defined(MALLOC_LIMIT)
        {
            const char *const malloc_limit_str = getenv("MALLOC_LIMIT");
            if (malloc_limit_str != NULL)
            {
                mem_limit = atoi(malloc_limit_str);
            }
        }
#endif
        (void)countdown_to_fail;
        (void)mem_limit;
        atexit(PrintMemInfo);
        exit_registered = 1;
    }
    ++*v;
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