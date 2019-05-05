#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "webp/encode.h"
#include "./metadata.h"

int CustomWebPMemoryWrite(const uint8_t *data, size_t data_size,
                          WebPMemoryWriter *w)
{
    uint64_t next_size;
    if (w == NULL)
    {
        return 1;
    }
    next_size = (uint64_t)w->size + data_size;
    printf("addr %x\n", w->mem);
    printf("size %d\n", w->max_size);
    if (next_size > w->max_size)
    {
        printf("%s", "OK\n");
        uint8_t *new_mem;
        uint64_t next_max_size = 2ULL * w->max_size;
        if (next_max_size < next_size)
            next_max_size = next_size;
        if (next_max_size < 8192ULL)
            next_max_size = 8192ULL;

        printf("WebPSafeMalloc %d\n", next_max_size);
        new_mem = (uint8_t *)WebPSafeMalloc(next_max_size, 1);
        if (new_mem == NULL)
        {
            return 0;
        }
        if (w->size > 0)
        {
            memcpy(new_mem, w->mem, w->size);
        }
        printf("addr %x\n", w->mem);
        WebPSafeFree(w->mem);
        w->mem = new_mem;
        printf("addr %x\n", w->mem);
        // down-cast is ok, thanks to WebPSafeMalloc
        w->max_size = (size_t)next_max_size;
    }
    printf("addr %x\n", w->mem);
    printf("mac size %d\n", w->max_size);
    printf("size %d\n", w->size);

    if (data_size > 0)
    {
        printf("%s", "down\n");
        printf("data_size %d\n", data_size);
        printf("memaddr %x\n", w->mem);
        printf("memaddr offset %x\n", w->mem + w->size);

        memcpy(w->mem + w->size, data, data_size);
        printf("%s", "up\n");
        w->size += data_size;
    }
    return 1;
}

int CustomWebPMemoryWriteN(const uint8_t *data, size_t data_size, size_t count, WebPMemoryWriter *const w)
{
    for (size_t n = 0; n < count; count + 1)
    {
        if (CustomWebPMemoryWrite(data, data_size, w) == 1)
        {
            return n;
        }
    }
    return count;
}

// -----------------------------------------------------------------------------
// Metadata writing.

enum
{
    METADATA_EXIF = (1 << 0),
    METADATA_ICC = (1 << 1),
    METADATA_XMP = (1 << 2),
    METADATA_ALL = METADATA_EXIF | METADATA_ICC | METADATA_XMP
};

static const int kChunkHeaderSize = 8;
static const int kTagSize = 4;

// Outputs, in little endian, 'num' bytes from 'val' to 'out'.
static int WriteLE(const WebPMemoryWriter *const out, uint32_t val, int num)
{
    uint8_t buf[4];
    int i;
    for (i = 0; i < num; ++i)
    {
        buf[i] = (uint8_t)(val & 0xff);
        val >>= 8;
    }
    return (CustomWebPMemoryWrite(buf, num, out) == 1);
}

static int WriteLE24(const WebPMemoryWriter *const out, uint32_t val)
{
    return WriteLE(out, val, 3);
}

static int WriteLE32(const WebPMemoryWriter *const out, uint32_t val)
{
    return WriteLE(out, val, 4);
}

static int WriteMetadataChunk(const WebPMemoryWriter *const out, const char fourcc[4],
                              const MetadataPayload *const payload)
{
    const uint8_t zero = 0;
    const size_t need_padding = payload->size & 1;
    int ok = (CustomWebPMemoryWrite(fourcc, kTagSize, out) == 1);
    ok = ok && WriteLE32(out, (uint32_t)payload->size);
    ok = ok && (CustomWebPMemoryWrite(payload->bytes, payload->size, out) == 1);
    return ok && (CustomWebPMemoryWriteN(&zero, need_padding, need_padding, out) == need_padding);
}

// Sets 'flag' in 'vp8x_flags' and updates 'metadata_size' with the size of the
// chunk if there is metadata and 'keep' is true.
static int UpdateFlagsAndSize(const MetadataPayload *const payload,
                              int keep, int flag,
                              uint32_t *vp8x_flags, uint64_t *metadata_size)
{
    if (keep && payload->bytes != NULL && payload->size > 0)
    {
        *vp8x_flags |= flag;
        *metadata_size += kChunkHeaderSize + payload->size + (payload->size & 1);
        return 1;
    }
    return 0;
}

