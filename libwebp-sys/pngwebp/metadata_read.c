#include <stdio.h>
#include <stdlib.h>
#include <stddef.h>

#include <libexif/exif-data.h>
#include <libexif/exif-utils.h>
#include <libexif/exif-ifd.h>
#include <libexif/exif-tag.h>

#define NULL 0

static long get_int(ExifData *ed, ExifEntry *ee)
{
    ExifByteOrder o = exif_data_get_byte_order(ed);
    long value;

    switch (ee->format)
    {
    case EXIF_FORMAT_SHORT:
        value = exif_get_short(ee->data, o);
        break;
    case EXIF_FORMAT_LONG:
        value = exif_get_long(ee->data, o);
        break;
    case EXIF_FORMAT_SLONG:
        value = exif_get_slong(ee->data, o);
        break;
    default:
        fprintf(stderr, "get_int oops\n");
        exit(1);
    }
    return value;
}

long ReadMetadata(const unsigned char *data,
                  unsigned int size)
{
    ExifData *ed;
    ed = exif_data_new_from_data(data, size);

    ExifEntry *ee;

    ee = exif_content_get_entry(ed->ifd[EXIF_IFD_0], 0x0112);
    if (NULL == ee)
        return 1; /* top - left */
    long result = get_int(ed, ee);
    exif_data_free(ed);
    exif_entry_free(ed);
    return result;
}