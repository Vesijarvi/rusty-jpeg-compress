# rusty-jpeg-compress

## About 

The program is for educational jpeg compress. 
  
Written for Data Compression Couse 2020, National Central Univesity, Taiwan

## How to run code 

```
$ cargo run -- -c your-raw-file.raw
```


## Total steps for implementation 
- [x] 1. Read file in as u8 vector streaem 
- [x] 2. Convert u8 into YCbCr structure
- [x] 3. Blockize to MCUs
- [x] 4. DCT
- [x] 5. Zigzag
- [ ] 6. Quantilization
- [ ] 7. Entropy coding Huffman
- [ ] 8. Add necessary header
