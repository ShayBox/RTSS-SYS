#define MAX_PATH 260
typedef __int64 LONGLONG;
typedef float FLOAT;
typedef int BOOL;
typedef long LONG;
typedef unsigned char BYTE;
typedef BYTE *PBYTE;
typedef BYTE *LPBYTE;
typedef unsigned long DWORD;
typedef union _LARGE_INTEGER
{
    struct
    {
        DWORD LowPart;
        LONG HighPart;
    } DUMMYSTRUCTNAME;
    struct
    {
        DWORD LowPart;
        LONG HighPart;
    } u;
    LONGLONG QuadPart;
} LARGE_INTEGER;

#include <RTSSSharedMemory.h>
