# plyg

## Name

`plyg` -- A playground utility to safely try out different programming languages in a single program file.

## Synopsis

    ./plyg [extension_of_new_file] [file_to_use]
		
    OR 

    plyg [extension_of_new_file] [file_to_use]

## Description

`plyg` - contracted name from *pl*a*yg*round, is a utility for trying out different programming langauges using a single file.

When learning/using several programming languages, you might want to test/check out some concepts you have learnt in one with the other. You don't have to create a different file to do this.

Like what playgrounds are for, test/check out these concepts in the same file, by stating the language of your choice in a block starting like so `////start <extension of language of choice>` e.g `////start js` or `////start go` or `////start rs`.

The file could start out with _any_ programming langauge extension. See Example under the subtitle `Examples`.

Note that the string that starts the block of code for any language, has no _EMPTY SPACE_ between the _four_ forward slashes, and the word _start_. That is followed with _EMPTY SPACE_ and then the _extension_ of the langauge to use.

You can then write out the syntax of the langauge you want. To start another language just use the same block `////start <lang ext>`.

Every programming language to use in this file, _MUST_ start with `////start <extension of language of choice>` like so:

```
////start c
#include <...>
#include <...>

int main() {
	...
}

////start js

console.log("You are getting it...")
...

```

Each langauge extension specified on the cli, _MUST_ have a block of code in the file. If you specify an extension, which syntax is not in the file, no new file is created. And you are told the language doesn't exist in the file.

You should be able to check out several languages concepts in a single, without having several files with same concepts in different programming languages.

## Examples

Let say you have a file named `practice.go` having the following:

```
////start go
package main

import "fmt"

func main() {
	fmt.Println("hello world")
}

////start js
console.log("Hello, World, ja...");

////start java
import static java.lang.System.out;

class Hello {
    public static void main(String[] args) {
		    out.println("Hello, World Java...");
		}
}

////start rs
fn main() {
    println!("Hello, World of Rusty...");
}

////start c
#include <stdio.h>
#include <stdlib.h>
int main()
{
   char ch, file_name[25];
   FILE *fp;

   printf("Enter name of a file you wish to see\n");
   gets(file_name);

   fp = fopen(file_name, "r"); // read mode

   if (fp == NULL)
   {
      perror("Error while opening the file.\n");
      exit(EXIT_FAILURE);
   }

   printf("The contents of %s file are:\n", file_name);

   while((ch = fgetc(fp)) != EOF)
      printf("%c", ch);

   fclose(fp);
   return 0;
}
```

If you want to see how you can try out `C` program in that same file.

You can do this: `./plyg c practice.go` from the cli.

You get an output saying you now have a new file namely: `practice.c`. The file then looks like so:

```
////start c
#include <stdio.h>
#include <stdlib.h>
int main()
{
   char ch, file_name[25];
   FILE *fp;
////
   printf("Enter name of a file you wish to see\n");
   gets(file_name);
////
   fp = fopen(file_name, "r"); // read mode
////
   if (fp == NULL)
   {
      perror("Error while opening the file.\n");
      exit(EXIT_FAILURE);
   }
////
   printf("The contents of %s file are:\n", file_name);
////
   while ((ch = fgetc(fp)) != EOF)
      printf("%c", ch);
////
   fclose(fp);
   return 0;
}
////start rs
////fn
////main()
////{
////   println !("Hello, World of Rusty...");
////}
////
////start java
////import static java.lang.System.out;
////
////class Hello
////{
////public
////   static void main(String[] args)
////   {
////      out.println("Hello, World Java...");
////   }
////}
////
////start go
////package main
////
////    import "fmt"
////
////    func
////    main(){
////        fmt.Println("hello world")}
////
////start js
////console.log("Hello, World, ja...");
////
```

You can compile your `C` file using any compiler of your choice, and then run to get what you want.

Note that the C program in the practice file is brought to the top of the file and other languages syntax are in comment.

## Installation And Usage

If you have `cargo` installed on your system; simply do like so:

```cargo install plyg```

However, if you download this from the github link. You might have to compile it using `cargo build` or `cargo run`. 

You can then get the file `plyg` or `plyg.exe` from the target folder, then use it from your command line interface without installing it.


## Caveat

1. Presently in this version, you cannot interlace the programming languages.

   Each of the languages have to be in a block of its own.

   #### RIGHT

```

////start js
"use strict";

const fac = (num) => {
    if (num <= 1) {
        return 1;
    } else {
        return num * fac(num-1);
    }
};

[...Array(10).keys()].map((n) => fac(n)).forEach((n) => console.log(n));

////start go
package main

import "fmt"

type SliceParam struct {
	start int
	stop int
	step int
}

func main() {
	for i, val := range newSlice(SliceParam{start: 0, stop: 10, step: 1}) {
		fmt.Println(i, " => ", fac(val))
	}
}

func fac(num int) (result int) {
	if num <= 1 {
		result = 1
	} else {
		result = num * fac(num-1)
	}
		return result
}

func newSlice(arg SliceParam) (arr []int) {
	if arg.stop < arg.start {
		arg.start, arg.stop = arg.stop, arg.start
	}
	arr = make([]int, 0, arg.stop - arg.start)
	for i := arg.start; i < arg.stop; i += arg.step {
		arr = append(arr, i)
	}
	return
}

```

#### WRONG

```

////start js
"use strict";

const fac = (num) => {
	if (num <= 1) {
		return 1;
	} else {
		return num \* fac(num-1);
	}
};

////start go
package main

import "fmt"

type SliceParam struct {
	start int
	stop int
	step int
}

func main() {
	for i, val := range newSlice(SliceParam{start: 0, stop: 10, step: 1}) {
		fmt.Println(i, " => ", fac(val))
	}
}

////start js
[...Array(10).keys()].map((n) => fac(n)).forEach((n) => console.log(n));

////start go
func fac(num int) (result int) {
	if num <= 1 {
		result = 1
	} else {
		result = num \* fac(num-1)
	}
	return result
}

func newSlice(arg SliceParam) (arr []int) {
	if arg.stop < arg.start {
		arg.start, arg.stop = arg.stop, arg.start
	}
	arr = make([]int, 0, arg.stop - arg.start)
	for i := arg.start; i < arg.stop; i += arg.step {
		arr = append(arr, i)
	}
	return
}

```

2. Programming language that has other `comments` other than `//` like python, perl, ruby, haskel or others _does NOT_ play well with this utility for now.

Though they can be used, the user will have to `manually` edit the file after using this program, which defeat the purpose of this utility.
The user just need to run this program specifying extension of the language of choice, and the resulting file is ready for use, no manual editing of the file needed.
