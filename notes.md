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

Must support URI's of 8000 octets
```
http-URI = "http" "://" authority path-abempty [ "?" query ]
```

The "origin" for a given URI is the triple of scheme, host, and port after normalizing the scheme and host to lowercase and normalizing the port to remove any leading zeros. If port is elided from the URI, the default port for that scheme is used. For example, the URI

   https://Example.Com/happy.js

would have the origin

   { "https", "example.com", "443" }

which can also be described as the normalized URI prefix with port always present:

   https://example.com:443

Frame example:
```
GET /docs/tutorials/linux/shellscripts/howto.html HTTP/1.1
Host: Linode.com
User-Agent: Mozilla/5.0 (Windows; U; Windows NT 6.1; en-US; rv:1.9.1.8) Gecko/20091102 Firefox/3.5.5
Accept: text/html,application/xhtml+xml,
Accept-Language: en-us
Accept-Encoding: gzip,deflate
Accept-Charset: ISO-8859-1,utf-8
Cache-Control: no-cache
```


# HTTP/2 Frames
separates data and header frames for compression. multiple streams of data frames can be multiplexed for high concurrency.

