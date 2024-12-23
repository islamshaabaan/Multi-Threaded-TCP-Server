# Solution

# Old Single-Threaded Server Bugs and design flaws:
   1- Single message type client handling:
      The Client struct only handled a single type of message (EchoMessage).
   2- Single client blocking:
      It used a blocking read method, which could stall the server if the client didn’t send data
      When bytes_read == 0, the server returns Ok(()), which cause a "Client Disconnecting" logs infinitly as well.
   3- Static Port Number:
      The port number was fixed "8080" which cause an error in testing since the port your server is trying to bind to is already in use by another process. 
      This can happen if:
         Another instance of your server is already running and bound to the same port.
         Another application is using the same port.
         The previous instance of your server didn't close properly, and the port is still in a "waiting" state (TIME_WAIT).
   4- Single client at a time and single message at a time.
   5- Single-threaded:
      while multiple clients are served, heavy tasks could be low everyone because only one CPU core is used at a time.

# Bugs and Flaws Solutions:
   1- Supports double message types using the oneof field from Protobuf (EchoMessage and AddRequest):
      The server now support both client message "EchoMessage", and "AddRequest".
   2- Implements a non-blocking read mechanism with a match statement to handle various I/O scenarios:
         i.    Ok(0): Indicates the client disconnected.
         ii.   Ok(bytes_read): Decodes and processes messages.
         iii.  Err(ErrorKind::WouldBlock): No data yet, gracefully handles this without blocking.
         iv.   Err(e): Handles other I/O errors explicitly.
         The code now returns an error (ErrorKind::ConnectionAborted) instead of Ok(()) if bytes_read == 0 so it will match the first scenario.
   3- Use Dynamic Port:
      Use the port of the address to be the dynamic port, let port = local_addr.port();,
      a get_port method to retrieve the port programmatically, supporting dynamic port assignment and debugging.
   4- Multi Client serving with scheduling approach to make time sliced concurrency.
   5- Use multi-threaded approch.   