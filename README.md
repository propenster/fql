# FQL - File Query Language

FQL (File Query Language) is a simple and powerful file querying language designed to help you efficiently search and extract information from text files. Whether you need to analyze log files, parse JSON, or extract specific content from text files, FQL provides a versatile set of commands to simplify the process.

## Table of Contents

- [Introduction](#introduction)
- [Basic Queries](#basic-queries)
  - [SELECT](#select)
  - [SELECT TOP](#select-top)
  - [SELECT TAIL](#select-tail)
- [Searching](#searching)
  - [SELECT WHERE LIKE](#select-where-like)
  - [SELECT WHERE NOTLIKE](#select-where-notlike)
- [Counting](#counting)
  - [SELECT COUNTL](#select-countl)
  - [SELECT COUNTW](#select-countw)
  - [SELECT COUNTC](#select-countc)
  - [SELECT COUNTW TOP](#select-countw-top)
  - [SELECT COUNTC TOP](#select-countc-top)
- [Compiler Binaries](#compiler-binaries)
- [License](#license)
- [Issues and Pull Requests](#issues-and-pull-requests)
- [Contributing](#contributing)

---

## Introduction

FQL - File Query Language is a versatile tool for querying text files, providing a range of capabilities to efficiently work with textual data. With FQL, you can perform tasks such as extracting lines, counting words, and searching for specific content within your files. Whether you're a developer, data analyst, or system administrator, FQL simplifies the process of working with text files.

## Basic Queries

### SELECT

The `SELECT` statement is used to retrieve content (lines) from a file. You specify the file path within double quotes. For example:
```SELECT * FROM "C:\temp\workflow2.json";```


### SELECT TOP

The `SELECT TOP` statement allows you to select a specific number of lines from the beginning of the file. Replace `15` with the desired number of lines:
```SELECT TOP(15) FROM "C:\temp\workflow2.json";```


### SELECT TAIL

The `SELECT TAIL` statement lets you select a specific number of lines from the end of the file. Replace `15` with the desired number of lines:

```SELECT TAIL(15) FROM "C:\temp\workflow2.json";```


## Searching

### SELECT WHERE LIKE

Use the `SELECT WHERE LIKE` statement to find lines containing a particular phrase. Replace `"contains this word"` with your desired phrase:

```SELECT * FROM "C:\temp\workflow2.json" WHERE LIKE "contains this word";```


### SELECT WHERE NOTLIKE

The `SELECT WHERE NOTLIKE` statement helps you find lines that do not contain a specific phrase. Replace `"contains this word"` with the phrase you want to exclude:

```SELECT * FROM "C:\temp\workflow2.json" WHERE NOTLIKE "contains this word";```


## Counting

### SELECT COUNTL

To count the total number of lines in a file, use the `SELECT COUNTL` statement:

```SELECT COUNTL * FROM "C:\temp\workflow2.json"; //return count of all Lines in this file...```
```SELECT COUNTL TOP(20) FROM "C:\temp\workflow2.json"; // return count of lines from the top 20 lines of the file```

### SELECT COUNTW

To count the number of words (space-separated) in a file, use the `SELECT COUNTW` statement:

```SELECT COUNTW * FROM "C:\temp\workflow2.json"; //return count of words from the entire file```
```SELECT COUNTW TOP(20) FROM "C:\temp\workflow2.json"; // return count of words from the top 20 lines of the file```


### SELECT COUNTC

The `SELECT COUNTC` statement allows you to count the total number of characters in a file:
```SELECT COUNTC * FROM "C:\temp\workflow2.json"; //return count of characters from the entire file```
```SELECT COUNTC TOP(20) FROM "C:\temp\workflow2.json"; // return count of characters from the top 20 lines of the file```



## Compiler Binaries

You can download the FQL compiler binaries from the "bin" folder of this repository.

## License

FQL is licensed under the [MIT License](https://github.com/propenster/fql/blob/main/MIT-LICENSE).

## Issues and Pull Requests

If you encounter issues or have suggestions for improvement, please feel free to [open an issue](../../issues) or [create a pull request](../../pulls).

## Contributing

Contributions are welcome! Read the [Contributing Guidelines](CONTRIBUTING.md) to get started.

---

FQL simplifies the process of working with text files, making it easier to extract information and perform various operations on textual data. Whether you need to extract specific lines, count words, or search for phrases, FQL's intuitive syntax helps you achieve your goals efficiently.





