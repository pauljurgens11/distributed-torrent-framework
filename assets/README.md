# Assets

`example.torrent` in JSON would be:

```json
{
    "announce": "http://localhost:8080/announce",
    "info": {
        "length": 12345,
        "name": "example.txt",
        "piece length": 16384,
        "pieces": [u8; 20] // one SHA-1 hash (20 bytes)
    }
}
```
