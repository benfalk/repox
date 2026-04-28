# 🔭 `Project Overview`

> A fairly vanilla rust library setup. In case its not, or you just
> want a high level tour, here it is!

## 🗂️ `Directory Structure`

> ```text
> ── .github ------------------ CI workflow defintions
>    crates ------------------- directory for all workspace crates
>    ├── repox  --------------- main library crate
>        ├── doc -------------- mdbook and crate documentation source
>        ├── src -------------- crate source code
>        ├── theme ------------ asset dir for mdbook
>    ├── repox_derive --------- supporting proc macro crate
>        ├── src -------------- crate source code
>    ├── showtime ------------- example binary with sqlx and sqlite
>        ├── migrations ------- sqlx migrations
>        ├── src -------------- crate source code
> ```
