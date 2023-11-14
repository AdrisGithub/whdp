# WHDP - Wizards hypermedia protocol parser

---

A library to parse the raw (byte array / string literal / String) types 
into workable types and vice versa.


## Explanation:

---
Http is a text-based protocol, and requests have this format

```text
Method Request-URI HTTP-Version CRLF
headers CRLF
message-body
```

