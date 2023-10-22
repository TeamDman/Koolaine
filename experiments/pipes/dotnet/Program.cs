using System;
using System.IO;
using System.IO.Pipes;

using (NamedPipeServerStream pipeServer = new NamedPipeServerStream("testpipe"))
{
    Console.WriteLine("NamedPipeServerStream object created.");

    // Wait for a client to connect
    Console.Write("Waiting for client connection...");
    pipeServer.WaitForConnection();

    Console.WriteLine("Client connected.");

    using (StreamWriter sw = new StreamWriter(pipeServer))
    {
        while (true)
        {
            sw.Write("Hello World!");
            sw.Flush();
            Thread.Sleep(100);
        }
    }
}