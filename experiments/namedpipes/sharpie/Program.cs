using System;
using System.IO;
using System.IO.Pipes;

class Program
{
    static void Main()
    {
        using (NamedPipeServerStream pipeServer = new NamedPipeServerStream("testpipe"))
        {
            Console.WriteLine("NamedPipeServerStream object created.");

            // Wait for a client to connect
            Console.Write("Waiting for client connection...");
            pipeServer.WaitForConnection();

            Console.WriteLine("Client connected.");

            // Write Fibonacci sequence to client
            using (StreamWriter sw = new StreamWriter(pipeServer))
            {
                sw.AutoFlush = true;
                long a = 0;
                long b = 1;
                while (true)
                {
                    long temp = a;
                    a = b;
                    b = temp + b;

                    sw.WriteLine(a);
                    Console.WriteLine("Sent to client: {0}", a);
                    System.Threading.Thread.Sleep(100);
                }
            }
        }
    }
}
