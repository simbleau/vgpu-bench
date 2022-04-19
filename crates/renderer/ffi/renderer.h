#ifndef RENDERER_H
#define RENDERER_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

    int32_t init();
    int32_t stage(const char *input);
    int32_t render(uint64_t *frame_times, size_t frames);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // RENDERER_H