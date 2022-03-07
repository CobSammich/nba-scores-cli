# nba-scores-cli
A command line program to view current NBA scores from the terminal.

![](images/output1.png)

> Game scores are scraped from [NBC
 Sports](https://scores.nbcsports.com/nba/scoreboard.asp?meta=true)

<!-- ## Installation -->

## Usage
To get the scores for the current day:
```bash
./target/release/nba-scores-cli
```

To get the scores from yesterday:
```bash
./target/release/nba-scores-cli -d y
```

To see the games scheduled for tomorrow:
```bash
./target/release/nba-scores-cli -d T
```

To continuously run the program every 10 seconds and display output. This unfortunately causes a
flicker when clearing the terminal and running the program again. Another alternative is to use
`watch`, but this does not preserve the color of the team names.
```bash
while sleep 10; do clear; ./target/release/nba-scores-cli; done
```

I've written a small script to streamline running the program:
```bash
./nba-scores-cli
```


