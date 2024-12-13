using csharp_wrapper;

class Program
{
    [STAThread]
    static void Main(string[] args)
    {
        var graphicsAPI = new GraphicsAPI();

        Console.WriteLine("Initializing window...");
        graphicsAPI.Initialize();

        Console.WriteLine("Running event loop...");
        graphicsAPI.Run();

        Thread.Sleep(10000);

        Console.WriteLine("Terminating window...");
        graphicsAPI.Terminate();
    }
}