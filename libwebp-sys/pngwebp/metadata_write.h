
#include "webp/types.h"
#include "webp/encode.h"
#include "./metadata.h"

#ifdef __cplusplus
extern "C"
{
#endif

    int WriteWebPWithMetadata(WebPMemoryWriter *const out,
                              const WebPPicture *const picture,
                              const WebPMemoryWriter *const memory_writer,
                              const Metadata *const metadata,
                              int keep_metadata,
                              int *const metadata_written);

#ifdef __cplusplus
} // extern "C"
#endif
