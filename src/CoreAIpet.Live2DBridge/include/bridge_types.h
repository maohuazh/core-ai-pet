#ifndef BRIDGE_TYPES_H
#define BRIDGE_TYPES_H

#include <windows.h>

// 与 C# 共享的结构体定义 (LayoutKind.Sequential)

#pragma pack(push, 1)

typedef struct {
    float X;
    float Y;
    float Z;
} Vector3;

typedef struct {
    float R;
    float G;
    float B;
    float A;
} Color4;

typedef struct {
    int Width;
    int Height;
    float DpiScale;
} RenderConfig;

#pragma pack(pop)

#endif // BRIDGE_TYPES_H
