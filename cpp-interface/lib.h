class Renderer
{
public:
    virtual int init() = -1;
    virtual int stage(const char *input) = -1;
    virtual unsigned long[] render(int frames) = -1;
};