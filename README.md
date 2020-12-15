# rusty-jpeg-compress

## About 

The program is for educational jpeg compress, 
written for Data Compression Couse 2020, National Central Univesity, Taiwan

## How to run code 

```
$ cargo run -- -c your-raw-file.raw
```


## Total steps for implementation 
- [x] 1. Get in u8 string 
- [x] 2. Convert into YCbCr
- [x] 3. Blockize 
- [x] 4. DCT
- [x] 5. Zigzag
- [ ] 6. quantilization
- [ ] 7. Entropy coding Huffman
- [ ] 8. Add necessary header
