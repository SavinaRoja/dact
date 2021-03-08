# dact
Dedupe And Combine Tool

Created to solve a seemingly basic problem that kept coming up in various work without an obvious simple solution.

Do you have multiple lists (as CSV) of things that you want to de-duplicate and combine on some primary column? DACT is here for you.

## Example

A common usage for this tool is to work with mailing lists. Suppose that we have two files that are coming from different systems:

```
# student_list.csv
id,E-mail,FirstName
00000123,alice@school.edu,Alice
00000456,bob@school.edu,Robert
00010789,cailin@school.edu,Cailin
...
```

and

```
# faculty_list.csv
id,email,first_name,department
f9d8cc1c-4c57-4026-84b6-ea036d32fc33,alice@school.edu,Alice,Biology
c1863c3e-d36a-45c6-9b35-568e17b49ab8,eric@school.edu,Eric,Chemistry
10c1e9d3-3a49-4fd4-9d83-a450c45c1cff,francisca@school.edu,Francisca,Chemistry
...
```

We want to combine these inputs and get the unique set of email addresses, excluding duplications. To perform this job with DACT, we would run the following command:

`dact "student_list.csv|E-mail" "faculty_list.csv|email"`

You can specify a primary header for joining the lists on each input by appending `|<header>` to the input's filename. If you don't do this, it will simply use the first header.

The expected results would be:

```
# deduped_and_combined.csv
id,E-mail,FirstName,email,first_name,department
00000123,alice@school.edu,Alice,,,
00000456,bob@school.edu,Robert,,,
00010789,cailin@school.edu,Cailin,,,
c1863c3e-d36a-45c6-9b35-568e17b49ab8,,,eric@school.edu,Eric,Chemistry
10c1e9d3-3a49-4fd4-9d83-a450c45c1cff,,,francisca@school.edu,Francisca,Chemistry
```

If you'd rather not post-process your output CSV, then you might want to pre-process your input CSVs.
