#include "../../renderer.h"

int32_t init()
{
    // Perform your renderer's initialization.
    return 0;
}
int32_t stage(const char *input)
{
    // Input here comes as the source of an SVG document.
    return 0;
}
int32_t render(uint64_t *frame_times, size_t frames)
{
    for (size_t i = 0; i < frames; i++)
    {
        // Make a call to render a frame here, and record the frame time.
        frame_times[i] = i % 5; // Record the frame's time in nanoseconds here.
    }
    return 0;
}