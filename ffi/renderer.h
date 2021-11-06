#ifndef RENDERER_H
#define RENDERER_H

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C"
{
#endif

    int32_t renderer_init();
    int32_t renderer_stage(const char *input);
    int32_t renderer_render(const uint64_t *frame_times, size_t frames);

#ifdef __cplusplus
} // extern "C"
#endif

#endif // RENDERER_H