class Renderer
{
public:
    virtual int init() = -1;
    virtual int stage(const char *input) = -1;
    virtual int render(unsigned long *frame_times, size_t frames) = -1;
};