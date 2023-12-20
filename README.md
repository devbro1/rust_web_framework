# rust_web_framework


so far `cargo run` works fine as intended. however, `cargo test` will throw an error. I suspect test import process has something that I am not doing correctly.

```
   Compiling web_framework v0.1.0 (/root/source_code/rust_web_framework)
error[E0433]: failed to resolve: unresolved import
 --> src/routes/api.rs:8:12
  |
8 | use crate::app::controllers::*;
  |            ^^^
  |            |
  |            unresolved import
  |            help: a similar path exists: `web_framework::app`

error[E0433]: failed to resolve: use of undeclared crate or module `foo_controller`
  --> src/routes/api.rs:12:25
   |
12 |         .route("/", get(foo_controller::root))
   |                         ^^^^^^^^^^^^^^ use of undeclared crate or module `foo_controller`
   |
help: consider importing this module
   |
1  + use web_framework::app::controllers::foo_controller;
   |

error[E0433]: failed to resolve: use of undeclared crate or module `foo_controller`
  --> src/routes/api.rs:13:29
   |
13 |          .route("/foo", get(foo_controller::get_foo))
   |                             ^^^^^^^^^^^^^^ use of undeclared crate or module `foo_controller`
   |
help: consider importing this module
   |
1  + use web_framework::app::controllers::foo_controller;
   |

For more information about this error, try `rustc --explain E0433`.
error: could not compile `web_framework` (bin "aws_lambda" test) due to 3 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `web_framework` (bin "aws_lambda") due to 3 previous errors
10:24:18 root@af9ffe141539 [af9ffe141539:~/source_code/rust_web_framework@ master]
# 
```