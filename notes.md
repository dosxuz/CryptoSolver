# General notes

## Usage of 2D array

1) Creating a 2D array intialised with a particular number :

```
let mut code:[[i32;100];1000] = [2;100[;1000]];
```

Here the datatype of the array is `i32` and has 100 rows and 1000 columns filled with 2.

2) Accessing an individual element of a 2D array : 

```
println!("{}",code[1][2]);
```

3) Changing an individual element of an array : 

```
code[1][2] = 1;
```

4) Printing the matrix :

```
for (i,row) in code.iter().enumerate() {
	for (j,col) in row.iter().enumerate() {
		println!("[row={}][column={}]={}",i,j,col);
	}
}
```


## Creating a character vector from a given string

```
let mut str_vec : Vec<char> = str_vec.chars().collect();
```

## Separating a string into words 


```
let mut split = "some string 123 abcd".split(" ");
```

This will give us an iterator which you can loop over, or `collect()` into a vector.

```
for s in split {  
	println!("{}",s);
}

let str_vec : Vec<&str> = split.collect();
```

