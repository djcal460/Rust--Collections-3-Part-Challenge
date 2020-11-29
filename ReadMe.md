# Rustastic: Collections 3-Part Challenge 

## Rust Challenge Part 1
Given a list of integers, use a vector and return the mean, median, and mode. Mean is calculated and the integers are cast to a float. Median returns the middle value for odd-numbered vectors, or averages the middle two elements for even-numbered vectors. Mode returns the most common element, however if all elements are unique, a message is displayed saying there is no mode. 

## Rust Challenge Part 2
Convert strings to pig latin. The first consonant of each word is moved to the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added to the end instead “apple” becomes “apple-hay”). Takes into account 2-4 byte UTF-8 characters, so is able to convert other languages to pig latin as well. 

## Rust Challenge Part 3
Using a hash map and vectors, I created a text interface that allows users to add employee names to a departments in a company. The user selects the number of workers they want to enter. Then enters the worker and gives an associated department. Then the user can retrieve a list of all people in a particular department or list all people in the company sorted alphabetically with their appropriate department.

### Tech
Tech uses the very very very advanced terminal shell and cargo compiler.

### Run the File
```
$ cargo run
```