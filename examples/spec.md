# FQL - File Query Language

* FQL - File Query Language is a simple file querying language that is used for searching through files*



```

# SELECT content (Lines) from FILE...
SELECT * FROM "C:\temp\workflow2.json";
SELECT TOP(15) FROM "C:\temp\workflow2.json"; //select the top 15 lines from the file...
SELECT TAIL(15) FROM "C:\temp\workflow2.json"; //select bottom 15 lines from the file...

# Searching ....
SELECT * FROM "C:\temp\workflow2.json" WHERE LIKE "contains this word"; //select all lines where this particular phrase is present.
SELECT * FROM "C:\temp\workflow2.json" WHERE NOTLIKE "contains this word"; //select all lines where this particular phrase is NOT present.

# Count....LInes..
SELECT COUNTL FROM "C:\temp\workflow2.json"; // RETURN TOTAL NUMBER OF LINES in this FILE...
SELECT COUNTW FROM "C:\temp\workflow2.json"; //RETURN NUMBER OF WORDS (space-splitted) in this FILE
SELECT COUNTC FROM "C:\temp\workflow2.json"; // RETURN NUMBER OF CHARACTERS in this file...

SELECT COUNTW TOP(30) FROM "C:\temp\workflow2.json"; //COUNT NUMBER OF WORDS in the TOP 30 line of this file...
SELECT COUNTC TOP(30) FROM "C:\temp\workflow2.json"; //COUNT NUMBER OF CHARACTERS in the TOP 30 line of this file...


```