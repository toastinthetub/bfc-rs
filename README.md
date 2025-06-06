<p align="center">
  <a href=""></a>
  <img src="https://drive.google.com/file/d/17bMdrGslpp36oWKKMzc6i86RpPGxz9-2/view?usp=sharing" width="60%" alt="link to my video">
</p>

<h1 align="center">bfc-rs: simple brainfuck to NASM compiler!</h1>

<p align="center">
  slightly optimizing compiler. NASM is required to run this <br/>
  <i>also only works on linux. sorry</i>
</p>

---
## you can run this too!
```bash
# clone this repo
git clone https://github.com/toastinthetub/bfc-rs

# cd
cd bfc-rs

# run it
cargo run -h

```
## and do this to run things
```bash
# after you've compiled something obviously
nasm -f elf64 out.asm -o out.o

# link it with ld (or something)
ld out.o -o out

# run it!
./out
```
## or if you don't want to do that, run these
```bash
# execute runme.py, it'll check if you have the deps
python3 runme.py

# install whatever packages you don't have
sudo apt update && sudo apt install nasm binutils

# run bfc-rs with --feelinglazy flag. compiler will type those scary commands for you
bfc-rs -i input.bf -e executable_file_name --feelinglazy 
