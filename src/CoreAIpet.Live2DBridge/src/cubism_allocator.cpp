// cubism_allocator.cpp — Simple allocator for Cubism Framework
// Uses standard malloc/free for the Framework's memory management.

#include "bridge_internal.h"

#ifdef LIVE2D_HAS_SDK

#include <cstdlib>

namespace CsmAllocator {

using namespace Live2D::Cubism::Framework;

class BridgeAllocator : public ICubismAllocator
{
public:
    void* Allocate(const csmSizeType size) override
    {
        return std::malloc(size);
    }

    void Deallocate(void* memory) override
    {
        std::free(memory);
    }

    void* AllocateAligned(const csmSizeType size, const csmUint32 alignment) override
    {
        return _aligned_malloc(size, alignment);
    }

    void DeallocateAligned(void* alignedMemory) override
    {
        _aligned_free(alignedMemory);
    }
};

static BridgeAllocator g_allocator;

ICubismAllocator* Get() { return &g_allocator; }

} // namespace CsmAllocator

#endif // LIVE2D_HAS_SDK
