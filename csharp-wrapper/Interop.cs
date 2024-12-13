using System.Runtime.InteropServices;

namespace csharp_wrapper;

public static class GraphicsInterop
{
    static GraphicsInterop()
    {
        string libraryName = "graphics_engine.dll";

        if (File.Exists(libraryName))
        {
            NativeLibrary.Load(libraryName);
        }
        else
        {
            Console.WriteLine("DLL not found!");
        }
    }

    [DllImport("graphics_engine.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern IntPtr initialize_window();

    [DllImport("graphics_engine.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void run_event_loop();

    [DllImport("graphics_engine.dll", CallingConvention = CallingConvention.Cdecl)]
    public static extern void terminate_window();

    public static string InitializeWindow()
    {
        IntPtr result = initialize_window();
        return Marshal.PtrToStringAnsi(result) ?? "Initialization failed";
    }
}