using System.Runtime.InteropServices;

namespace csharp_wrapper;

public static class GraphicsInterop
{
    static GraphicsInterop()
    {
        string dll = "graphics_engine.dll";

        if (File.Exists(dll))
        {
            NativeLibrary.Load(dll);
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

    [DllImport("graphics_engine.dll", CallingConvention = CallingConvention.Cdecl)]
    private static extern int print_text(string printerName, string text);

    [DllImport("graphics_engine.dll", CallingConvention = CallingConvention.Cdecl)]
    private static extern int cut_paper(string printerName, bool fullCut);

    public static bool PrintText(string printerName, string text)
    {
        int result = print_text(printerName, text);
        return result == 0; // Success if result is 0
    }

    public static bool CutPaper(string printerName, bool fullCut)
    {
        int result = cut_paper(printerName, fullCut);
        return result == 0; // Success if result is 0
    }

    public static string InitializeWindow()
    {
        IntPtr result = initialize_window();
        return Marshal.PtrToStringAnsi(result) ?? "Initialization failed";
    }
}