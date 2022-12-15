# leaktk-jf

A utility/lib for taking JSON and converting it to a directory tree for scanning.

## WARNING - Very WIP

This project isn't ready yet.

TODO:

* Unit tests
* Make arg parsing a little cleaner
* Better security checks (potentially done but needs to be tested)
* Return correct exit codes on an error
* Replace println with `info!` statments

Nice to Have:

* Optionally provide your own regex with a `--valid-key-regex` flag that
defaults to `^[\w\-\+]+$`.
* Be able to run it in reverse to take a directory tree and turn it into JSON
  * A dir full of only numbers with no missing numbers in a range would be
    parsed as an array.


## Example Usage

```sh
$ rm -rf /tmp/jf-example
$ curl https://api.github.com/events | cargo run -- /tmp/jf-example/gh-events
Writing /tmp/jf-example/gh-events/0/actor
Writing /tmp/jf-example/gh-events/0/actor/avatar_url
Writing /tmp/jf-example/gh-events/0/actor/display_login
Writing /tmp/jf-example/gh-events/0/actor/gravatar_id
Writing /tmp/jf-example/gh-events/0/actor/id
Writing /tmp/jf-example/gh-events/0/actor/login
Writing /tmp/jf-example/gh-events/0/actor/url
Writing /tmp/jf-example/gh-events/0/created_at
Writing /tmp/jf-example/gh-events/0/id
Writing /tmp/jf-example/gh-events/0/payload
Writing /tmp/jf-example/gh-events/0/payload/before
Writing /tmp/jf-example/gh-events/0/payload/commits
Writing /tmp/jf-example/gh-events/0/payload/commits/0/author
Writing /tmp/jf-example/gh-events/0/payload/commits/0/author/email
...snip...
Files written to /tmp/jf-example/gh-events
```
