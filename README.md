# ipmotd

Shell MOTD/cli utility that's prints random quotes from [inpearls.ru](https://www.inpearls.ru) webstie in specified category or a main page

# usage

``ipmotd (category)``

### example:

``ipmotd юмор``

Prints a random quote in the `юмор` category


# building

``cargo build --release``

``cd target/release``

``./ipmotd``