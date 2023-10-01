using System;
using System.IO.Pipes;

using (NamedPipeServerStream pipeServer = new NamedPipeServerStream("testpipe"))
{
    Console.WriteLine("NamedPipeServerStream object created.");

    // Wait for a client to connect
    Console.Write("Waiting for client connection...");
    pipeServer.WaitForConnection();

    Console.WriteLine("Client connected.");
    
    // Read user input and send that to client process.
    using (StreamReader sr = new StreamReader(pipeServer))
    {
        string temp;
        // Read the stream until end
        while ((temp = sr.ReadLine()) != null)
        {
            Console.WriteLine("Received from client: {0}", temp);
        }
    }
}