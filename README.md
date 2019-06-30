
# Google CTF 2019 (beginners) - "reversing-emoji"

This is a failed attempt at the level of the Google 2019 CTF beginners challenge named "[reversing-emoji](https://capturetheflag.withgoogle.com/#beginners/reversing-emoji)".

_Hint from the level:_

 > Having snooped around like the expert spy you were never trained to be, you found something that takes your interest: "Cookie/www.FriendSpaceBookPlusAllAccessRedPremium.com"  But unbeknownst to you, it was only the  700nm Wavelength herring rather than a delicious cookie that you could have found.   It looks exactly like a credential for another system.  You find yourself in search of a friendly book to read.
 Having already spent some time trying to find a way to gain more intelligence... and learn about those fluffy creatures, you (several)-momentarily divert your attention here.  It's a place of all the individuals in the world sharing large amounts of data with one another. Strangely enough, all of the inhabitants seem to speak using this weird pictorial language. And there is hot disagreement over what the meaning of an eggplant is.
 But not much Cauliflower here.  They must be very private creatures.  SarahH has left open some proprietary tools, surely running this will take you to them.  Decipher this language and move forth!


Downloading and unzipping [the level attachment](./775e97ff94e7dfe79293b62abed7e1ad17cdc6ebc82c4873cdca201c40569624.zip) yields two files '[vm.py](./vm.py)' and '[program](./program)'.

The 'vm.py' file contains a class named `VM` with the comment explaining that it "_Implements a simple stack-based VM_".

The `VM` class contains a bunch of methods such as `push`, `pop`, `jump_to` etc... that contain python implementations of assembly-esque instructions operating on class properties.

It also contains code that maps different emoji symbols to calls of `VM` instance methods.

Towards the bottom of the script is some code that loads the contents of a file and uses it to initialize the `rom` property of a `VM` class instance before starting a loop calling `vm.step`.

Inside the `program` file are the emojis which is the program that `VM` is going to execute.

Running the file you can see it begins to print a URL but slows down seemingly exponentially.

```bash
$ python3 vm.py program
Running ....
http://emoji-t0anaxn
```
CPU usage also spikes when the script is running so it's doing something computationally expensive.

Initially I tried just waiting until enough of the URL was printed for me to guess the full URL as:

http://emoji-t0anaxnr3nacpt4na.web.ctfcompetition.com

This site is full of cat pictures and after spending some time on it I wasn't able to figure out what to do next so I cheated and googled to see if anyone else had already solved this level.

I found this [excellent write-up by Julian Runnels](https://medium.com/@julianrunnels/google-ctf-reversing-a-program-made-of-emojis-b4c8af473278) in which Julian points out that the link I had was a red herring because it's incomplete. The full URL contains additional path segments and when opened shows a page containing the flag.

Even after reading Julian's article I wanted to explore whether the challenge could be solved in another way by speeding up the execution of the program. If I could speed it up enough it should be able print out the full URL.

For the first couple attempts I tried using [nuitka](https://nuitka.net/pages/overview.html) and then [pypy](https://pypy.org) but this didn't work and resulted in "unkown byte" errors when I tried to run the script.

 Partly just because I thought it would be fun I decided to translate `vm.py` to the [rust language](https://www.rust-lang.org).

The result of my efforts are located [here](./rust-vm/src/main.rs).

The rust version is significantly faster than the python version but unfortunately is still far too slow to solve this challenge.

Even thought it didn't work, it was still fun to try this approach and a good learning experience.
