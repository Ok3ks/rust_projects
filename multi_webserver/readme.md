two main protocols in building web servers are hypertext transfer protocol and transmission control protocol. Both protocols ar request-response protocols, where tcp is the lower level protocol which describes how information gets from one server to another but not the content, whereas http builds on tcp by defining the contents of the requests and responses.
bind function in  returns a Result<T,E> and allows nonadministrators only connect on ports higher than 1023, any other connection lesser than 1023 requires administrator privileges
bind function in 'TcpListener' returns a Result<T,E> and allows nonadministrators only connect on ports higher than 1023, any other connection lesser than 1023 requires administrator privileges


### Understanding Server Responses/

HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body

CRLF = carriage return and line feed (\r\n)


### Improving thoroughput of a webserver

- Spawning threads with a threadpool
- Fork/Join model
- Single-threaded async I/O model
- Multi-threaded async I/O model

When trying to design code, writing the client interface first can help guide design. Write API of the code, so it's structured in the way we want to call it, then implement the functionality, rather than implementing the functionality and designing the public API.

<!-- Using Compiler driven development in Rust. -->
Write the code for the function, then use compiler to correct


In communication between threads as in a threadpool, channels is used in Rust, which is multiple producers, one consumer. Channels functions as the queue of jobs.

In the case of a multithreaded application, this means the threadpool holds the sender, while each worker holds the receiver.

Caveat here is receiver from `std::mpsc` is a single while there are multiple workers, with no copy trait implemented, cloning the receiver can lead to tasks being processed multiple times. In this case we use a mutex to ensure a task is processed only once while we use Arc to make multiple copies of the receiver