# leaktk-jf

A utility/lib for taking JSON and converting it to a directory tree for scanning.

## WARNING - Very WIP

This project isn't ready yet.

TODO:

* Unit tests
* Make arg parsing a little cleaner
* Better security checks (potentially done but needs to be tested)
* Return correct exit codes on an error

Nice to Have:

* Optionally provide your own regex with a `--valid-key-regex` flag that
defaults to `^[\w\-]+$`.
* Be able to run it in reverse to take a directory tree and turn it into JSON
  * A dir full of only numbers with no missing numbers in a range would be
    parsed as an array.


## Example Usage

```sh
rm -rf /tmp/jf-example
jf /tmp/jf-example < ./examples/file.json

grep . -R /tmp/jf-example
/tmp/jf-example/key:"value"
/tmp/jf-example/nested/even/0:"arrays"
/tmp/jf-example/nested/even/1/with:"interesting"
/tmp/jf-example/nested/even/2:true
/tmp/jf-example/nested/even/3:null
/tmp/jf-example/nested/even/4:0
/tmp/jf-example/nested/even/5:"stuff"
/tmp/jf-example/nested/keys:"and values"
/tmp/jf-example/
```
