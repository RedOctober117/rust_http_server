# Requests
Structure:
 - request-line:
      - http method (start-line): GET PUT POST HEAD OPTIONS
      - request target: URL, or absolute path of protocol, port, and domain
      - http version: string, defines structure of body
 - headers (dictionary format):
     - request
     - general
     - representation
 - body (optional):
     - single-resource body:
         - one file, with two headers:
             - Content-Type
             - Content-Length
     - multiple-resource body:
         - multipart body, ie. HTML Forms

# Responses
Structure:
 - status line:
     - protocol version
     - status code, ie., 404, 500
     - status text, human readable status code 
 - headers:
     - general, ie., Via
     - response., Vary & Accept-Ranges
     - representation, Content-Type
 - body (optional):
     - single-resource, known length,
     - single-resource, unknown length,
     - multiple-resource, rare

# HTTP/2 Frames
separates data and header frames for compression. multiple streams of data frames can be multiplexed for high concurrency.

