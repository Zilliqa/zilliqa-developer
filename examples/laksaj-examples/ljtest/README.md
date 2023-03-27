# ljtest

This is a tiny maven test to prove that we uploaded our JAR file to
Maven Central properly and that it can be used to build a trivial
application.

All it tests is that the maven release process completed successfully.

To run it:

```sh
mvn packagej
java -jar target/ljtest-1.0-SNAPSHOT-shaded.jar
```

Enjoy!

<richard@zilliqa.com>
2023-03-20
