# WHDP - Wizards hypermedia protocol parser

A library to parse the raw (byte array / string literal / String) types 
into workable types and vice versa.


## Explanation:

Http is a text-based protocol. They follow a rather simple format

Requests:
```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

Response:
```text
HTTP-Version Status-Code Reason-Phrase CRLF
headers CRLF
message-body
```


