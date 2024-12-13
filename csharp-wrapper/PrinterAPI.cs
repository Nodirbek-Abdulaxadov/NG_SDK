namespace csharp_wrapper;

public class PrinterAPI
{
    public string PrinterName { get; }

    public PrinterAPI(string printerName)
    {
        PrinterName = printerName ?? throw new ArgumentNullException(nameof(printerName));
    }

    public void PrintText(string text)
    {
        if (string.IsNullOrEmpty(text))
        {
            throw new ArgumentException("Text cannot be null or empty.", nameof(text));
        }

        if (!GraphicsInterop.PrintText(PrinterName, text))
        {
            throw new InvalidOperationException("Failed to print text.");
        }
    }

    public void CutPaper(bool fullCut = true)
    {
        if (!GraphicsInterop.CutPaper(PrinterName, fullCut))
        {
            throw new InvalidOperationException("Failed to cut paper.");
        }
    }
}
