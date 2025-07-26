messages = [
    "Q: failed",
    "L: failed to compile",
    "Q: crash compilation",
    "L: compiled meshes"
    "L: compilation failed system crash"
]


expected = [
    "ACK: failed; ID=1",
    "M: failed to compile; Q=1"
    "ACK: crash compilation; ID=2",
    # log line does not match anything
    "M: compilation failed system crash; Q=1,2" # match 2 IDs
]
