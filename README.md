A simple lightweight file encryption tool written in Rust using the XOR cipher. It safely encrypts any file and outputs ASCII-friendly text that can be decrypted using the same key. 
To use, make sure you have Rust and Cargo installed and clone this repo. Then, to encrypt a file just run 'cargo run -- encrypt example.txt secretKey' (or whatever file names or key names you'd like), 
and then to decrypt, just run the same command but use decrypt instead of encrypt. Please know that the original file will always remain unchanged and the encrypt function will just create a new file (ending in .enc) 
and then the decrypt function will decrypt said file and return the decrypted text back to the original file, so you can safely delete the contents of the first file after encrypting.
WARNING: This is not secure for real-world use. XOR encryption is easily breakable and should only be used for education or practice projects.