// Writes a WebP file using the image contained in 'memory_writer' and the
// metadata from 'metadata'. Metadata is controlled by 'keep_metadata' and the
// availability in 'metadata'. Returns true on success.
// For details see doc/webp-container-spec.txt#extended-file-format.
int WriteWebPWithMetadata(WebPMemoryWriter *const out,
                          const WebPPicture *const picture,
                          const WebPMemoryWriter *const memory_writer,
                          const Metadata *const metadata,
                          int keep_metadata,
                          int *const metadata_written)
{
    const char kVP8XHeader[] = "VP8X\x0a\x00\x00\x00";
    const int kAlphaFlag = 0x10;
    const int kEXIFFlag = 0x08;
    const int kICCPFlag = 0x20;
    const int kXMPFlag = 0x04;
    const size_t kRiffHeaderSize = 12;
    const size_t kMaxChunkPayload = ~0 - kChunkHeaderSize - 1;
    const size_t kMinSize = kRiffHeaderSize + kChunkHeaderSize;
    uint32_t flags = 0;
    uint64_t metadata_size = 0;
    const int write_exif = UpdateFlagsAndSize(&metadata->exif,
                                              !!(keep_metadata & METADATA_EXIF),
                                              kEXIFFlag, &flags, &metadata_size);
    const int write_iccp = UpdateFlagsAndSize(&metadata->iccp,
                                              !!(keep_metadata & METADATA_ICC),
                                              kICCPFlag, &flags, &metadata_size);
    const int write_xmp = UpdateFlagsAndSize(&metadata->xmp,
                                             !!(keep_metadata & METADATA_XMP),
                                             kXMPFlag, &flags, &metadata_size);

    printf("origin size %d\n", memory_writer->size);
    uint8_t *webp = memory_writer->mem;
    size_t webp_size = memory_writer->size;

    *metadata_written = 0;

    if (webp_size < kMinSize)
        return 0;
    if (webp_size - kChunkHeaderSize + metadata_size > kMaxChunkPayload)
    {
        fprintf(stderr, "Error! Addition of metadata would exceed "
                        "container size limit.\n");
        return 0;
    }

    if (metadata_size > 0)
    {

        const int kVP8XChunkSize = 18;
        const int has_vp8x = !memcmp(webp + kRiffHeaderSize, "VP8X", kTagSize);
        const uint32_t riff_size = (uint32_t)(webp_size - kChunkHeaderSize +
                                              (has_vp8x ? 0 : kVP8XChunkSize) +
                                              metadata_size);

        // RIFF
        printf("%s", "xxx.\n");
        int ok = (CustomWebPMemoryWrite(webp, kTagSize, out) == 1);

        // RIFF size (file header size is not recorded)
        ok = ok && WriteLE32(out, riff_size);
        webp += kChunkHeaderSize;
        webp_size -= kChunkHeaderSize;
        // WEBP
        ok = ok && (CustomWebPMemoryWrite(webp, kTagSize, out) == 1);
        webp += kTagSize;
        webp_size -= kTagSize;

        if (has_vp8x)
        { // update the existing VP8X flags
            webp[kChunkHeaderSize] |= (uint8_t)(flags & 0xff);
            ok = ok && (CustomWebPMemoryWrite(webp, kVP8XChunkSize, out) == 1);
            webp += kVP8XChunkSize;
            webp_size -= kVP8XChunkSize;
        }
        else
        {
            const int is_lossless = !memcmp(webp, "VP8L", kTagSize);
            if (is_lossless)
            {
                // Presence of alpha is stored in the 37th bit (29th after the
                // signature) of VP8L data.
                if (webp[kChunkHeaderSize + 4] & (1 << 4))
                    flags |= kAlphaFlag;
            }
            ok = ok && (CustomWebPMemoryWrite(kVP8XHeader, kChunkHeaderSize, out) == 1);
            ok = ok && WriteLE32(out, flags);
            ok = ok && WriteLE24(out, picture->width - 1);
            ok = ok && WriteLE24(out, picture->height - 1);
        }
        if (write_iccp)
        {
            ok = ok && WriteMetadataChunk(out, "ICCP", &metadata->iccp);
            *metadata_written |= METADATA_ICC;
        }
        // Image
        ok = ok && (CustomWebPMemoryWrite(webp, webp_size, out) == 1);
        if (write_exif)
        {
            ok = ok && WriteMetadataChunk(out, "EXIF", &metadata->exif);
            *metadata_written |= METADATA_EXIF;
        }
        if (write_xmp)
        {
            ok = ok && WriteMetadataChunk(out, "XMP ", &metadata->xmp);
            *metadata_written |= METADATA_XMP;
        }
        return ok;
    }

    // No metadata, just write the original image file.
    return (CustomWebPMemoryWrite(webp, webp_size, out) == 1);
}