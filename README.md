# leaktk-jf

A utility/lib for taking JSON and converting it to a directory tree for scanning.

## WARNING - Very WIP

This project isn't "ready" yet. It still needs:

* Cleaning up
* Unittests
* Better security checks (potentially done but needs to be tested)
* Return correct exit codes on an error


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
