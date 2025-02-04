two main protocols in building web servers are hypertext transfer protocol and transmission control protocol. Both protocols ar request-response protocols, where tcp is the lower level protocol which describes how information gets from one server to another but not the content, whereas http builds on tcp by defining the contents of the requests and responses.
bind function in  returns a Result<T,E> and allows nonadministrators only connect on ports higher than 1023, any other connection lesser than 1023 requires administrator privileges
bind function in 'TcpListener' returns a Result<T,E> and allows nonadministrators only connect on ports higher than 1023, any other connection lesser than 1023 requires administrator privileges


### Understanding Server Responses/

HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body

CRLF = carriage return and line feed (\r\n)