namespace csharp_wrapper;

public class GraphicsAPI
{
    public void Initialize()
    {
        string result = GraphicsInterop.InitializeWindow();
        Console.WriteLine(result);
    }

    public void Run()
    {
        GraphicsInterop.run_event_loop();
    }

    public void Terminate()
    {
        GraphicsInterop.terminate_window();
    }
}