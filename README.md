# Sqlup

A basic capitalization tool for SQL syntax!

## Usage

```bash
sqlup <filepath>
```

The program will replace any SQL keywords that are fully lowercase.

The idea was to make it so you can write your SQL queries a little easier (lowercase) then convert everything all at once, similar to how a code formatter works.

## Extra

While this is technically a tool intended for SQL syntax, it can technically work as a more general capitalization tool. Simply place the target words into the HashSet in the get_keywords() function, and that should be it! 

Additional SQL keywords that I missed or any other custom options can also be added in this way. 

