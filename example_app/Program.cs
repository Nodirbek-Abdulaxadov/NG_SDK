using csharp_wrapper;

class Program
{
    [STAThread]
    static void Main(string[] args)
    {
        try
        {
            // Set the printer name or IP address
            string printerName = "192.168.1.100:9100"; // Update with your printer name or address

            // Create Printer API instance
            PrinterAPI printer = new PrinterAPI(printerName);

            // Print text
            printer.PrintText("Hello, ESC/POS Printer!");

            Console.WriteLine("Text printed successfully.");

            // Perform a full paper cut
            printer.CutPaper(true);

            Console.WriteLine("Paper cut successfully.");
        }
        catch (Exception ex)
        {
            Console.WriteLine($"Error: {ex.Message}");
        }

        // var graphicsAPI = new GraphicsAPI();

        // Console.WriteLine("Initializing window...");
        // graphicsAPI.Initialize();

        // Console.WriteLine("Running event loop...");
        // graphicsAPI.Run();

        // Thread.Sleep(10000);

        // Console.WriteLine("Terminating window...");
        // graphicsAPI.Terminate();
    }
}