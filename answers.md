Title: Problem Set 1 Answers
Author: James Wang (jqw3ha)

1.User-Agent: Mozilla/5.0 (X11; Linux i686) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/31.0.1650.63 Safari/537.36
  The User-Agent tells me about the browser I am using. I assume the AppleWebKit is the style of HTML and the Mozilla
  is something about the OS.

2.Perhaps Rust thinks that modifying a global variable in this fashion is unsafe because a global variable is global and doing so would
  change other instances throughout the program that uses the variable. Whether intentional or unintential it may be dangerous to the
  code to allow modifying variables like this.
