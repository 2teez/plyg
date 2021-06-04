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
   while((ch = fgetc(fp)) != EOF)
      printf("%c", ch);
////
   fclose(fp);
   return 0;
}
////start rs
////fn main() {
////    println!("Hello, World of Rusty...");
////}
////start java
////import static java.lang.System.out;
////
////class Hello {
////    public static void main(String[] args) {
////		    out.println("Hello, World Java...");
////		}
////}
////
////start go
////package main
////
////import "fmt"
////
////func main() {
////	fmt.Println("hello world")
////}
////start js
////console.log("Hello, World, ja...");
////
