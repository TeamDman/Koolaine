using System;
using System.Runtime.InteropServices;

class Program
{
    [DllImport("kernel32.dll", SetLastError = true, CharSet = CharSet.Unicode)]
    static extern IntPtr CreateFile(
        string lpFileName,
        uint dwDesiredAccess,
        uint dwShareMode,
        IntPtr lpSecurityAttributes,
        uint dwCreationDisposition,
        uint dwFlagsAndAttributes,
        IntPtr hTemplateFile);

    [DllImport("cfapi.dll")]
    static extern int CfHydratePlaceholder(
        IntPtr FileHandle,
        long StartingOffset,
        long Length,
        uint HydrateFlags,
        IntPtr Overlapped);

    [DllImport("kernel32.dll", SetLastError = true)]
    [return: MarshalAs(UnmanagedType.Bool)]
    static extern bool CloseHandle(IntPtr hObject);

    static void Main(string[] args)
    {
        string filePath = @"C:\Users\TeamD\OneDrive\Documents\onedrivetesting\hello.txt";

        IntPtr fileHandle = CreateFile(
            filePath,
            0x80000000,  // GENERIC_READ
            1,  // FILE_SHARE_READ
            IntPtr.Zero,
            3,  // OPEN_EXISTING
            0,
            IntPtr.Zero);

        if (fileHandle == IntPtr.Zero)
        {
            Console.WriteLine("Failed to open file.");
            return;
        }

        int hydrateResult = CfHydratePlaceholder(fileHandle, 0, -1, 0, IntPtr.Zero);

        if (hydrateResult != 0)
        {
            Console.WriteLine($"Failed to hydrate placeholder. Error code: {hydrateResult}");
        }

        CloseHandle(fileHandle);
    }
}
