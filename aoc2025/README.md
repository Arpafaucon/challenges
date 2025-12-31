## Notes

### Day 5

Step 2:
- my first approach was simply to collect all seen indices into a large set of integers...🤯 OOM
- my smarter backup is to process the input fresh ranges and fuse any overlapping ones. Once that's done, the count of fresh ID is simply the sum of their width
